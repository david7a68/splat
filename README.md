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
Campaign data remains in the vault; data loading is not implemented yet.

The link is listed in Obsidian's excluded files. This reduces visibility in
search and related views but is not a complete indexing or filesystem exclusion.
Build artifacts stay in the code repository's ignored `target/` directory.

Keep D&D Beyond session cookies and tokens outside Git. Authentication is available in Settings; data sync is not implemented. Simulations will use local snapshots of character data.

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

This is an internal D&D Beyond endpoint, not a supported public API. No character
or campaign data is imported yet. Authentication tests use a local mock server;
a successful live login requires a real cookie entered in the application.

## User settings and projects

Settings stores the project list at `$XDG_CONFIG_HOME/splat/settings.json`, or
`~/.config/splat/settings.json` when XDG_CONFIG_HOME is not set on Linux.
Project entries contain a UUID v4, display name, and absolute folder path.
`active_project_id` refers to the selected project's UUID. Renaming or relocating
a project keeps its UUID. The JSON schema begins at version 1.

Use the Projects section in Settings to add, select, edit, or remove an entry.
Changes save immediately through a temporary file and atomic replacement.
Removing an entry does not delete its folder. Unavailable folders remain listed
and can be located again by editing their paths. Project contents are not yet
managed by Splat.

Missing settings start with an empty list. Invalid files and unsupported versions
block writes and display an error. Repair the file and select **Reload project
list** to continue. External edits detected before a save also require a reload.
Session cookies and access tokens are never written to this JSON file.
