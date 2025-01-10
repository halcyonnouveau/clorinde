mod cli;
mod codegen;
mod error;
mod load_schema;
mod parser;
mod prepare_queries;
mod read_queries;
mod type_registrar;
mod utils;
mod validation;

pub mod config;
/// Helpers to establish connections to database instances.
pub mod conn;
/// High-level interfaces to work with Clorinde's container manager.
pub mod container;

use config::Config;

use std::path::Path;

use postgres::Client;

use parser::parse_query_module;
use prepare_queries::prepare;
use read_queries::read_query_modules;

#[doc(hidden)]
pub use cli::run;

pub use error::Error;
pub use load_schema::load_schema;

#[allow(clippy::result_large_err)]
/// Generates Rust queries from PostgreSQL queries located at `queries_path`,
/// using a live database managed by you. Code generation settings are
/// set using the `settings` parameter.
pub fn gen_live(client: &mut Client, config: Config) -> Result<(), Error> {
    // Read
    let modules = read_query_modules(config.queries.as_ref())?
        .into_iter()
        .map(parse_query_module)
        .collect::<Result<_, parser::error::Error>>()?;

    // Generate
    let prepared_modules = prepare(client, modules, &config)?;
    let generated = codegen::gen(prepared_modules, &config);

    // Write
    generated.persist(config.destination)?;

    Ok(())
}

#[allow(clippy::result_large_err)]
/// Generates Rust queries from PostgreSQL queries located at `queries_path`, using
/// a container managed by clorinde. The database schema is created using `schema_files`.
/// Code generation settings are set using the `settings` parameter.
///
/// By default, the container manager is Docker, but Podman can be used by setting the
/// `podman` parameter to `true`.
pub fn gen_managed<P: AsRef<Path>>(schema_files: &[P], config: Config) -> Result<(), Error> {
    // Read
    let modules = read_query_modules(config.queries.as_ref())?
        .into_iter()
        .map(parse_query_module)
        .collect::<Result<_, parser::error::Error>>()?;

    container::setup(config.podman)?;
    let mut client = conn::clorinde_conn()?;
    load_schema(&mut client, schema_files)?;
    let prepared_modules = prepare(&mut client, modules, &config)?;
    let generated = codegen::gen(prepared_modules, &config);
    container::cleanup(config.podman)?;

    // Write
    generated.persist(config.destination)?;

    Ok(())
}
