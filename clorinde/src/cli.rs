use clap::{Parser, Subcommand};
use std::io::Write;
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
    /// Ignore query files prefixed with underscore
    #[clap(long)]
    ignore_underscore_files: Option<bool>,
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
        ignore_underscore_files,
    } = action.args();

    let mut cfg = match config.is_file() {
        true => Config::from_file(config)?,
        false => Config::default(),
    };

    cfg.podman = podman.unwrap_or(cfg.podman);
    cfg.queries = queries_path.unwrap_or(cfg.queries);
    cfg.destination = destination.unwrap_or(cfg.destination);
    cfg.sync = sync.unwrap_or(cfg.sync);
    cfg.r#async = r#async.unwrap_or(false) || !cfg.sync;
    cfg.serialize = serialize.unwrap_or(cfg.serialize);
    cfg.ignore_underscore_files = ignore_underscore_files.unwrap_or(cfg.ignore_underscore_files);

    let podman = cfg.podman;

    // Prevent wrong directory being accidentally deleted
    if !cfg.destination.ends_with("clorinde")
        && (cfg.destination.exists() && !cfg.destination.join("Cargo.toml").exists())
    {
        println!(
            "The directory '{}' already exists. Running `clorinde` on this directory will delete all files contained within it.",
            cfg.destination.display()
        );
        println!("Do you want to continue? [y/N]");
        std::io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("Aborting.");
            std::process::exit(0);
        }
    }

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
