# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.15.2...clorinde-v0.16.0) - 2025-06-05

### Added

- add style setting to configure enum variant style
- add search_path option to Live action and set in Postgres client ([#110](https://github.com/halcyonnouveau/clorinde/pull/110))
- allow user custom rust types for all pg types ([#109](https://github.com/halcyonnouveau/clorinde/pull/109))

## [0.15.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.15.1...clorinde-v0.15.2) - 2025-05-29

### Fixed

- codegen directory rename on windows

## [0.15.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.15.0...clorinde-v0.15.1) - 2025-05-26

### Added

- directory based query modules ([#99](https://github.com/halcyonnouveau/clorinde/pull/99))

## [0.15.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.4...clorinde-v0.15.0) - 2025-05-12

### Added

- allow custom field attributes in type mappings ([#96](https://github.com/halcyonnouveau/clorinde/pull/96))
- Ignore files with names prefixed with `_` ([#90](https://github.com/halcyonnouveau/clorinde/pull/90))

### Fixed

- ensure generated crate is only deleted when new generation succeeds ([#95](https://github.com/halcyonnouveau/clorinde/pull/95))

## [0.14.4](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.3...clorinde-v0.14.4) - 2025-04-14

### Added

- improve error handling with `try_get` in query extractors

### Fixed

- *(codegen)* make "chrono" and "time" features mutually exclusive ([#88](https://github.com/halcyonnouveau/clorinde/pull/88))

## [0.14.3](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.2...clorinde-v0.14.3) - 2025-04-03

### Added

- overwrite generated dep from config
- better error message when docker not installed
- add builder pattern for config

## [0.14.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.1...clorinde-v0.14.2) - 2025-03-28

### Fixed

- context aware bind parsing

## [0.14.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.0...clorinde-v0.14.1) - 2025-03-27

### Fixed

- `time` feature defined multiple times

## [0.14.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.13.2...clorinde-v0.14.0) - 2025-03-21

### Added

- add `types.type-traits-mapping` to set traits on specific postgres types

## [0.13.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.13.1...clorinde-v0.13.2) - 2025-03-07

### Fixed

- add serde to chrono and uuid features

## [0.13.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.13.0...clorinde-v0.13.1) - 2025-02-27

### Fixed

- adding custom deps without type mapping (#61)

### Breaking

- `ctypes` is no longer a default custom type mapping crate

## [0.13.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.12.1...clorinde-v0.13.0) - 2025-02-25

### Added

- add derive traits (#58)

## [0.12.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.12.0...clorinde-v0.12.1) - 2025-02-22

### Added

- add query doc strings (#55)

## [0.12.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.4...clorinde-v0.12.0) - 2025-02-16

### Added

- Some CLI improvements (#54)
- feat; add `use-workspace-deps` option ([#50](https://github.com/halcyonnouveau/clorinde/pull/50))

### Fixed

- fix; config defaults

### Refactor

- refactor; type register for better custom type support

## [0.11.4](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.3...clorinde-v0.11.4) - 2025-02-07

### Added

- feat; add static files config ([#49](https://github.com/halcyonnouveau/clorinde/pull/49))
- feat; add prompt for generating on a non-default directory

## [0.11.3](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.2...clorinde-v0.11.3) - 2025-01-29

### Fixed

- publish to specific repo wasn't supported in clorinde.toml (#40)

### Refactor

- use quote crate instead of codegen_template and run `cargo fmt` after generation (#35)

### Added

- add citext and other extension friends (#44)

## [0.11.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.1...clorinde-v0.11.2) - 2025-01-23

### Fixed

- lifetimes and generics (#36)

## [0.11.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.0...clorinde-v0.11.1) - 2025-01-21

### Fixed

- add serde for serialize without json (#27)
- Don't force enable optional dependencies if wasm-async is enabled (#19)
- Detect borrowed type based on std Rust types ([#17](https://github.com/halcyonnouveau/clorinde/pull/17))
- Only depend on "ctypes" crate if it is referenced ([#18](https://github.com/halcyonnouveau/clorinde/pull/18))

### Other

- rename settings parameter to config (#24)

### Refactor

- remove async-trait dependency (#28)

## [0.11.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.10.2...clorinde-v0.11.0) - 2025-01-12

### Added

- add bpchar to string types (#14)
- clorinde.toml adds to generated crate package (#11)
- add optional time feature

## [0.10.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.10.1...clorinde-v0.10.2) - 2025-01-07

### Fixed

- Don't generate imports specific to async for the sync client
- Clippy warnings in generated code
- fix warning placment
