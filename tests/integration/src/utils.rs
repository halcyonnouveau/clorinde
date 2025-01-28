/// Reset the current database
pub(crate) fn reset_db(client: &mut postgres::Client) -> Result<(), postgres::Error> {
    client.batch_execute("DROP SCHEMA public CASCADE;CREATE SCHEMA public;")
}
