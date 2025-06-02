use clorinde::{Error, config::Config};
use std::path::Path;

#[allow(clippy::result_large_err)]
fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=../queries");
    println!("cargo:rerun-if-changed=../schema.sql");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let parent_dir = Path::new(&manifest_dir).parent().unwrap();

    let queries_path = parent_dir.join("queries");
    let schema_path = parent_dir.join("schema.sql");

    let cfg = Config::builder()
        .name("enum_fromstr_codegen")
        .destination(std::env::var("OUT_DIR").unwrap())
        .queries(queries_path)
        .build();

    clorinde::gen_managed(&[schema_path], cfg)?;

    Ok(())
}
