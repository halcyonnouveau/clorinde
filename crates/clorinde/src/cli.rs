use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::{config::Config, conn, container, error::Error, gen_live, gen_managed};

/// Command line interface to interact with Clorinde SQL.
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(subcommand)]
    action: Action,

    /// Config file path
    #[clap(short, long, default_value = "clorinde.toml")]
    config: PathBuf,

    /// Use `podman` instead of `docker`
    #[clap(short, long)]
    podman: bool,
    /// Folder containing the queries
    #[clap(short, long, default_value = "queries/")]
    queries_path: PathBuf,
    /// Destination folder for generated modules
    #[clap(short, long, default_value = "clorinde")]
    destination: PathBuf,
    /// Generate synchronous rust code
    #[clap(long)]
    sync: bool,
    /// Generate asynchronous rust code
    #[clap(long)]
    r#async: bool,
    /// Derive serde's `Serialize` trait for generated types.
    #[clap(long)]
    serialize: bool,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Generate your modules against your own db
    Live {
        /// Postgres url to the database
        url: String,
    },
    /// Generate your modules against schema files
    Schema {
        /// SQL files containing the database schema
        schema_files: Vec<PathBuf>,
    },
}

#[allow(clippy::result_large_err)]
// Main entrypoint of the CLI. Parses the args and calls the appropriate routines.
pub fn run() -> Result<(), Error> {
    let Args {
        podman,
        queries_path,
        destination,
        action,
        sync,
        r#async,
        serialize,
        config,
    } = Args::parse();

    let mut cfg = match config.is_file() {
        true => Config::from_file(config)?,
        false => Config::default(),
    };

    cfg.podman = podman;
    cfg.queries = queries_path;
    cfg.destination = destination;
    cfg.sync = sync;
    cfg.r#async = r#async || !sync;
    cfg.serialize = serialize;

    match action {
        Action::Live { url } => {
            let mut client = conn::from_url(&url)?;
            gen_live(&mut client, cfg)?;
        }
        Action::Schema { schema_files } => {
            // Run the generate command. If the command is unsuccessful, cleanup Clorinde's container
            if let Err(e) = gen_managed(&schema_files, cfg) {
                container::cleanup(podman).ok();
                return Err(e);
            }
        }
    };
    Ok(())
}
