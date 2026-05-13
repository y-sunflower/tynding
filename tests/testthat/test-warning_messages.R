test_that("Unknown font produces a warning", {
  input_file <- test_path("typst", "warning-messages.typ")

  expect_warning(
    typst_compile(input_file),
    'unknown font family: not a real font
  ┌─ warning-messages.typ:6:17
  │
6 │ #set text(font: "not a real font")
  │                 ^^^^^^^^^^^^^^^^^ ',
    fixed = TRUE
  )
})
