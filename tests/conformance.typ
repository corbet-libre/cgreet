// Cross-port conformance: runs every `tests/vectors/*.json` vector through
// `typst/greet.typ` and compares with `expected` exactly. A failing vector
// aborts compilation with its `file :: name`.
//
// Compile from the repository root:
//   typst compile --root . tests/conformance.typ /tmp/cgreet-conformance.pdf
#import "../typst/greet.typ" as api

#let run-vector(file, vector) = {
  let what = file + " :: " + vector.at("name", default: "<unnamed>")
  let actual = if vector.fn == "region_parse" {
    api.parse-region(vector.input)
  } else if vector.fn == "region_uses_comma" {
    api.region-uses-comma(vector.input)
  } else if vector.fn == "salutation_last_name" {
    api.salutation-last-name(vector.input)
  } else if vector.fn == "salutation_honorific" {
    api.salutation-honorific(vector.input)
  } else if vector.fn == "salutation_surname" {
    api.salutation-surname(vector.input)
  } else if vector.fn == "salutation_titles" {
    api.salutation-titles(vector.input)
  } else if vector.fn == "de_salutation" {
    api.de-salutation(vector.input, region: vector.region)
  } else if vector.fn == "recipient_salutation_warning" {
    api.recipient-salutation-warning(vector.location, vector.input)
  } else if vector.fn == "de_honorific_warning" {
    api.de-honorific-warning(vector.location, vector.input)
  } else {
    panic("unknown fn " + repr(vector.fn) + " in " + what)
  }
  assert.eq(actual, vector.at("expected", default: none), message: what)
}

#let run-file(file) = {
  let vectors = json("vectors/" + file)
  for vector in vectors {
    run-vector(file, vector)
  }
  vectors.len()
}

#let total = run-file("de_salutation.json") + run-file("honorific_titles.json") + run-file("last_name.json") + run-file("region.json") + run-file("warnings.json")

Typst conformance green: #total vectors across 5 files.
