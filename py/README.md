# cgreet

> **Superseded by [`cnice`](https://github.com/corbet-foss/cnice).**
> New development continues as `cnice.greet` (same API, same vectors);
> this crate stays frozen at 0.3.0 and published grants are unchanged.

**German salutations with the right titles and punctuation.**

[![crates.io](https://img.shields.io/crates/v/cgreet.svg)](https://crates.io/crates/cgreet) [![npm](https://img.shields.io/npm/v/@corbet-foss/cgreet.svg)](https://www.npmjs.com/package/@corbet-foss/cgreet) [![PyPI](https://img.shields.io/pypi/v/cgreet.svg)](https://pypi.org/project/cgreet/) [![Rust API](https://docs.rs/cgreet/badge.svg)](https://docs.rs/cgreet)

Turn an explicitly supplied honorific, academic titles, and surname into a formal salutation for Switzerland, Liechtenstein, Germany, or Austria. Deterministic rules, no model and no network calls.

```js
import { deSalutation } from '@corbet-foss/cgreet';

deSalutation('Frau Dr. Müller', 'ch');
// Sehr geehrte Frau Dr. Müller
```

## Install

| Environment | Command |
| --- | --- |
| Rust / Cargo | `cargo add cgreet` |
| Python / pip | `python -m pip install cgreet` |
| Python / uv | `uv add cgreet` |
| Node.js / npm | `npm install @corbet-foss/cgreet` |
| pnpm | `pnpm add @corbet-foss/cgreet` |
| Yarn | `yarn add @corbet-foss/cgreet` |
| Bun | `bun add @corbet-foss/cgreet` |
| Deno | `deno add npm:@corbet-foss/cgreet` |
| Typst | `#import "@preview/cgreet:0.2.1": *` |

The 0.2.1 JavaScript distribution includes compiled ESM, CommonJS,
TypeScript declarations, and a standalone browser module. Node.js 20+ is
supported; no TypeScript loader is required.

```js
// CommonJS
const { deSalutation } = require('@corbet-foss/cgreet');
```

```html
<script type="module">
  import { deSalutation } from 'https://cdn.jsdelivr.net/npm/@corbet-foss/cgreet@0.2.1/dist/browser.js';
  console.log(deSalutation('Frau Dr. Müller', 'ch'));
</script>
```

Python 3.10+ packages are available on [PyPI](https://pypi.org/project/cgreet/).
See the [installation guide](https://github.com/corbet-foss/cgreet/blob/main/docs/installation.md)
for CLI commands and other distribution options.
JSR installation is documented there too. The Typst interface is available on
[Typst Universe](https://typst.app/universe/package/cgreet).

## Rust

```rust
use cgreet::{de_salutation, Region};

assert_eq!(de_salutation("Frau Dr. Müller", Region::Ch), "Sehr geehrte Frau Dr. Müller");
```

## Python

```python
from cgreet import de_salutation

assert de_salutation("Frau Dr. Müller", "ch") == 'Sehr geehrte Frau Dr. Müller'
```

## API

| Function | Purpose |
| --- | --- |
| `deSalutation` / `de_salutation` | Complete German salutation |
| `salutationHonorific`, `salutationTitles`, `salutationSurname` | Parse explicitly supplied recipient details |
| `parseRegion` / `parse_region` | Accept `ch`, `li`, `de`, or `at` |
| `recipientSalutationWarning`, `deHonorificWarning` | Report incomplete input |

Region codes are lowercase. German and Austrian salutations end with a comma; Swiss and Liechtenstein salutations do not. A recognized Professor title takes precedence over Dr. Missing honorifics or surnames produce the generic greeting. The library does not infer a person’s gender from their name.

## Correspondence family

| Library | Responsibility |
| --- | --- |
| [cletter](https://github.com/corbet-foss/cletter) | Compose the correspondence helpers |
| [cgreet](https://github.com/corbet-foss/cgreet) | German salutations and titles |
| [cfarewell](https://github.com/corbet-foss/cfarewell) | Locale-specific closings |
| [cdate](https://github.com/corbet-foss/cdate) | Calendar-date formatting |
| [cink](https://github.com/corbet-foss/cink) | Handwritten signature images |


## Development

Behavior is defined by [the locale tables](https://github.com/corbet-foss/cgreet/tree/main/tables)
and [shared conformance vectors](https://github.com/corbet-foss/cgreet/tree/main/tests/vectors).
Rust, JavaScript, and Python run the same vectors. Selected CI checks exercise
installed JavaScript tarballs, Python wheels and command-line entrypoints, and
Typst packages. Release validation records the actual runtime and platform;
Linux results do not establish native Windows or macOS coverage.
All five Rust crates forbid unsafe code in their own source.

See [the release guide](https://github.com/corbet-foss/cgreet/blob/main/docs/releasing.md)
for generation, verification, and publication commands.

## License

Copyright 2026 Julian Y. Richard Corbet. The 0.3.0 release line is licensed
under [LGPL-3.0-only](https://github.com/corbet-foss/cgreet/blob/main/LICENSES/LGPL-3.0-only.txt)
[WITH LGPL-3.0-linking-exception](https://github.com/corbet-foss/cgreet/blob/main/LICENSES/LGPL-3.0-only%20WITH%20LGPL-3.0-linking-exception.txt),
with the incorporated [GPL version 3](https://github.com/corbet-foss/cgreet/blob/main/LICENSES/GPL-3.0-only.txt).
Combined works may link statically or dynamically without relinking duties;
library modifications stay LGPL. Applications can use a different license
subject to the LGPL's conditions.
Versions 0.2.0 and 0.2.1 retain MIT OR Apache-2.0. The installation examples above refer to
available releases; 0.3.0 has not yet been published to registries.

See the [licensing notes](https://github.com/corbet-foss/cgreet/blob/main/LICENSE.md) for distribution conditions and retained notices.
Contributions are accepted under the [Contributor License Agreement](CLA.md).
