#' @title Compile a `.typ` file and return the output path.
#'
#' @description This function uses the Typst Rust library to compile a
#' `.typ` file to a supported output format and return the output path.
#'
#' @param file Path to an existing `.typ` file.
#' @param output Optional output path. Defaults to the input path with the
#' extension implied by the output format.
#' @param font_path Optional path to font files.
#' @param pdf_standard Optional PDF standard specification. Options are: : `1.4`,
#' `1.5`, `1.6`, `1.7`, `2.0`, `a-1b`, `a-1a`, `a-2b`, `a-2u`, `a-2a`, `a-3b`,
#' `a-3u`, `a-3a`, `a-4`, `a-4f`, `a-4e`, `ua-1`. Only used for PDF output.
#' @param output_format Optional output format. Supported values are `pdf`,
#' `html`, `png`, and `svg`. Defaults to `NULL`, which means "infer from
#' `output` when possible, otherwise use `pdf`". For multi-page `png` and `svg`
#' outputs, `output` must be a template path containing at least one of
#' `{p}`, `{0p}`, or `{t}`.
#' @param root Optional Typst project root. Defaults to the parent directory of
#' `file`. When provided, `file` must be contained in that directory's subtree.
#' @param ppi Optional pixels per inch value when exporting to png. If NULL,
#' default to 144.0.
#' @param ... Named inputs passed to the Typst document via `sys.inputs`.
#' Each argument must be named. Scalar values are passed as-is; other values
#' are JSON-encoded.
#'
#' @return Output path, invisibly.
#'
#' @export
typst_compile <- function(
  file,
  output = NULL,
  font_path = NULL,
  pdf_standard = NULL,
  output_format = NULL,
  root = NULL,
  ppi = NULL,
  ...
) {
  inputs_list <- list(...)

  inputs <- vapply(
    names(inputs_list),
    function(name) {
      value <- inputs_list[[name]]

      if (is.character(value) && length(value) == 1) {
        paste0(name, "=", value)
      } else {
        json <- jsonlite::toJSON(value, auto_unbox = TRUE)
        paste0(name, "=", json)
      }
    },
    character(1)
  )

  invisible(typst_compile_rust(
    file = file,
    output = output,
    font_path = font_path,
    pdf_standard = pdf_standard,
    output_format = output_format,
    root = root,
    ppi = ppi,
    inputs
  ))
}
