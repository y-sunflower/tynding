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

  multipage_typ_file <- typst_write(c("= First page", "#pagebreak()", "= Second page"))
  generated_files <- c(generated_files, multipage_typ_file)

  expect_error(
    typst_compile(multipage_typ_file, output_format = "png"),
    regexp = "Multi-page png output requires an `output` path template"
  )

  expect_error(
    typst_compile(multipage_typ_file, output_format = "svg"),
    regexp = "Multi-page svg output requires an `output` path template"
  )

  png_template <- file.path(tempdir(), "multi-{p}.png")
  generated_files <- c(generated_files, file.path(tempdir(), c("multi-1.png", "multi-2.png")))
  png_template_output <- typst_compile(
    multipage_typ_file,
    output = png_template,
    output_format = "png"
  )
  expect_true(png_template_output == png_template)
  expect_true(file.exists(file.path(tempdir(), "multi-1.png")))
  expect_true(file.exists(file.path(tempdir(), "multi-2.png")))

  svg_template <- file.path(tempdir(), "multi-{0p}-of-{t}.svg")
  generated_files <- c(generated_files, file.path(tempdir(), c("multi-1-of-2.svg", "multi-2-of-2.svg")))
  svg_template_output <- typst_compile(
    multipage_typ_file,
    output = svg_template,
    output_format = "svg"
  )
  expect_true(svg_template_output == svg_template)
  expect_true(file.exists(file.path(tempdir(), "multi-1-of-2.svg")))
  expect_true(file.exists(file.path(tempdir(), "multi-2-of-2.svg")))
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
