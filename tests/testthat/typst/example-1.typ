#set page(width: 10cm, height: 4cm)
#let title = sys.inputs.at("title", default: "No title found")
#let author = sys.inputs.at("author", default: "No author found")
#let persons = json.decode(sys.inputs.at("persons", default: none))

= #title
*Author:* #author

#for person in persons [
  #strong(person.name) is #text(fill: red, weight: "bold", [#person.age]) years old. \
]
