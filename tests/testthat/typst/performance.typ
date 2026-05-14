

#let title = "Quarterly report"
#let author = "Pepito"
#let report_date = "today"
#let revision = "v3"

#set document(title: title, author: author)
#set page(
  paper: "a4",
  margin: (x: 2cm, y: 2cm),
  header: [#text(size: 8pt, fill: gray)[#title — rev #revision]],
  footer: context [
    #set align(center)
    #text(size: 8pt, fill: gray)[
      Page #counter(page).display() of #counter(page).final().first()
    ]
  ],
)
#set text(size: 10pt)
#set heading(numbering: "1.1")
#show heading.where(level: 1): set text(size: 16pt, weight: "bold")
#show heading.where(level: 2): set text(size: 13pt, weight: "bold")

// ---- Title block ----
#align(center)[
  #text(size: 22pt, weight: "bold")[#title] \
  #v(0.3em)
  #text(size: 11pt, fill: rgb("#555"))[#author · #report_date · rev #revision]
]

#v(1em)
#line(length: 100%, stroke: 0.5pt + gray)

// ---- Section 1: Executive summary ----
= Executive summary

This is a synthetic report generated to benchmark `tynding` against the
`typst` CLI. It exercises sys inputs, JSON decoding, table generation,
math, and a small amount of layout work.

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 1em,
  rect(width: 100%, inset: 8pt, fill: rgb("#fef3c7"))[
    *Records* \
  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dbeafe"))[
    *Team size* \
  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dcfce7"))[
    *Revision* \
    #text(size: 18pt, weight: "bold")[#revision]
  ],
)

// ---- Section 2: Data table ----
= Field measurements

The table below is built from a dataframe passed via `sys.inputs`.



// ---- Section 3: Math ----
= Some math

The treated area is computed as
$ A_"treated" = sum_(i=1)^n a_i dot phi(s_i) $
where $phi(s_i) = 1$ when water is present and $0$ otherwise.

For a quadratic approximation:
$ y = beta_0 + beta_1 x + beta_2 x^2 + epsilon, quad epsilon tilde cal(N)(0, sigma^2) $

// ---- Section 4: Team ----
= Team



// ---- Section 5: Lorem-ish content over a few pages ----
= Detailed notes

#for i in range(1, 4) [
  == Note #i

  #lorem(80)

  #lorem(60)
]

// ---- Section 6: Nested table from data again, different layout ----
= Appendix: raw data echo


// ---- Title block ----
#align(center)[
  #text(size: 22pt, weight: "bold")[#title] \
  #v(0.3em)
  #text(size: 11pt, fill: rgb("#555"))[#author · #report_date · rev #revision]
]

#v(1em)
#line(length: 100%, stroke: 0.5pt + gray)

// ---- Section 1: Executive summary ----
= Executive summary

This is a synthetic report generated to benchmark `tynding` against the
`typst` CLI. It exercises sys inputs, JSON decoding, table generation,
math, and a small amount of layout work.

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 1em,
  rect(width: 100%, inset: 8pt, fill: rgb("#fef3c7"))[
    *Records* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dbeafe"))[
    *Team size* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dcfce7"))[
    *Revision* \
    #text(size: 18pt, weight: "bold")[#revision]
  ],
)

// ---- Section 2: Data table ----
= Field measurements

The table below is built from a dataframe passed via `sys.inputs`.



// ---- Section 3: Math ----
= Some math

The treated area is computed as
$ A_"treated" = sum_(i=1)^n a_i dot phi(s_i) $
where $phi(s_i) = 1$ when water is present and $0$ otherwise.

For a quadratic approximation:
$ y = beta_0 + beta_1 x + beta_2 x^2 + epsilon, quad epsilon tilde cal(N)(0, sigma^2) $

// ---- Section 4: Team ----
= Team



// ---- Section 5: Lorem-ish content over a few pages ----
= Detailed notes

#for i in range(1, 4) [
  == Note #i

  #lorem(80)

  #lorem(60)
]

// ---- Section 6: Nested table from data again, different layout ----
= Appendix: raw data echo


// ---- Title block ----
#align(center)[
  #text(size: 22pt, weight: "bold")[#title] \
  #v(0.3em)
  #text(size: 11pt, fill: rgb("#555"))[#author · #report_date · rev #revision]
]

#v(1em)
#line(length: 100%, stroke: 0.5pt + gray)

// ---- Section 1: Executive summary ----
= Executive summary

This is a synthetic report generated to benchmark `tynding` against the
`typst` CLI. It exercises sys inputs, JSON decoding, table generation,
math, and a small amount of layout work.

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 1em,
  rect(width: 100%, inset: 8pt, fill: rgb("#fef3c7"))[
    *Records* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dbeafe"))[
    *Team size* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dcfce7"))[
    *Revision* \
    #text(size: 18pt, weight: "bold")[#revision]
  ],
)

// ---- Section 2: Data table ----
= Field measurements

The table below is built from a dataframe passed via `sys.inputs`.



// ---- Section 3: Math ----
= Some math

The treated area is computed as
$ A_"treated" = sum_(i=1)^n a_i dot phi(s_i) $
where $phi(s_i) = 1$ when water is present and $0$ otherwise.

For a quadratic approximation:
$ y = beta_0 + beta_1 x + beta_2 x^2 + epsilon, quad epsilon tilde cal(N)(0, sigma^2) $

// ---- Section 4: Team ----
= Team



// ---- Section 5: Lorem-ish content over a few pages ----
= Detailed notes

#for i in range(1, 4) [
  == Note #i

  #lorem(80)

  #lorem(60)
]

// ---- Section 6: Nested table from data again, different layout ----
= Appendix: raw data echo


// ---- Title block ----
#align(center)[
  #text(size: 22pt, weight: "bold")[#title] \
  #v(0.3em)
  #text(size: 11pt, fill: rgb("#555"))[#author · #report_date · rev #revision]
]

#v(1em)
#line(length: 100%, stroke: 0.5pt + gray)

// ---- Section 1: Executive summary ----
= Executive summary

This is a synthetic report generated to benchmark `tynding` against the
`typst` CLI. It exercises sys inputs, JSON decoding, table generation,
math, and a small amount of layout work.

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 1em,
  rect(width: 100%, inset: 8pt, fill: rgb("#fef3c7"))[
    *Records* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dbeafe"))[
    *Team size* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dcfce7"))[
    *Revision* \
    #text(size: 18pt, weight: "bold")[#revision]
  ],
)

// ---- Section 2: Data table ----
= Field measurements

The table below is built from a dataframe passed via `sys.inputs`.



// ---- Section 3: Math ----
= Some math

The treated area is computed as
$ A_"treated" = sum_(i=1)^n a_i dot phi(s_i) $
where $phi(s_i) = 1$ when water is present and $0$ otherwise.

For a quadratic approximation:
$ y = beta_0 + beta_1 x + beta_2 x^2 + epsilon, quad epsilon tilde cal(N)(0, sigma^2) $

// ---- Section 4: Team ----
= Team



// ---- Section 5: Lorem-ish content over a few pages ----
= Detailed notes

#for i in range(1, 4) [
  == Note #i

  #lorem(80)

  #lorem(60)
]

// ---- Section 6: Nested table from data again, different layout ----
= Appendix: raw data echo


// ---- Title block ----
#align(center)[
  #text(size: 22pt, weight: "bold")[#title] \
  #v(0.3em)
  #text(size: 11pt, fill: rgb("#555"))[#author · #report_date · rev #revision]
]

#v(1em)
#line(length: 100%, stroke: 0.5pt + gray)

// ---- Section 1: Executive summary ----
= Executive summary

This is a synthetic report generated to benchmark `tynding` against the
`typst` CLI. It exercises sys inputs, JSON decoding, table generation,
math, and a small amount of layout work.

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 1em,
  rect(width: 100%, inset: 8pt, fill: rgb("#fef3c7"))[
    *Records* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dbeafe"))[
    *Team size* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dcfce7"))[
    *Revision* \
    #text(size: 18pt, weight: "bold")[#revision]
  ],
)

// ---- Section 2: Data table ----
= Field measurements

The table below is built from a dataframe passed via `sys.inputs`.



// ---- Section 3: Math ----
= Some math

The treated area is computed as
$ A_"treated" = sum_(i=1)^n a_i dot phi(s_i) $
where $phi(s_i) = 1$ when water is present and $0$ otherwise.

For a quadratic approximation:
$ y = beta_0 + beta_1 x + beta_2 x^2 + epsilon, quad epsilon tilde cal(N)(0, sigma^2) $

// ---- Section 4: Team ----
= Team



// ---- Section 5: Lorem-ish content over a few pages ----
= Detailed notes

#for i in range(1, 4) [
  == Note #i

  #lorem(80)

  #lorem(60)
]

// ---- Section 6: Nested table from data again, different layout ----
= Appendix: raw data echo


// ---- Title block ----
#align(center)[
  #text(size: 22pt, weight: "bold")[#title] \
  #v(0.3em)
  #text(size: 11pt, fill: rgb("#555"))[#author · #report_date · rev #revision]
]

#v(1em)
#line(length: 100%, stroke: 0.5pt + gray)

// ---- Section 1: Executive summary ----
= Executive summary

This is a synthetic report generated to benchmark `tynding` against the
`typst` CLI. It exercises sys inputs, JSON decoding, table generation,
math, and a small amount of layout work.

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 1em,
  rect(width: 100%, inset: 8pt, fill: rgb("#fef3c7"))[
    *Records* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dbeafe"))[
    *Team size* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dcfce7"))[
    *Revision* \
    #text(size: 18pt, weight: "bold")[#revision]
  ],
)

// ---- Section 2: Data table ----
= Field measurements

The table below is built from a dataframe passed via `sys.inputs`.



// ---- Section 3: Math ----
= Some math

The treated area is computed as
$ A_"treated" = sum_(i=1)^n a_i dot phi(s_i) $
where $phi(s_i) = 1$ when water is present and $0$ otherwise.

For a quadratic approximation:
$ y = beta_0 + beta_1 x + beta_2 x^2 + epsilon, quad epsilon tilde cal(N)(0, sigma^2) $

// ---- Section 4: Team ----
= Team



// ---- Section 5: Lorem-ish content over a few pages ----
= Detailed notes

#for i in range(1, 4) [
  == Note #i

  #lorem(80)

  #lorem(60)
]

// ---- Section 6: Nested table from data again, different layout ----
= Appendix: raw data echo


// ---- Title block ----
#align(center)[
  #text(size: 22pt, weight: "bold")[#title] \
  #v(0.3em)
  #text(size: 11pt, fill: rgb("#555"))[#author · #report_date · rev #revision]
]

#v(1em)
#line(length: 100%, stroke: 0.5pt + gray)

// ---- Section 1: Executive summary ----
= Executive summary

This is a synthetic report generated to benchmark `tynding` against the
`typst` CLI. It exercises sys inputs, JSON decoding, table generation,
math, and a small amount of layout work.

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 1em,
  rect(width: 100%, inset: 8pt, fill: rgb("#fef3c7"))[
    *Records* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dbeafe"))[
    *Team size* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dcfce7"))[
    *Revision* \
    #text(size: 18pt, weight: "bold")[#revision]
  ],
)

// ---- Section 2: Data table ----
= Field measurements

The table below is built from a dataframe passed via `sys.inputs`.



// ---- Section 3: Math ----
= Some math

The treated area is computed as
$ A_"treated" = sum_(i=1)^n a_i dot phi(s_i) $
where $phi(s_i) = 1$ when water is present and $0$ otherwise.

For a quadratic approximation:
$ y = beta_0 + beta_1 x + beta_2 x^2 + epsilon, quad epsilon tilde cal(N)(0, sigma^2) $

// ---- Section 4: Team ----
= Team



// ---- Section 5: Lorem-ish content over a few pages ----
= Detailed notes

#for i in range(1, 4) [
  == Note #i

  #lorem(80)

  #lorem(60)
]

// ---- Section 6: Nested table from data again, different layout ----
= Appendix: raw data echo


// ---- Title block ----
#align(center)[
  #text(size: 22pt, weight: "bold")[#title] \
  #v(0.3em)
  #text(size: 11pt, fill: rgb("#555"))[#author · #report_date · rev #revision]
]

#v(1em)
#line(length: 100%, stroke: 0.5pt + gray)

// ---- Section 1: Executive summary ----
= Executive summary

This is a synthetic report generated to benchmark `tynding` against the
`typst` CLI. It exercises sys inputs, JSON decoding, table generation,
math, and a small amount of layout work.

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 1em,
  rect(width: 100%, inset: 8pt, fill: rgb("#fef3c7"))[
    *Records* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dbeafe"))[
    *Team size* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dcfce7"))[
    *Revision* \
    #text(size: 18pt, weight: "bold")[#revision]
  ],
)

// ---- Section 2: Data table ----
= Field measurements

The table below is built from a dataframe passed via `sys.inputs`.



// ---- Section 3: Math ----
= Some math

The treated area is computed as
$ A_"treated" = sum_(i=1)^n a_i dot phi(s_i) $
where $phi(s_i) = 1$ when water is present and $0$ otherwise.

For a quadratic approximation:
$ y = beta_0 + beta_1 x + beta_2 x^2 + epsilon, quad epsilon tilde cal(N)(0, sigma^2) $

// ---- Section 4: Team ----
= Team



// ---- Section 5: Lorem-ish content over a few pages ----
= Detailed notes

#for i in range(1, 4) [
  == Note #i

  #lorem(80)

  #lorem(60)
]

// ---- Section 6: Nested table from data again, different layout ----
= Appendix: raw data echo


// ---- Title block ----
#align(center)[
  #text(size: 22pt, weight: "bold")[#title] \
  #v(0.3em)
  #text(size: 11pt, fill: rgb("#555"))[#author · #report_date · rev #revision]
]

#v(1em)
#line(length: 100%, stroke: 0.5pt + gray)

// ---- Section 1: Executive summary ----
= Executive summary

This is a synthetic report generated to benchmark `tynding` against the
`typst` CLI. It exercises sys inputs, JSON decoding, table generation,
math, and a small amount of layout work.

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 1em,
  rect(width: 100%, inset: 8pt, fill: rgb("#fef3c7"))[
    *Records* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dbeafe"))[
    *Team size* \

  ],
  rect(width: 100%, inset: 8pt, fill: rgb("#dcfce7"))[
    *Revision* \
    #text(size: 18pt, weight: "bold")[#revision]
  ],
)
