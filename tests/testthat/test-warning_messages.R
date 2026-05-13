test_that("Unknown font produces a warning", {
  skip_on_cran()
  input_file <- test_path("typst", "warning-messages.typ")

  expect_warning(
    typst_compile(input_file, output = tempfile(fileext = ".pdf")),
    'warning: unknown font family: not a real font
  ┌─ warning-messages.typ:6:17
  │
6 │ #set text(font: "not a real font")
  │                 ^^^^^^^^^^^^^^^^^',
    fixed = TRUE
  )
})

test_that("Unknown font produces a different warning", {
  skip_on_cran()
  input_file <- test_path("typst", "warning-other-message.typ")

  expect_warning(
    typst_compile(input_file, output = tempfile(fileext = ".pdf")),
    'warning: no text within stars
  ┌─ warning-other-message.typ:15:1
  │
15 │ **test
  │ ^^
hint: using multiple consecutive stars (e.g. **) has no additional effect',
    fixed = TRUE
  )
})
