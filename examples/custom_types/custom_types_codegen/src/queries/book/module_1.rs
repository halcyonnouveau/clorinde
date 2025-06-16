// This file was generated with `clorinde`. Do not modify.

#[derive(serde::Serialize, Debug, Clone, PartialEq, Default, Hash)]
pub struct Book {
    pub title: String,
}
pub struct BookBorrowed<'a> {
    pub title: &'a str,
}
impl<'a> From<BookBorrowed<'a>> for Book {
    fn from(BookBorrowed { title }: BookBorrowed<'a>) -> Self {
        Self {
            title: title.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct BookQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<BookBorrowed, tokio_postgres::Error>,
    mapper: fn(BookBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> BookQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(BookBorrowed) -> R) -> BookQuery<'c, 'a, 's, C, R, N> {
        BookQuery {
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
pub fn insert_book() -> InsertBookStmt {
    InsertBookStmt(crate::client::async_::Stmt::new(
        "INSERT INTO Book (title) VALUES ($1)",
    ))
}
pub struct InsertBookStmt(crate::client::async_::Stmt);
impl InsertBookStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        title: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[title]).await
    }
}
pub fn books() -> BooksStmt {
    BooksStmt(crate::client::async_::Stmt::new("SELECT Title FROM Book"))
}
pub struct BooksStmt(crate::client::async_::Stmt);
impl BooksStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> BookQuery<'c, 'a, 's, C, Book, 0> {
        BookQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<BookBorrowed, tokio_postgres::Error> {
                Ok(BookBorrowed {
                    title: row.try_get(0)?,
                })
            },
            mapper: |it| Book::from(it),
        }
    }
}
