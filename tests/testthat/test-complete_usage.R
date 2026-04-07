test_that("Example 1: Pass additional named arguments to Typst, with JSON encoding", {
  pdf_file <- typst_compile(
    "typst/example-1.typ",
    output = "typst/example-11.pdf",
    title = "Quarterly report",
    author = "Joseph",
    persons = list(
      list(name = "Joseph", age = 25),
      list(name = "Justine", age = 24),
      list(name = "Isaac", age = 2)
    )
  )

  expect_true(pdf_file == "typst/example-11.pdf")
  expect_true(file.exists(pdf_file))
})

test_that("Example 2: A simple Typst document", {
  pdf_file <- typst_compile("typst/example-2.typ")
  expect_true(file.exists(pdf_file))
})

test_that("Example 3: Use font from a parent directory", {
  pdf_file <- typst_compile(
    "typst/subdir/example-3.typ",
    root = "typst",
    font_path = "fonts"
  )
  expect_true(file.exists(pdf_file))
})

test_that("Example 4: More complex usage", {
  pdf_file <- typst_compile(
    "typst/example-4.typ",
    output_format = "png",
    food = "cookies"
  )
  expect_true(file.exists(pdf_file))
})
