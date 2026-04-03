test_that("Examples", {
  pdf_file <- typst_compile(
    "typst/example-1.typ",
    title = "Quarterly report",
    author = "Joseph",
    persons = list(
      list(name = "Joseph", age = 25),
      list(name = "Justine", age = 24),
      list(name = "Isaac", age = 2)
    )
  )

  expect_true(file.exists(pdf_file))
})
