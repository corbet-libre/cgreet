# Typst salutations

Copy `typst/greet.typ` together with `tables/de.json`, preserving their relative
paths. The module reads the canonical table used by Rust, TypeScript and Python.

```typst
#import "typst/greet.typ": de-salutation, salutation-titles
#de-salutation("Frau Dr. Müller", region: "ch")
#assert.eq(salutation-titles("Herr Dipl.-Ing. Müller"), ("Dipl.-Ing.",))
```

Exports cover region parsing/comma behavior; last-name, honorific, title and
surname extraction; German salutations; and recipient advisories. Region codes
are `ch`, `li`, `de`, `at`. Names are preserved; no spelling transformation runs
implicitly. Token normalization strips outer periods and lowercases ASCII A–Z.

The complete cletter Typst facade vendors this module. Its
`scripts/typst-conformance.py` driver runs every canonical cgreet vector against
this port using an existing Typst or Tinymist compiler.
