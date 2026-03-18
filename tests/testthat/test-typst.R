test_font_path <- testthat::test_path("fonts")
cat("Font path for tests:", test_font_path, "\n")

test_that("Valid Typst compile usage", {
  if (!dir.exists(test_font_path)) {
    dir.create(test_font_path)
  }

  markup <- c("= Hello World", "This is a Typst document.")
  typ_file <- typst_write(markup)
  expect_true(file.exists(typ_file))

  pdf_file <- typst_compile(typ_file)
  expect_true(file.exists(pdf_file))

  pdf_file <- tempfile(fileext = ".pdf")
  typst_compile(typ_file, output = pdf_file)
  expect_true(file.exists(pdf_file))

  markup <- c(
    '#set text(font: "Ultra")',
    "= Hello World",
    "This is a Typst document."
  )
  typ_file <- typst_write(markup)
  pdf_file <- typst_compile(typ_file, font_path = test_font_path)
  expect_true(file.exists(pdf_file))

  markup <- c(
    '#set text(font: "Ultra")',
    '#set document(title: "Title of the document")',
    "= Hello World",
    "This is a Typst document."
  )
  typ_file <- typst_write(markup)
  pdf_file <- typst_compile(
    typ_file,
    font_path = test_font_path #,
    #pdf_standard = "ua-1"
  )
  expect_true(file.exists(pdf_file))
})

test_that("Invalid Typst CLI usage", {
  markup <- c("= Hello World", "This is a Typst document.")
  expect_error(
    typst_write(markup, output = "invalid.py"),
    regexp = "`output` must have a .typ extension"
  )

  markup <- c(
    '#set text(font: "Ultra")',
    "= Hello World",
    "This is a Typst document."
  )
  typ_file <- typst_write(markup)
  expect_error(
    typst_compile(
      typ_file,
      pdf_standard = "ua-1"
    ),
    regexp = '"PDF/UA-1 error: missing document title"'
  )
})
