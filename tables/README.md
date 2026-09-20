# tables — canonical salutation data

One file per locale (`<locale>.json`). Code in every language reads the
same tables and runs the same matcher, so behavior is defined here, not
in any single implementation. `tests/vectors/` is the executable form of
this contract: a port is done when every vector passes.

## Normative token normalization

All table lookups use this normalization on each whitespace-separated
token — implementations must match it byte-for-byte:

1. Split the name on whitespace using exactly Rust `split_whitespace`
   semantics (Unicode White_Space: tab, newline, vertical tab, form feed,
   carriage return, space, U+0085, U+00A0, U+1680, U+2000–U+200A, U+2028,
   U+2029, U+202F, U+205F, U+3000). JavaScript `\s` differs on U+0085 and
   U+FEFF, so ports must use an explicit set, not a regex shorthand.
2. Strip leading and trailing `.` characters (`trim_matches('.')`).
3. Lowercase **ASCII A–Z only**. Rust uses `to_ascii_lowercase`; ports must
   NOT use Unicode-aware lowercasing (e.g. JavaScript `toLowerCase()` also
   folds non-ASCII, which would diverge — fold `[A-Z]` explicitly).

All keys in these files are already normalized. The schema test in the
Rust crate (`tables_schema`) asserts this on every table.

## Table schema

| Key | Meaning |
|-----|---------|
| `locale` | Locale id, matches the filename. |
| `source_norms` | Free text: which correspondence norms the rules come from. |
| `honorifics` | Normalized first-token → `frau` or `herr`. Anything absent (including abbreviations like `hr`, `fr`) is unparsable and triggers the generic fallback. The Anrede always uses `Herr`, never accusative `Herrn` and never abbreviated `Frau`. |
| `titles` | Normalized token → display form. `prof`/`professor` normalise to spelled-out `Professor`; `dr` stays abbreviated. |
| `sole_titles` | Display forms that suppress every other title (protocol keeps only the highest: `Professor` suppresses `Dr.`). |
| `filler` | Normalized tokens ignored when finding the surname (honorifics, titles, post-nominal grades). |
| `comma_regions` | Lowercase region codes whose salutation carries a trailing comma (de/at per DIN 5008; ch/li per SN 010130 have none). |
| `generic` | Formally safe fallback when honorific or surname is unparsable. |

## Matcher (all languages)

- **Surname**: last whitespace-separated token that is not in `filler`
  (after normalization); empty when none qualifies.
- **Salutation**: without a parsable honorific or surname → `generic`
  (+ `,` when the region is in `comma_regions`). Otherwise
  `Sehr geehrte Frau …` / `Sehr geehrter Herr …` with kept titles joined
  by spaces (empty when none), surname, and the optional comma.
