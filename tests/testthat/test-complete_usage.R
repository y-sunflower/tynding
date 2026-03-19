test_that("Complete usage", {
  markup <- c(
    '#set page(width: 10cm, height: 4cm)',
    '#let title = sys.inputs.at("title")',
    '#let author = sys.inputs.at("author")',
    '#let persons = json.decode(sys.inputs.at("persons"))',
    '= #title',
    '*Author:* #author',
    '#for person in persons [',
    '  #strong(person.name) is #text(fill: red, weight: "bold", [#person.age]) years old. \ ',
    ']'
  )
  typst_file <- tempfile(fileext = ".typ")
  writeLines(markup, typst_file)

  pdf_file <- typst_compile(
    typst_file,
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
