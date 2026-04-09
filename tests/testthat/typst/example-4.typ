#set page(height: 400pt)
#set text(purple)
#set circle(width: 50%)

#let food = sys.inputs.at("food", default: "no food found")

#align(center + horizon)[
  #circle(fill: aqua, stroke: 5pt + red)[
    #align(right)[
      #text(
        font: "Roboto",
        size: 1.2em,
      )[My favorite food is #food!]
    ]
  ]
]
