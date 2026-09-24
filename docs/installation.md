# Installation and distribution

The main branch ships 0.3.1 under LGPL-3.0-only WITH LGPL-3.0-linking-exception, published to registries. The existing releases documented below keep their original
license grants; this change does not replace their artifacts.

This guide describes version 0.3.1. Check the linked registry or release
for availability; a source manifest alone does not establish publication.

## JavaScript and Rust

Use Cargo for Rust and npm, pnpm, Yarn, or Bun for JavaScript. These JavaScript
package managers share the npm registry; each consumes the same package.
Deno can use `npm:@corbet-labs/cgreet`. The browser export bundles runtime
dependencies and needs no import map. The declared Rust minimum is 1.94. Release checks use the worker's current stable compiler; a separate minimum-version check is required to
verify that lower bound.

## Python and the command line

The pure Python package requires Python 3.10+ and is published on
[PyPI](https://pypi.org/project/cgreet/0.3.1/). Install this release with pip
or uv in your Python environment:

```sh
python -m pip install cgreet==0.3.1
```

```sh
uv pip install cgreet==0.3.1
```

For a uv project, `uv add cgreet==0.3.1` adds the package to your dependencies.

After installation, functions can be called from Python or through either CLI
entrypoint:

```sh
cgreet --help
python -m cgreet --help
```

The CLI takes a function name and a JSON array of positional arguments or an
object of keyword arguments. Use `-` to read arguments from stdin. It writes
JSON to stdout; errors use stderr and a nonzero exit status.

For an isolated CLI environment, use `pipx install cgreet==0.3.1` or
`uv tool install cgreet==0.3.1`.

The wheel contains no native extensions and is platform independent. Release
evidence records the Python version and operating system actually exercised.
Verified wheels and source distributions are also attached to the
[GitHub release](https://github.com/corbet-foss/cgreet/releases/tag/v0.3.1).

## JSR

Version 0.3.1 targets
[`@corbet-labs/cgreet`](https://jsr.io/@corbet-labs/cgreet@0.3.1):

```sh
deno add jsr:@corbet-labs/cgreet@0.3.1
```

## Typst

Version 0.2.1 is published on
[Typst Universe](https://typst.app/universe/package/cgreet). Import it directly
in the Typst web app or a local Typst document:

```typst
#import "@preview/cgreet:0.2.1": *
```

For a local installation, download `cgreet-0.3.1-typst.tar.gz` from the matching GitHub release and
extract its contents into `typst/packages/local/cgreet/0.3.1` under your
[Typst data directory](https://github.com/typst/packages#local-packages):

| System | Data directory |
| --- | --- |
| Linux | `$XDG_DATA_HOME`, or `~/.local/share` |
| macOS | `~/Library/Application Support` |
| Windows | `%APPDATA%` |

```typst
#import "@local/cgreet:0.3.1": *
```

The archive includes its manifest, tables, source, and licenses. CI compiles
an example against a fresh installation of the actual archive with Typst 0.15.
For the Typst web app, upload the extracted files and import the entrypoint
listed in `typst.toml` by its relative path.

## System package managers

These are language libraries, with a portable Python CLI. Homebrew, APT, RPM,
WinGet, Chocolatey, and Scoop are not additional registries for importing a
Rust crate or JavaScript module. Use their Python or Node runtime and the
language package manager above. No native system-package listing is claimed.
