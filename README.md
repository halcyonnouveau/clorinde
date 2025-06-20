<img src="https://raw.githubusercontent.com/halcyonnouveau/clorinde/refs/heads/main/docs/assets/clorinde_hat.png" alt="cool hat" style="max-width: 100%;">

# Clorinde

[![crate](https://img.shields.io/crates/v/clorinde.svg)](https://crates.io/crates/clorinde)
[![docs](https://img.shields.io/badge/book-latest-blue?logo=mdbook)](https://halcyonnouveau.github.io/clorinde/)
![license](https://img.shields.io/badge/License-APACHE--2.0%2FMIT-blue)
[![dependency status](https://deps.rs/repo/github/halcyonnouveau/clorinde/status.svg)](https://deps.rs/repo/github/halcyonnouveau/clorinde)

Clorinde generates type-checked Rust interfaces from PostgreSQL queries, with an emphasis on compile-time safety and high performance. It works by preparing your queries against an actual database and then running an extensive validation suite on them. Rust code is then generated into a separate crate, which can be imported and used in your project.

The basic premise is thus to:

1. Write your PostgreSQL queries.
2. Use Clorinde to generate a crate with type-safe interfaces to those queries.
3. Import and use the generated code in your project.

You can learn more about Clorinde by reading the [book](https://halcyonnouveau.github.io/clorinde/), or you can get a quickstart by looking at the [examples](https://halcyonnouveau.github.io/clorinde/examples.html).

> [!NOTE]
> Clorinde is a fork of [Cornucopia](https://github.com/cornucopia-rs/cornucopia) which enhances the original with an improved architecture and expanded capabilities. Visit the [migration guide](https://halcyonnouveau.github.io/clorinde/introduction/migration_from_cornucopia.html) if you are moving over an existing codebase with Cornucopia.

## Key Features

- **Type Safety** - Catch SQL errors at compile time and get catch errors before runtime with powerful diagnostics.
- **SQL-First** - Write plain SQL queries, get generated Rust code. No ORMs or query builders, just the SQL you know and love.
- **Fast** - Performance close to hand-written `rust-postgres` code.
- **Flexible** - Works with sync/async code and connection pools.
- **PostgreSQL Native** - Full support for custom types, enums, and arrays. Leverage PostgreSQL's advanced features without compromise.
- **Custom Types** - Map database types to your own Rust structs.

## Installation

Install with:

```bash
cargo install clorinde
```

## Quick Example
Write your PostgreSQL queries with annotations and named parameters:
```sql
-- queries/authors.sql

--! insert_author
INSERT INTO authors
    (first_name, last_name, country)
VALUES
    (:first_name, :last_name, :country);

--! authors
SELECT first_name, last_name, country FROM authors;
```

Generate the crate with `clorinde`, then you can import it into your project after adding it to your `Cargo.toml`:
```toml
clorinde = { path = "./clorinde" }
```

And use the generated crate in your code:
```rust
use clorinde::queries::authors::{authors, insert_author};

insert_author.bind(&client, "Agatha", "Christie", "England");

let all_authors = authors().bind(&client).all();

for author in all_authors {
  println!("[{}] {}, {}",
    author.country,
    author.last_name.to_uppercase(),
    author.first_name
  )
}
```

For more examples go to the [examples](https://github.com/halcyonnouveau/clorinde/tree/main/examples) directory, or head over to the [book](https://halcyonnouveau.github.io/clorinde/) to learn more.

## MSRV

This crate uses Rust 2021 edition, which requires at least version 1.62.1.

## Prior Art

- [sqlc](https://github.com/sqlc-dev/sqlc) (Go) - Generate type-safe code from SQL
- [Kanel](https://github.com/kristiandupont/kanel) (TypeScript) - Generate TypeScript types from Postgres
- [jOOQ](https://github.com/jOOQ/jOOQ) (Java) - Generate typesafe SQL from your database schema

## License

Licensed under either of
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
