# Development

## Set up the development shell

Install Nix and devenv, then run `devenv shell`. If you use direnv, run
`direnv allow` once. To launch the app, follow [Start Splat](../README.md#start-splat).

`devenv.lock` pins the shell inputs. The shell uses Rust 1.95.0.
`Cargo.lock` pins the application dependencies. The Linux shell provides Wayland, X11 and OpenGL libraries. It
uses Nix Mesa for EGL so the app can start on other Linux distributions.

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

### Tests that draw a page

A GPUI page test builds the whole app in a headless window with `gui::harness::Shell` over a
fixture (`crate::fixtures::Seed`), presses and types by the kit's target names, and captures PNGs
to `artifacts/screenshots/gpui/` ([GUI tests](#gui-tests)).

The egui `egui_kittest` tests that remain until each area's egui code is deleted build their
harness with `design::test_harness` (`themed_state`, `themed_ui_state`, `themed`, `themed_ui`), whose
first frame installs the app's theme.

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
| `src/design` | Shared egui widgets and messages (`splat-design`) |
| `src/trackers` | Trackers panel and history (`splat-trackers`) |
| `src/content_ui` | Content catalogue, Lookup, Campaign registration, and shared readers (`splat-content-ui`) |
| `src/library_page` | Library browsing and authoring (`splat-library-page`) |
| `src/world_page` | World page (`splat-world-page`) |
| `src/campaign_packages` | Campaign Packages page (`splat-campaign-packages`) |
| `src/session_records` | Saved fight records, difficulty summaries, and the Timeline book (`splat-session-records`) |
| `src/encounters_ui` | Encounter planning and fight runs (`splat-encounters-ui`) |
| `src/party_support` | Live party sheets, sync, and sheet UI shared with runs (`splat-party`) |
| `src/party_page` | Party page (`splat-party-page`) |
| `src/sessions_ui` | Sessions and Timeline pages, recording, and processing (`splat-sessions-ui`) |
| `src` | Desktop app and public content, character and execution APIs |

Character, execution and campaign can compile in parallel once content metadata
is ready. The desktop library depends on the logic crates directly. The
verification tools use content, character and execution.

The UI crates keep their sources under `src/` so the design checks follow the same files.
A World page, Campaign Packages, Library page, or Trackers edit rebuilds that feature and the desktop.
An edit to the catalogue, Lookup, or a shared reader rebuilds Content UI and the crates that use it.
Encounters UI and Sessions UI share the saved fight records and the Timeline book.
In an app build, edits to either UI crate rebuild that crate and the desktop.
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

## GUI tests

`devenv shell -- gui-test` runs the GPUI page tests (`cargo test --locked --package splat --lib
-- gui::`). Each test builds the app headless with Splat's own fonts and Mesa's Vulkan renderer
(an Intel or AMD GPU when there is one, otherwise the software rasterizer), and writes its
captures to `artifacts/screenshots/gpui/`, which git ignores. No test compares the pictures: look
at the captures your change can affect. `gui-test <filter> [-- <test args>]` runs a subset, for
example `devenv shell -- gui-test gui::pages::run -- --nocapture`.

- Name captures `<scenario>__<step>__<w>x<h>`. A shell's ids and dice come from a hash of its
  name (`first_uuid` in `src/gui/harness.rs`), so a run repeats; renaming a shell gives it new ids
  and dice. App code that makes a new identity on the screen thread takes it from
  `crate::hermetic::new_uuid`, which follows that seed.
- Under load a GUI test can time out. Run it again alone before you call it a fault.
- To prove that a cache keeps input-sized work (a parse, a lookup, a read of the whole draft)
  off unchanged frames, use `CachedWork` in `src/test_support.rs`. Count the work inside the
  function that does it, then move the work outside its guard once: `steady` must fail.

`src/design/lint.rs` fails on raw colours, text sizes and raw icon text outside `src/design/`, on
a straight apostrophe in a UI string, on a phrase or fact written outside its owner, and on an
alias of a `clippy.toml` ban; it reads every Rust file of the app, `src/gui/` included.
`src/design/tokens.rs` tests WCAG contrast.

## Local development layout

This layout describes the original development machine.

Code lives in `~/Projects/splat`. The vault link is
`~/Documents/vault/D20 D&D/00-09 System/09 Tools/splat`.

The link is listed in Obsidian's excluded files. This reduces visibility in
search and related views but is not a complete indexing or filesystem exclusion.
Build artifacts stay in the code repository's ignored `target/` directory.
