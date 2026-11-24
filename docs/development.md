# Development

## Set up the development shell

Install Nix and devenv, then run `devenv shell`. If you use direnv, run
`direnv allow` once. To launch the app, follow [Start Splat](../README.md#start-splat).

`devenv.lock` pins the shell inputs. The shell uses Rust 1.95.0.
`Cargo.lock` pins the application dependencies. The Linux shell provides the libraries that GPUI needs:
fontconfig, FreeType, xkbcommon, Wayland, xcb and the Vulkan loader. It uses Nix Mesa for Vulkan and EGL,
so the app can start on other Linux distributions.

## Check a change

Use the [dependency trace in AGENTS.md](../AGENTS.md#choose-code-checks) to
select checks for the proposed merge source. For prose with no effect on code
or its inputs, review the text, links and commands as applicable.

| Affected domain | Checks |
| --- | --- |
| Rust code in the app or workspace crates | Run formatting, Clippy and the tests for the affected packages. Include the callers that the trace reaches. |
| App workflows, UI, rendering or captures | Run the affected pages' GUI tests (`devenv shell -- gui-test gui::pages::run`) and review their captures in `artifacts/screenshots/gpui/`. Before review, run all of them (`devenv shell -- gui-test`). |
| Scripts and command-line tools | Run the affected tests and exercise the changed command through its normal entry point. Check arguments, output, exit codes and file changes that callers depend on. |
| Generators, schemas and bundled content | Run the relevant generator or verifier, compare the outputs and test the readers the trace reaches. See [SRD conversion checks](srd-definitions.md#reproduce-and-verify). |
| Build, shell and test setup | Run the changed build, shell or check through its normal entry point. Follow any effect on its users. Use `devenv test` for the devenv setup. |

For a package-level Rust check, substitute the affected package name for
`splat-content` in the Clippy and test commands.

```sh
devenv shell -- cargo fmt --all --check
devenv shell -- cargo clippy --locked -p splat-content --all-targets -- -D warnings
devenv shell -- cargo test --locked -p splat-content
```

`devenv shell -- check` runs formatting and Clippy for the workspace.
`devenv test` and `cargo test --locked --workspace` run every test, the GUI tests too.

A script may change files that the app embeds through `crates/assets`,
even when the app has no import of that script. Follow the generated files
to their readers. To stop at that boundary, show that the build and readers
get equivalent inputs and behavior, including metadata they use. File extensions
do not tell you the scope. For example, the tracker tests load
`docs/design/world-examples/heralds-tournament.md` as a fixture.

GPUI turns on serde_json's `preserve_order` in every build that includes it (the app, `--workspace`
tests). There a `serde_json::Map` keeps insertion order and does not sort its keys. A build of one crate
without GPUI (`cargo test -p splat-character`) still sorts. So a test of order must run in a build with
GPUI. Code that iterates a map, compares key lists, hashes serialized bytes or writes a map to a file, and
needs an order, sorts the map itself (`Value::sort_all_objects` or `Map::sort_keys`). Map equality does not
change. These already sort: `content::contracts::fingerprint`, D&D Beyond snapshots on read
(`character::Source`), D&D Beyond pins (`character::pin`), importer notes (`rules::written`), campaign
package keys, coverage metadata, combat sync writes and `execution-proof` output.

### Red runs

A red run proves a new or changed check can fail: a mutation breaks the code it checks, and the
check fails at its own assertion. `scripts/red_run.py` does it the same way in every lane. It
takes the mutation as a patch (`git diff > mutant.diff` on a scratch edit, then undo the edit),
the expected assertion's words, and the focused command after `--`:

```sh
scripts/red_run.py --patch mutant.diff --expect "the two long names are fitted once" \
  -- devenv shell -- cargo test --locked --lib -p splat-trackers -- a_name_in_its_column
scripts/red_run.py --patch t19.diff --expect "the tracker's row moves down" \
  -- devenv shell -- gui-test gui::pages::trackers
```

It applies the patch in a scratch copy of the proved head (`--head`, default `HEAD`) under
`artifacts/screenshots/red/`, runs the command there, and keeps the log in
`artifacts/screenshots/red-<label>.log`. It rejects the run (exit 1) when the patch does not
apply, the mutant does not compile, the command passes, or it fails without the expected words in
a failure message (another assertion). Otherwise it prints one row for the PR:
`| patch summary | command | failed assertion | log path |`. Put the command under your build
lease as for any other build (`local-compile.sh <label> -- …`).

## Build layout

| Path | Contents |
| --- | --- |
| `crates/assets` | Bundled assets |
| `crates/content` | Content and search ranking |
| `crates/character` | Character data and rules |
| `crates/execution` | Rules execution |
| `crates/campaign` | Campaign data, tracker rules and ledger operations |
| `crates/cli` | Verification tools |
| `src/design` | Design tokens, the plain-data design parts (typed messages, kept results, the save row) and the design lint (`splat-design`) |
| `src/trackers` | Trackers state and history (`splat-trackers`) |
| `src/content_ui` | Content catalogue, Lookup, Campaign registration, and shared readers (`splat-content-ui`) |
| `src/library_page` | Library browsing and authoring state (`splat-library-page`) |
| `src/world_page` | World page state (`splat-world-page`) |
| `src/campaign_packages` | Campaign Packages state (`splat-campaign-packages`) |
| `src/session_records` | Saved fight records, difficulty summaries, and the Timeline book (`splat-session-records`) |
| `src/encounters_ui` | Encounter planning and fight runs (`splat-encounters-ui`) |
| `src/party_support` | Live party sheets and sync, shared with runs (`splat-party`) |
| `src/party_page` | Party page state (`splat-party-page`) |
| `src/sessions_ui` | Sessions and Timeline state, recording, and processing (`splat-sessions-ui`) |
| `src/gui` | The GPUI app: root view, theme, kit, pages and test harness (in `splat`) |
| `src` | Desktop app and public content, character and execution APIs |

Character, execution and campaign can compile in parallel once content metadata
is ready. The desktop library depends on the logic crates directly. The
verification tools use content, character and execution.

The feature crates keep their sources under `src/` so the design checks follow the same files.
Each page's view is in `src/gui/pages/<area>/` in the `splat` crate. It uses its crate's public API.
A World page, Campaign Packages, Library page, or Trackers edit rebuilds that feature and the desktop.
An edit to the catalogue, Lookup, or a shared reader rebuilds Content UI and the crates that use it.
Encounters UI and Sessions UI share the saved fight records and the Timeline book.
In an app build, edits to either crate rebuild that crate and the desktop.
The Sessions tests use the Encounters crate to create saved-run fixtures.
Party page edits rebuild that page and the desktop. Edits to the shared sheets or
sync also rebuild the Encounters UI that uses them.

Development and test builds use 16 code-generation units and include file and
line information in backtraces. They disable incremental compilation by
default. Set `CARGO_INCREMENTAL=1` to enable it while editing. To see local
variables in a debugger, pass `--config 'profile.dev.debug=2'` to `cargo run`
or `--config 'profile.test.debug=2'` to `cargo test`. External dependencies
keep their optimization level.

### Build in parallel worktrees

1. In the seed worktree, run `devenv shell -- audit-warm`. This builds test
   binaries and the metadata for the Clippy command that `check` uses.
2. Copy the seed worktree's `target/` with `cp -a --reflink=always`.
3. Give each worktree its own build directory to avoid Cargo lock waits.
   Keep build output on disk. The audit machine uses RAM for `/tmp`.

## How the app runs

The UI is GPUI:

- `gpui` is `gpui-pre` 0.3.14, a crates.io snapshot of Zed's GPUI. `gpui_platform` is
  `gpui-pre-platform` with the `wayland` and `x11` features. Tests add the `test-support` feature.
- `gpui-component` 0.11 draws the hard widgets: text input and editor, scroll bars, resizable panels,
  popovers, menus and virtual lists. Pages never call it directly. They call the kit (`src/gui/kit/`),
  which wraps it in Splat's tokens.
- Do not add `gpui` 0.2.x. Its `xattr` dependency does not build against the locked `libc`.

`splat::run()` (`src/main.rs`) runs a command when there are arguments, takes the instance lock, then
opens the GPUI app (`gui::run`). The code is in `src/gui/`:

| Path | Contents |
| --- | --- |
| `src/gui/mod.rs` | `run()`, the window, the root view (`SplatRoot`), the rail and the shell around the pages |
| `src/gui/theme.rs` | The tokens (`src/design/tokens.rs`) as GPUI values, the fonts, and gpui-component's theme |
| `src/gui/icons.rs` | The icon names |
| `src/gui/kit/` | The shared parts, one file for each part ([design system: Widgets](design-system.md#widgets)) |
| `src/gui/pages/<area>/` | One folder for each page area |
| `src/gui/harness.rs` | Headless test windows (`#[cfg(test)]`) |
| `src/gui/gallery.rs` | The kit gallery (tests only) |

**The root.** `gui::SplatRoot` is the root view. `root.app` is the app's state (`crate::Splat`, with
`app.sessions`, `app.encounters`, `app.lookup` and the rest). The private items of the crate root are
visible to `src/gui/`. `root.pages.<area>` is an area's GPUI-only state: input entities, scroll handles,
focus handles. Logic stays in the state types. The view reads the state and calls its methods.

**The context.** Some state methods take the state context (`state_context::StateContext`, in
`src/state_context.rs`) for data only. It has typed slots (`Slot<T>`: requests that one page leaves for
another, listed in `src/requests.rs`, and values a part keeps for one view), the pass (`begin_pass` moves
the frame clock and counts passes) and repaint requests. The root keeps it as `root.ctx` and pages pass
`&root.ctx` to those methods. It is a cheap handle for the UI thread; a background thread takes its
`Waker` (`ctx.waker()`) and asks for a frame with it, which wakes the GPUI window. A state method's colour
is a `design::Color` token: `theme::color` turns it into a GPUI colour, and `kit::Status::of` turns a
`design::Status` into the kit's.

**Each frame.** `SplatRoot::render` first runs `frame_logic`, which draws nothing, in this order: the
shared background polls (`Splat::poll_background`), `sync_party`, each area's `poll`, the tracker acts,
`Splat::route` and `follow_page_change`. Then it saves `ui_state` when it changed. Between frames one
timer task (every 50 ms) runs `frame_logic`. It asks for a frame only when background work asked for a
repaint. So an event handler sets a `wants_*` flag or calls a state method, then calls `cx.notify()`. The
next frame routes the request.

**Pages and widths.** `root.show_page(Page::X, cx)` changes the page. `root.modal_open(cx)` says whether a
modal layer is open (the close prompt, the copy dialog, a kit dialog or menu). `root.page_width(window)` is
the page's own width: the window less the rail, the side panel and the page's sides. `root.rail_width()`
and `root.side_width()` are the shell's columns in this frame. Never compute them from the rail or panel
tokens: the DM can drag both. The touchpad speed (Settings → Touchpad) scales every touchpad scroll in the
window (`gui::touchpad`), so a page does nothing for it.

**Keys.** The global keys are actions in `gui/mod.rs`: Ctrl+K Lookup, Ctrl+J notes, Ctrl+T Trackers,
Ctrl+\ the session list, Ctrl+S, and Escape (an open menu, then the notes panel, then Lookup). A page binds
its own keys with `actions!` and `KeyBinding` in its `init` (`cx.bind_keys`), and
`.on_action(cx.listener(..))` on its element, with its own `key_context`.

- Ctrl+S has one owner, the root (`SplatRoot::save`). It saves nothing while a modal layer is open
  (`modal_open`). Else it saves the Notes panel's note when that note had the keyboard. Else it saves the
  page shown, through its area's `save_key(root, cx)` hook (`pages::save_key`). No page binds Ctrl+S, so
  the key saves the same draft wherever the keyboard is (a rail item, the Lookup field). A page with a
  new draft adds its save to its `save_key`, and saves exactly when its Save button can.
- Escape: a page may bind it in its own `key_context`. When it has nothing to close, it calls
  `cx.propagate()`, and the root's `Dismiss` runs.
- Key bindings run before `on_key_down` listeners. So a menu over a text editor (the note's `/` menu)
  cannot take Up, Down, Enter or Escape from a `capture_key_down`. Use `cx.intercept_keystrokes` (keep
  the `Subscription` in the page's state) and `cx.stop_propagation()`, as `pages/sessions/slash.rs` does.

**How a page hooks in.** Each `src/gui/pages/<area>/mod.rs` has these hooks. The shell calls them. Keep
their names and signatures (`R` is `crate::gui::SplatRoot`):

| Hook | Signature | Called |
| --- | --- | --- |
| `State` | `#[derive(Default)] pub struct State` | kept in `root.pages.<area>` |
| `init` | `fn(&mut R, &mut Window, &mut Context<R>)` | once, when the root is built |
| `poll` | same | each frame, before anything draws |
| `render` | `fn(&mut R, &mut Window, &mut Context<R>) -> AnyElement` | the page, inside the page's `M`/`S` padding |

The shell also calls these: `run::render_strip` (above other pages while a run is open),
`party::render_bar` (`Option`, on the Session page; it sets `sessions.party_hidden`),
`sessions::render_banner` (`Option`, at the window's bottom under the page), `sessions::render_notes_panel`
and `lookup::render_panel` (the side panel, `PANEL_MIN_WIDTH` wide; Lookup calls `trackers::render_view`
while `lookup.on_trackers()`), `library::render_copy_dialog` (while `app.copy_dialog` is open),
`settings::render_account`, `render_recording`, `render_add_campaign_form`, and
`campaigns::render_packages`, `render_run_settings` and `render_link`. The shell draws the Settings and
Campaign headers and tabs, Settings → Touchpad, the no-campaign state of pages that need a campaign, the
rail and the close prompt. To add a hook, add it to the area's folder and one call in `gui/mod.rs`.

Event handlers are `cx.listener(|root: &mut SplatRoot, event, window, cx| { …; cx.notify() })`. Kit parts
take `Fn(&ClickEvent, &mut Window, &mut App)`, which `cx.listener` makes.

## GUI tests

`devenv shell -- gui-test` runs the GPUI page tests (`cargo test --locked --package splat --lib
-- gui::`). Each test builds the app headless with Splat's own fonts and Mesa's Vulkan renderer
(an Intel or AMD GPU when there is one, otherwise the software rasterizer), and writes its
captures to `artifacts/screenshots/gpui/`, which git ignores. No test compares the pictures: look
at the captures your change can affect. `gui-test <filter> [-- <test args>]` runs a subset, for
example `devenv shell -- gui-test gui::pages::run -- --nocapture`.

### The harness

`gui::harness::Shell` (`src/gui/harness.rs`) runs the whole app in a headless window over a fixture
(`crate::fixtures::Seed`). It has only Splat's fonts, so text is deterministic, including Hack for
monospace (`splat_assets::MONO`). It uses the real Vulkan renderer at scale 1.

```rust
use crate::gui::harness::Shell;
use crate::fixtures::Seed;

#[test]
fn library_browse() {
    let mut shell = Shell::new("library-browse", Seed::Campaign);       // 1200×800
    shell.click("Library");                    // a kit target by its words
    shell.click("id:library-new");             // or by its element id
    shell.type_text("Ash");                    // into the focused field
    shell.press("ctrl-s escape");              // keys, as GPUI writes them
    shell.capture("library-browse__01-list__1200x800");  // artifacts/screenshots/gpui/
    assert_eq!(shell.read(|root| root.app.page), crate::Page::Library);
}
```

- `Shell::new(name, Seed)` builds the fixture of that seed (`fixtures::World`). Use
  `Shell::with_size(name, seed, size(px(w), px(h)))` for another window size: a window cannot change
  size after it opens. Use `Shell::over(app, tempdir, size)` for a `Splat` that you built.
- `update(|root, window, cx| …)` changes state as a test's setup. `read(|root| …)` reads it. `settle()`
  runs frames until the app is idle. It also waits for a read of the party's sheets in flight, on the
  read's own result, as egui's driver did: the party bar, the run's Sheet tab and synced roster rows
  show those sheets. `wait(Duration)` moves the clock on (timers, the poll).
- `bounds(name)` and `click_at(point)` find and press a place. `drag(from, to)` drags (a splitter), and
  `drag_target(from, to)` drags between two targets by name. `scroll(at, ScrollDelta, Modifiers)`
  scrolls: `Pixels` is a touchpad, `Lines` a wheel.
- `frame_cost(frames)` gives the mean time of a quiet frame (`gui::tests::frame_cost_report`).
- A control's harness names are its words, its accessible name when `.name(..)` says more (`Button`,
  `Link`), and `id:<element id>`: `shell.click("Undo remove Ash")` or `shell.click("id:…")`. Use
  `kit::target(element, name)` to name a pressable element that is not a kit part. GPUI's
  `debug_selector` bounds cannot be read from a headless window, so the harness needs these targets.
- Put an area's tests in its folder (`pages/<area>/tests.rs`, `#[cfg(test)] mod tests;`). Run them with
  `devenv shell -- gui-test gui::pages::<area>`.

### Captures and repeat runs

- Name captures `<scenario>__<step>__<w>x<h>`.
- A shell's ids and dice come from a hash of its name (`first_uuid` in `src/gui/harness.rs`), so a run
  repeats. Renaming a shell gives it new ids and dice. App code that makes a new identity on the screen
  thread takes it from `crate::hermetic::new_uuid`, which follows that seed.
- The kit gallery (`src/gui/gallery.rs`) shows every kit part on one page:
  `artifacts/screenshots/gpui/gpui-kit__01-gallery__1200x800.png`, `__02-menu` and `__03-dialog`. A test
  sets `root.gallery = Some(Gallery::default())` to show it in place of the page. A test sets
  `root.pages.reader.preview = Some(Rc::new(|root, window, cx| element))` to show one part on its own
  (the Reader's tests show an entry view without its page this way).
- Under load a GUI test can time out. Run it again alone before you call it a fault.
- To prove that a cache keeps input-sized work (a parse, a lookup, a read of the whole draft)
  off unchanged frames, use `CachedWork` in `src/test_support.rs`. Count the work inside the
  function that does it, then move the work outside its guard once: `steady` must fail.

### Lints

`src/design/lint.rs` fails on raw colours, text sizes and raw icon text outside `src/design/`, on
a straight apostrophe in a UI string, on a phrase or fact written outside its owner, and on an
alias of a `clippy.toml` ban. It reads every Rust file of the app, `src/gui/` included.
`src/design/tokens.rs` tests WCAG contrast.

### GPUI notes

- `ElementId::NamedChild` takes an `Arc<ElementId>`. Tuple ids exist for `(&str, u32/u64/usize…)`, not
  for `(&str, &str)`. Use `ElementId::Name(format!(..).into())`.
- `anchored()` starts at its parent's origin, not where it is in a flex column. Put it in a zero-height
  box after the trigger, as `kit::menu` does, to open it under the trigger.
- `deferred` lays out its child with the main tree, before anything is prepainted. So a layer that needs
  its trigger's place (`kit::layer`) takes the size from the last frame's place, and asks for one more
  frame when the place changed.
- `Keystroke::parse` needs a `&'static str`. Use `parse_with_os` for a string that you build.
- `window.focus(&handle, cx)` takes `cx`. `focus_handle(cx)` on an input state needs `gpui::Focusable` in
  scope.
- `window.use_keyed_state(id, cx, init)` keeps per-element state between frames (the kit's text fields
  and dialogs use it). Its key joins the ids of the elements around it.
- A headless window draws at scale 2 unless told otherwise. The harness sets 1.
- A test window does not change size: `window.resize` leaves the viewport as it was.
- There is no letter spacing. `kit::text` draws `Type::Label`'s +0.6 tracking with a hair space (U+200A,
  0.65 px in IBM Plex at 11 px) between the letters of each word. A page that sets `.ty(Type::Label)` on
  its own element gets no tracking: use `kit::text`.
- A node without `.role(..)` is not in the accessibility tree, whatever its `aria_label`.
- A focusable element is not a Tab stop unless it has `.tab_stop(true)`.

## Local development layout

This layout describes the original development machine.

Code lives in `~/Projects/splat`. The vault link is
`~/Documents/vault/D20 D&D/00-09 System/09 Tools/splat`.

The link is listed in Obsidian's excluded files. This reduces visibility in
search and related views but is not a complete indexing or filesystem exclusion.
Build artifacts stay in the code repository's ignored `target/` directory.
