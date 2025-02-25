# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
