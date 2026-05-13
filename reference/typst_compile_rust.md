# Compile a `.typ` file and return the output path.

This function uses the Typst Rust library to compile a `.typ` file to a
supported output format and return the output path.

## Usage

``` r
typst_compile_rust(
  file,
  output = NULL,
  font_path = NULL,
  pdf_standard = NULL,
  output_format = NULL,
  root = NULL,
  inputs = NULL,
  ppi = NULL
)
```

## Arguments

- file:

  Path to an existing `.typ` file.

- output:

  Optional output path.

- font_path:

  Optional path to font files.

- pdf_standard:

  Optional PDF standard specification.

- output_format:

  Optional output format.

- root:

  Optional Typst project root. If `None`, it defaults to the parent
  directory of `file`. When provided, `file` must be contained in the
  root directory's subtree.

- inputs:

  Optional additional sys inputs parameters.

- ppi:

  Optional pixels per inch value when exporting to png. If NULL, default
  to 144.0.

## Value

Output path
