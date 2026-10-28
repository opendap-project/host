# OpenDAP Desktop

OpenDAP Desktop: the desktop app for everything you can do with an OpenDAP, from a casual user's first sync to advanced development. Written in Rust.

## Working here

- Requirements are edited in `docs/requirements/requirements.toml`; `docs/requirements/README.md` is rendered from it with `odapkit req render` and never edited by hand.
- Do not duplicate logic the `opendap` CLI already has: both build on one shared core and domain model.
