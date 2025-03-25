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
        .then(none_of(":!").rewind())
        .then(none_of('\n').repeated());

    any::<&'src str, _>()
        .filter(|c: &char| c.is_whitespace())
        .ignored()
        .or(comment.ignored())
        .repeated()
        .ignored()
}

#[derive(Debug, Clone)]
pub struct NullableIdent {
    pub name: Span<String>,
    pub nullable: bool,
    pub inner_nullable: bool,
}

fn parse_nullable_ident<'src>()
-> impl Parser<'src, &'src str, Vec<NullableIdent>, extra::Err<Simple<'src, char>>> {
    let single_ident = space()
        .ignore_then(ident())
        .then(just('?').or_not())
        .then(just("[?]").or_not())
        .map(|((name, null), inner_null)| NullableIdent {
            name,
            nullable: null.is_some(),
            inner_nullable: inner_null.is_some(),
        })
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
            .map(|((name, fields), traits)| Self {
                name,
                fields,
                traits,
            })
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
}

impl Query {
    /// Escape sql string and pattern that are not bind
    fn sql_escaping<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Simple<'src, char>>> {
        // ::bind
        let cast = just("::").ignored();

        // ":bind"
        let constant = none_of('"')
            .repeated()
            .delimited_by(just('"'), just('"'))
            .ignored();

        // ':bind'
        let string = none_of("'")
            .repeated()
            .delimited_by(just("'"), just("'"))
            .ignored();

        // E'\':bind\''
        let c_style_string = just("\\'")
            .or(just("''"))
            .ignored()
            .or(none_of("'").ignored())
            .repeated()
            .delimited_by(just("e'").or(just("E'")), just("'"))
            .ignored();

        // $:bind$:bind$:bind$
        let dollar_tag = just("$").then(none_of("$").repeated()).then(just("$"));
        let dollar_quoted = none_of("$")
            .repeated()
            .delimited_by(dollar_tag, dollar_tag)
            .ignored();

        c_style_string
            .or(cast)
            .or(string)
            .or(constant)
            .or(dollar_quoted)
            // Non c_style_string e
            .or(one_of("eE").then(none_of("'").rewind()).ignored())
            // Non binding sql
            .or(none_of("\"':$eE").ignored())
            .repeated()
            .at_least(1)
            .ignored()
    }

    /// Parse all bind from an SQL query
    fn parse_bind<'src>()
    -> impl Parser<'src, &'src str, Vec<Span<String>>, extra::Err<Simple<'src, char>>> {
        just(':')
            .ignore_then(plain_ident())
            .separated_by(Self::sql_escaping())
            .allow_leading()
            .allow_trailing()
            .collect()
    }

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

                // In a real implementation, we would parse the SQL here
                // For this test example, we'll simulate a parse result
                let bind_params = Self::parse_bind()
                    .parse(&sql_str)
                    .into_output()
                    .unwrap_or_default();

                // Remove duplicates
                let dedup_params: Vec<_> = bind_params
                    .iter()
                    .enumerate()
                    .rev()
                    .filter(|(i, u)| !bind_params[..*i].contains(u))
                    .map(|(_, u)| u.clone())
                    .rev()
                    .collect();

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
            .then(Self::parse_comments())
            .then(Self::parse_sql_query())
            .map(
                |(((name, param, row), comments), (sql_str, sql_span, bind_params))| Self {
                    name,
                    param,
                    comments,
                    row,
                    sql_span,
                    sql_str,
                    bind_params,
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
    ) -> (&'a [NullableIdent], Vec<String>, Span<String>) {
        if let Some(named) = &self.name {
            let registered = registered_structs.iter().find(|it| it.name == *named);

            (
                self.idents.as_ref().map_or_else(
                    || registered.map(|it| it.fields.as_slice()).unwrap_or(&[]),
                    Vec::as_slice,
                ),
                registered.map(|it| it.traits.clone()).unwrap_or_default(),
                named.clone(),
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
