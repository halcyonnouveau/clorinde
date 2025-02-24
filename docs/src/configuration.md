# Configuration
Clorinde can be configured using a configuration file (`clorinde.toml` by default) in your project. This file allows you to customise generated code behaviour, specify static files, manage dependencies, and override type mappings.

## Package configuration
The `[package]` section allows you to configure any standard Cargo.toml package field in the generated crate:

```toml
[package]
name = "furinapp-queries"
version = "0.1.0"
description = "Today I wanted to eat a *quaso*."
license = "MIT"
edition = "2021"
```

All fields specified in this section will be directly copied to the `[package]` section of the generated crate's Cargo.toml. This gives you complete control over the package metadata and configuration of the generated crate.

## Workspace dependencies
The `use-workspace-deps` option allows you to integrate the generated crate with your workspace's dependency management:

```toml
# Use workspace dependencies from the current directory's Cargo.toml
use-workspace-deps = true

# Use workspace dependencies from a specific Cargo.toml
use-workspace-deps = "../../Cargo.toml"
```

When this option is set, Clorinde will:
1. Look for dependencies in the specified Cargo.toml file (or `./Cargo.toml` if set to `true`)
2. Set `workspace = true` for any dependencies that exist in the workspace manifest
3. Fall back to regular dependency declarations for packages not found in the workspace

## Custom type mappings
You can configure custom type mappings and their required dependencies using the `types` section:

```toml
[types.crates]
# Dependencies required for custom type mappings
ctypes = { path = "../ctypes" }
postgres_range = { version = "0.11.1", features = ["with-chrono-0_4"] }

[types.mapping]
# Map PostgreSQL types to custom Rust types
"pg_catalog.date" = "ctypes::date::Date"
"pg_catalog.tstzrange" = "postgres_range::Range<chrono::DateTime<chrono::FixedOffset>>"
```

The `types.crates` table specifies any dependencies needed for your custom type mappings. These will be added to the generated crate's `Cargo.toml`.

The `types.mapping` table allows you to map PostgreSQL types to Rust types. You can use this to either override Clorinde's default mappings or add support for PostgreSQL types that aren't supported by default, such as types from extensions.

~~~admonish note
Your custom types must implement the [`ToSql`](https://docs.rs/postgres-types/latest/postgres_types/trait.ToSql.html) and [`FromSql`](https://docs.rs/postgres-types/latest/postgres_types/trait.FromSql.html)
traits from the [`postgres-types`](https://crates.io/crates/postgres-types) crate:

```rust
use postgres_types::{ToSql, FromSql};

impl ToSql for CustomType {
    // ...
}

impl FromSql for CustomType {
    // ...
}
```

See the [custom_types](https://github.com/halcyonnouveau/clorinde/blob/main/examples/custom_types/ctypes/src/date.rs) example for a reference implementation.

This ensures that your types can be properly serialized to and deserialized from PostgreSQL's wire format.
~~~

## Derive traits
You can specify `#[derive]` traits for generated structs using this field.

```toml
[types]
derive-traits = ["Default", "serde::Deserialize"]
```

This will add the `Default` and `serde::Deserialize` traits to **all** structs. If you only want them added to specific structs, see this section in ["Type annotations"](/writing_queries/type_annotations.html#derive-traits).

## Static files
The `static` field allows you to copy or link files into your generated crate directory. This is useful for including files like licenses, build configurations, or other assets that should persist across code generation.

```toml
# Simple copy of files
static = ["LICENSE.txt", "build.rs"]

# Advanced configuration with hard linking
static = [
    { path = "large_asset.bin", hard-link = true },
    "README.md"  # Mixed with simple paths
]
```

When `hard-link = true` is specified, Clorinde will create a hard link instead of copying the file. This is particularly useful for large files to save disk space.
