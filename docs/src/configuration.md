# Configuration
Clorinde can be configured using a configuration file (`clorinde.toml` by default) in your project. This file allows you to customise generated code behaviour, specify static files, manage dependencies, and override type mappings.

## Manifest configuration
The `[manifest]` section allows you to configure the entire Cargo.toml for the generated crate:

```toml
[manifest.package]
name = "furinapp-queries"
version = "1.0.0"
description = "Today I wanted to eat a *quaso*."
license = "MIT"
edition = "2021"

[manifest.dependencies]
serde = { version = "1.0", features = ["derive"] }
my_custom_types = { path = "../types" }
```

This gives you complete control over the generated Cargo.toml. Clorinde will automatically merge your configuration with the required PostgreSQL dependencies based on the types found in your SQL queries.

### Dependency merging
Clorinde automatically adds dependencies based on your PostgreSQL schema:
- Core dependencies: `postgres-types`, `postgres-protocol`, `postgres`
- Type-specific dependencies: `chrono`, `uuid`, `serde_json`, etc. (based on column types)
- Async dependencies: `tokio-postgres`, `futures`, `deadpool-postgres` (when async enabled)

Your custom dependencies in `[manifest.dependencies]` will be preserved and merged with these auto-generated ones.

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
You can configure custom type mappings using the `types` section:

```toml
[manifest.dependencies]
# Dependencies required for custom type mappings
ctypes = { path = "../ctypes" }
postgres_range = { version = "0.11.1", features = ["with-chrono-0_4"] }

[types.mapping]
# Map PostgreSQL types to custom Rust types
"pg_catalog.date" = "ctypes::date::Date"
"pg_catalog.tstzrange" = "postgres_range::Range<chrono::DateTime<chrono::FixedOffset>>"
```

Dependencies needed for your custom type mappings should be specified in `[manifest.dependencies]`.

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
derive-traits = ["serde::Serialize", "serde::Deserialize", "Hash"]
```

This will add the the traits to **all** structs. If you only want them added to specific structs, see this section in ["Type annotations"](./writing_queries/type_annotations.html#derive-traits).

~~~admonish note
Adding any `serde` trait will automatically add `serde` as a dependency in the package manifest. This is for backwards compatibility with the deprecated `serialize` config value.
~~~

### Custom PostgreSQL type derive traits
For more granular control in addition to traits in type annotations, you can specify traits that should only be derived for particular [custom PostgreSQL types](./introduction/types.html#custom-postgresql-types):

```toml
[types]
# Applied to all generated structs and postgres types
derive-traits = ["Default"]

[types.derive-traits-mapping]
# Applied to specfic custom postgres types (eg. enums, domains, composites)
fontaine_region = ["serde::Deserialize"]
```

This configuration will add the `Clone` trait to all generated types (and structs), but will only add `serde::Deserialize` to the `fontaine_region` enum.

~~~admonish note
PostgreSQL identifiers (including type names) are case-insensitive unless quoted during creation.
This means that a type created as `CREATE TYPE Fontaine_Region` will be stored as `fontaine_region`
in the PostgreSQL system catalogs. When referencing custom PostgreSQL types in the `derive-traits-mapping`,
you should use the lowercase form unless the type was explicitly created with quotes.
~~~

You can combine global and type-specific derive traits - the traits will be merged for the specified custom PostgreSQL types.

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
