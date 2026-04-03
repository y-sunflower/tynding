test_that("Example 1", {
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

test_that("Example 2", {
  pdf_file <- typst_compile("typst/example-2.typ")
  expect_true(file.exists(pdf_file))
})

test_that("Example 3", {
  old_wd <- setwd(test_path())
  on.exit(setwd(old_wd), add = TRUE)

  pdf_file <- typst_compile(
    "typst/subdir/example-3.typ",
    root = "typst",
    font_path = "fonts"
  )
  expect_true(file.exists(pdf_file))
})
