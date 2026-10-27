# `opendap` CLI

- **Status:** Accepted (2026-10-22), with the open questions resolved as recommended (§7)
- **Date:** 2026-10-21
- **Requirements:** TK-002 to TK-011

OpenDAP's adb: the tool for what an advanced user, a package developer or an agent does with an OpenDAP device, real or virtual (the T1 first). It sits next to OpenDAP Desktop on the user's side. It is not the Developer Toolkit: the platform's maintainers have their own tools, in the toolkit repositories ([RFC 0008](https://github.com/opendap-project/rfcs/blob/main/text/0008-repository-restructure.md)). Package developers use the SDK and the emulator through it, but do not develop them.

## 1. Principles

- **Agentic-first, human-friendly.** Every command works without a TTY, has JSON output, a `--dry-run`, stable error codes with a hint and a fix, and machine-readable help. Humans get readable tables, colour and confirmations.
- **One domain model.** Device, target, transport, image, capability, chip, job and finding are shared types (crate `opendap-model`, versioned JSON schemas `opendap.<kind>/v1`). Help, schemas, MCP tools, docs and the conformance test are generated from command metadata, following Cloudflare Forge.
- **Standard and trick-less.** SSH, USB, rockusb, QMP, journald, systemd and RAUC underneath. No proprietary daemon is required in v0.
- **UNIX / POSIX only, tested on macOS** (TK-010, TK-011).

## 2. Shared conventions

The full conventions are in [Appendix A](#appendix-a-shared-conventions): grammar, global flags, output, errors and exit codes, interactivity, targets, configuration, authentication, logging, help for agents and versioning; install is §4. In short:

- Grammar `opendap <noun> <verb> [subject] [--flags]`, with a closed verb vocabulary. ADB-style shortcuts (`shell`, `push`, `pull`, `logs`, `devices`, `flash`, `screenshot`) are documented aliases of canonical commands.
- Global flags: `-t/--target`, `--context`, `-o/--output human|json|ndjson|plain` (`--json`), `--fields`, `-n/--dry-run`, `-f/--force` (the only way to skip confirmations), `--no-input`, `-q`/`-v`, `--log-format`, `--color`, `--config`, `--offline`, `--timeout`.
- stdout carries only data. JSON lists are `{"items":[…]}`, and every document has a `schema`. Streams are NDJSON whose last line is `result` or `error`.
- Errors carry a stable dotted `code`, `hint`, `fix`, `retryable` and `docs`. There are 13 coarse exit codes (0–11, 130).
- Target selectors: `usb:<serial>`, `serial:<path>`, `net:<host>`, `vm:<name>`, `maskrom:<bus>-<addr>`, or a context name. A single candidate is used automatically, except for destructive operations.
- XDG paths on macOS too. One shared `config.toml`, with a table per tool.

## 3. Architecture

- **One binary, user-side plugins.** `opendap` is a single Rust binary. Extensions for its own audience (for example a HAT's user tools) ship as `opendap-<tool>` binaries and run as external subcommands (the cargo and git pattern). Maintainer tooling is not a plugin: it lives in the toolkit repositories as its own tools (`odapkit image`, `odap-fab`…).
- **Crates:** `opendap-model` (types and schemas), `opendap-cli-kit` (flags, output, errors, targets, config, logging, agent detection, help, schema, MCP, `doctor` framework, self-update), and one crate per binary. Use cases live in `opendap-core` and its adapter crates, layered as the [Rust crate architecture](https://github.com/opendap-project/system/blob/main/docs/specs/rust-crate-architecture.md) sets out.
- **Transports in v0:**
  - SSH, over the network or over USB networking from the main USB-C in gadget mode.
  - Serial console, over the same USB-C path (BRD-039).
  - rockusb, for MaskROM flashing.
  - The virtual devices of a virtual-devices checkout, through its tool `odap-vdev` (status as JSON, lifecycle, management link, screen and input).
- **No host daemon in v0.** Each invocation talks to the device directly.

## 4. Installer

```sh
curl --proto '=https' --tlsv1.2 -fsSL "$OPENDAP_INSTALL_BASE/install.sh" | sh -s -- [options]
```

| Option | Env | Behavior |
|---|---|---|
| (none) | | Install the latest release for the detected OS and architecture into `~/.local/bin` |
| `--version X.Y.Z` | `OPENDAP_VERSION` | Pin a specific release instead of the latest |
| `--uninstall` | | Remove the binary, the aliases it created and its receipt. `--purge` also removes config, state and caches |
| `--yes` | `OPENDAP_NO_INPUT=1` | Non-interactive: never prompt |
| `--verbose` | | Print every step |
| `--install-dir DIR` | `OPENDAP_INSTALL_DIR` | Install location |
| `--no-modify-path` | | Do not edit shell profiles |
| `--aliases auto\|none` | | `auto` (default): create `dap` and `odap` only if those names are free on the host; never overwrite |
| `--json` | | Machine-readable result for agents |

Behavior:

- POSIX `sh`, wrapped in a function so a partial download cannot run.
- Detects OS (macOS, Linux), architecture (arm64, x86_64) and libc. Linux binaries are static musl.
- Verifies the sha256 checksum and the minisign signature of the checksum file before installing.
- Writes a receipt that `opendap self update` and `opendap self uninstall` use.

## 5. Essential commands (v0)

Canonical form first, then the ADB-style shortcut where one exists.

### Targets and devices

| Command | Shortcut | What it does |
|---|---|---|
| `device list` | `devices` | Every reachable T1 (USB, network, MaskROM) and virtual T1, with mode and transport |
| `device get` | | Model, revision, serial, Blackbird OS version, mode, battery, storage, attached faceplate and HAT |
| `device wait --mode <mode>` | | Waits until the target reaches a mode (`normal`, `recovery`, `maskrom`, `gadget:*`) |
| `device reboot [--mode normal\|debug\|recovery\|maskrom]` | `reboot` | Reboots, optionally into another boot mode: a direct request to the device's `odap-reboot` (blackbird-os), like `adb reboot recovery`, never an imitation of the button combinations. The device says which modes it has; another fails with `device.mode_unsupported` |
| `device poweroff` | | Shuts the device down in order |
| `device pair` / `device unpair` | | Exchanges the host key with the device, which confirms on screen or over the console |
| `context list\|get\|create\|use\|delete` | | Named target and defaults bundles |

### Access

| Command | Shortcut | What it does |
|---|---|---|
| `exec [-- cmd…]` | `shell` | Interactive shell, or runs a command and returns its output and exit code |
| `file push <local> <remote>` / `file pull <remote> <local>` | `push` / `pull` | Copies files; idempotent by hash |
| `file list <path>` | | Lists a remote directory |
| `log watch` / `log get` | `logs` | Streams or fetches the system journal, filterable by service and priority |
| `forward create <local> <remote>` / `forward list` / `forward delete` | | Port forwarding between host and device |
| `screen capture [file]` | `screenshot` | Captures the display |
| `input send key <role>` / `input send key-down <role>` / `input send key-up <role>` / `input send tap <x> <y>` | | Injects presses of a control by its role (the T1: `action`, `volume-up`, `volume-down`, `main`), a control held down and released, and touches |
| `hardware get` / `hardware update --attach\|--detach <what>` / `hardware update --battery <percent>` | | The target's physical side: what is attached by hand (`microsd`, `usb-storage`, `usb-audio`, `charger`) and the battery's charge, as the hardware has it, not the system; changing it is for virtual devices |

### Software and system

| Command | Shortcut | What it does |
|---|---|---|
| `capability list\|get\|install\|uninstall\|update` | | Third-party capabilities, the T1's equivalent of apps |
| `os get` | | Blackbird OS version, A/B slot, pending update |
| `os check` / `os install <bundle\|version>` | | Looks for an update / applies an OTA bundle, with rollback |
| `service list\|get\|start\|stop\|restart` | | System services |

### Images and flashing

| Command | Shortcut | What it does |
|---|---|---|
| `image list` | | Available Blackbird OS releases |
| `image pull <version>` | | Downloads and verifies a release image |
| `image flash <file\|version>` | `flash` | Flashes over MaskROM / loader or USB mass storage. Destructive: requires an explicit target and confirmation (type the serial) or `--force` |

### Peripheral hosting (OS-008)

| Command | What it does |
|---|---|
| `chip list` | Every programmable chip on the board and on attached HATs: RP2350 coprocessor, HAT microcontrollers, FPGAs, other SoCs |
| `chip get <chip>` | Identity, firmware version, how it is reached (SWD, JTAG, USB, SPI) |
| `chip flash <chip> <firmware>` | Flashes firmware from Blackbird OS. Destructive |
| `chip restart <chip>` | Resets a chip, optionally into its boot mode |
| `chip start <chip> --debug` | Starts a debug server (GDB) on the device and forwards it to the host |
| `hat get` | EEPROM contents, capability manifest and signature status |
| `faceplate get` | Identity and controls (HID descriptor) |

### Virtual T1

| Command | What it does |
|---|---|
| `vm list` / `vm get` | Virtual T1 instances and their endpoints (SSH, VNC, QMP) |
| `vm start` / `vm stop` | Boots or stops the virtual T1 headless. Every other command works against it with `--target vm:<name>` |

### Host and CLI

| Command | What it does |
|---|---|
| `doctor` | Checks the host for the CLI's own work: binary and plugins, USB permissions, transports, SSH keys, QEMU and the virtual-devices checkout for the emulator. Every finding has a severity and a fix, plus `--json` and `--report`. Maintainer tools (Lima, KiCad, FreeCAD, Blender, pre-commit) are toolkit-host's |
| `self update` / `self uninstall` / `self alias` | Same behavior as the installer, using its receipt |
| `cli search "<task>"`, `schema [<command>]`, `tools`, `help errors [<code>]` | Discovery for agents and humans, generated from command metadata |
| `mcp serve` | MCP server over stdio, with a small tool set: search, describe, run, plus `device_list` and `doctor` |
| `version`, `completion <shell>` | Version details (`--json`) and shell completions |

### Maintainer tools are not CLI commands

What the platform's maintainers use lives in the toolkit repositories, as tools of their own ([RFC 0008](https://github.com/opendap-project/rfcs/blob/main/text/0008-repository-restructure.md)), reached through one command, `odapkit` ([RFC 0009](https://github.com/opendap-project/rfcs/blob/main/text/0009-odapkit.md)).

- **`opendap` never needs `odapkit`.** It does not call it, does not require its paths, and its hints never name a maintainer's tool as the only way out.
- **`odapkit` may use `opendap`** for device operations it would otherwise duplicate.

| Task | Tool | Repository |
|---|---|---|
| Build images, the sysroot, cross-build, deploy a development extension, security check | `odapkit image`, `odapkit sysroot`, `odapkit cross`, `odapkit deploy`, `odapkit security-check` | toolkit-image |
| Fab packages, mechanical exports | `fab-package.sh`, `freecad-export.py` | toolkit-hardware |
| Verification | in progress | toolkit-verify |

The emulator is run by virtual-devices' `odap-vdev`; the CLI targets its devices as `vm:<name>`.

## 6. Release

Built locally (no CI) for macOS arm64 and x86_64 and Linux arm64 and x86_64 (static musl). Packaged with `dist` in local mode, signed with minisign, and published as an immutable GitHub release (draft, upload, publish) with `gh`, by the maintainers.

## 7. Decisions

1. Piped stdout defaults to JSON (agent-first, like `cf`); humans pass `-o plain` for greppable text.
2. One binary, `opendap`, with external `opendap-<tool>` plugins for its own audience; maintainer tools stay in the toolkit (amended by RFC 0008).
3. Every destructive operation needs an explicit target, even when only one device is present.
4. v0 has no dedicated device-side agent: SSH and standard tools on the device.
5. No Apple Developer ID (open item CLI-OI-1).
6. Packaging with `dist` in local mode.

## Appendix A. Shared conventions

Where §7 decides differently, §7 applies.

### A.1 Naming and grammar

- Binaries: `opendap` (front door) and `opendap-<tool>` (plugins). Aliases `dap` and `odap` are symlinks created only when free (§4). Lowercase and kebab-case only.
- Canonical form: **`opendap <noun> <verb> [subject] [--flags]`** (noun-verb, as in `cf`, `gh` and clig.dev). Nouns are singular: `device`, `target`, `app`, `file`, `service`, `image`, `vm`, `log`, `config`, `context`, `key`, `self`.
- **Verb vocabulary (closed set):** `list`, `get`, `create`, `update`, `delete`, `install`, `uninstall`, `start`, `stop`, `restart`, `push`, `pull`, `flash`, `boot`, `reboot`, `exec`, `watch`, `wait`, `pair`, `unpair`, `use`, `check`, `fix`, `search`. `get`, never `info` or `show`. `delete`, never `rm` or `remove`. `list`, never `ls`. The short forms exist only as hidden aliases.
- **ADB-style shortcuts** are documented aliases of canonical commands and never have their own semantics. `opendap shell` → `exec --interactive`. `opendap push`/`pull` → `file push`/`pull`. `opendap logs` → `log watch`. `opendap devices` → `device list`. `opendap flash` → `image flash`. `schema` and `tools` always print the canonical path.
- The subject is the last positional argument. Containers are flags. `list` takes no positional argument.
- No prefix abbreviation of subcommands (clig.dev). Unknown flags are errors.
- Long-running operations come in two forms: blocking by default with a `--detach` that returns a `Job`, and `opendap job get|watch|wait <id>`.

### A.2 Global flags (identical in every tool)

| Flag | Env | Meaning |
|---|---|---|
| `-t, --target <sel>` | `OPENDAP_TARGET` | Device or VM selector (§A.6). The flag wins over the variable, as `-s` beats `ANDROID_SERIAL` in adb. |
| `--context <name>` | `OPENDAP_CONTEXT` | Named target and config bundle. |
| `-o, --output <human\|json\|ndjson\|plain>` | `OPENDAP_OUTPUT` | Output mode. `--json` is shorthand for `-o json`. |
| `--fields <a,b>` | — | Project the JSON fields. `--fields` with no value lists them (the `gh` pattern). |
| `-n, --dry-run` | — | Print the plan and change nothing. Accepted by every command; read-only commands show what they would query. |
| `-f, --force` | — | The **only** way to skip confirmation. Required for destructive operations when non-interactive. |
| `--no-input` | `OPENDAP_NO_INPUT=1` | Never prompt, even on a TTY. Implied when stdin is not a TTY, under `CI`, or when an agent is detected. |
| `-q, --quiet` / `-v…` | `OPENDAP_LOG` | Verbosity. Logs go to stderr only. |
| `--log-format <text\|json>` | `OPENDAP_LOG_FORMAT` | Structured logs on stderr. |
| `--color <auto\|always\|never>` | `NO_COLOR`, `TERM=dumb` | Follows clig.dev. Never colours JSON on non-TTY. |
| `--config <path>` | `OPENDAP_CONFIG` | Explicit config file. |
| `--offline` | `OPENDAP_OFFLINE=1` | No network except to the explicit target. |
| `--timeout <dur>` | — | A global ceiling. Commands that wait have their own `--wait-timeout`. |
| `-h, --help` / `-V, --version` | — | `--version` prints the bare version on stdout. `version --json` gives the details. |

- Reserve `--experimental-*` for feature gates.
- Never add credential flags (`--token`); use env or the keychain (§A.8).

### A.3 Output

- **stdout is data only.** Progress, spinners, warnings, hints and `✓` lines go to stderr. A mutation with nothing to return prints nothing on stdout.
- **Default mode, in priority order:** `--output` > `OPENDAP_OUTPUT` > **`json` when an agent is detected** > `human` when stdout is a TTY > **`json` otherwise**. This deliberately differs from `gh`, where piping is still human-formatted, and is close to `cf`, where JSON is always the default. Humans who want greppable tables when piping use `-o plain`. (There is a trade-off here; the alternative is human everywhere unless `--json` is passed.)
- **JSON contract:**
  - single results are an object;
  - lists are `{"items":[…],"next_cursor":…}`, never a bare array, so pagination can be added later without breaking consumers;
  - every top-level document carries `"schema":"opendap.<kind>/v1"`;
  - mutations include `"changed": true|false` (idempotency signal) and, under `--dry-run`, `"plan":[…]`.
- JSON is compact (one line) when not a TTY or when an agent is detected, and pretty when it is a TTY. This follows `cf`'s "condensed for agents".
- **NDJSON for streams** (`log watch`, `device watch`, `image flash`, `vm start --follow`): one event per line, `{"type":"progress|log|state|warning|result|error", "ts":…, …}`. The last line is always `result` or `error`.
- Output schemas are published (`opendap schema --output-schemas`). Fields are only **added** within a `/v1`. Removing or renaming a field means `/v2`, with `/v1` still selectable for one minor-release cycle through `--schema-version`.
- The human mode can change freely. Scripts must not parse it, and the docs say so.

### A.4 Errors and exit codes

- **Error envelope.** Under `json` or `ndjson` it goes to stdout; it always goes to stderr as human text.

```json
{"schema":"opendap.error/v1","ok":false,
 "error":{"code":"target.ambiguous","message":"3 devices match; pick one with --target",
  "hint":"Run `opendap device list` and pass --target usb:<serial>",
  "fix":[{"command":"opendap device list --json"}],
  "retryable":false,"details":{"candidates":["usb:T1A3F2","net:t1.local","vm:default"]},
  "docs":"opendap help errors target.ambiguous"}}
```

- `code` is a **stable dotted identifier**: `<area>.<condition>`, for example `usb.permission_denied`, `device.not_found`, `device.mode_mismatch`, `image.checksum_mismatch`, `auth.required`, `confirm.required`, `input.invalid`. Codes live in the model crate with a message template, a hint and a docs entry. This follows the Forge rule: remediation knowledge lives in the model, not in ad-hoc switches.
- `opendap help errors [<code>]` lists and explains codes and is generated from the model.

#### Exit codes (coarse, stable)

| Code | Meaning |
|---|---|
| 0 | Success, including a no-op with `changed:false` |
| 1 | Generic failure, or a doctor/check error-level finding |
| 2 | Usage error: bad flag or argument (clap default) |
| 3 | Not found (device, file, app, image) |
| 4 | Authentication or pairing required (same as `gh`) |
| 5 | Permission denied on the host (USB, serial, filesystem) |
| 6 | Target ambiguous: several matches and no `--target` |
| 7 | Target busy or locked, or a conflicting operation in progress |
| 8 | Timed out |
| 9 | Confirmation required: non-interactive and `--force` missing (`cf` uses 1; a distinct code tells agents exactly what to do) |
| 10 | Precondition failed or version incompatible (device agent too old, wrong mode) |
| 11 | Unsupported on this host or target |
| 130 | Interrupted (SIGINT) |

- The JSON `code` is the precise contract. Exit codes are for shells.

### A.5 Interactivity, safety and idempotency

- Prompt only when stdin **and** stderr are TTYs, no agent is detected, and `--no-input` is not set. In every other case a missing value fails with `input.missing` (exit 2), and for enums the error lists the valid values.
- Destructive operations need either an interactive confirmation or `--force`. Destructive operations are: flashing, wiping, deleting user data, factory reset, `self uninstall --purge`. Interactive confirmation of a flash or wipe makes the user **type the device serial**. Non-interactive runs without `--force` fail with code 9 and change nothing.
- `--dry-run` is accepted by every command. For mutating commands it prints the exact plan: device, partitions, image hash, bytes. A plan is reproducible: `--plan-out plan.json` then `apply --plan plan.json` (optional, later).
- **Idempotent by design:** `install` of the version already installed, `push` of an identical file (hash compared), `vm start` of a running VM and `config set` to the same value all return `changed:false` with exit 0. Retrying after a timeout is safe. Commands that are not idempotent say so in their metadata.
- `@file` and `-` (stdin) work on every value flag that takes content. Secrets are read only from stdin or a file, never from argv.

### A.6 Targets, transports and contexts

- **Selector grammar:**
  - `usb:<serial>`
  - `serial:<path>` (console)
  - `net:<host>[:port]` (SSH)
  - `vm:<name>` (virtual T1: SSH, VNC, QMP)
  - `maskrom:<bus>-<addr>` (rockusb)
  - or a bare name that resolves through contexts.

  Resolution: when exactly one candidate is present it is used automatically, *except* for destructive operations, which need an explicit target (selector or context). Otherwise the command fails with code 6 and lists the candidates.
- **Modes are part of the Device model**, as adb and fastboot treat them: `normal`, `gadget:{serial,net,storage}`, `maskrom`, `recovery`. `opendap device wait --mode maskrom --timeout 60s` replaces ad-hoc sleeps. A command run against the wrong mode fails with `device.mode_mismatch` (exit 10), and the hint names the transition command (`opendap device reboot --mode maskrom`).
- **Contexts**, kubectl style: `opendap context list|get|create|use|delete`. A context bundles target, defaults and credentials reference. They are stored in the user config. `OPENDAP_CONTEXT` sets one per shell. **Every output says whether it acted on a physical device or the virtual T1** (`"target":{"kind":"vm","name":"default"}`), which is the counterpart of `cf`'s local vs remote signal.
- **No persistent host server in v0.** Each invocation talks to devices directly (USB through `nusb`, SSH, QMP). Re-evaluate an adb-style background server only if port forwarding or multi-client arbitration needs one.

### A.7 Configuration and files

- Precedence (clig.dev): **flags > env > project file (`opendap.toml`, found by walking up from cwd) > user (`$XDG_CONFIG_HOME/opendap/config.toml`) > system (`/etc/opendap/config.toml`)**. Use XDG paths **on macOS too**, not `~/Library/Application Support`, which is what CLI users expect. The `etcetera` crate (0.11) supports an XDG strategy on macOS; the `directories` crate defaults to Apple paths.
- Other locations:
  - state: `$XDG_STATE_HOME/opendap`
  - caches (images): `$XDG_CACHE_HOME/opendap`
  - data (keys): `$XDG_DATA_HOME/opendap`, with mode `0600`/`0700`
- `opendap config get|set|list|unset [--scope project|user]` and `opendap config path`. Unknown keys produce a warning, not an error. Each subsection is validated lazily, so a broken `vm` section does not break `device list`, matching `cf`'s "lazy config validation".
- All tools share one config file with a table per tool (`[image]`, `[fab]`) and shared `[targets]`/`[contexts]`.

### A.8 Authentication and trust

- Device pairing follows adb's RSA-prompt model:
  - `opendap device pair` generates or uses a host key (ed25519) and sends it to the device;
  - the device confirms on screen or over the serial console;
  - the host key is stored under `$XDG_DATA_HOME/opendap/keys`;
  - SSH host keys are pinned per device.
- For VMs, the key is injected at image build time.
- Credentials for any remote service (for example release mirrors) come from `OPENDAP_TOKEN`, then the OS keychain (`keyring` 4.2, opt-in), then a `0600` file. There are never credential flags. Remote logins use the **device authorization grant**, as `cf` does, so they work over SSH and in containers.
- `opendap auth status --json` reports what is configured and **never prints secrets**.

### A.9 Logging and diagnostics

- `tracing` with `OPENDAP_LOG` (EnvFilter syntax, for example `opendap=debug,rockusb=trace`). `-v` = info, `-vv` = debug, `-vvv` = trace. Logs go to stderr only. `--log-format json` gives agents and bug reports structured logs. `--log-file <path>` mirrors them.
- USB and serial protocol traces only at trace level, with payload bytes redacted unless `--trace-payload` is set.
- Every error under `-v` includes the `details` chain. `doctor --report` bundles the version, config (redacted), the last N log lines and device inventory.

### A.10 Help and discoverability for agents

- `--help` is generated, concise and example-first. Every command has at least two examples, including one with `--json`.
- **Agent detection.** Detection uses `AI_AGENT`, `CLAUDECODE`, `CODEX_THREAD_ID`, `CURSOR_AGENT`, `GEMINI_CLI`, `OPENCODE`, `COPILOT_AGENT` and similar variables, the same list `cf` uses. `OPENDAP_AGENT=1|0` forces it on or off. When an agent is detected:
  - prompts are disabled;
  - JSON is compact;
  - help starts with a short discovery banner: use `opendap cli search "<task>"`, then `<cmd> --help`, then `opendap schema <cmd>`, and do not crawl nested `--help`;
  - update notices are suppressed.
- **Introspection commands** (front door, also available in each plugin):
  - `opendap cli search "<intent>"`: top 5 commands as compact JSON `{command, summary}` (`cf` pattern; offline index built into the binary).
  - `opendap schema [<command…>]`: JSON with arguments, flags (types, enums, defaults), category, idempotency, output schema ID and error codes. `opendap schema --all` dumps the whole tree.
  - `opendap tools`: MCP tool definitions derived from the same metadata.
  - `opendap help errors [<code>]`.
- **MCP server mode: `opendap mcp serve`** (stdio first, using `rmcp`).
  - It exposes a **small, curated tool set** rather than one tool per command, following Code Mode and Anthropic's advice on token budgets: `opendap_search`, `opendap_describe` (schema for one command) and `opendap_run` (runs any canonical command with schema-validated arguments and returns its JSON).
  - A few high-value direct tools such as `device_list` and `doctor`.
  - MCP tool annotations (`readOnlyHint`, `destructiveHint`, `idempotentHint`) come from command metadata. Destructive commands still need `force: true` in the arguments, and the host's consent flow applies.
  - Long operations (flash, image build) map to the MCP **Tasks** extension when the client supports it, and to `Job` handles otherwise.
- **Docs for LLMs.** The docs site publishes `llms.txt` and a Markdown twin of every page, including a generated CLI reference (from `schema --all`) and an error-code catalogue. These are served under `$OPENDAP_INSTALL_BASE`, or wherever the docs live.
- An optional `AGENTS.md` snippet in each repo explains how agents should drive the toolkit: target selection, `--json`, `--dry-run` before `--force`.
- Shell completions (`opendap completion bash|zsh|fish`) and man pages are shipped in the tarball.

### A.11 Versioning, compatibility and telemetry

- **SemVer** for each binary. Pre-1.0 minors may break the human output, but **never** a published JSON schema version or an error code.
- `opendap version --json` returns `{version, commit, build_date, target, model_schema_versions, device_protocol: {min,max}}`. Before every command the device agent and host negotiate a protocol range, and a mismatch gives `device.incompatible` (exit 10) with an upgrade hint.
- Deprecations: a stderr warning plus a `warnings:[{code:"deprecated.flag",…}]` entry in the JSON, kept for at least one minor release. Deprecated commands then **disappear** from help, search and schema (the `cf` rule) but keep running until the next major.
- Toolkit tools share the kit crate version. A tool that finds an incompatible `opendap-<tool>` plugin warns from `doctor`.
- **Telemetry:** none by default. If it is ever added, it is opt-in (`opendap cli telemetry enable`), respects `DO_NOT_TRACK=1`, documents exactly what it sends, and offers a `status` command. Neither the CLI nor the installer phones home. The only network call is the optional, user-enabled update check.

## Open items

- **CLI-OI-1** An Apple Developer ID, revisited for browser downloads, a `.pkg` or a Homebrew cask (§7 item 5).
