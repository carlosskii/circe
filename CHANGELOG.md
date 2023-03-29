# Changelog

## [0.0.1] - 2023-03-29

### Added

- This changelog
- `cce-ast` crate
  - Adds a parser for the Circe language
- `cce-lowlevel` crate
  - Adds a parser for low-level Circe instructions
- `cce-stream` crate
  - Adds an `InputStream` struct used across parser crates
- `cce-infer` crate
  - Adds the Circe Inference Engine (CIE)
- `cce-infer-ast` crate
  - Converts the AST from `cce-ast` to a form the CIE can use
- `ccec` crate
  - A command-line interface for the Circe compiler