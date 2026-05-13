# Is valid Typst?

Check that a character vector is valid Typst markup by compiling it. If
no error, it assumes the code is valid.

## Usage

``` r
is_valid_typst(x, error_on_failure = FALSE)
```

## Arguments

- x:

  A character vector

- error_on_failure:

  Whether to raise an error if the code is invalid. Default to `FALSE`.

## Value

Indicates whether the output PDF file exists (for example, if `TRUE`,
then Typst has been compiled successfully).

## Examples

``` r
typst_code <- c("= Hello World", "This is a Typst document.")
is_valid_typst(typst_code) # TRUE
#> [1] TRUE

typst_code <- c("= Hello World", "#This is a Typst document.")
is_valid_typst(typst_code) # FALSE
#> [1] FALSE

if (FALSE) { # \dontrun{
typst_code <- c("= Hello World", "#This is a Typst document.")
is_valid_typst(typst_code, error_on_failure = TRUE) # ERROR
} # }
```
