# Focused validation and release

`.ci/ccid.toml` describes this repository's CI; `ccid render` writes the
workflows from it. Every push to `main` and every pull request runs the `verify`
job on the primary forge (`ci / verify`): the cached `verify` selection, then
the live-registry consumer checks fresh. The same repository commands work on a
suitably provisioned build host. Mirror forges (GitHub, GitLab, Bitbucket) run
no CI. Rust selects current `stable`; no older minimum version is checked or
promised.

Checks requiring operator-supplied artifact manifests (`rust-dependencies` and,
where defined, `typst-preview`) are run by hand. Private inputs and publishing
credentials never enter the verify job. Public registry consumer checks skip a
version that is not on the registry yet (a release in waiting) and fail on any other answer.

## Releasing

A release is deliberate, never per commit. Dispatch the `release` workflow on
`main`: ccid reads the version from `Cargo.toml`, packs the crate, the npm
tarball and the PyPI files once (`[checks.pack]`, before anything is public),
and promotes one draft to the public release `v<version>` with `SHA256SUMS`.
The publisher uploads exactly those files to the registries. See
[releasing](../docs/releasing.md). `py/build-constraints.txt` is the hash-locked
Python build backend, a lock file refreshed with the other locks. JSR is not
published for now.

Select checks from changed inputs and missing evidence. A changed README does not
require another Rust build. Table/API changes need affected language vectors;
package metadata, exports and dependency changes need installed-package checks.
Use a broader selection when the impact is unknown. Existing results are reusable
only for matching source, dependency closure, tools, configuration and environment.

| Selector | Coverage |
| --- | --- |
| `metadata` | Version alignment and byte-for-byte generated assets |
| `fmt` | Rust formatting only |
| `rust` | Locked Rust tests including doctests, formatting, Clippy with denied warnings |
| `javascript` | Strict TypeScript and shared source vectors |
| `rust-package` | Build and verify the registry-resolved Cargo package |
| `js-package` | Pack once; installed npm/Node/Bun imports and declarations, browser module, JSR dry-run |
| `js-pnpm`, `js-yarn`, `js-bun` | Selected additional manager consuming the same verified npm tarball |
| `python-package` | Wheel/sdist, installed-wheel vectors, metadata and JSON CLI consumers |
| `typst-package` | Deterministic archive and actual installed Typst import |
| `typst-preview` | Exact prepared Universe archive and README examples importing `@preview/cgreet:0.2.1` |
| `rust-dependencies` | Pre-publication source tests using checksum-verified sibling crates in private scratch |
| `published-npm`, `published-python` | Consumers of the exact version on the selected live registry |

For example, run selected checks with ccid on a provisioned host:

```sh
ccid check --repo . --check metadata,rust
```

To exercise every source and distribution check before a release, select them
all and supply an `ARTIFACT_ROOT`:

```sh
ARTIFACT_ROOT=/absolute/persistent/release-artifacts \
  ccid check --repo . \
  --check metadata,rust,javascript,rust-package,js-package,python-package,typst-package
```

`ARTIFACT_ROOT` is a worker path supplied by the operator. Each package selector
exports immutable files under `<root>/<package>/<commit>/`, plus its own JSON
receipt, `SOURCE_COMMIT`, and an aggregate `SHA256SUMS`. Additional manager checks
require `js-package` evidence for that exact commit. They never rebuild the tarball.
Use them when manager/export/packaging behavior changed; do not repeat all managers
for every source edit. Publication uploads the verified files separately using
registry credentials outside these verification jobs.

For unpublished sibling dependencies, supply `DEPENDENCY_MANIFEST` and
`DEPENDENCY_MANIFEST_SHA256`. The manifest is an exact JSON document:

```json
{
  "schema": 1,
  "artifacts": [
    {
      "kind": "wheel",
      "name": "cgreet",
      "path": "/absolute/path/to/verified/cgreet.whl",
      "sha256": "the actual 64-character lowercase SHA-256"
    }
  ]
}
```

Kinds are `wheel`, `crate`, or `npm`; each consuming check selects its own kind.
Python installs the specified sibling wheels alongside its own wheel. The
`rust-dependencies` selector copies source and verified sibling crates to owned
scratch, uses temporary Cargo patches and a scratch-only lock, and runs the tests.
This is integration evidence, not registry package verification. `rust-package`
always uses normal registry resolution and requires published dependencies.
Neither the source tree nor the released manifest acquires local path patches.

For a prepared Universe submission, pass one `DEPENDENCY_MANIFEST` artifact of
kind `typst-preview` and name `cgreet`, with its exact tarball path and SHA-256.
Select `typst-preview` to unpack that candidate into an isolated preview package
directory and compile its README examples with the pinned Typst binding. The
candidate archive and `typst-preview.json` receipt are exported unchanged. This
check verifies the submission artifact; upstream review controls publication.

The shared runner bounds concurrency to two build/test threads by default and
selected checks to 900 seconds; `CI_JOBS`, `CI_TEST_THREADS`, and `CI_TIMEOUT` can
supply a concrete different budget. It inherits dedicated package caches and
namespaces Cargo targets by canonical repository identity. It verifies source and
tool digests, preserves unchanged source freshness, supervises timeout/cancellation,
and never clears caches or installs system tools. Commands that generate source or
install dependencies run in owned scratch copies, preserving the verified input
tree and its reusable Rust timestamps. The dispatcher checks host
headroom, attaches to identical active work, and refuses a repeated completed
request unless an operator explicitly requests a diagnosed rerun.

Inspect existing runs before dispatch. After a failed Python check, select Python
again after correcting its inputs; a passed unrelated Rust check remains evidence.
Keep exact check receipts and durations when assessing whether wider validation is
needed. Linux results do not establish native Windows/macOS behavior or another
Rust/Node/Python version. Unavailable checks remain explicitly unverified.
