# We skip tests on R CMD CHECK here because path resolution seems a bit too complex
# in order to test the "root" argument of typst_compile().

test_that("Example 1: Pass additional named arguments to Typst, with JSON encoding", {
  skip_on_cran()
  input_file <- test_path("typst", "example-1.typ")
  output_file <- tempfile(fileext = ".pdf")

  pdf_file <- typst_compile(
    input_file,
    output = output_file,
    title = "Quarterly report",
    author = "Joseph",
    persons = list(
      list(name = "Joseph", age = 25),
      list(name = "Justine", age = 24),
      list(name = "Isaac", age = 2)
    )
  )

  expect_true(pdf_file == output_file)
  expect_true(file.exists(pdf_file))
  unlink(output_file)
})

test_that("Example 2: A simple Typst document", {
  input_file <- test_path("typst", "example-2.typ")

  pdf_file <- typst_compile(input_file)
  expect_true(file.exists(pdf_file))
  unlink(pdf_file)
})

test_that("Example 3: Use font from a parent directory", {
  skip_on_cran()
  input_file <- test_path("typst", "subdir", "example-3.typ")
  root_dir <- test_path("typst")
  font_path <- test_path("fonts")
  output_file <- tempfile(fileext = ".pdf")

  pdf_file <- typst_compile(
    input_file,
    output = output_file,
    root = root_dir,
    font_path = font_path
  )

  expect_true(file.exists(pdf_file))
  expect_true(pdf_file == output_file)
  unlink(output_file)
})

test_that("Example 4: More complex usage", {
  skip_on_cran()
  input_file <- test_path("typst", "example-4.typ")
  output_file <- tempfile(fileext = ".png")

  png_file <- typst_compile(
    input_file,
    output = output_file,
    output_format = "png",
    food = "cookies"
  )
  expect_true(file.exists(png_file))
  expect_true(png_file == output_file)
  unlink(output_file)
})
