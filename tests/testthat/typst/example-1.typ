#set page(width: 10cm, height: 4cm)
#let title = sys.inputs.at("title")
#let author = sys.inputs.at("author")
#let persons = json.decode(sys.inputs.at("persons"))
= #title
*Author:* #author
#for person in persons [
  #strong(person.name) is #text(fill: red, weight: "bold", [#person.age]) years old. \
]
