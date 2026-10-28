# CLI

The `opendap` command-line tool: OpenDAP's adb, agentic-first, for advanced users and package developers working with OpenDAP devices, real or virtual. Written in Rust; it sits next to OpenDAP Desktop, not in the Developer Toolkit.

## Working here

- Rust style is the standard rustfmt style; clippy runs with `-D warnings` (pre-push hook, `pre-commit install --hook-type pre-push`, and the root's `check.toml`). The Cargo workspace is at the root of `host`; the library crates are in `../crates/`. Build with `CARGO_BUILD_JOBS=4` on the shared machine.
- Requirements are edited in `docs/requirements/requirements.toml`; `docs/requirements/README.md` is rendered from it with `odapkit req render` and never edited by hand.
- This is the user-side CLI (like adb), not the Developer Toolkit: maintainer tooling belongs in the toolkit repositories. Use other repositories (virtual-devices `odap-vdev`, toolkit tools) through their documented interfaces, not their internals.
