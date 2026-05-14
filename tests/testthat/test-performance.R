# We skip tests on R CMD CHECK here because path resolution seems a bit too complex

test_that("Check average performance", {
  skip_on_cran()

  output <- tempfile(fileext = ".pdf")
  input_file <- test_path("typst", "performance.typ")

  timings <- replicate(
    10,
    system.time(typst_compile(input_file, output = output))["elapsed"]
  )

  expect_lt(mean(timings), 0.05)
  expect_true(file.exists(output))
  on.exit(unlink(output))
})
