# host

| Folder | Content |
| --- | --- |
| [`cli/`](cli/README.md) | The `opendap` command-line tool: OpenDAP's adb, agentic-first, for advanced users and package developers working with OpenDAP devices, real or virtual |
| [`crates/`](crates) | The library crates the CLI and Desktop share: domain model, application core, command conventions, host platform and transports (MPL-2.0) |
| [`desktop/`](desktop/README.md) | OpenDAP Desktop: the desktop app for everything you can do with an OpenDAP, from a casual user's first sync to advanced development |

One Cargo workspace at the root (`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `check.toml`) builds `cli/` and `crates/`. `cli/` and `desktop/` keep their own `README.md`, `CLAUDE.md` and `REUSE.toml`; the licence texts are in `LICENSES/`. The folders came from the former `cli` and `desktop` repositories with their history ([RFC 0017](https://github.com/opendap-project/rfcs/blob/main/text/0017-naming-and-topology.md)).

## License

This repository follows the [REUSE](https://reuse.software) layout: licence texts in `LICENSES/`; the root `REUSE.toml` maps the workspace files and `crates/`, and each folder's `REUSE.toml` maps its own paths. Copyright © 2026 The OpenDAP Project.
