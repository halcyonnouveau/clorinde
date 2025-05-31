use postgres::{Client, Config, NoTls};

use self::error::Error;

/// Creates a non-TLS connection from a URL.
pub(crate) fn from_url(url: &str) -> Result<Client, Error> {
    Ok(Client::connect(url, NoTls)?)
}

/// Create a non-TLS connection to the container managed by Clorinde.
pub fn clorinde_conn() -> Result<Client, Error> {
    Ok(Config::new()
        .user("postgres")
        .password("postgres")
        .host("127.0.0.1")
        .port(5435)
        .dbname("postgres")
        .connect(NoTls)?)
}

// Sets the search path for the given client.
pub fn set_search_path(client: &mut Client, search_path: &str) -> Result<(), Error> {
    client
        .execute(&format!("SET search_path TO {}", search_path), &[])
        .map_err(Error::from)?;
    Ok(())
}

pub(crate) mod error {
    use miette::Diagnostic;

    #[derive(Debug, thiserror::Error, Diagnostic)]
    #[error("Couldn't establish a connection with the database.")]
    pub struct Error(#[from] pub postgres::Error);
}
