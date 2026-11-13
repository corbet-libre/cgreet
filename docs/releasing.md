# Releasing cgreet

Keep Cargo.toml, Cargo.lock, package.json, jsr.json, pyproject.toml, and
typst.toml versions consistent. Published contents are immutable: use a new
version for corrections.

1. Update versions and CHANGELOG.md (add a `## X.Y.Z - date` section; the text
   of CHANGELOG.md, at most 64 KiB, is the release notes), then regenerate
   canonical tables with
   `bash js/@corbet-labs/cgreet/scripts/sync-assets.sh`. Commit to `main`; the
   commit's own `ci / verify` run must be green.
2. Dispatch the `release` workflow on `main` (a release is deliberate, never per
   commit). It runs to the public release by itself:
   - The version comes from `Cargo.toml`; `package.json` and `pyproject.toml`
     must carry the same version, or the run stops before anything is public.
     `ccid release version` prints it.
   - `pack` (the `[checks.pack]` check) makes the crate, the npm tarball and the
     PyPI sdist and wheel once, from the clean commit, without credentials:
     `<name>-<version>.crate`, `corbet-labs-cgreet-<version>.tgz`,
     `cgreet-<version>.tar.gz` and `cgreet-<version>-py3-none-any.whl`.
   - One draft collects those files; promotion adds `SHA256SUMS` and turns the
     draft into the public release `v<version>`, whose notes are
     `CHANGELOG.md`.
3. The publisher then uploads exactly those files to crates.io, npm and PyPI
   and records every upload. Copies of the repository on other forges receive
   the same release files. Nothing is built twice.
4. Run the published-consumer checks (`published-npm`, `published-python`) and
   state the status per registry. A successful upload response or version
   collision is insufficient proof.

JSR is not published for now: it accepts a publish only from a GitHub workflow,
which this project does not run on a mirror forge. `jsr.json` stays in the
tree for a later route, and the `js-package` check still dry-runs it. The Typst
archive is checked by `typst-package` but is not a file of the release; Typst
Universe requires a separate eligible submission.

`py/build-constraints.txt` pins the Python build backend (hatchling and its
requirements) by hashes, so no build input is resolved at release time. It is a
lock file, refreshed with the other locks. To refresh it by hand:

```sh
cd py
python3 -c 'import tomllib;print(*tomllib.load(open("pyproject.toml","rb"))["build-system"]["requires"],sep="\n")' |
  uv pip compile --universal --generate-hashes --no-config --python-version 3.10 - -o build-constraints.txt
```

Publish sibling dependencies before cletter and resolve its committed locks
against those registry versions. Local path overrides are integration evidence.

## Re-running

A transient failure is retried by dispatching `release` again: the run resumes
the draft of the same commit. A draft whose files differ from this run's is
refused until it is discarded (`ccid release discard`). A version is released
once; a published version is never replaced. If the pack fails, fix `main` and
dispatch again: nothing was public.
