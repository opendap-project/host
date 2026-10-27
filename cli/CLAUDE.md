# CLI

The `opendap` command-line tool: OpenDAP's adb, agentic-first, for advanced users and package developers working with OpenDAP devices, real or virtual. Written in Rust; it sits next to OpenDAP Desktop, not in the Developer Toolkit.

<!-- >>> odapkit sync: managed block from the workspace templates; edit them, not this block -->
Part of [OpenDAP](https://github.com/opendap-project). This repository is Open Source.

Contributor rules (English, confirmed information only, REUSE licensing, the `docs/` layout and its skills): the [contributor guide](https://github.com/opendap-project/.github/blob/main/CONTRIBUTING.md).
<!-- <<< odapkit sync -->

## Working here

- Rust style is the standard rustfmt style; clippy runs with `-D warnings` (pre-push hook, `pre-commit install --hook-type pre-push`, and `check.toml`). Build with `CARGO_BUILD_JOBS=4` on the shared machine.
- Requirements are edited in `docs/requirements/requirements.toml`; `docs/requirements/README.md` is rendered from it with `odapkit req render` and never edited by hand.
- This is the user-side CLI (like adb), not the Developer Toolkit: maintainer tooling belongs in the toolkit repositories. Use other repositories (virtual-devices `odap-vdev`, toolkit tools) through their documented interfaces, not their internals.
