// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct InsertUserParams<
    T1: crate::StringSql,
    T2: crate::ArraySql<Item = T1>,
    T3: crate::StringSql,
    T4: crate::ArraySql<Item = T3>,
> {
    pub names: T2,
    pub hair_colors: T4,
}
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
}
pub struct UserBorrowed<'a> {
    pub id: i32,
    pub name: &'a str,
    pub hair_color: Option<&'a str>,
}
impl<'a> From<UserBorrowed<'a>> for User {
    fn from(
        UserBorrowed {
            id,
            name,
            hair_color,
        }: UserBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            hair_color: hair_color.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: Option<String>,
}
pub struct PostBorrowed<'a> {
    pub id: i32,
    pub user_id: i32,
    pub title: &'a str,
    pub body: Option<&'a str>,
}
impl<'a> From<PostBorrowed<'a>> for Post {
    fn from(
        PostBorrowed {
            id,
            user_id,
            title,
            body,
        }: PostBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            user_id,
            title: title.into(),
            body: body.map(|v| v.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub text: String,
}
pub struct CommentBorrowed<'a> {
    pub id: i32,
    pub post_id: i32,
    pub text: &'a str,
}
impl<'a> From<CommentBorrowed<'a>> for Comment {
    fn from(CommentBorrowed { id, post_id, text }: CommentBorrowed<'a>) -> Self {
        Self {
            id,
            post_id,
            text: text.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SelectComplex {
    pub myuser_id: i32,
    pub name: String,
    pub hair_color: Option<String>,
    pub post_id: Option<i32>,
    pub user_id: Option<i32>,
    pub title: Option<String>,
    pub body: Option<String>,
}
pub struct SelectComplexBorrowed<'a> {
    pub myuser_id: i32,
    pub name: &'a str,
    pub hair_color: Option<&'a str>,
    pub post_id: Option<i32>,
    pub user_id: Option<i32>,
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
}
impl<'a> From<SelectComplexBorrowed<'a>> for SelectComplex {
    fn from(
        SelectComplexBorrowed {
            myuser_id,
            name,
            hair_color,
            post_id,
            user_id,
            title,
            body,
        }: SelectComplexBorrowed<'a>,
    ) -> Self {
        Self {
            myuser_id,
            name: name.into(),
            hair_color: hair_color.map(|v| v.into()),
            post_id,
            user_id,
            title: title.map(|v| v.into()),
            body: body.map(|v| v.into()),
        }
    }
}
pub mod sync {
    use postgres::{GenericClient, fallible_iterator::FallibleIterator};
    pub struct UserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> Result<super::UserBorrowed, postgres::Error>,
        mapper: fn(super::UserBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> UserQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::UserBorrowed) -> R,
        ) -> UserQuery<'c, 'a, 's, C, R, N> {
            UserQuery {
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
    pub struct PostQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> Result<super::PostBorrowed, postgres::Error>,
        mapper: fn(super::PostBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> PostQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::PostBorrowed) -> R,
        ) -> PostQuery<'c, 'a, 's, C, R, N> {
            PostQuery {
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
    pub struct CommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> Result<super::CommentBorrowed, postgres::Error>,
        mapper: fn(super::CommentBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> CommentQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::CommentBorrowed) -> R,
        ) -> CommentQuery<'c, 'a, 's, C, R, N> {
            CommentQuery {
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
    pub struct SelectComplexQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::sync::Stmt,
        extractor: fn(&postgres::Row) -> Result<super::SelectComplexBorrowed, postgres::Error>,
        mapper: fn(super::SelectComplexBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectComplexQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectComplexBorrowed) -> R,
        ) -> SelectComplexQuery<'c, 'a, 's, C, R, N> {
            SelectComplexQuery {
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
    pub fn users() -> UsersStmt {
        UsersStmt(crate::client::sync::Stmt::new("SELECT * FROM users"))
    }
    pub struct UsersStmt(crate::client::sync::Stmt);
    impl UsersStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
        ) -> UserQuery<'c, 'a, 's, C, super::User, 0> {
            UserQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row: &postgres::Row| -> Result<super::UserBorrowed, postgres::Error> {
                    Ok(super::UserBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        hair_color: row.try_get(2)?,
                    })
                },
                mapper: |it| super::User::from(it),
            }
        }
    }
    /// Performs a bulk insert of multiple users.
    ///
    /// Clorinde doesn't support multi-value inserts, so we use `unnest` to transform two arrays
    /// (names and hair_colors) into rows of values that can be inserted together.
    pub fn insert_user() -> InsertUserStmt {
        InsertUserStmt(crate::client::sync::Stmt::new(
            "INSERT INTO users (name, hair_color) SELECT unnest($1::text[]) as name, unnest($2::text[]) as hair_color",
        ))
    }
    pub struct InsertUserStmt(crate::client::sync::Stmt);
    impl InsertUserStmt {
        pub fn bind<
            'c,
            'a,
            's,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::ArraySql<Item = T1>,
            T3: crate::StringSql,
            T4: crate::ArraySql<Item = T3>,
        >(
            &'s mut self,
            client: &'c mut C,
            names: &'a T2,
            hair_colors: &'a T4,
        ) -> Result<u64, postgres::Error> {
            let stmt = self.0.prepare(client)?;
            client.execute(stmt, &[names, hair_colors])
        }
    }
    impl<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::ArraySql<Item = T1>,
        T3: crate::StringSql,
        T4: crate::ArraySql<Item = T3>,
    >
        crate::client::sync::Params<
            'c,
            'a,
            's,
            super::InsertUserParams<T1, T2, T3, T4>,
            Result<u64, postgres::Error>,
            C,
        > for InsertUserStmt
    {
        fn params(
            &'s mut self,
            client: &'c mut C,
            params: &'a super::InsertUserParams<T1, T2, T3, T4>,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.names, &params.hair_colors)
        }
    }
    pub fn posts() -> PostsStmt {
        PostsStmt(crate::client::sync::Stmt::new("SELECT * FROM posts"))
    }
    pub struct PostsStmt(crate::client::sync::Stmt);
    impl PostsStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
        ) -> PostQuery<'c, 'a, 's, C, super::Post, 0> {
            PostQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |row: &postgres::Row| -> Result<super::PostBorrowed, postgres::Error> {
                    Ok(super::PostBorrowed {
                        id: row.try_get(0)?,
                        user_id: row.try_get(1)?,
                        title: row.try_get(2)?,
                        body: row.try_get(3)?,
                    })
                },
                mapper: |it| super::Post::from(it),
            }
        }
    }
    pub fn post_by_user_ids() -> PostByUserIdsStmt {
        PostByUserIdsStmt(crate::client::sync::Stmt::new(
            "SELECT * FROM posts WHERE user_id = ANY($1)",
        ))
    }
    pub struct PostByUserIdsStmt(crate::client::sync::Stmt);
    impl PostByUserIdsStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
            &'s mut self,
            client: &'c mut C,
            ids: &'a T1,
        ) -> PostQuery<'c, 'a, 's, C, super::Post, 1> {
            PostQuery {
                client,
                params: [ids],
                stmt: &mut self.0,
                extractor: |row: &postgres::Row| -> Result<super::PostBorrowed, postgres::Error> {
                    Ok(super::PostBorrowed {
                        id: row.try_get(0)?,
                        user_id: row.try_get(1)?,
                        title: row.try_get(2)?,
                        body: row.try_get(3)?,
                    })
                },
                mapper: |it| super::Post::from(it),
            }
        }
    }
    pub fn comments() -> CommentsStmt {
        CommentsStmt(crate::client::sync::Stmt::new("SELECT * FROM comments"))
    }
    pub struct CommentsStmt(crate::client::sync::Stmt);
    impl CommentsStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
        ) -> CommentQuery<'c, 'a, 's, C, super::Comment, 0> {
            CommentQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor:
                    |row: &postgres::Row| -> Result<super::CommentBorrowed, postgres::Error> {
                        Ok(super::CommentBorrowed {
                            id: row.try_get(0)?,
                            post_id: row.try_get(1)?,
                            text: row.try_get(2)?,
                        })
                    },
                mapper: |it| super::Comment::from(it),
            }
        }
    }
    pub fn comments_by_post_id() -> CommentsByPostIdStmt {
        CommentsByPostIdStmt(crate::client::sync::Stmt::new(
            "SELECT * FROM comments WHERE post_id = ANY($1)",
        ))
    }
    pub struct CommentsByPostIdStmt(crate::client::sync::Stmt);
    impl CommentsByPostIdStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
            &'s mut self,
            client: &'c mut C,
            ids: &'a T1,
        ) -> CommentQuery<'c, 'a, 's, C, super::Comment, 1> {
            CommentQuery {
                client,
                params: [ids],
                stmt: &mut self.0,
                extractor:
                    |row: &postgres::Row| -> Result<super::CommentBorrowed, postgres::Error> {
                        Ok(super::CommentBorrowed {
                            id: row.try_get(0)?,
                            post_id: row.try_get(1)?,
                            text: row.try_get(2)?,
                        })
                    },
                mapper: |it| super::Comment::from(it),
            }
        }
    }
    pub fn select_complex() -> SelectComplexStmt {
        SelectComplexStmt(crate::client::sync::Stmt::new(
            "SELECT u.id as myuser_id, u.name, u.hair_color, p.id as post_id, p.user_id, p.title, p.body FROM users as u LEFT JOIN posts as p on u.id = p.user_id",
        ))
    }
    pub struct SelectComplexStmt(crate::client::sync::Stmt);
    impl SelectComplexStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c mut C,
        ) -> SelectComplexQuery<'c, 'a, 's, C, super::SelectComplex, 0> {
            SelectComplexQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor:
                    |row: &postgres::Row| -> Result<super::SelectComplexBorrowed, postgres::Error> {
                        Ok(super::SelectComplexBorrowed {
                            myuser_id: row.try_get(0)?,
                            name: row.try_get(1)?,
                            hair_color: row.try_get(2)?,
                            post_id: row.try_get(3)?,
                            user_id: row.try_get(4)?,
                            title: row.try_get(5)?,
                            body: row.try_get(6)?,
                        })
                    },
                mapper: |it| super::SelectComplex::from(it),
            }
        }
    }
}
pub mod async_ {
    use crate::client::async_::GenericClient;
    use futures::{self, StreamExt, TryStreamExt};
    pub struct UserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> Result<super::UserBorrowed, tokio_postgres::Error>,
        mapper: fn(super::UserBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> UserQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::UserBorrowed) -> R,
        ) -> UserQuery<'c, 'a, 's, C, R, N> {
            UserQuery {
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
    pub struct PostQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::async_::Stmt,
        extractor: fn(&tokio_postgres::Row) -> Result<super::PostBorrowed, tokio_postgres::Error>,
        mapper: fn(super::PostBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> PostQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::PostBorrowed) -> R,
        ) -> PostQuery<'c, 'a, 's, C, R, N> {
            PostQuery {
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
    pub struct CommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::async_::Stmt,
        extractor:
            fn(&tokio_postgres::Row) -> Result<super::CommentBorrowed, tokio_postgres::Error>,
        mapper: fn(super::CommentBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> CommentQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::CommentBorrowed) -> R,
        ) -> CommentQuery<'c, 'a, 's, C, R, N> {
            CommentQuery {
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
    pub struct SelectComplexQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
        client: &'c C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'s mut crate::client::async_::Stmt,
        extractor:
            fn(&tokio_postgres::Row) -> Result<super::SelectComplexBorrowed, tokio_postgres::Error>,
        mapper: fn(super::SelectComplexBorrowed) -> T,
    }
    impl<'c, 'a, 's, C, T: 'c, const N: usize> SelectComplexQuery<'c, 'a, 's, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::SelectComplexBorrowed) -> R,
        ) -> SelectComplexQuery<'c, 'a, 's, C, R, N> {
            SelectComplexQuery {
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
    pub fn users() -> UsersStmt {
        UsersStmt(crate::client::async_::Stmt::new("SELECT * FROM users"))
    }
    pub struct UsersStmt(crate::client::async_::Stmt);
    impl UsersStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
        ) -> UserQuery<'c, 'a, 's, C, super::User, 0> {
            UserQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::UserBorrowed, tokio_postgres::Error> {
                    Ok(super::UserBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        hair_color: row.try_get(2)?,
                    })
                },
                mapper: |it| super::User::from(it),
            }
        }
    }
    /// Performs a bulk insert of multiple users.
    ///
    /// Clorinde doesn't support multi-value inserts, so we use `unnest` to transform two arrays
    /// (names and hair_colors) into rows of values that can be inserted together.
    pub fn insert_user() -> InsertUserStmt {
        InsertUserStmt(crate::client::async_::Stmt::new(
            "INSERT INTO users (name, hair_color) SELECT unnest($1::text[]) as name, unnest($2::text[]) as hair_color",
        ))
    }
    pub struct InsertUserStmt(crate::client::async_::Stmt);
    impl InsertUserStmt {
        pub async fn bind<
            'c,
            'a,
            's,
            C: GenericClient,
            T1: crate::StringSql,
            T2: crate::ArraySql<Item = T1>,
            T3: crate::StringSql,
            T4: crate::ArraySql<Item = T3>,
        >(
            &'s mut self,
            client: &'c C,
            names: &'a T2,
            hair_colors: &'a T4,
        ) -> Result<u64, tokio_postgres::Error> {
            let stmt = self.0.prepare(client).await?;
            client.execute(stmt, &[names, hair_colors]).await
        }
    }
    impl<
        'a,
        C: GenericClient + Send + Sync,
        T1: crate::StringSql,
        T2: crate::ArraySql<Item = T1>,
        T3: crate::StringSql,
        T4: crate::ArraySql<Item = T3>,
    >
        crate::client::async_::Params<
            'a,
            'a,
            'a,
            super::InsertUserParams<T1, T2, T3, T4>,
            std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            >,
            C,
        > for InsertUserStmt
    {
        fn params(
            &'a mut self,
            client: &'a C,
            params: &'a super::InsertUserParams<T1, T2, T3, T4>,
        ) -> std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.names, &params.hair_colors))
        }
    }
    pub fn posts() -> PostsStmt {
        PostsStmt(crate::client::async_::Stmt::new("SELECT * FROM posts"))
    }
    pub struct PostsStmt(crate::client::async_::Stmt);
    impl PostsStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
        ) -> PostQuery<'c, 'a, 's, C, super::Post, 0> {
            PostQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::PostBorrowed, tokio_postgres::Error> {
                    Ok(super::PostBorrowed {
                        id: row.try_get(0)?,
                        user_id: row.try_get(1)?,
                        title: row.try_get(2)?,
                        body: row.try_get(3)?,
                    })
                },
                mapper: |it| super::Post::from(it),
            }
        }
    }
    pub fn post_by_user_ids() -> PostByUserIdsStmt {
        PostByUserIdsStmt(crate::client::async_::Stmt::new(
            "SELECT * FROM posts WHERE user_id = ANY($1)",
        ))
    }
    pub struct PostByUserIdsStmt(crate::client::async_::Stmt);
    impl PostByUserIdsStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
            &'s mut self,
            client: &'c C,
            ids: &'a T1,
        ) -> PostQuery<'c, 'a, 's, C, super::Post, 1> {
            PostQuery {
                client,
                params: [ids],
                stmt: &mut self.0,
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::PostBorrowed, tokio_postgres::Error> {
                    Ok(super::PostBorrowed {
                        id: row.try_get(0)?,
                        user_id: row.try_get(1)?,
                        title: row.try_get(2)?,
                        body: row.try_get(3)?,
                    })
                },
                mapper: |it| super::Post::from(it),
            }
        }
    }
    pub fn comments() -> CommentsStmt {
        CommentsStmt(crate::client::async_::Stmt::new("SELECT * FROM comments"))
    }
    pub struct CommentsStmt(crate::client::async_::Stmt);
    impl CommentsStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
        ) -> CommentQuery<'c, 'a, 's, C, super::Comment, 0> {
            CommentQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::CommentBorrowed, tokio_postgres::Error> {
                    Ok(super::CommentBorrowed {
                        id: row.try_get(0)?,
                        post_id: row.try_get(1)?,
                        text: row.try_get(2)?,
                    })
                },
                mapper: |it| super::Comment::from(it),
            }
        }
    }
    pub fn comments_by_post_id() -> CommentsByPostIdStmt {
        CommentsByPostIdStmt(crate::client::async_::Stmt::new(
            "SELECT * FROM comments WHERE post_id = ANY($1)",
        ))
    }
    pub struct CommentsByPostIdStmt(crate::client::async_::Stmt);
    impl CommentsByPostIdStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
            &'s mut self,
            client: &'c C,
            ids: &'a T1,
        ) -> CommentQuery<'c, 'a, 's, C, super::Comment, 1> {
            CommentQuery {
                client,
                params: [ids],
                stmt: &mut self.0,
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::CommentBorrowed, tokio_postgres::Error> {
                    Ok(super::CommentBorrowed {
                        id: row.try_get(0)?,
                        post_id: row.try_get(1)?,
                        text: row.try_get(2)?,
                    })
                },
                mapper: |it| super::Comment::from(it),
            }
        }
    }
    pub fn select_complex() -> SelectComplexStmt {
        SelectComplexStmt(crate::client::async_::Stmt::new(
            "SELECT u.id as myuser_id, u.name, u.hair_color, p.id as post_id, p.user_id, p.title, p.body FROM users as u LEFT JOIN posts as p on u.id = p.user_id",
        ))
    }
    pub struct SelectComplexStmt(crate::client::async_::Stmt);
    impl SelectComplexStmt {
        pub fn bind<'c, 'a, 's, C: GenericClient>(
            &'s mut self,
            client: &'c C,
        ) -> SelectComplexQuery<'c, 'a, 's, C, super::SelectComplex, 0> {
            SelectComplexQuery {
                client,
                params: [],
                stmt: &mut self.0,
                extractor: |
                    row: &tokio_postgres::Row,
                | -> Result<super::SelectComplexBorrowed, tokio_postgres::Error> {
                    Ok(super::SelectComplexBorrowed {
                        myuser_id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        hair_color: row.try_get(2)?,
                        post_id: row.try_get(3)?,
                        user_id: row.try_get(4)?,
                        title: row.try_get(5)?,
                        body: row.try_get(6)?,
                    })
                },
                mapper: |it| super::SelectComplex::from(it),
            }
        }
    }
}
