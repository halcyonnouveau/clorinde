use std::io::Write;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use rand::{Rng, distr::Alphanumeric};

use crate::{config::Config, conn, container, error::Error, gen_fresh, gen_live, gen_managed};

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
        /// Postgres url to the database
        #[clap(env = "DATABASE_URL")]
        url: String,

        /// Postgres search path to use for the queries
        #[clap(long)]
        search_path: Option<String>,

        #[clap(flatten)]
        args: CommonArgs,
    },
    /// Generate your modules against schema files
    Schema {
        /// SQL files containing the database schema
        #[clap(required = true, value_parser = validate_path_exists)]
        schema_files: Vec<PathBuf>,

        /// Container image to use
        #[clap(long)]
        container_image: Option<String>,

        /// Container wait time in milliseconds after health check
        #[clap(long)]
        container_wait: Option<u64>,

        /// Use `podman` instead of `docker`
        #[clap(short, long)]
        podman: Option<bool>,

        #[clap(flatten)]
        args: CommonArgs,
    },
    /// Generate your modules against schema files using a fresh database on an existing server
    Fresh {
        /// SQL files containing the database schema
        #[clap(required = true, value_parser = validate_path_exists)]
        schema_files: Vec<PathBuf>,

        /// Postgres server url (without database name)
        #[clap(long, short, env = "DATABASE_URL")]
        url: Option<String>,

        /// Postgres search path to use for the queries
        #[clap(long)]
        search_path: Option<String>,

        /// Name for the temporary database (defaults to clorinde_temp_<random>)
        #[clap(long)]
        db_name: Option<String>,

        /// Keep the temporary database after generation (don't cleanup)
        #[clap(long)]
        keep_db: bool,

        #[clap(flatten)]
        args: CommonArgs,
    },
}

impl Action {
    fn args(&self) -> CommonArgs {
        match self {
            Self::Live { args, .. } => args,
            Self::Schema { args, .. } => args,
            Self::Fresh { args, .. } => args,
        }
        .clone()
    }
}

#[derive(Parser, Debug, Clone)]
struct CommonArgs {
    /// Config file path
    #[clap(short, long, default_value = "clorinde.toml", value_parser = validate_path_exists)]
    config: PathBuf,

    /// Folder containing the queries
    #[clap(short, long, value_parser = validate_path_exists)]
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

    /// (DEPRECATED) Derive serde's `Serialize` trait for generated types.
    #[clap(long)]
    #[deprecated(
        since = "1.0.0",
        note = "please use the `types.derive-traits` configuration instead"
    )]
    serialize: Option<bool>,
}

#[allow(clippy::result_large_err, deprecated)]
// Main entrypoint of the CLI. Parses the args and calls the appropriate routines.
pub fn run() -> Result<(), Error> {
    let Args { action } = Args::parse();
    let CommonArgs {
        config,
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

    cfg.queries = queries_path.unwrap_or(cfg.queries);
    cfg.destination = destination.unwrap_or(cfg.destination);
    cfg.sync = sync.unwrap_or(cfg.sync);
    cfg.r#async = r#async.unwrap_or(false) || !cfg.sync;
    cfg.serialize = serialize.unwrap_or(cfg.serialize);

    if serialize.is_some() {
        eprintln!(
            "Warning: --serialize is deprecated (since 1.0.0), please use the `types.derive-traits` configuration instead"
        );
    }

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
        Action::Live {
            url, search_path, ..
        } => {
            let mut client = conn::from_url(&url)?;
            if let Some(search_path) = search_path.as_ref() {
                conn::set_search_path(&mut client, search_path)?;
            }

            gen_live(&mut client, cfg)?;
        }
        Action::Schema {
            schema_files,
            container_image,
            container_wait,
            podman,
            ..
        } => {
            cfg.podman = podman.unwrap_or(cfg.podman);
            cfg.container_image = container_image.unwrap_or(cfg.container_image);
            cfg.container_wait = container_wait.unwrap_or(cfg.container_wait);

            // Run the generate command. If the command is unsuccessful, cleanup Clorinde's container
            if let Err(e) = gen_managed(&schema_files, cfg.clone()) {
                container::cleanup(cfg.podman).ok();
                return Err(e);
            }
        }
        Action::Fresh {
            url,
            schema_files,
            search_path,
            db_name,
            keep_db,
            ..
        } => {
            // Generate random database name if not provided
            let final_db_name = db_name.unwrap_or_else(|| {
                let random_suffix: String = rand::rng()
                    .sample_iter(&Alphanumeric)
                    .take(8)
                    .map(char::from)
                    .collect();
                format!("clorinde_temp_{}", random_suffix.to_lowercase())
            });

            gen_fresh(
                &url.unwrap(),
                &final_db_name,
                &schema_files,
                search_path.as_deref(),
                keep_db,
                cfg,
            )?;
        }
    };
    Ok(())
}

fn validate_path_exists(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.exists() {
        Ok(path)
    } else {
        Err(format!("invalid path '{}'", path.display()))
    }
}
