test_that("Valid Typst compile usage", {
  generated_files <- character(0)
  on.exit(unlink(generated_files), add = TRUE)

  markup <- c("= Hello World", "This is a Typst document.")
  typ_file <- typst_write(markup)
  generated_files <- c(generated_files, typ_file)
  expect_true(file.exists(typ_file))

  pdf_file <- typst_compile(typ_file)
  generated_files <- c(generated_files, pdf_file)
  expect_true(file.exists(pdf_file))

  pdf_file <- tempfile(fileext = ".pdf")
  generated_files <- c(generated_files, pdf_file)
  typst_compile(typ_file, output = pdf_file)
  expect_true(file.exists(pdf_file))

  html_file <- typst_compile(typ_file, output_format = "html")
  generated_files <- c(generated_files, html_file)
  expect_true(file.exists(html_file))

  png_file <- typst_compile(typ_file, output_format = "png")
  generated_files <- c(generated_files, png_file)
  expect_true(file.exists(png_file))

  svg_file <- typst_compile(typ_file, output_format = "svg")
  generated_files <- c(generated_files, svg_file)
  expect_true(file.exists(svg_file))
})

test_that("Invalid Typst CLI usage", {
  generated_files <- character(0)
  on.exit(unlink(generated_files), add = TRUE)

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
  generated_files <- c(generated_files, typ_file)
  expect_error(
    typst_compile(
      typ_file,
      pdf_standard = "ua-1"
    ),
    regexp = '"PDF/UA-1 error: missing document title"'
  )

  expect_error(
    typst_compile(
      typ_file,
      pdf_standard = "1.7",
      output_format = "html"
    ),
    regexp = "`pdf_standard` is only supported when `output_format` is `pdf`"
  )
})
