# Changelog

All notable changes to `cgreet` are documented here. The project follows
Semantic Versioning.

## 0.3.1 - 2026-09-24

- Repository moved to github.com/corbet-foss/cgreet; registry metadata points there.
- Released from a single tag through CI (crates.io and JSR trusted publishing).
- Drop the duplicate `LICENSES/LGPL-3.0-only WITH LGPL-3.0-linking-exception.txt`
  (identical to `LGPL-3.0-linking-exception.txt`); JSR rejects paths with spaces.

## 0.3.0 - 2026-09-13

- License this new release line under LGPL-3.0-only WITH LGPL-3.0-linking-exception across Cargo, npm, JSR,
  Python and Typst, with the complete LGPL and incorporated GPL notices.
- Keep runtime behavior, correspondence tables and dependency versions unchanged.

## 0.2.1 - 2026-09-09

- Reuse packed distributions and run selected checks through Crow.
- Add shared Typst salutations and engineering title aliases.

- Ship compiled ESM and CommonJS, declaration files, and a standalone browser module.
- Add a Python distribution with shared-vector conformance and a JSON CLI.
- Add JSR packaging, installed-artifact tests, and complete registry license files.
- Clarify scope, examples, installation options, and family links on the product page.

## 0.2.0 - 2026-09-06

- Move the salutation rules into `tables/de.json`: honorifics, titles,
  filler tokens, comma regions, and the generic fallback are data,
  documented in `tables/README.md` with the normative token normalization.
- Add the shared conformance suite `tests/vectors/*.json` (70 vectors):
  the Rust crate runs it in `cargo test`, every language port runs the
  same files.
- Add the pure-TypeScript port `@corbet-labs/cgreet` (`js/`): zero
  dependencies, zero Node APIs, green on all 70 vectors.

## 0.1.0 - 2026-09-05

- Initial release: deterministic locale-correct German salutations
  (SN 010130 for ch/li, DIN 5008 for de/at), title handling, surname
  extraction, and validation warnings.
