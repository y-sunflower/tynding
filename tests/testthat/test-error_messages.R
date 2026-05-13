test_that("Unknown variable produces a rich error message", {
  input_file <- test_path("typst", "error-unknown-variable.typ")

  expect_error(
    typst_compile(input_file),
    "error: unknown variable: undefined_variable
  ┌─ error-unknown-variable.typ:2:2
  │
2 │ #undefined_variable
  │  ^^^^^^^^^^^^^^^^^^",
    fixed = TRUE
  )
})

test_that("Type mismatch on set rule produces a rich error message", {
  input_file <- test_path("typst", "error-type-mismatch.typ")

  expect_error(
    typst_compile(input_file),
    'error: expected length, found string
  ┌─ error-type-mismatch.typ:1:17
  │
1 │ #set text(size: "big")
  │                 ^^^^^',
    fixed = TRUE
  )
})

test_that("Missing import file produces a rich error message", {
  input_file <- test_path("typst", "error-missing-import.typ")
  searched_path <- file.path(
    normalizePath(test_path("typst"), winslash = "/"),
    "does-not-exist.typ"
  )

  error <- expect_error(typst_compile(input_file))
  error_message <- conditionMessage(error)
  error_message <- gsub("\r\n", "\n", error_message, fixed = TRUE)
  error_message <- gsub("\\", "/", error_message, fixed = TRUE)
  error_message <- gsub("(searched at //?/", "(searched at ", error_message, fixed = TRUE)

  expect_equal(
    error_message,
    sprintf(
      'error: file not found (searched at %s)
  ┌─ error-missing-import.typ:1:9
  │
1 │ #import "does-not-exist.typ": *
  │         ^^^^^^^^^^^^^^^^^^^^',
      searched_path
    )
  )
})

test_that("More complex error message", {
  input_file <- test_path("typst", "error-complete.typ")

  expect_error(
    typst_compile(input_file),
    "error: unknown variable: this-variable-doesnt-exist
  ┌─ error-complete.typ:6:2
  │
6 │ #this-variable-doesnt-exist
  │  ^^^^^^^^^^^^^^^^^^^^^^^^^^
hint: if you meant to use subtraction, try adding spaces around the minus signs: `this - variable - doesnt - exist`",
    fixed = TRUE
  )
})
