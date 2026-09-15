# Splat

A desktop foundation for D&D 2024 encounter planning and simulation.
Built with Rust and egui (eframe). The initial app provides navigation for
party, encounters, creatures, and simulations; domain features are not yet implemented.

## Development

Install Nix and devenv, then run:

```sh
devenv shell
cargo run --locked
```

Alternatively, with direnv installed, run `direnv allow` once.

```sh
devenv shell check
devenv test
```

Rust 1.95.0 and the Rust overlay follow the other local projects.
`devenv.lock` pins the environment; `Cargo.lock` pins application dependencies.
The Linux shell includes the libraries needed for Wayland, X11, and OpenGL.
It selects Nix Mesa for EGL so the app also starts on non-NixOS hosts.

## Local layout

Code lives in `~/Projects/splat`. The vault link is
`~/Documents/vault/D20 D&D/00-09 System/09 Tools/splat`.
Campaign data remains in the vault. A read-only content verifier and the complete
[built-in SRD 5.2.1 source package](docs/srd-import.md) are available; campaign
content loading in the UI is not implemented yet. The
[execution proof](docs/execution-proof.md) runs converted SRD mechanics from Markdown.

The link is listed in Obsidian's excluded files. This reduces visibility in
search and related views but is not a complete indexing or filesystem exclusion.
Build artifacts stay in the code repository's ignored `target/` directory.

Keep D&D Beyond session cookies and tokens outside Git. Authentication is available
in Settings; character data sync is not implemented. Future simulations will use
live character inputs held in memory, without saved character snapshots.

## D&D Beyond connection

Select **⚙ Settings** at the bottom left. Open the sign-in link and sign in
in your browser. In browser developer tools, open Application (or Storage),
select Cookies for dndbeyond.com, and copy the `CobaltSession` value. Paste
that value into Splat's masked field and select **Connect**.

Splat exchanges the cookie directly with
`https://auth-service.dndbeyond.com/v1/cobalt-token`. It uses HTTPS, disables
redirects, and keeps the bearer token in memory. With **Remember session in
desktop keyring** selected, it saves the cookie through Linux Secret Service
(for example, KWallet or GNOME Keyring). Startup restores that cookie and obtains
a fresh token. If the keyring is unavailable, you can still connect for the
current run; Splat reports that the cookie was not saved.

Tokens refresh automatically; a failed refresh clears the live connection and
displays an error. Disconnect clears local credentials and removes the saved
cookie. If removal fails, Splat reports it and lets you retry. It does not
invalidate the browser session. Network and keyring operations run in a worker;
credential writes and removals run in order.

This is an internal D&D Beyond endpoint, not a supported public API. Character data is not imported yet. Campaign details are fetched into memory, with linked names cached in settings. Authentication tests use a local mock server;
a successful live login requires a real cookie entered in the application.

## User settings and campaigns

The sidebar dropdown selects one active local campaign. Manage campaigns and
configure their D&D Beyond links in **Settings**. Each local campaign has a UUID,
name, and absolute folder path. Renaming or relocating it preserves its UUID.
Removing its entry does not delete files.

Settings uses `$XDG_CONFIG_HOME/splat/settings.json`, or
`~/.config/splat/settings.json` on Linux. Schema version 3 contains
`campaigns` and `active_campaign_id`. Each campaign has one optional
`dndbeyond_campaign_id`. The same D&D Beyond campaign cannot be linked to two
local campaigns. Select **Not linked** to remove a link, or choose another
campaign to replace it.

In Settings, select a local campaign, then **Refresh D&D Beyond campaigns**.
Choose its remote campaign from the D&D Beyond link dropdown. Campaigns linked
elsewhere are disabled. The internal `/api/campaign/stt/user-campaigns` endpoint
supplies campaign details. The remote ID and last fetched name are saved;
DM names and player counts stay in memory. No campaign snapshots or character sheets are stored.

Versions 1 and 2 migrate old project records without changing UUIDs, names, paths,
or unique links. If an old record has multiple remote IDs, or two records share
one remote ID, loading reports the conflict and blocks writes. Keep at most one
unique remote ID per old entry in settings.json and reload. The original file
remains untouched until a successful edit saves version 3.

Settings save with atomic replacement. Invalid or unsupported files are preserved.
External edits detected before a save require a reload. Session credentials stay
in the desktop keyring and memory, never in settings JSON.

The linked campaign name refreshes once whenever its link control becomes visible,
including returning to Settings, scrolling it back into view, switching campaigns,
or reconnecting. UI repaints do not repeatedly request data. The last successful
name is cached as `dndbeyond_campaign_name` and shown while offline or refreshing.
Failed requests retain the cache. Replacing or removing a link clears its name;
late responses cannot apply an old name to a new link.
