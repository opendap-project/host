# host

One folder per former repository: `cli/` and `desktop/`.

<!-- >>> odapkit sync: managed block from the workspace templates; edit them, not this block -->
Part of [OpenDAP](https://github.com/opendap-project). This repository is Open Source.

Contributor rules (English, confirmed information only, REUSE licensing, the `docs/` layout and its skills): the [contributor guide](https://github.com/opendap-project/.github/blob/main/CONTRIBUTING.md).
<!-- <<< odapkit sync -->

## Working here

- `cli/` and `desktop/` have their own `CLAUDE.md` with their rules; read the one for the folder you change. `crates/` follows `cli/CLAUDE.md`.
- One Cargo workspace at the root builds `cli/` and `crates/`: run `cargo` from the root (`CARGO_BUILD_JOBS=4` on the shared machine); clippy and the tests run at pre-push (`pre-commit install --hook-type pre-push`).
- Requirements are edited in each folder's `docs/requirements/requirements.toml`; its README is rendered with `odapkit req render` and never edited by hand.
