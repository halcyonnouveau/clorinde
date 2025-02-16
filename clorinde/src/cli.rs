use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::{config::Config, conn, container, error::Error, gen_live, gen_managed};

/// Command line interface to interact with Clorinde SQL.
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Generate your modules against your own db
    Live {
        #[clap(env = "DATABASE_URL")]
        /// Postgres url to the database
        url: String,

        #[clap(flatten)]
        args: CommonArgs,
    },
    /// Generate your modules against schema files
    Schema {
        /// SQL files containing the database schema
        schema_files: Vec<PathBuf>,

        #[clap(flatten)]
        args: CommonArgs,
    },
}

impl Action {
    fn args(&self) -> CommonArgs {
        match self {
            Self::Live { args, .. } => args,
            Self::Schema { args, .. } => args,
        }
        .clone()
    }
}

#[derive(Parser, Debug, Clone)]
struct CommonArgs {
    /// Config file path
    #[clap(short, long, default_value = "clorinde.toml")]
    config: PathBuf,
    /// Use `podman` instead of `docker`
    #[clap(short, long)]
    podman: Option<bool>,
    /// Folder containing the queries
    #[clap(short, long)]
    queries_path: Option<PathBuf>,
    /// Destination folder for generated modules
    #[clap(short, long)]
    destination: Option<PathBuf>,
    /// Generate synchronous rust code
    #[clap(long)]
    sync: Option<bool>,
    /// Generate asynchronous rust code
    #[clap(long)]
    r#async: Option<bool>,
    /// Derive serde's `Serialize` trait for generated types.
    #[clap(long)]
    serialize: Option<bool>,
}

#[allow(clippy::result_large_err)]
// Main entrypoint of the CLI. Parses the args and calls the appropriate routines.
pub fn run() -> Result<(), Error> {
    let Args { action } = Args::parse();
    let CommonArgs {
        config,
        podman,
        queries_path,
        destination,
        sync,
        r#async,
        serialize,
    } = action.args();

    let mut cfg = match config.is_file() {
        true => Config::from_file(config)?,
        false => Config::default(),
    };

    if let Some(podman) = podman {
        cfg.podman = podman;
    }
    if let Some(queries_path) = queries_path {
        cfg.queries = queries_path;
    }
    if let Some(destination) = destination {
        cfg.destination = destination;
    }

    if let Some(sync) = sync {
        cfg.sync = sync;
    }
    if let Some(r#async) = r#async {
        cfg.r#async = r#async || !cfg.sync;
    }
    if let Some(serialize) = serialize {
        cfg.serialize = serialize;
    }
    let podman = cfg.podman;

    match action {
        Action::Live { url, .. } => {
            let mut client = conn::from_url(&url)?;
            gen_live(&mut client, cfg)?;
        }
        Action::Schema { schema_files, .. } => {
            // Run the generate command. If the command is unsuccessful, cleanup Clorinde's container
            if let Err(e) = gen_managed(&schema_files, cfg) {
                container::cleanup(podman).ok();
                return Err(e);
            }
        }
    };
    Ok(())
}
