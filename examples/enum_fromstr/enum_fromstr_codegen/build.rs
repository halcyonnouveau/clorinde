use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=../queries");
    println!("cargo:rerun-if-changed=../schema.sql");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let parent_dir = Path::new(&manifest_dir).parent().unwrap();

    let queries_path = parent_dir.join("queries");
    let schema_path = parent_dir.join("schema.sql");

    clorinde::generate_to_file(
        clorinde::Config::new(
            out_dir,
            schema_path,
            queries_path,
            "enum_fromstr_codegen".to_string(),
            true,
        )
        .verbose(true),
    )?;

    Ok(())
}