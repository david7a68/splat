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
| App workflows, UI, rendering or visual checks | Run `devenv shell -- audit-gate final` and review the affected captures. During edits, use `devenv shell -- visual-test work --only run-lookup,party` with the affected scenarios. |
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
`audit-gate` runs both, plus all workspace tests and captures in one test run.
Reuse the results when a gate already covers the checks you need.

A script may change files that the app embeds through `crates/assets`,
even when the app has no import of that script. Follow the generated files
to their readers. To stop at that boundary, show that the build and readers
get equivalent inputs and behavior, including metadata they use. File extensions
do not tell you the scope. For example, the tracker tests load
`docs/design/world-examples/heralds-tournament.md` as a fixture.

### Tests that draw a page

Build an `egui_kittest` harness with `design::test_harness` (`themed_state`, `themed_ui_state`,
`themed`, `themed_ui`). Its first frame installs the app's theme, so the page never draws at
egui's default sizes; a test that calls `theme::install(&harness.ctx)` after the builder drew
measures a page's one-time results (an open jump, a first fit) at the wrong sizes (issue #810).
The design lint counts those calls per file (`late-theme`), also on a context taken from a
harness (`let ctx = harness.ctx.clone();`), and the count can only shrink. A test that changes the
theme on purpose after a draw (a new font generation, as in `src/content_ui/tests/quiet_reader.rs`
and `src/trackers/view.rs`) keeps its visible call on the harness's own context and a counted row
with its reason in `lint_allowlist.txt`; it never hides the call behind another binding.
`scripts/late_theme.py` moves the standard cases: by default it prints a diff and the cases it
leaves for a hand edit (`MANUAL`); `--write` applies it.

### Red runs

A red run proves a new or changed check can fail: a mutation breaks the code it checks, and the
check fails at its own assertion. `scripts/red_run.py` does it the same way in every lane. It
takes the mutation as a patch (`git diff > mutant.diff` on a scratch edit, then undo the edit),
the expected assertion's words or visual check id, and the focused command after `--`:

```sh
scripts/red_run.py --patch mutant.diff --expect "the two long names are fitted once" \
  -- devenv shell -- cargo test --locked --lib -p splat-trackers -- a_name_in_its_column
scripts/red_run.py --patch t19.diff --expect "T19.4" \
  -- devenv shell -- visual-test red-t19 --only trackers
```

It applies the patch in a scratch copy of the proved head (`--head`, default `HEAD`) under
`artifacts/screenshots/red/`, runs the command there, and keeps the log in
`artifacts/screenshots/red-<label>.log`. It rejects the run (exit 1) when the patch does not
apply, the mutant does not compile, the command passes, or it fails without the expected words in
a failure message (another assertion). Otherwise it prints one row for the PR:
`| patch summary | command | failed assertion | log path |`. Put the command under your build
lease as for any other build (`local-compile.sh <label> -- …`).

### Compare two captures

`scripts/compare_captures.py A B` compares the findings of two visual captures (two
`artifacts/screenshots/<label>` folders), read-only. It first checks that they can be compared (the
same state images and the same recorded renderer; else `PRECHECK_FAILED`, exit 2). Then it prints
the finding rows of `ergonomics.json` and `stillness.json` that are only in A (REMOVED) or only in
B (ADDED), by scenario, check, state and width, with the control's name, the other control of an
overlap, the bounds, and the source file. Numbers in a row's words do not count as a change.
`--scenario a,b` and `--check G1,no-overlap` narrow it; `--skip-preflight` compares rows anyway.

```sh
scripts/compare_captures.py artifacts/screenshots/base-f1 artifacts/screenshots/head-f1 --check G1
```

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

## Visual tests

The visual suite runs the app headlessly with temporary folders and synthetic
data. The test build cannot access your settings, character cache, keyring or
D&D Beyond. The suite compares ergonomics and task costs with
`src/visual_tests/baseline.json`. A worse count fails the test. An improved
count appears in the report.

### Run with captures

```sh
devenv shell -- visual-test iter-0
```

Read the output in `artifacts/screenshots/iter-0/`. It contains PNGs at
960×640 and 1200×800, plus 1920×1080 for width-sensitive states and 2× density
where selected. It also contains `ergonomics.json`, `ergonomics.md`,
`stillness.json`, `manifest.txt`, `renderer.txt` and one
`video/<scenario>.mp4` per scenario when video is on.

Suites without captures write the ergonomics and stillness reports to
`artifacts/screenshots/latest/` unless you set `SPLAT_VISUAL_DIR`. The suite
clears old reports before running. If the child exits early, there is no
completed report for that run.

### Pick a renderer and output format

- On a compatible Intel or AMD GPU, rendering uses Mesa's Vulkan driver.
  Otherwise it uses the software rasterizer. Set
  `SPLAT_VISUAL_RENDERER=software` to force software rendering. Compare pixels
  on the same renderer and computer because GPU and software output can differ
  at edges.
- Video needs the host's `ffmpeg`. Videos run at 12 frames per second and store
  each picture once. The encoder uses the first working option in this order:
  VA-API, NVENC, x264. Check `renderer.txt` for the result.
- Set `SPLAT_VISUAL_VIDEO=0` to skip video. Set
  `SPLAT_VISUAL_ENCODER=vaapi`, `nvenc` or `x264` to choose an encoder.
  If it is unavailable, the run uses x264. At most six videos use NVENC at once.
- Run `devenv shell -- visual-contact-sheet iter-0` to build a contact sheet
  from existing captures, or set `SPLAT_VISUAL_CONTACT_SHEET=1` before a full
  visual run. Contact sheets need `montage` and can use several GB of RAM.

### Read the checks and update baselines

- `SPLAT_VISUAL_ENFORCE=strict` fails on any violation. `report` never fails on a check.
- `SPLAT_VISUAL_UPDATE_BASELINE=1` rewrites the baseline after a full run.
- `no-overlap/app-vs-chrome` counts an app control or text that overlaps a scroll bar or a
  column splitter. There, egui can give the press to the bar or splitter, so it is an app
  fault. No baseline may allow it: a baseline that allows it does not load, and the full
  writer and the chrome refresh refuse it. Fix the clearance. A menu or other popup lies
  over the pane below it: its choices and text over that pane's splitter or scroll bar
  count here too. A menu item over its own popup background stays an egui chrome count,
  because the item takes every press.
  `KNOWN_APP_VS_CHROME` in `src/visual_tests/ergonomics.rs` is empty and may only stay
  empty, so any pair fails the run.
### Stillness checks

Read each finding in `stillness.json`. It gives the scenario, state, window,
control, old and new rectangles and cause. The terminal and `ergonomics.md`
give counts per check. G1 compares consecutive states at the same size.

| Check | Finding |
| --- | --- |
| G1a | A message appears or disappears and moves the content below it while the content above stays in place. |
| G1b | The pressed control moves more than 1 px. Links are exempt. A selected toggle can grow by `TOGGLE_WEIGHT` if it keeps its line and one edge. Matches by ID alone are report-only. |
| G1c | A scroll bar appears or disappears and a right edge moves by the width of the scroll bar lane. |
| G3 | At 1920×1080, a registered row's first action is more than `3 × L` from its last words. Form and table rows are exempt. |
| G6 | A footer sits more than `2 × L` below a short body. |
| G7 | Two live widgets share an ID. |
| N2 | An icon control's centre is more than 1 px from the centre of the adjacent text line. |
| N4 | A quiet action at the start of a line fails to align with adjacent text. In the middle of a line it overlaps what comes before it. |
| N5 | Icon-led text wraps inside its own layout job. |

G2 (header actions disappearing or changing text at narrower widths), G4
(repeated text of four or more words in one pane) and N8 (a word repeated
back to back) are report-only. The check code is in
[`src/visual_tests/stillness.rs`](../src/visual_tests/stillness.rs).

### Review and refresh known counts

- The known lines are `src/visual_tests/stillness_known.json`: per scenario, one line per
  `[check, state, window, name, count]`, where the name is the control or text the check found.
  A blocking check fails the run on a finding that has no line or more than its count, and on a
  listed line of a scenario that ran with fewer. The gate also reads the list of the merge base
  with `origin/main` (`git show`) and fails on any line that is new or higher there, naming its
  key, so the list can only shrink. A raise passes only when `SPLAT_STILLNESS_RAISE` names it, one
  line per raise: `<check> / <scenario> / <state> / <window> / <name> => <reason>`; the report
  prints each raise for the reviewer and `land.sh`. When the base cannot be read, the report says
  so, and the landing gate (`land.sh`) fails.
- To lower your lines after a fix, run your scenarios (`visual-test <label> --only a,b`). Each
  run writes `artifacts/screenshots/<label>/stillness-known.json` with the lines those scenarios
  need now. Replace your scenarios' blocks in `stillness_known.json` with those blocks. Pasting a
  block cannot raise the list: the gate compares it with the merge base.
- `d.expect_moves("reason")` before a state where a move is the point (a reorder, an answered
  question that goes) is the one escape from G1a and G1b; it never covers G1c. Its findings
  stay in `stillness.json` with the reason, and the report counts each use per scenario.
- After reviewing chrome changes, `devenv shell -- visual-baseline-chrome work`
  applies the saved chrome counts from label `work` without running scenarios again.
  This also works after `--only`: it changes only those scenarios' egui chrome counts,
  including counts that fell to zero. App allowances, task costs and other scenarios
  stay as they were. The run must finish without scenario, workflow or app regressions;
  the helper refuses a baseline that changed since the run. A started run clears any
  earlier observation; incomplete scenarios do not generate one. Run the merge checks
  selected for the change.

### Diagnose failed or slow runs

- Targeted runs collect missing `d.expect(...)` text by default.
  `SPLAT_VISUAL_DIAGNOSE=1` also enables collection for a full run. Missing text
  still fails the suite at the end, even in report mode. Missing controls, failed raw assertions
  and async timeouts still stop that scenario. When the scenarios finish, the terminal
  and `ergonomics.md` show all measured baseline changes, even if a scenario panics.
  An early child exit has no completed report. Targeted
  runs keep the normal 180-second painted-text wait; diagnosis uses five seconds.
- If a settle wait reaches 180 seconds in the isolated Linux suite child, it prints
  the scenario diagnostic and exits with status 101. The parent fails and releases
  its suite slot without waiting for scenario cleanup or thread joins. Captures from
  that failed run are incomplete. Outside the isolated child, the check still panics.
- `SPLAT_VISUAL_TIMINGS=1` prints world-lock wait, fixture setup, app/font/renderer
  setup, settling, rendering (captures and video frames) and PNG encoding/writing
  times, then counts of settles, settle frames, renders and PNGs. Per-scenario times overlap
  because the scenarios run in parallel; do not add them as wall time.
- `SPLAT_VISUAL_JOBS=4` (the default) runs four scenarios at a time. A full capture run
  then needs about 1.1 GiB and keeps about four cores busy (about 4.6 with video). More jobs are faster when
  cores are free, and use more memory: about 1.6 GiB at eight.
- `SPLAT_VISUAL_SLOTS` sets how many suites may run at once on this computer, across
  worktrees. The default is one per five cores, and at most one per 4 GiB of RAM.
### Keep scenarios deterministic

- Each scenario's ids and run dice come from a hash of its name. Adding, removing or
  moving another scenario, or running a scenario alone, changes nothing in it. Renaming
  a scenario gives it new ids and new dice. Scenario code, and app code that makes a new
  identity on the screen thread (a copy, a separate entry), takes each new id from
  `crate::hermetic::new_uuid`, which follows that seed. The release app still makes random
  ids. A unit test fails on a direct random id anywhere in `src/visual_tests/`.
- To prove that a cache keeps input-sized work (a parse, a lookup, a read of the whole draft)
  off unchanged frames, use `CachedWork` in `src/test_support.rs`. Count the work inside the
  function that does it. `warm` checks that the view shows the state and reached the work,
  `steady` that unchanged frames and a resize reach the guard but do no work, and `refresh`
  that an edit does the work again. Then move the work outside its guard once: `steady` must
  fail.
- `SPLAT_VISUAL_SCENARIOS=party,settings` runs only those scenarios. `visual-test <label> --only
  party,settings` is the same, and keeps the captures of the label's other scenarios.
### Write a scenario and compare states

- `Sizes::Standard` captures and checks 960x640 and 1200x800. `StandardAndHiDpi`
  adds 2x density. Use `All` or `AllAndHiDpi` for readers, forms, lists, panes,
  action notices and wide proof states where width matters. The older `Default`
  sets capture both sizes but run 1x ergonomics only at the minimum window.
- Each capture must differ from every earlier step of its scenario at the same size
  in at least 0.1% of pixels. For an intended return, first assert the restored state,
  then use `d.assert_state_matches("return-name", "earlier-name")`. It runs the
  earlier state's sizes and checks, and compares every pixel within the existing
  two-level channel tolerance. It writes a PNG only if the comparison fails.
  `state_changed_at` saves the sizes that change and compares the others.
  `state_changed_at_prepared` also retains per-size setup and observations. For example,
  `Sizes::Small` keeps a narrow tab change when a wide layout already shows both panes.
  `check_state` retains all checks when another scenario supplies the same image.
  `state_captured_at` keeps distinct sizes when only some views are shared.
  Use `d.expect_same("reason")` only for an intended small visual change, after its
  behaviour assertion. The repeated-capture check still runs on later captures.
  Captures and frame comparisons finish pending note and Timeline saves first.
### Check the frozen workflows

- `dm-audit` (G6) plays the measured evening of `artifacts/dm-audit/` in every gate and fails when a
  task's page transitions, interactions or path differ from `artifacts/dm-audit/counts.json`. A
  change that makes a task cost more or less on purpose runs `devenv shell -- audit-freeze counts`
  and says why in the pull request; a new task goes in with `devenv shell -- audit-freeze add`.
- `journey-j2`, `journey-j3`, `journey-j4` and `journey-j6` are G6 too: they play the workflow score's other
  journeys the same way and compare them with `artifacts/dm-audit/journeys/<J>/counts.json`. Each fails
  first when a saved record that its journey states is wrong (the note, a Log, `Timeline.md`,
  `Trackers.md`, the run file). `devenv shell -- audit-freeze journeys` writes those rows again;
  `audit-freeze add` adds new tasks to every journey and refuses when a frozen task changed
  (`artifacts/dm-audit/README.md`). A refused freeze changes no frozen file; a failed copy of a
  passed freeze fails the command and keeps the staged files.
- `plain-evening` (G7) always runs and captures, but compares with
  `src/visual_tests/plain_evening/reference.json` only with the switch `SPLAT_FROZEN=1`, as in
  `SPLAT_FROZEN=1 devenv shell -- audit-gate <label>`: any change of the user interface fails it
  by design. Code changes in the session-cycle build plan use the switch, and so can anyone
  who wants the proof. The reference is written by `devenv shell -- audit-freeze reference` on an
  unchanged commit of `main`, which it records in `frozen_at`; a feature adds its views with
  `devenv shell -- audit-freeze add` and rows in `plain_evening/allowed.json`.
  `devenv shell -- audit-freeze verify` writes the reference into a scratch folder at the current
  commit and fails unless the tree's reference is byte for byte the same.
- `audit-gate [label]` runs formatting, Clippy, all tests and all captures. It
  clears scenario filters and baseline-update mode, and enforces the baseline.
  Video and optional contact-sheet settings work as for `visual-test`.

### Scenario coverage

The `gallery` scenario captures every design token and component, including page headers, blocked reasons, full rows with menus, save rows, discard recovery, the Add strip, paused runs and message actions. Each scroll capture adds a section to the evidence. The `missing-glyph` check rejects painted characters missing from the app fonts. The
`clipped` check also catches text partly covered by a later opaque pane. `src/design/lint.rs` fails on raw
colours, text sizes and raw icon text outside `src/design/`, and on a straight apostrophe in a UI string, and `src/design/tokens.rs` tests WCAG contrast.

Scenario labels are at the top of each scenario in `src/visual_tests/scenarios.rs`, and the
ones they share are in `steps.rs`. Goals use the [test IDs in the UX brief](ux-intent.md#8-workflow-to-test-mapping).

The visual suite includes `npc-in-lookup`, `package-rename` and `package-about`. `run-continue` covers Undo after a
close and a restart, Continue fight and the sentence that stands in its place for a fight that was not played last,
and unread Undo steps, each with Lookup closed and open. `encounter-difficulty` covers the difficulty word on
the Ready to run rows, in the editor, on the plan's rows and their opened line, with Lookup closed and open, and
the encounters whose XP or party is not known, whose opened line says whose XP is not known. `run-effects` covers
effects and turn reminders: the dock's **Effect**, a count through a turn and a restart, the row's targets, and Lookup
open. `share-to-package` checks the new references after **Copy both**, and Library navigation checks the dependency copy action and its Undo.

The `package-file-changed`, `package-delete-entry`, `book-to-package` and
`world-note-deleted` scenarios cover disk changes and copies.

## Local development layout

This layout describes the original development machine.

Code lives in `~/Projects/splat`. The vault link is
`~/Documents/vault/D20 D&D/00-09 System/09 Tools/splat`.

The link is listed in Obsidian's excluded files. This reduces visibility in
search and related views but is not a complete indexing or filesystem exclusion.
Build artifacts stay in the code repository's ignored `target/` directory.
