# Desktop requirements

| ID | Requirement | Priority | Status |
|---|---|---|---|
| DESK-001 | OpenDAP shall have a full-featured desktop app, written in Rust, besides the `opendap` CLI. | must | Confirmed |
| DESK-002 | The desktop app shall let users do everything that can be done with an OpenDAP, from casual use to advanced use. | must | Confirmed |
| DESK-003 | The app shall serve casual users first and keep advanced features available without imposing them. | must | Draft |
| DESK-004 | The app and the `opendap` CLI shall share one core and one domain model (TK-009), so a feature exists once and both expose it. | must | Draft |
| DESK-005 | OpenDAP Desktop shall run on UNIX/POSIX hosts, tested on macOS, like the rest of the project. | must | Confirmed |
| DESK-006 | OpenDAP Desktop shall include a per-user background service on the host, started at login without administrator rights, for work that must happen while the app is closed: device detection, library and playlist sync, backups, update downloads, notifications and persistent connections. | should | Draft |
| DESK-007 | The background service shall be another interface over the same core as the CLI and the app (Rust crate architecture). The app and the CLI use it when it runs; the CLI keeps working without it. | should | Draft |
| DESK-008 | The background service shall integrate with each host's standard service manager and device access (launchd on macOS, systemd user units and udev rules on Linux; Windows equivalents if Windows is supported), and stay light on CPU, memory and battery when idle. | should | Draft |
| DESK-009 | OpenDAP Desktop should also support Windows (split from DESK-005). | should | Confirmed |
| DESK-010 | OpenDAP Desktop should offer optional transcoding per device when syncing, to fit more music, without ever changing the library. | should | Confirmed |
| DESK-011 | Sync and the connected features between the T1 and a computer (over USB, Wi-Fi, Bluetooth or any other link) shall go through OpenDAP Desktop or the `opendap` CLI. Without them, the T1 still offers plain USB storage (T1-065) and complete standalone use (T1-066). | must | Confirmed |
| DESK-012 | The listener's audio files in the folders they choose shall be the library; OpenDAP Desktop indexes and enriches them, and keeps the shared collection separate from each person's own state (SYS-043). | must | Confirmed |
| DESK-013 | What goes to each device shall be set by rules (everything, selections, smart rules, fill the free space), always previewed before it is applied; removing something from a device's selection never removes it from the library. | must | Confirmed |
| DESK-014 | Providers shall be optional adapters that declare what they can do (search, checkout, download, purchases, metadata, scrobbling, playlist import); purchases land as files in the library with their provenance, and results from several providers appear side by side, none by default. | must | Confirmed |
| DESK-015 | Playlists shall be kept by open identifiers and resolved to local files first; an entry that cannot be resolved stays in place with the reason. | must | Confirmed |
| DESK-016 | OpenDAP Desktop should show the collection's health (missing art, inconsistent tags, duplicates, editions, unidentified files, files on no device) and fix it in batches with a preview and undo. | should | Confirmed |
| DESK-017 | OpenDAP Desktop shall manage the listener's devices: pairing and removal, updates staged for them, versioned backups, restore to a new device, and diagnostics the owner reads and chooses to send. | must | Confirmed |
| DESK-018 | OpenDAP Desktop should offer a drop folder whose files reach the T1 at once, outside the library (for example mixes to check). | should | Confirmed |
| DESK-019 | OpenDAP Desktop shall be usable at once with any library size, without loading the whole library first. | must | Confirmed |
| DESK-020 | An older device and a newer OpenDAP Desktop, or the reverse, shall keep working together: what one side lacks is disabled, never a reason to refuse the connection. | must | Confirmed |

## Context

- DESK-012 to DESK-020: confirmed.
- DESK-011: the client on the computer is what makes sync and connected features possible; the T1 never needs it to play, and plain files over USB storage need no client.
- DESK-010: whatever is useful may be offered. Off by default; lossless stays lossless unless the listener chooses.
- DESK-006 to DESK-008: Draft until confirmed. The MCP server of the CLI spec (`mcp serve`) can run inside this service.
- DESK-003 and DESK-004 follow from the request ("from the casual user to the advanced one") and from TK-009 (every toolkit tool shares concepts, conventions and one domain model), applying the brand rule "complexity available, never imposed".
- DESK-005: UNIX/POSIX is the main target, tested on macOS (as TK-010, TK-011); Windows is desirable for casual users, so the app avoids POSIX-only assumptions in its core and keeps Windows support possible.
- Scope to define: music library and sync, device setup and updates, settings, faceplates and HATs, capabilities, backups and recovery, and the developer features the CLI covers (shell, logs, flashing, virtual T1).
