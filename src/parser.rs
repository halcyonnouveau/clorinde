use std::{fmt::Display, ops::Range};

use chumsky::prelude::*;
use error::Error;
use heck::ToUpperCamelCase;
use miette::SourceSpan;

use crate::read_queries::ModuleInfo;

/// This data structure holds a value and the context in which it was parsed.
/// This context is used for error reporting.
#[derive(Debug, Clone)]
pub struct Span<T> {
    pub(crate) span: SourceSpan,
    pub(crate) value: T,
}

impl<T: std::hash::Hash> std::hash::Hash for Span<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: PartialEq> PartialEq<Self> for Span<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Display> Display for Span<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: Eq> Eq for Span<T> {}

impl<T: PartialOrd + PartialEq> PartialOrd<Self> for Span<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord> Ord for Span<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> Span<T> {
    pub(crate) fn map<U>(&self, f: impl Fn(&T) -> U) -> Span<U> {
        Span {
            value: f(&self.value),
            span: self.span,
        }
    }
}

fn plain_ident<'src>() -> impl Parser<'src, &'src str, Span<String>, extra::Err<Simple<'src, char>>>
{
    any::<&'src str, _>()
        .filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map_with(|value, e| {
            let span: SimpleSpan = e.span();
            let range: Range<usize> = span.start()..span.end();

            Span {
                value,
                span: range.into(),
            }
        })
}

fn quoted_ident<'src>() -> impl Parser<'src, &'src str, Span<String>, extra::Err<Simple<'src, char>>>
{
    none_of('"')
        .repeated()
        .at_least(1)
        .collect::<String>()
        .delimited_by(just('"'), just('"'))
        .map_with(|value, e| {
            let span: SimpleSpan = e.span();
            let range: Range<usize> = span.start()..span.end();

            Span {
                value,
                span: range.into(),
            }
        })
}

fn ident<'src>() -> impl Parser<'src, &'src str, Span<String>, extra::Err<Simple<'src, char>>> {
    plain_ident().or(quoted_ident())
}

fn ln<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Simple<'src, char>>> {
    just("\n").or(just("\n\r")).ignored()
}

fn space<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Simple<'src, char>>> {
    any::<&'src str, _>()
        .filter(|c: &char| c.is_whitespace() && *c != '\n')
        .repeated()
        .ignored()
}

fn blank<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Simple<'src, char>>> {
    // We want to escape valid SQL comment beginning with -- while not escaping our syntax --: or --!
    let comment = just("--")
        .then(none_of(":!#").rewind())
        .then(none_of('\n').repeated());

    any::<&'src str, _>()
        .filter(|c: &char| c.is_whitespace())
        .ignored()
        .or(comment.ignored())
        .repeated()
        .ignored()
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldSegment {
    pub name: Span<String>,
    pub nullable: bool,
    pub is_array: bool,
}

#[derive(Debug, Clone)]
pub struct NullableIdent {
    pub name: Span<String>,
    pub nullable: bool,
    pub inner_nullable: bool,
    pub nested_fields: Vec<FieldSegment>,
}

impl NullableIdent {
    /// Get all field nullability specifications for this identifier
    pub fn get_field_nullability(&self) -> impl Iterator<Item = (&str, bool)> + '_ {
        self.nested_fields
            .iter()
            .map(|segment| (segment.name.value.as_str(), segment.nullable))
    }
}

fn parse_field_segment<'src>()
-> impl Parser<'src, &'src str, FieldSegment, extra::Err<Simple<'src, char>>> {
    ident()
        .then(just('?').or_not())
        .map(|(name, nullable)| FieldSegment {
            name,
            nullable: nullable.is_some(),
            is_array: false,
        })
}

fn parse_nullable_ident<'src>()
-> impl Parser<'src, &'src str, Vec<NullableIdent>, extra::Err<Simple<'src, char>>> {
    let single_ident = space()
        .ignore_then(
            ident()
                .then(just('?').or_not())
                .then(just("[?]").or_not())
                .then(
                    // Parse nested field paths like .field or [].field
                    choice((
                        // Handle [].field? syntax for array element field access
                        just("[]")
                            .ignore_then(just('.'))
                            .ignore_then(parse_field_segment())
                            .map(|segment| (segment, true)), // true indicates this is an array access
                        // Handle .field? syntax for direct field access
                        just('.')
                            .ignore_then(parse_field_segment())
                            .map(|segment| (segment, false)), // false indicates this is direct access
                    ))
                    .repeated()
                    .collect::<Vec<_>>(),
                )
                .map(|(((name, nullable), array_nullable), nested_accesses)| {
                    let mut nested_fields = Vec::new();

                    for (mut segment, is_array_access) in nested_accesses {
                        segment.is_array = is_array_access;
                        nested_fields.push(segment);
                    }

                    NullableIdent {
                        name,
                        nullable: nullable.is_some(),
                        inner_nullable: array_nullable.is_some(),
                        nested_fields,
                    }
                }),
        )
        .then_ignore(space());

    single_ident
        .separated_by(just(','))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(just('('), just(')'))
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub name: Span<String>,
    pub fields: Vec<NullableIdent>,
    pub traits: Vec<String>,
    pub attributes: Vec<String>,
    pub attributes_borrowed: Vec<String>,
}

impl TypeAnnotation {
    fn path_ident<'src>()
    -> impl Parser<'src, &'src str, Span<String>, extra::Err<Simple<'src, char>>> {
        let path_segment = any::<&'src str, _>()
            .filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
            .repeated()
            .at_least(1)
            .collect::<String>();

        path_segment
            .separated_by(just("::"))
            .at_least(1)
            .collect::<Vec<_>>()
            .map(|segments| segments.join("::"))
            .map_with(|value, e| {
                let span: SimpleSpan = e.span();
                let range: Range<usize> = span.start()..span.end();

                Span {
                    value,
                    span: range.into(),
                }
            })
    }

    fn parse_attributes<'src>()
    -> impl Parser<'src, &'src str, Vec<String>, extra::Err<Simple<'src, char>>> {
        just("--#")
            .ignore_then(space())
            .ignore_then(none_of('\n').repeated().collect::<String>())
            .map(|s| s.trim().to_string())
            .then_ignore(ln())
            .repeated()
            .collect()
    }

    fn parse_attributes_borrowed<'src>()
    -> impl Parser<'src, &'src str, Vec<String>, extra::Err<Simple<'src, char>>> {
        just("--&")
            .ignore_then(space())
            .ignore_then(none_of('\n').repeated().collect::<String>())
            .map(|s| s.trim().to_string())
            .then_ignore(ln())
            .repeated()
            .collect()
    }

    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<Simple<'src, char>>> {
        let trait_parser = Self::path_ident()
            .map(|s: Span<String>| s.value)
            .separated_by(just(',').padded())
            .collect::<Vec<_>>();

        just("--:")
            .ignore_then(space())
            .ignore_then(ident())
            .then_ignore(space())
            .then(parse_nullable_ident())
            .then_ignore(space())
            .then(
                just(':')
                    .ignore_then(space())
                    .ignore_then(trait_parser)
                    .or_not()
                    .map(|opt| opt.unwrap_or_default()),
            )
            .then_ignore(space())
            .then_ignore(ln())
            .then(Self::parse_attributes())
            .then(Self::parse_attributes_borrowed())
            .map(
                |((((name, fields), traits), attributes), attributes_borrowed)| Self {
                    name,
                    fields,
                    traits,
                    attributes,
                    attributes_borrowed,
                },
            )
    }
}

#[derive(Debug)]
pub(crate) struct Query {
    pub(crate) name: Span<String>,
    pub(crate) param: QueryDataStruct,
    pub(crate) comments: Vec<String>,
    pub(crate) row: QueryDataStruct,
    pub(crate) sql_span: SourceSpan,
    pub(crate) sql_str: String,
    pub(crate) bind_params: Vec<Span<String>>,
    pub(crate) attributes: Vec<String>,
}

impl Query {
    /// Remove all comments from a query
    fn clean_sql_comments(sql: &str) -> String {
        let mut result = String::new();
        let mut chars = sql.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                // Preserve everything in quotes
                '$' | '\'' | '"' => {
                    let mut content = String::from(c);
                    let end_marker = if c == '$' {
                        // For dollar quotes, read until $ and use that as tag
                        for x in chars.by_ref() {
                            content.push(x);
                            if x == '$' {
                                break;
                            }
                        }
                        content.clone()
                    } else {
                        content.clone()
                    };

                    while let Some(x) = chars.next() {
                        content.push(x);
                        if x == '\\' && c != '$' {
                            // Handle escapes in regular strings
                            if let Some(escaped) = chars.next() {
                                content.push(escaped);
                            }
                        } else if content.ends_with(&end_marker) {
                            break;
                        }
                    }
                    result.push_str(&content);
                }
                // Remove comments
                '-' if chars.peek() == Some(&'-') => {
                    chars.next();
                    while let Some(&x) = chars.peek() {
                        if x == '\n' {
                            break;
                        }
                        chars.next();
                    }
                }
                _ => result.push(c),
            }
        }
        result
    }

    /// Extract bind parameters from SQL - context aware, more robust than using combinators
    fn extract_bind_params(sql: &str) -> Vec<Span<String>> {
        let mut params = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = sql.chars().collect();

        struct ParseContext {
            in_string: bool,
            in_escape_string: bool,
            in_identifier: bool,
            in_comment: bool,
            in_dollar_quote: bool,
            dollar_tag: String,
        }

        let mut ctx = ParseContext {
            in_string: false,
            in_escape_string: false,
            in_identifier: false,
            in_comment: false,
            in_dollar_quote: false,
            dollar_tag: String::new(),
        };

        let in_context = |ctx: &ParseContext| -> bool {
            ctx.in_string || ctx.in_identifier || ctx.in_comment || ctx.in_dollar_quote
        };

        while i < chars.len() {
            let c = chars[i];

            match c {
                // Handle string literals
                '\'' => {
                    if !ctx.in_comment && !ctx.in_dollar_quote {
                        if ctx.in_string {
                            // Handle escaped quotes
                            if i + 1 < chars.len() && chars[i + 1] == '\'' {
                                i += 1; // Skip the escaped quote
                            } else {
                                ctx.in_string = false;
                                ctx.in_escape_string = false;
                            }
                        } else {
                            ctx.in_string = true;
                        }
                    }
                }

                // Handle escape strings (E'...' or e'...')
                'e' | 'E' => {
                    if !in_context(&ctx) && i + 1 < chars.len() && chars[i + 1] == '\'' {
                        i += 1; // Skip the quote
                        ctx.in_string = true;
                        ctx.in_escape_string = true;
                    }
                }

                // Handle backslash escapes in escape strings
                '\\' => {
                    if ctx.in_escape_string && i + 1 < chars.len() {
                        i += 1; // Skip the escaped character
                    }
                }

                // Handle quoted identifiers
                '"' => {
                    if !ctx.in_string && !ctx.in_comment && !ctx.in_dollar_quote {
                        if ctx.in_identifier {
                            // Handle escaped quotes
                            if i + 1 < chars.len() && chars[i + 1] == '"' {
                                i += 1; // Skip the escaped quote
                            } else {
                                ctx.in_identifier = false;
                            }
                        } else {
                            ctx.in_identifier = true;
                        }
                    }
                }

                // Handle line comments
                '-' => {
                    if !in_context(&ctx) && i + 1 < chars.len() && chars[i + 1] == '-' {
                        i += 1; // Skip the second dash
                        ctx.in_comment = true;
                    }
                }

                // End of line comment
                '\n' => {
                    if ctx.in_comment {
                        ctx.in_comment = false;
                    }
                }

                // Handle dollar-quoted strings
                '$' => {
                    if !in_context(&ctx) {
                        // Start of dollar quote
                        let tag_start = i + 1;
                        let mut tag_end = tag_start;

                        // Find end of tag
                        while tag_end < chars.len()
                            && (chars[tag_end].is_alphanumeric() || chars[tag_end] == '_')
                        {
                            tag_end += 1;
                        }

                        // Check for closing $
                        if tag_end < chars.len() && chars[tag_end] == '$' {
                            ctx.dollar_tag = sql[tag_start..tag_end].to_string();
                            ctx.in_dollar_quote = true;
                            i = tag_end; // Position at the closing $
                        }
                    } else if ctx.in_dollar_quote {
                        // Potential end of dollar quote
                        let tag_length = ctx.dollar_tag.len();

                        if i + 1 + tag_length < chars.len() {
                            let potential_end = &sql[i + 1..i + 1 + tag_length];

                            if potential_end == ctx.dollar_tag
                                && i + 1 + tag_length < chars.len()
                                && chars[i + 1 + tag_length] == '$'
                            {
                                // Found matching end tag
                                ctx.in_dollar_quote = false;
                                i += tag_length + 1; // Skip to the end of the closing tag
                                ctx.dollar_tag.clear();
                            }
                        }
                    }
                }

                // Handle bind parameters
                ':' => {
                    if !in_context(&ctx) {
                        // Skip type cast (::)
                        if i + 1 < chars.len() && chars[i + 1] == ':' {
                            i += 1;
                        } else {
                            // Extract parameter name
                            let param_start = i + 1;
                            let mut param_end = param_start;

                            while param_end < chars.len()
                                && (chars[param_end].is_alphanumeric() || chars[param_end] == '_')
                            {
                                param_end += 1;
                            }

                            if param_end > param_start {
                                let param = sql[param_start..param_end].to_string();
                                params.push(Span {
                                    value: param,
                                    span: (param_start..param_end).into(),
                                });
                                i = param_end - 1; // Position at the last character of the parameter
                            }
                        }
                    }
                }

                _ => {}
            }

            i += 1;
        }

        params
    }

    /// Remove duplicates from bind parameters while preserving order of first occurrence
    fn dedup_bind_params(params: Vec<Span<String>>) -> Vec<Span<String>> {
        let mut seen = std::collections::HashSet::new();
        params
            .into_iter()
            .filter(|param| seen.insert(param.value.clone()))
            .collect()
    }

    /// Parse sql query, normalizing named parameters
    fn parse_sql_query<'src>() -> impl Parser<
        'src,
        &'src str,
        (String, SourceSpan, Vec<Span<String>>),
        extra::Err<Simple<'src, char>>,
    > {
        // TODO(bug): Using none_of(";") breaks on the first semicolon it encounters, even if that semicolon is inside:
        // - String literals: 'text with ; in it'
        // - Dollar-quoted strings: $$text with ; in it$$
        // - Comments: -- comment with ; in it
        // We need proper SQL token awareness to know if a semicolon is part of these constructs or if it's the actual query terminator.
        none_of(";")
            .repeated()
            .collect::<String>()
            .then_ignore(just(";"))
            .map_with(move |sql_str: String, e| {
                let span: SimpleSpan = e.span();
                let range: Range<usize> = span.start()..span.end();
                let source_span: SourceSpan = range.into();

                let mut sql_str = Self::clean_sql_comments(&sql_str)
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .collect::<Vec<_>>()
                    .join("\n");

                let bind_params = Self::extract_bind_params(&sql_str);
                let dedup_params = Self::dedup_bind_params(bind_params.clone());

                for bind_param in bind_params.iter().rev() {
                    let index = dedup_params.iter().position(|bp| bp == bind_param).unwrap();
                    let start = bind_param.span.offset() - 1;
                    let end = start + bind_param.span.len();
                    sql_str.replace_range(start..=end, &format!("${}", index + 1));
                }

                (sql_str, source_span, dedup_params)
            })
    }

    fn parse_comments<'src>()
    -> impl Parser<'src, &'src str, Vec<String>, extra::Err<Simple<'src, char>>> {
        just("---")
            .ignore_then(
                none_of('\n')
                    .repeated()
                    .collect::<String>()
                    .map(|s| s.trim().to_string()),
            )
            .then_ignore(ln())
            .repeated()
            .collect()
    }

    fn parse_query_annotation<'src>() -> impl Parser<
        'src,
        &'src str,
        (Span<String>, QueryDataStruct, QueryDataStruct),
        extra::Err<Simple<'src, char>>,
    > {
        just("--!")
            .ignore_then(space())
            .ignore_then(plain_ident())
            .then_ignore(space())
            .then(QueryDataStruct::parser())
            .then_ignore(space())
            .then(
                just(":")
                    .ignore_then(space())
                    .ignore_then(QueryDataStruct::parser())
                    .or_not(),
            )
            .map(|((name, param), row)| (name, param, row.unwrap_or_default()))
    }

    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<Simple<'src, char>>> {
        Self::parse_query_annotation()
            .then_ignore(space())
            .then_ignore(ln())
            .then(TypeAnnotation::parse_attributes())
            .then(Self::parse_comments())
            .then(Self::parse_sql_query())
            .map(
                |(
                    (((name, param, row), attributes), comments),
                    (sql_str, sql_span, bind_params),
                )| Self {
                    name,
                    param,
                    comments,
                    row,
                    sql_span,
                    sql_str,
                    bind_params,
                    attributes,
                },
            )
    }
}

#[derive(Debug)]
pub(crate) struct QueryDataStruct {
    pub span: SourceSpan,
    pub name: Option<Span<String>>,
    pub idents: Option<Vec<NullableIdent>>,
}

impl QueryDataStruct {
    pub fn is_implicit(&self) -> bool {
        self.name.is_none()
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.idents.is_none()
    }

    pub fn inlined(&self) -> bool {
        self.idents.is_some() && self.name.is_some()
    }

    pub(crate) fn name_and_fields<'a>(
        &'a self,
        registered_structs: &'a [TypeAnnotation],
        query_name: &Span<String>,
        name_suffix: Option<&str>,
    ) -> (
        &'a [NullableIdent],
        Vec<String>,
        Span<String>,
        Vec<String>,
        Vec<String>,
    ) {
        if let Some(named) = &self.name {
            let registered = registered_structs.iter().find(|it| it.name == *named);
            (
                self.idents.as_ref().map_or_else(
                    || registered.map(|it| it.fields.as_slice()).unwrap_or(&[]),
                    Vec::as_slice,
                ),
                registered.map(|it| it.traits.clone()).unwrap_or_default(),
                named.clone(),
                registered
                    .map(|it| it.attributes.clone())
                    .unwrap_or_default(),
                registered
                    .map(|it| it.attributes_borrowed.clone())
                    .unwrap_or_default(),
            )
        } else {
            (
                self.idents.as_ref().map_or(&[], Vec::as_slice),
                vec![],
                query_name.map(|x| {
                    format!(
                        "{}{}",
                        x.to_owned().to_upper_camel_case(),
                        name_suffix.unwrap_or_default()
                    )
                }),
                vec![],
                vec![],
            )
        }
    }

    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<Simple<'src, char>>> {
        plain_ident()
            .or_not()
            .then_ignore(space())
            .then(parse_nullable_ident().or_not())
            .map_with(|(name, idents), e| {
                let span: SimpleSpan = e.span();
                let range: Range<usize> = span.start()..span.end();

                Self {
                    span: range.into(),
                    name,
                    idents,
                }
            })
    }
}

impl Default for QueryDataStruct {
    fn default() -> Self {
        Self {
            span: (0..0).into(),
            name: None,
            idents: None,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
enum Statement {
    Type(TypeAnnotation),
    Query(Query),
}

#[derive(Debug)]
pub(crate) struct Module {
    pub(crate) info: ModuleInfo,
    pub(crate) types: Vec<TypeAnnotation>,
    pub(crate) queries: Vec<Query>,
}

pub(crate) fn parse_query_module(info: ModuleInfo) -> Result<Module, Error> {
    let result = TypeAnnotation::parser()
        .map(Statement::Type)
        .or(Query::parser().map(Statement::Query))
        .separated_by(blank())
        .allow_leading()
        .allow_trailing()
        .collect::<Vec<_>>()
        .then_ignore(end())
        .parse(&info.content);

    match result.into_result() {
        Ok(statements) => {
            let mut types = Vec::new();
            let mut queries = Vec::new();
            for item in statements {
                match item {
                    Statement::Type(it) => types.push(it),
                    Statement::Query(it) => queries.push(it),
                }
            }
            Ok(Module {
                info,
                types,
                queries,
            })
        }
        Err(e) => {
            let span = e[0].span();
            let range: Range<usize> = span.start()..span.end();
            let source_span: SourceSpan = range.into();

            Err(Error {
                src: (&info).into(),
                err_span: source_span,
                help: e[0].to_string().replace("\n", "\\n"),
            })
        }
    }
}

pub(crate) mod error {
    use miette::{Diagnostic, NamedSource, SourceSpan};
    use std::sync::Arc;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError, Diagnostic)]
    #[error("Couldn't parse queries")]
    pub struct Error {
        #[source_code]
        pub src: NamedSource<Arc<String>>,

        #[help]
        pub help: String,

        #[label("unexpected token")]
        pub err_span: SourceSpan,
    }
}
