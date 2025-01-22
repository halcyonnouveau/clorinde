// This file was generated with `clorinde`. Do not modify.

use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub fn insert_book() -> InsertBookStmt {
    InsertBookStmt(crate::client::async_::Stmt::new(
        "INSERT INTO Book (title)
  VALUES ($1)",
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
