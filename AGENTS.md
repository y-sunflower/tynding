# Repository Guidelines

## Project Structure & Module Organization

This repository is an R package named `tynding` that provides bindings to
Typst through Rust.

- `R/` contains exported R functions and wrappers.
- `src/rust/` contains the Rust implementation, Cargo metadata, and vendored
  Rust dependencies.
- `src/` contains R package build glue such as `Makevars` and C entrypoints.
- `tests/testthat/` contains the R test suite, Typst fixtures, and test fonts.
- `man/` contains generated Rd documentation from roxygen comments.
- `docs/` contains the generated pkgdown site.
- `sandbox/` is for local experiments and generated examples.

## Build, Test, and Development Commands


- Compile Rust source:

```R
devtools::document()
```

- Load latest version:

```R
devtools::load_all()
```

- Run R tests:

```R
devtools::test()
```

- Run Rust tests:


```bash
cd src/rust && cargo test
```


## Coding Style & Naming Conventions

R code uses 2-space indentation and an 80-column line width, as configured in
`air.toml`. Prefer clear snake_case names for R functions, variables, and test
files. Document exported R functions with roxygen2 comments and keep generated
`man/` files in sync when documentation changes.

Rust code in `src/rust/` uses Rust 2021 and requires `rustc >= 1.89.0`. Follow
standard `rustfmt` formatting and keep Rust-facing names consistent with the
R wrappers they support.


