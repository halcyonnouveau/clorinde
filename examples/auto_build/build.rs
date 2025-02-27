use std::{path::PathBuf, str::FromStr};

use clorinde::{
    Error,
    config::{Config, Package},
};

// This script will generate a new clorinde crate every time your schema or queries change.
// In this example, we generate the module in our project, but
// we could also generate it elsewhere

#[allow(clippy::result_large_err)]
fn main() -> Result<(), Error> {
    let queries_path = "queries";
    let schema_file = "schema.sql";

    let cfg = Config {
        destination: PathBuf::from_str("auto_build_codegen").unwrap(),
        package: Package {
            name: "auto_build_codegen".into(),
            ..Package::default()
        },
        ..Default::default()
    };

    // This can be removed in your code
    let run_build = std::env::var("RUN_AUTO_BUILD");
    if run_build.is_ok() {
        println!("cargo:rerun-if-changed={queries_path}");
        println!("cargo:rerun-if-changed={schema_file}");
        clorinde::gen_managed(&[schema_file], cfg)?;
    }

    Ok(())
}
