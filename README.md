# tynding: Typst bindings for R

`tynding` is an R package that compiles Typst documents from R through a Rust backend. It exposes a small API for writing and compiling `.typ` files.

<br>

## Installation

Install it from GitHub with `pak`:

```r
#install.packages("pak")
pak::pak("y-sunflower/tynding")
```

The package builds Rust code during installation, so you need:

- `R >= 4.2`
- `cargo`
- `rustc >= 1.89.0`
- `xz`

If you prefer `remotes`, this also works:

```r
#install.packages("remotes")
remotes::install_github("y-sunflower/tynding")
```

<br>

## Quick start

```r
library(tynding)

markup <- c(
  '#set document(title: "hello from tynding")',
  "= hello world",
  "this PDF was compiled from R."
)

typ_file <- typst_write(markup)
pdf_file <- typst_compile(typ_file)

pdf_file
```

`typst_write()` writes a character vector to a `.typ` file. `typst_compile()` compiles that file to PDF and returns the output path. If you do not pass `output`, the PDF is written next to the source file with the same name and a `.pdf` extension.

<br>

## Features

- fonts: pass `font_path` to load `.ttf`, `.otf`, or `.ttc` files from a directory before compiling.

```r
markup <- c(
  '#set document(title: "custom font example")',
  '#set text(font: "Ultra")',
  "= hello world"
)

typ_file <- typst_write(markup)
pdf_file <- typst_compile(typ_file, font_path = "path/to/fonts")
```

- pdf standard: pass `pdf_standard` to request a Typst PDF profile such as `"1.7"`, `"2.0"`, `"a-2b"`, or `"ua-1"`.

```r
markup <- c(
  '#set document(title: "accessible PDF")',
  "= hello world"
)

typ_file <- typst_write(markup)
pdf_file <- typst_compile(typ_file, pdf_standard = "ua-1")
```

For `ua-1`, your document needs a title. Unsupported or invalid standards raise an error.

<br>

## Related project

[`typr`](https://christophertkenny.com/typr/) is a package with a very similar goal, but it works quite differently under the hood. `typr` compiles your document using the Typst/Quarto CLI, while `tynding` uses the Typst compiler itself via the Typst Rust library.

Both have their pros and cons, but `tynding` is designed to be more portable in the sense that you don’t have to worry about installing Typst separately and/or adding it to your PATH.
