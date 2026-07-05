# Compile a `.typ` file and return the output path.

This function uses the Typst Rust library to compile a `.typ` file to a
supported output format and return the output path.

## Usage

``` r
typst_compile(
  file,
  output = NULL,
  font_path = NULL,
  pdf_standard = NULL,
  output_format = NULL,
  root = NULL,
  ppi = NULL,
  ignore_system_fonts = FALSE,
  ...
)
```

## Arguments

- file:

  Path to an existing `.typ` file.

- output:

  Optional output path. Defaults to the input path with the extension
  implied by the output format.

- font_path:

  Optional path to font files.

- pdf_standard:

  Optional PDF standard specification. Can be a single standard or a
  character vector of compatible standards. Options are: `1.4`, `1.5`,
  `1.6`, `1.7`, `2.0`, `a-1b`, `a-1a`, `a-2b`, `a-2u`, `a-2a`, `a-3b`,
  `a-3u`, `a-3a`, `a-4`, `a-4f`, `a-4e`, `ua-1`. Only used for PDF
  output.

- output_format:

  Optional output format. Supported values are `pdf`, `html`, `png`, and
  `svg`. Defaults to `NULL`, which means "infer from `output` when
  possible, otherwise use `pdf`". For multi-page `png` and `svg`
  outputs, `output` must be a template path containing at least one of
  `{p}`, `{0p}`, or `{t}`.

- root:

  Optional Typst project root. Defaults to the parent directory of
  `file`. When provided, `file` must be contained in that directory's
  subtree.

- ppi:

  Optional pixels per inch value when exporting to png. If NULL, default
  to 144.0.

- ignore_system_fonts:

  Whether to skip system font discovery. Embedded Typst fonts and fonts
  from `font_path` are still available.

- ...:

  Named inputs passed to the Typst document via `sys.inputs`. Each
  argument must be named. Scalar values are passed as-is; other values
  are JSON-encoded.

## Value

Output path, invisibly.
