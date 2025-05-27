# Introduction
Clorinde is a tool powered by [`rust-postgres`](https://github.com/sfackler/rust-postgres) designed to generate type-checked Rust interfaces from your PostgreSQL queries. It works by preparing your queries against an actual database and then running an extensive validation suite on them. Once the queries are prepared and validated, Rust code is generated into a module, which can be imported and used in your project. The basic premise is thus to:
1. Write your PostgreSQL queries.
2. Use Clorinde to generate Rust code.
3. Use the generated code in your project.

Compared to other Rust database interfaces, Clorinde's approach has the benefits of being simple to understand while also generating code that is both ergonomic and free of heavy macros or complex generics. Since Clorinde generates plain Rust structs, you can also easily build upon the generated items.

Here are some defining features:
* SQL-first. Your SQL is the only source of truth. No intricate ORM.
* Powerful query validation. Catch errors before runtime, with powerful (and pretty) diagnostics.
* Supports custom user types (composites, domains, and enums) and one-dimensional arrays.
* Sync and async driver support, with optional pooling.
* Ergonomic non-allocating row mapping.
* Granular type nullity control.
* Available as a library and a CLI.
* As close to native `rust-postgres` performance as we can make it.

```admonish info
If you just want to get started without having to read all of this, you can take a look at our [examples](examples.html).

*This book is pretty short and to the point though, so you should probably at least take a glance.*
```
