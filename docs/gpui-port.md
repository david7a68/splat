# Port from egui to GPUI

This file is the shared plan for the agents that port Splat's UI from egui to GPUI. When the port
merges, its architecture notes move into `docs/development.md` and `docs/design-system.md`, and
this file is deleted.

## Goal

Splat runs on GPUI. Every page, dialog and flow that the egui app has works in the GPUI app, and
looks substantially the same or better. The visible words stay the same (`CONTEXT.md`,
`docs/user-guide.md`). At the end, no crate depends on `egui`, `eframe`, `egui_extras`,
`egui_kittest` or `egui-phosphor`.

The user accepts a smaller visual test harness than the egui one: headless GPUI screenshots of
each page's main states, plus interaction tests of the main flows.

## Versions

- `gpui = { package = "gpui-pre", version = "0.3.14" }` (a crates.io snapshot of Zed's GPUI).
- `gpui_platform = { package = "gpui-pre-platform", version = "0.3.14", features = ["wayland", "x11"] }`.
  Start the app with `gpui_platform::application()`. Tests use the `test-support` feature,
  `gpui_platform::current_headless_renderer()` and `HeadlessAppContext::capture_screenshot`.
- `gpui-component = "0.11"` (built on `gpui-pre` 0.3.14) for the hard widgets: text input and
  editor, scroll bars, resizable panels, popovers and menus, virtual lists. Pages never call
  `gpui-component` directly; they call the Splat kit (below), which wraps it in Splat's tokens.
- Do not add `gpui` 0.2.x: its `xattr` dependency does not build against the locked `libc`.

## Build

- Every cargo command goes through the shared build-slot wrapper, from the worktree root:
  `/home/david/.t3/worktrees/splat/gpui-port/cargo.sh build`, `... cargo.sh test --lib gui::`,
  `... cargo.sh clippy --all-targets -- -D warnings`. It holds one of four machine-wide slots and
  runs cargo inside `devenv shell`. Do not call `cargo` directly, and do not raise the slot count.
- Keep the target in the worktree's own `target/`. Never put a target or a big log in `/tmp`
  (it is RAM). Put logs and screenshots under `artifacts/screenshots/` (ignored by git).
- `devenv.nix` provides the libraries GPUI needs (fontconfig, FreeType, xkbcommon, Wayland, xcb,
  Vulkan loader).

## Shape of the code

- The GPUI UI lives in the `splat` crate as `src/gui/`, so it can use the app's private state.
  - `src/gui/mod.rs`: `run()`, the window, the root view and the page rail.
  - `src/gui/theme.rs`: Splat's tokens (`src/design/tokens.rs`, `docs/design-system.md`) as GPUI
    values: colours, type sizes, spacing, radii, fonts.
  - `src/gui/kit/`: shared parts, one file per part. Pages build from these.
  - `src/gui/pages/<area>/`: one folder per page area. Each page agent owns its folder.
  - `src/gui/harness.rs` (`#[cfg(test)]`): headless screenshots and interaction helpers.
- Other workspace crates with egui views (`splat-trackers`, `splat-party`, `splat-content-ui`, …)
  keep their state and logic. Their GPUI view goes in `src/gui/pages/<area>/` and uses their
  public API; make an item `pub` when the view needs it.
- The root view owns the app state (the existing `Splat` state and page structs) and the GPUI-only
  state (input entities, scroll handles, focus handles). Event handlers use `cx.listener` and call
  the same state methods that the egui code calls. Keep logic in the state types; the view only
  reads state and sends actions.
- Background work (file watch, sync, recording, network) that egui polled each frame is polled by
  one timer task in the root view, which calls `cx.notify()` only when something changed.

## Waves

1. **Foundation** (one agent, this checkout): dependencies, `devenv.nix`, `src/gui/` with the
   window and rail, the theme, the core kit, a basic Markdown view, the screenshot harness, and a
   placeholder slot for each page. `cargo run` opens the GPUI app. The egui code still compiles and
   still runs its tests; nothing in it is deleted yet.
2. **Pages** (one agent per area, each in its own worktree branched from the foundation commit).
   Each agent ports its area completely, adds kit parts it needs as new files in `src/gui/kit/`,
   and adds screenshot scenarios and interaction tests for its area. It does not delete egui code.
3. **Cleanup**: switch every remaining caller, delete the egui UI code, the egui visual tests,
   the egui-only lints and audit tools, and the egui dependencies; move logic tests that lived in
   egui files; update the docs; final review.

## Page areas (wave 2)

| Area | egui source (reference) | GPUI folder |
| --- | --- | --- |
| Sessions | `src/sessions/`, `src/sessions.rs`, `src/sessions_ui/`, `src/session_records/` | `pages/sessions/` |
| Encounter run | `src/encounter_run.rs`, `src/encounter_run/`, `src/combat_sync.rs`, `src/rolls.rs`, `src/roll_table.rs` | `pages/run/` |
| Encounters | `src/encounters.rs`, `src/encounters/`, `src/difficulty.rs`, `src/creature_filter.rs`, `src/loot_editor.rs`, `src/encounters_ui/` | `pages/encounters/` |
| Library | `src/library.rs`, `src/library/`, `src/processing.rs`, `src/processing/`, `src/dndbeyond.rs`, `src/credentials.rs`, `src/copy_dialog.rs`, `src/library_page/` | `pages/library/` |
| World | `src/world.rs`, `src/world/`, `src/world_page/` | `pages/world/` |
| Timeline and trackers | `src/timeline.rs`, `src/timeline/`, `src/trackers/` | `pages/timeline/`, `pages/trackers/` |
| Campaigns, party, settings, lookup | `src/campaigns.rs`, `src/campaign_content.rs`, `src/campaign_content/`, `src/party*.rs`, `src/party_support/`, `src/party_page/`, `src/settings.rs`, `src/lookup.rs` | `pages/campaigns/`, `pages/party/`, `pages/settings/`, `pages/lookup/` |
| Reader | `src/markdown_view.rs`, `src/markdown_source.rs`, `src/content_ui/` | `kit/markdown/`, `pages/reader/` |

## Rules for every agent

- Read `AGENTS.md`, `CONTEXT.md`, `docs/design-system.md`, `docs/user-guide.md` and the egui
  source of your area before you write code. The egui code is the specification of behaviour:
  every control, keyboard path, message, Undo, confirm and empty state it has, the port has.
- Reference screenshots of the egui app at 1200×800 are in
  `artifacts/screenshots/egui-reference/` (named `<scenario>__<step>__1200x800.png`; the scenarios
  are in `src/visual_tests/scenarios*`). Render your GPUI page in the same states with the harness
  and compare them by eye. Write the differences you kept on purpose in your final message.
- UI text: same words as the egui app. Colours, sizes and type come from `gui::theme`, never
  literals in a page.
- Targets are at least 24 px. Keep the egui layout's spacing tokens.
- Do not edit another area's folder. Shared kit: add new files; change an existing kit file only
  when the change is small and additive, and say so in your final message.
- Commit on your branch in small steps with clear messages. Do not push and do not open a PR; the
  lead merges.
- Before you finish: `cargo.sh build`, `cargo.sh clippy --all-targets -- -D warnings`,
  `cargo.sh test --lib gui::` and the tests of your area pass. Report what you ported, what you
  did not, and the screenshots you made.
