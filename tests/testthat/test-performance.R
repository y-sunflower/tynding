# We skip tests on R CMD CHECK here because path resolution seems a bit too complex

test_that("Check average performance", {
  skip_on_cran()

  input_file <- test_path("typst", "performance.typ")

  timings <- replicate(30, system.time(typst_compile(input_file))["elapsed"])

  expect_lt(mean(timings), 0.05)
})
