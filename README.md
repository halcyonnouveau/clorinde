![cool hat](https://raw.githubusercontent.com/halcyonnouveau/clorinde/refs/heads/main/docs/assets/clorinde_hat.png)

# Clorinde

<a href="https://crates.io/crates/clorinde">
  <img src="https://img.shields.io/crates/v/clorinde.svg?style=flat-square"
  alt="Crates.io version" />
</a>

<a href="https://halcyonnouveau.github.io/clorinde/">
  <img src="https://img.shields.io/badge/book-latest-blue?logo=mdbook&style=flat-square" alt="book">
</a>

<a href="https://github.com/halcyonnouveau/clorinde#License">
  <img src="https://img.shields.io/badge/License-APACHE--2.0%2FMIT-blue?style=flat-square" alt="License">
</a>

Clorinde generates type-checked Rust interfaces from PostgreSQL queries, with an emphasis on compile-time safety and high performance. It is a fork of [Cornucopia](https://github.com/cornucopia-rs/cornucopia) that enhances the original with an improved architecture and expanded capabilities.

## Key Features

* SQL-first approach with powerful query validation
* Sync and async driver support with optional connection pooling
* Non-allocating row mapping
* Available as both a library and CLI tool
* Close to native `rust-postgres` performance
* Complete support for custom PostgreSQL types (composites, domains, and enums)
* Custom Rust type mapping
* One-dimensional array handling for all supported types
* Granular type nullity control

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
INSERT INTO Author
    (first_name, last_name, country)
VALUES
    (:first_name, :last_name, :country);

--! authors
SELECT first_name, last_name, country FROM Author;
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

For more examples go to the [/examples](https://github.com/halcyonnouveau/clorinde/tree/main/examples) directory, or head over to the [book](https://halcyonnouveau.github.io/clorinde/) to learn more.

## MSRV

This crate uses Rust 2021 edition, which requires at least version 1.62.1.

## Similar Libraries

- [sqlc](https://github.com/sqlc-dev/sqlc) (Go) - Generate type-safe code from SQL
- [Kanel](https://github.com/kristiandupont/kanel) (TypeScript) - Generate TypeScript types from Postgres
- [jOOQ](https://github.com/jOOQ/jOOQ) (Java) - Generate typesafe SQL from your database schema

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
