// This file was generated with `clorinde`. Do not modify.

use postgres::{fallible_iterator::FallibleIterator, GenericClient};
pub fn insert_book() -> InsertBookStmt {
    InsertBookStmt(crate::client::sync::Stmt::new(
        "INSERT INTO Book (title)
  VALUES ($1)",
    ))
}
pub struct InsertBookStmt(crate::client::sync::Stmt);
impl InsertBookStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c mut C,
        title: &'a T1,
    ) -> Result<u64, postgres::Error> {
        let stmt = self.0.prepare(client)?;
        client.execute(stmt, &[title])
    }
}
