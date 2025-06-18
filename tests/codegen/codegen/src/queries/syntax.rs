// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct ImplicitCompactParams<T1: crate::StringSql> {
    pub name: Option<T1>,
    pub price: Option<f64>,
}
#[derive(Debug)]
pub struct ImplicitSpacedParams<T1: crate::StringSql> {
    pub name: Option<T1>,
    pub price: Option<f64>,
}
#[derive(Debug)]
pub struct Params<T1: crate::StringSql> {
    pub name: T1,
    pub price: f64,
}
#[derive(Debug)]
pub struct ParamsSpace<T1: crate::StringSql> {
    pub name: T1,
    pub price: f64,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySqlParams {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql1Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql2Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql3Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql4Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql6Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql7Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql8Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql9Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql10Params {
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Row {
    pub id: i32,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct RowSpace {
    pub id: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Typeof {
    pub trick_y: String,
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
pub struct TypeofBorrowed<'a> {
    pub trick_y: &'a str,
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
impl<'a> From<TypeofBorrowed<'a>> for Typeof {
    fn from(
        TypeofBorrowed {
            trick_y,
            r#async,
            r#enum,
        }: TypeofBorrowed<'a>,
    ) -> Self {
        Self {
            trick_y: trick_y.into(),
            r#async,
            r#enum,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SelectComment {
    pub trick_y: String,
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
pub struct SelectCommentBorrowed<'a> {
    pub trick_y: &'a str,
    pub r#async: crate::types::SyntaxComposite,
    pub r#enum: crate::types::SyntaxEnum,
}
impl<'a> From<SelectCommentBorrowed<'a>> for SelectComment {
    fn from(
        SelectCommentBorrowed {
            trick_y,
            r#async,
            r#enum,
        }: SelectCommentBorrowed<'a>,
    ) -> Self {
        Self {
            trick_y: trick_y.into(),
            r#async,
            r#enum,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SelectInlineComment {
    pub c1: String,
    pub c2: String,
    pub c3: String,
    pub c4: String,
    pub c5: String,
}
pub struct SelectInlineCommentBorrowed<'a> {
    pub c1: &'a str,
    pub c2: &'a str,
    pub c3: &'a str,
    pub c4: &'a str,
    pub c5: &'a str,
}
impl<'a> From<SelectInlineCommentBorrowed<'a>> for SelectInlineComment {
    fn from(
        SelectInlineCommentBorrowed { c1, c2, c3, c4, c5 }: SelectInlineCommentBorrowed<'a>,
    ) -> Self {
        Self {
            c1: c1.into(),
            c2: c2.into(),
            c3: c3.into(),
            c4: c4.into(),
            c5: c5.into(),
        }
    }
}
pub mod sync {
    use postgres::{GenericClient, fallible_iterator::FallibleIterator};
    pub struct CloneCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::sync::Stmt,
        extractor:
            fn(&postgres::Row) -> Result<crate::types::CloneCompositeBorrowed, postgres::Error>,
        mapper: fn(crate::types::CloneCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> CloneCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::CloneCompositeBorrowed) -> R,
        ) -> CloneCompositeQuery<'c, 'a, 's, C, R, N> {
            CloneCompositeQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                });
            Ok(it)
        }
    }
    pub struct Optioni32Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> Result<Option<i32>, postgres::Error>,
        mapper: fn(Option<i32>) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> Optioni32Query<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(Option<i32>) -> R) -> Optioni32Query<'c, 'a, 's, C, R, N> {
            Optioni32Query {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                });
            Ok(it)
        }
    }
    pub struct RowQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> Result<super::Row, postgres::Error>,
        mapper: fn(super::Row) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> RowQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Row) -> R) -> RowQuery<'c, 'a, 's, C, R, N> {
            RowQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                });
            Ok(it)
        }
    }
    pub struct RowSpaceQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> Result<super::RowSpace, postgres::Error>,
        mapper: fn(super::RowSpace) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> RowSpaceQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::RowSpace) -> R,
        ) -> RowSpaceQuery<'c, 'a, 's, C, R, N> {
            RowSpaceQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                });
            Ok(it)
        }
    }
    pub struct TypeofQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> Result<super::TypeofBorrowed, postgres::Error>,
        mapper: fn(super::TypeofBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> TypeofQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::TypeofBorrowed) -> R,
        ) -> TypeofQuery<'c, 'a, 's, C, R, N> {
            TypeofQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                });
            Ok(it)
        }
    }
    pub struct SelectCommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> Result<super::SelectCommentBorrowed, postgres::Error>,
        mapper: fn(super::SelectCommentBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectCommentQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectCommentBorrowed) -> R,
        ) -> SelectCommentQuery<'c, 'a, 's, C, R, N> {
            SelectCommentQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                });
            Ok(it)
        }
    }
    pub struct SelectInlineCommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::sync::Stmt,
        extractor:
            fn(&postgres::Row) -> Result<super::SelectInlineCommentBorrowed, postgres::Error>,
        mapper: fn(super::SelectInlineCommentBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectInlineCommentQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectInlineCommentBorrowed) -> R,
        ) -> SelectInlineCommentQuery<'c, 'a, 's, C, R, N> {
            SelectInlineCommentQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            let row = self.client.query_one(stmt, &self.params)?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let stmt = self.stmt.prepare(self.client)?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'c, postgres::Error>
        {
            let stmt = self.stmt.prepare(self.client)?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))?
                .iterator()
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                });
            Ok(it)
        }
    }
    pub fn select_compact() -> SelectCompactStmt {
        SelectCompactStmt(crate::client::sync::Stmt::new("SELECT * FROM clone"))
    }
    pub struct SelectCompactStmt(crate::client::sync::Stmt);
    impl SelectCompactStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
        ) -> CloneCompositeQuery<'c, 'a, 's, C, crate::types::CloneComposite, 0> {
            CloneCompositeQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn select_spaced() -> SelectSpacedStmt {
        SelectSpacedStmt(crate::client::sync::Stmt::new("SELECT * FROM clone"))
    }
    pub struct SelectSpacedStmt(crate::client::sync::Stmt);
    impl SelectSpacedStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
        ) -> CloneCompositeQuery<'c, 'a, 's, C, crate::types::CloneComposite, 0> {
            CloneCompositeQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn implicit_compact() -> ImplicitCompactStmt {
        ImplicitCompactStmt(crate::client::sync::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct ImplicitCompactStmt(crate::client::sync::Stmt);
    impl ImplicitCompactStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
            &'s mut self,
            client: &'c mut C,
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it,
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::ImplicitCompactParams<T1>,
            Optioni32Query<'c, 'a, 's, C, Option<i32>, 2>,
            C,
        > for ImplicitCompactStmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::ImplicitCompactParams<T1>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn implicit_spaced() -> ImplicitSpacedStmt {
        ImplicitSpacedStmt(crate::client::sync::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct ImplicitSpacedStmt(crate::client::sync::Stmt);
    impl ImplicitSpacedStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
            &'s mut self,
            client: &'c mut C,
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it,
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::ImplicitSpacedParams<T1>,
            Optioni32Query<'c, 'a, 's, C, Option<i32>, 2>,
            C,
        > for ImplicitSpacedStmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::ImplicitSpacedParams<T1>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn named_compact() -> NamedCompactStmt {
        NamedCompactStmt(crate::client::sync::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct NamedCompactStmt(crate::client::sync::Stmt);
    impl NamedCompactStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
            &'s mut self,
            client: &'c mut C,
            name: &'a T1,
            price: &'a f64,
        ) -> RowQuery<'c, 'a, 's, C, super::Row, 2> {
            RowQuery {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row: &postgres::Row| -> Result<super::Row, postgres::Error> {
                    Ok(super::Row {
                        id: row.try_get(0)?,
                    })
                },
                mapper: |it| super::Row::from(it),
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::Params<T1>,
            RowQuery<'c, 'a, 's, C, super::Row, 2>,
            C,
        > for NamedCompactStmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::Params<T1>,
        ) -> RowQuery<'c, 'a, 's, C, super::Row, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn named_spaced() -> NamedSpacedStmt {
        NamedSpacedStmt(crate::client::sync::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct NamedSpacedStmt(crate::client::sync::Stmt);
    impl NamedSpacedStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
            &'s mut self,
            client: &'c mut C,
            name: &'a T1,
            price: &'a f64,
        ) -> RowSpaceQuery<'c, 'a, 's, C, super::RowSpace, 2> {
            RowSpaceQuery {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row: &postgres::Row| -> Result<super::RowSpace, postgres::Error> {
                    Ok(super::RowSpace {
                        id: row.try_get(0)?,
                    })
                },
                mapper: |it| super::RowSpace::from(it),
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::ParamsSpace<T1>,
            RowSpaceQuery<'c, 'a, 's, C, super::RowSpace, 2>,
            C,
        > for NamedSpacedStmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::ParamsSpace<T1>,
        ) -> RowSpaceQuery<'c, 'a, 's, C, super::RowSpace, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn tricky_sql() -> TrickySqlStmt {
        TrickySqlStmt(crate::client::sync::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a bind_param\\', $1, $2)",
        ))
    }
    pub struct TrickySqlStmt(crate::client::sync::Stmt);
    impl TrickySqlStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySqlParams,
            Result<u64, postgres::Error>,
            C,
        > for TrickySqlStmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::TrickySqlParams,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql1() -> TrickySql1Stmt {
        TrickySql1Stmt(crate::client::sync::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a :bind_param', $1, $2)",
        ))
    }
    pub struct TrickySql1Stmt(crate::client::sync::Stmt);
    impl TrickySql1Stmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql1Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql1Stmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::TrickySql1Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql2() -> TrickySql2Stmt {
        TrickySql2Stmt(crate::client::sync::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a '':bind_param''', $1, $2)",
        ))
    }
    pub struct TrickySql2Stmt(crate::client::sync::Stmt);
    impl TrickySql2Stmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql2Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql2Stmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::TrickySql2Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql3() -> TrickySql3Stmt {
        TrickySql3Stmt(crate::client::sync::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum)  VALUES ($$this is not a :bind_param$$, $1, $2)",
        ))
    }
    pub struct TrickySql3Stmt(crate::client::sync::Stmt);
    impl TrickySql3Stmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql3Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql3Stmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::TrickySql3Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql4() -> TrickySql4Stmt {
        TrickySql4Stmt(crate::client::sync::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($tag$this is not a :bind_param$tag$, $1, $2)",
        ))
    }
    pub struct TrickySql4Stmt(crate::client::sync::Stmt);
    impl TrickySql4Stmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql4Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql4Stmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::TrickySql4Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql6() -> TrickySql6Stmt {
        TrickySql6Stmt(crate::client::sync::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is not a '':bind_param''', $1, $2)",
        ))
    }
    pub struct TrickySql6Stmt(crate::client::sync::Stmt);
    impl TrickySql6Stmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql6Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql6Stmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::TrickySql6Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql7() -> TrickySql7Stmt {
        TrickySql7Stmt(crate::client::sync::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is not a \\':bind_param\\'', $1, $2)",
        ))
    }
    pub struct TrickySql7Stmt(crate::client::sync::Stmt);
    impl TrickySql7Stmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql7Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql7Stmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::TrickySql7Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql8() -> TrickySql8Stmt {
        TrickySql8Stmt(crate::client::sync::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is ''not'' a \\':bind_param\\'', $1, $2)",
        ))
    }
    pub struct TrickySql8Stmt(crate::client::sync::Stmt);
    impl TrickySql8Stmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql8Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql8Stmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::TrickySql8Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql9() -> TrickySql9Stmt {
        TrickySql9Stmt(crate::client::sync::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is \\'not\\' a \\':bind_param\\'', $1, $2)",
        ))
    }
    pub struct TrickySql9Stmt(crate::client::sync::Stmt);
    impl TrickySql9Stmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql9Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql9Stmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::TrickySql9Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql10() -> TrickySql10Stmt {
        TrickySql10Stmt(crate::client::sync::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is just a cast'::text, $1, $2)",
        ))
    }
    pub struct TrickySql10Stmt(crate::client::sync::Stmt);
    impl TrickySql10Stmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[r#async, r#enum])
        }
    }
    impl<'c, 'a, 's, C: GenericClient>
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::TrickySql10Params,
            Result<u64, postgres::Error>,
            C,
        > for TrickySql10Stmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::TrickySql10Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn r#typeof() -> RTypeofStmt {
        RTypeofStmt(crate::client::sync::Stmt::new("SELECT * FROM syntax"))
    }
    pub struct RTypeofStmt(crate::client::sync::Stmt);
    impl RTypeofStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
        ) -> TypeofQuery<'c, 'a, 's, C, super::Typeof, 0> {
            TypeofQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row: &postgres::Row| -> Result<super::TypeofBorrowed, postgres::Error> {
                    Ok(super::TypeofBorrowed {
                        trick_y: row.try_get(0)?,
                        r#async: row.try_get(1)?,
                        r#enum: row.try_get(2)?,
                    })
                },
                mapper: |it| super::Typeof::from(it),
            }
        }
    }
    /// Multi line
    ///
    /// Doc string comment
    pub fn select_comment() -> SelectCommentStmt {
        SelectCommentStmt(crate::client::sync::Stmt::new("SELECT * FROM syntax"))
    }
    pub struct SelectCommentStmt(crate::client::sync::Stmt);
    impl SelectCommentStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
        ) -> SelectCommentQuery<'c, 'a, 's, C, super::SelectComment, 0> {
            SelectCommentQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor:
                    |row: &postgres::Row| -> Result<super::SelectCommentBorrowed, postgres::Error> {
                        Ok(super::SelectCommentBorrowed {
                            trick_y: row.try_get(0)?,
                            r#async: row.try_get(1)?,
                            r#enum: row.try_get(2)?,
                        })
                    },
                mapper: |it| super::SelectComment::from(it),
            }
        }
    }
    pub fn select_inline_comment() -> SelectInlineCommentStmt {
        SelectInlineCommentStmt(crate::client::sync::Stmt::new(
            "SELECT '-- dont remove this\\n' as c1, $$-- or this$$ as c2, E'-- escape string here' as c3, e'-- another escape string' as c4, $tag$-- dollar quoted here$tag$ as c5",
        ))
    }
    pub struct SelectInlineCommentStmt(crate::client::sync::Stmt);
    impl SelectInlineCommentStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
        ) -> SelectInlineCommentQuery<'c, 'a, 's, C, super::SelectInlineComment, 0> {
            SelectInlineCommentQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |
                    row: &postgres::Row,
                | -> Result<super::SelectInlineCommentBorrowed, postgres::Error> {
                    Ok(super::SelectInlineCommentBorrowed {
                        c1: row.try_get(0)?,
                        c2: row.try_get(1)?,
                        c3: row.try_get(2)?,
                        c4: row.try_get(3)?,
                        c5: row.try_get(4)?,
                    })
                },
                mapper: |it| super::SelectInlineComment::from(it),
            }
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct CloneCompositeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::async_::Stmt,
        extractor: fn(
            &tokio_postgres::Row,
        )
            -> Result<crate::types::CloneCompositeBorrowed, tokio_postgres::Error>,
        mapper: fn(crate::types::CloneCompositeBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> CloneCompositeQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(crate::types::CloneCompositeBorrowed) -> R,
        ) -> CloneCompositeQuery<'c, 'a, 's, C, R, N> {
            CloneCompositeQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(it)
        }
    }
    pub struct Optioni32Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> Result<Option<i32>, tokio_postgres::Error>,
        mapper: fn(Option<i32>) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> Optioni32Query<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(Option<i32>) -> R) -> Optioni32Query<'c, 'a, 's, C, R, N> {
            Optioni32Query {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(it)
        }
    }
    pub struct RowQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> Result<super::Row, tokio_postgres::Error>,
        mapper: fn(super::Row) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> RowQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Row) -> R) -> RowQuery<'c, 'a, 's, C, R, N> {
            RowQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(it)
        }
    }
    pub struct RowSpaceQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> Result<super::RowSpace, tokio_postgres::Error>,
        mapper: fn(super::RowSpace) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> RowSpaceQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::RowSpace) -> R,
        ) -> RowSpaceQuery<'c, 'a, 's, C, R, N> {
            RowSpaceQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(it)
        }
    }
    pub struct TypeofQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> Result<super::TypeofBorrowed, tokio_postgres::Error>,
        mapper: fn(super::TypeofBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> TypeofQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::TypeofBorrowed) -> R,
        ) -> TypeofQuery<'c, 'a, 's, C, R, N> {
            TypeofQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(it)
        }
    }
    pub struct SelectCommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::async_::Stmt,
        extractor:
            fn(&tokio_postgres::Row) -> Result<super::SelectCommentBorrowed, tokio_postgres::Error>,
        mapper: fn(super::SelectCommentBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectCommentQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectCommentBorrowed) -> R,
        ) -> SelectCommentQuery<'c, 'a, 's, C, R, N> {
            SelectCommentQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(it)
        }
    }
    pub struct SelectInlineCommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::async_::Stmt,
        extractor: fn(
            &tokio_postgres::Row,
        ) -> Result<super::SelectInlineCommentBorrowed, tokio_postgres::Error>,
        mapper: fn(super::SelectInlineCommentBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectInlineCommentQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectInlineCommentBorrowed) -> R,
        ) -> SelectInlineCommentQuery<'c, 'a, 's, C, R, N> {
            SelectInlineCommentQuery {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            let row = self.client.query_one(stmt, &self.params).await?;
            Ok((self.mapper)((self.extractor)(&row)?))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let stmt = self.stmt.prepare(self.client).await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                .await?
                .map(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
                .transpose()?)
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
            tokio_postgres::Error,
        > {
            let stmt = self.stmt.prepare(self.client).await?;
            let it = self
                .client
                .query_raw(stmt, crate::slice_iter(&self.params))
                .await?
                .map(move |res| {
                    res.and_then(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                })
                .into_stream();
            Ok(it)
        }
    }
    pub fn select_compact() -> SelectCompactStmt {
        SelectCompactStmt(crate::client::async_::Stmt::new("SELECT * FROM clone"))
    }
    pub struct SelectCompactStmt(crate::client::async_::Stmt);
    impl SelectCompactStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
        ) -> CloneCompositeQuery<'c, 'a, 's, C, crate::types::CloneComposite, 0> {
            CloneCompositeQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn select_spaced() -> SelectSpacedStmt {
        SelectSpacedStmt(crate::client::async_::Stmt::new("SELECT * FROM clone"))
    }
    pub struct SelectSpacedStmt(crate::client::async_::Stmt);
    impl SelectSpacedStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
        ) -> CloneCompositeQuery<'c, 'a, 's, C, crate::types::CloneComposite, 0> {
            CloneCompositeQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn implicit_compact() -> ImplicitCompactStmt {
        ImplicitCompactStmt(crate::client::async_::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct ImplicitCompactStmt(crate::client::async_::Stmt);
    impl ImplicitCompactStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
            &'s mut self,
            client: &'c C,
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it,
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<
            'c,
            'a,
            's,
            super::ImplicitCompactParams<T1>,
            Optioni32Query<'c, 'a, 's, C, Option<i32>, 2>,
            C,
        > for ImplicitCompactStmt
    {
        fn params(
            &'s mut self,
            client: &'c C,
            params: &'a super::ImplicitCompactParams<T1>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn implicit_spaced() -> ImplicitSpacedStmt {
        ImplicitSpacedStmt(crate::client::async_::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct ImplicitSpacedStmt(crate::client::async_::Stmt);
    impl ImplicitSpacedStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
            &'s mut self,
            client: &'c C,
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor: |row| Ok(row.try_get(0)?),
                mapper: |it| it,
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<
            'c,
            'a,
            's,
            super::ImplicitSpacedParams<T1>,
            Optioni32Query<'c, 'a, 's, C, Option<i32>, 2>,
            C,
        > for ImplicitSpacedStmt
    {
        fn params(
            &'s mut self,
            client: &'c C,
            params: &'a super::ImplicitSpacedParams<T1>,
        ) -> Optioni32Query<'c, 'a, 's, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn named_compact() -> NamedCompactStmt {
        NamedCompactStmt(crate::client::async_::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct NamedCompactStmt(crate::client::async_::Stmt);
    impl NamedCompactStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
            &'s mut self,
            client: &'c C,
            name: &'a T1,
            price: &'a f64,
        ) -> RowQuery<'c, 'a, 's, C, super::Row, 2> {
            RowQuery {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor:
                    |row: &tokio_postgres::Row| -> Result<super::Row, tokio_postgres::Error> {
                        Ok(super::Row {
                            id: row.try_get(0)?,
                        })
                    },
                mapper: |it| super::Row::from(it),
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<
            'c,
            'a,
            's,
            super::Params<T1>,
            RowQuery<'c, 'a, 's, C, super::Row, 2>,
            C,
        > for NamedCompactStmt
    {
        fn params(
            &'s mut self,
            client: &'c C,
            params: &'a super::Params<T1>,
        ) -> RowQuery<'c, 'a, 's, C, super::Row, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn named_spaced() -> NamedSpacedStmt {
        NamedSpacedStmt(crate::client::async_::Stmt::new(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
        ))
    }
    pub struct NamedSpacedStmt(crate::client::async_::Stmt);
    impl NamedSpacedStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
            &'s mut self,
            client: &'c C,
            name: &'a T1,
            price: &'a f64,
        ) -> RowSpaceQuery<'c, 'a, 's, C, super::RowSpace, 2> {
            RowSpaceQuery {
                client,
                params: [name, price],
                stmt: &mut self.0,
                extractor:
                    |row: &tokio_postgres::Row| -> Result<super::RowSpace, tokio_postgres::Error> {
                        Ok(super::RowSpace {
                            id: row.try_get(0)?,
                        })
                    },
                mapper: |it| super::RowSpace::from(it),
            }
        }
    }
    impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
        crate::client::async_::Params<
            'c,
            'a,
            's,
            super::ParamsSpace<T1>,
            RowSpaceQuery<'c, 'a, 's, C, super::RowSpace, 2>,
            C,
        > for NamedSpacedStmt
    {
        fn params(
            &'s mut self,
            client: &'c C,
            params: &'a super::ParamsSpace<T1>,
        ) -> RowSpaceQuery<'c, 'a, 's, C, super::RowSpace, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn tricky_sql() -> TrickySqlStmt {
        TrickySqlStmt(crate::client::async_::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a bind_param\\', $1, $2)",
        ))
    }
    pub struct TrickySqlStmt(crate::client::async_::Stmt);
    impl TrickySqlStmt {
        pub async fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySqlParams,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySqlStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySqlParams,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql1() -> TrickySql1Stmt {
        TrickySql1Stmt(crate::client::async_::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a :bind_param', $1, $2)",
        ))
    }
    pub struct TrickySql1Stmt(crate::client::async_::Stmt);
    impl TrickySql1Stmt {
        pub async fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql1Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql1Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql1Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql2() -> TrickySql2Stmt {
        TrickySql2Stmt(crate::client::async_::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a '':bind_param''', $1, $2)",
        ))
    }
    pub struct TrickySql2Stmt(crate::client::async_::Stmt);
    impl TrickySql2Stmt {
        pub async fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql2Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql2Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql2Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql3() -> TrickySql3Stmt {
        TrickySql3Stmt(crate::client::async_::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum)  VALUES ($$this is not a :bind_param$$, $1, $2)",
        ))
    }
    pub struct TrickySql3Stmt(crate::client::async_::Stmt);
    impl TrickySql3Stmt {
        pub async fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql3Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql3Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql3Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql4() -> TrickySql4Stmt {
        TrickySql4Stmt(crate::client::async_::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($tag$this is not a :bind_param$tag$, $1, $2)",
        ))
    }
    pub struct TrickySql4Stmt(crate::client::async_::Stmt);
    impl TrickySql4Stmt {
        pub async fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql4Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql4Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql4Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql6() -> TrickySql6Stmt {
        TrickySql6Stmt(crate::client::async_::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is not a '':bind_param''', $1, $2)",
        ))
    }
    pub struct TrickySql6Stmt(crate::client::async_::Stmt);
    impl TrickySql6Stmt {
        pub async fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql6Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql6Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql6Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql7() -> TrickySql7Stmt {
        TrickySql7Stmt(crate::client::async_::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is not a \\':bind_param\\'', $1, $2)",
        ))
    }
    pub struct TrickySql7Stmt(crate::client::async_::Stmt);
    impl TrickySql7Stmt {
        pub async fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql7Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql7Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql7Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql8() -> TrickySql8Stmt {
        TrickySql8Stmt(crate::client::async_::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is ''not'' a \\':bind_param\\'', $1, $2)",
        ))
    }
    pub struct TrickySql8Stmt(crate::client::async_::Stmt);
    impl TrickySql8Stmt {
        pub async fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql8Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql8Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql8Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql9() -> TrickySql9Stmt {
        TrickySql9Stmt(crate::client::async_::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is \\'not\\' a \\':bind_param\\'', $1, $2)",
        ))
    }
    pub struct TrickySql9Stmt(crate::client::async_::Stmt);
    impl TrickySql9Stmt {
        pub async fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql9Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql9Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql9Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql10() -> TrickySql10Stmt {
        TrickySql10Stmt(crate::client::async_::Stmt::new(
            "INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is just a cast'::text, $1, $2)",
        ))
    }
    pub struct TrickySql10Stmt(crate::client::async_::Stmt);
    impl TrickySql10Stmt {
        pub async fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
            r#async: &'a crate::types::SyntaxComposite,
            r#enum: &'a crate::types::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::TrickySql10Params,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for TrickySql10Stmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::TrickySql10Params,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn r#typeof() -> RTypeofStmt {
        RTypeofStmt(crate::client::async_::Stmt::new("SELECT * FROM syntax"))
    }
    pub struct RTypeofStmt(crate::client::async_::Stmt);
    impl RTypeofStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
        ) -> TypeofQuery<'c, 'a, 's, C, super::Typeof, 0> {
            TypeofQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::TypeofBorrowed, tokio_postgres::Error> {
                    Ok(super::TypeofBorrowed {
                        trick_y: row.try_get(0)?,
                        r#async: row.try_get(1)?,
                        r#enum: row.try_get(2)?,
                    })
                },
                mapper: |it| super::Typeof::from(it),
            }
        }
    }
    /// Multi line
    ///
    /// Doc string comment
    pub fn select_comment() -> SelectCommentStmt {
        SelectCommentStmt(crate::client::async_::Stmt::new("SELECT * FROM syntax"))
    }
    pub struct SelectCommentStmt(crate::client::async_::Stmt);
    impl SelectCommentStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
        ) -> SelectCommentQuery<'c, 'a, 's, C, super::SelectComment, 0> {
            SelectCommentQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::SelectCommentBorrowed, tokio_postgres::Error> {
                    Ok(super::SelectCommentBorrowed {
                        trick_y: row.try_get(0)?,
                        r#async: row.try_get(1)?,
                        r#enum: row.try_get(2)?,
                    })
                },
                mapper: |it| super::SelectComment::from(it),
            }
        }
    }
    pub fn select_inline_comment() -> SelectInlineCommentStmt {
        SelectInlineCommentStmt(crate::client::async_::Stmt::new(
            "SELECT '-- dont remove this\\n' as c1, $$-- or this$$ as c2, E'-- escape string here' as c3, e'-- another escape string' as c4, $tag$-- dollar quoted here$tag$ as c5",
        ))
    }
    pub struct SelectInlineCommentStmt(crate::client::async_::Stmt);
    impl SelectInlineCommentStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
        ) -> SelectInlineCommentQuery<'c, 'a, 's, C, super::SelectInlineComment, 0> {
            SelectInlineCommentQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row: &tokio_postgres::Row| -> Result<
                    super::SelectInlineCommentBorrowed,
                    tokio_postgres::Error,
                > {
                    Ok(super::SelectInlineCommentBorrowed {
                        c1: row.try_get(0)?,
                        c2: row.try_get(1)?,
                        c3: row.try_get(2)?,
                        c4: row.try_get(3)?,
                        c5: row.try_get(4)?,
                    })
                },
                mapper: |it| super::SelectInlineComment::from(it),
            }
        }
    }
}
