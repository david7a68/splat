# Splat

A desktop foundation for D&D 2024 encounter planning and simulation.
Built with Rust and egui (eframe). The design system is in [docs/design-system.md](docs/design-system.md).
Form labels use sentence case. Placeholders use faint body text. Reader headings appear only above visible content;
Splat data stays out of the reading view. Dialogs keep red actions at the left and
keep and commit actions at the right on one line.
The sidebar has one flat level: **Run** (while a run is open), **Paused run** (one unfinished fight) or **Paused runs** (several unfinished fights), **Encounters**, **Session**, **Timeline**, **World**, **Party**,
**Library**, **Campaign**, and **Settings**. The campaign switcher is always above them. **Lookup** (Ctrl+K) searches every entry, every session note and the timeline, and
opens beside any page. While a run is open, other pages show a strip with the turn, primary **Next turn**, and secondary **Back to run**. Each page keeps its
state, and the app reopens on the last page or the open run. Drag the edge of the sidebar, a page's list, or a panel to
change its width. Tab also focuses the edge: Left and Right move it by 16 px, and
Home restores its natural width. The app keeps a chosen width after a restart. Simulation is later work.

The run's **Add to fight** button keeps its plus icon and the word **Add** at narrow widths.
Exhaustion level uses the shared number stepper at Scan height. Opening Correct does not change sync state.
Turn headings drop “’s turn” before shortening a name and keep each creature's copy number.
Party sheet tabs stay on one line; later pages appear in a quiet **More** menu. The menu names and underlines a selected page inside it. Stopped-sync notices sit below the Party header.

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

For parallel design work, run `devenv shell -- audit-warm` once in the seed worktree
before copying its `target/` with `cp -a --reflink=always`. This builds the test
binaries and warms the exact Clippy command used by `check`. A test-only seed does
not contain all the metadata Clippy needs. Keep a separate `target/` in each
worktree; a shared directory causes Cargo lock waits. Keep build output on disk,
not in `/tmp` (a RAM filesystem on the audit machine).

After a set of edits, run `check` first for fast compiler and lint feedback, then
`cargo test --locked`. `cargo check --locked --tests` is also useful while editing,
but does not replace either gate. Before merging, run
`devenv shell -- audit-gate final` and review all captures. This runs `check` and
every unit test, with the full visual suite capturing in the same test run.
It avoids running the scenarios once without captures and again with captures.
The separate `cargo test --locked`, `check`, and full `visual-test` commands remain
valid. `--only` is for local feedback.

The workspace separates bundled assets (`crates/assets`), content and rules
(`crates/core`), verification tools (`crates/cli`), and desktop code (`src`).
The desktop library can start compiling when core metadata is ready, while core
code generation continues. The tools depend on core and do not compile the GUI.
The `splat` library keeps its content, character, and execution APIs.
`cargo test --locked` runs tests in all workspace crates.

Development and test builds use 16 code-generation units, disable incremental
compilation, and keep file and line information for backtraces. Set
`CARGO_INCREMENTAL=1` to enable incremental compilation while editing. To include
local variables when debugging, use `--config 'profile.dev.debug=2'` with
`cargo run`, or `--config 'profile.test.debug=2'` with `cargo test`.
External dependencies keep their existing optimization level. Release settings
are unchanged.

## Visual tests

`cargo test --locked` runs every usage scenario on the real app, headlessly and with no
GPU, and checks ergonomics and task costs against `src/visual_tests/baseline.json`. A
worse count fails the test, and a better one is printed. The tests use temporary folders and
synthetic data. Test builds have no route to your settings, character cache, keyring or
D&D Beyond.

Each completed suite writes `ergonomics.json` and `ergonomics.md` to
`artifacts/screenshots/latest/`, or to `SPLAT_VISUAL_DIR` when set. Each suite clears
the old reports before starting its scenarios, even without captures.
If the child exits before reports are written, that folder has no ergonomics report
for the run.

```sh
devenv shell -- visual-test iter-0
devenv shell -- visual-test work --only run-lookup,party
devenv shell -- audit-gate final
```

This also writes to `artifacts/screenshots/iter-0/`: PNGs at 960x640 and 1200x800,
with 1920x1080 for width-sensitive layouts and recorded wide proof states. Selected
states also have a 2x-density capture. The folder holds `ergonomics.json`, `ergonomics.md`,
`manifest.txt`, `renderer.txt` and one `video/<scenario>.mp4` per scenario. Rendering uses
Mesa's Vulkan driver for an Intel or AMD GPU, or its software rasterizer when there is no
such GPU. `SPLAT_VISUAL_RENDERER=software` always uses the software rasterizer. Two runs
on the same renderer give the same pixels. A GPU and the software rasterizer differ on a
few edge pixels, so compare runs from the same computer. Video needs the host's
`ffmpeg`. Videos are 12 frames per second, and each picture is stored once. They use the
first H.264 encoder that works: VA-API for Intel or AMD, then NVIDIA's
NVENC, then x264 on the CPU. `renderer.txt` names the encoder. The optional contact sheet needs `montage` and can use several GB of RAM.
Use `devenv shell -- visual-contact-sheet iter-0` to make one from existing captures,
or set `SPLAT_VISUAL_CONTACT_SHEET=1` to make it with a full `visual-test` run.

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
- After reviewing chrome changes, `devenv shell -- visual-baseline-chrome work`
  applies the saved chrome counts from label `work` without running scenarios again.
  This also works after `--only`: it changes only those scenarios' egui chrome counts,
  including counts that fell to zero. App allowances, task costs and other scenarios
  stay as they were. The run must finish without scenario, workflow or app regressions;
  the helper refuses a baseline that changed since the run. A started run clears any
  earlier observation; incomplete scenarios do not generate one. The full merge gates still run.
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
- `SPLAT_VISUAL_VIDEO=0` turns video off for `visual-test`.
- `SPLAT_VISUAL_ENCODER=vaapi`, `nvenc` or `x264` uses only that kind of video encoder.
  An unavailable one falls back to x264. At most six videos at once use NVENC, because
  NVIDIA drivers limit encode sessions. Other videos then use x264.
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
  by design. The pull requests of the session-cycle build plan use the switch, and so can anyone
  who wants the proof. The reference is written by `devenv shell -- audit-freeze reference` on an
  unchanged commit of `main`, which it records in `frozen_at`; a feature adds its views with
  `devenv shell -- audit-freeze add` and rows in `plain_evening/allowed.json`.
  `devenv shell -- audit-freeze verify` writes the reference into a scratch folder at the current
  commit and fails unless the tree's reference is byte for byte the same.
- `audit-gate [label]` runs formatting, Clippy, all tests and all captures. It
  clears scenario filters and baseline-update mode, and enforces the baseline.
  Video and optional contact-sheet settings work as for `visual-test`.

Session Read and Write views keep the same left-aligned prose width of at most 75 characters per line.
The Write frame is no wider than the reader measure plus its padding and stays at the left edge. Read and Write sit at its top right edge.
Spare width stays outside it; stored line breaks stay unchanged. Read omits plan links already shown beside the note, including group links and scenes.
A task line under the plan heading is a scene: a row with a done box, whose tick writes the note at once.
When the plan's closed rows do not fit, or an opened row would push the next fight out of view with scenes above it, or, with no fight still to come, an opened row's details do not fit with the rows, the scenes fold to `N scenes`, with a section caret, on the Tonight line, so the next fight stays in view. A scene's words are a plan row title's size at the fights' title column.
`Scene`, the last line of the Add strip's More menu, adds one. With no run open, a creature's or an NPC's
reader, and a scene that links one, has `Start fight`, which saves and starts `Fight with <name>`. A campaign
World entry's reader, and a done scene that links one, has `Add to Log`, which adds one line in the DM's words
to the entry's Log, stamped as a Change is, with `Undo Log line`. A Ready to
run encounter with no saved run has `Remove…` in its menu, with `Undo remove encounter`.
A group editor stays beside the note when both fit at their width limits. Smaller pages use the Note tab.
A paused fight on the plan offers Resume even when it came from a timeline event.
The Add strip stays directly below Tonight, above messages and plan rows. Fight,
Group and Long rest stay visible. When all actions do not fit on one line, More holds
Day, Short rest, Travel and Major event. The menu uses a compact `⋯⌄` button named `More Add actions`; hover text names its actions. The strip stays on one line. Timeline
uses the same width rule. An empty note has one inline Write link. The note file keeps its Markdown and prose. Campaign dependency issues name the entry
that needs repair and keep raw references in Technical details.

The reader keeps ordinary bold and bold italic as emphasis. A bold phrase opens Lookup
only when it resolves to an entry. Explicit Markdown and wiki links still work. Tab reaches each
link in a paragraph in turn, and Enter or Space follows it. Each link, even a two-letter name, has its
own press target at least 24 px square.

A played fight that ended has `Add rewards`, on its row or as the opened row's last link. It writes the fight's
link, an XP line and its loot under `## Loot and rewards` in the session note, with `Undo rewards`. The XP line
follows the newest rewards block in the notes: delete it once to leave XP out from then on.

A Travel row's menu has `Roll for encounters` while the campaign has a marked table. It adds encounter checks
to that journey only: one roll a day (or as the table's `per` word says) on the chosen table, each result a
quoted `Check at <hours>h` line of the event in `Timeline.md`. A result that links an encounter offers to plan it.

Timeline files use `Fight`, `Travel`, `Short rest`, `Long rest` and `Major event`.
The reader also accepts `Encounter` and `Event`, in any case, with or without a colon.
Each save renumbers day headings. Separate edits in Splat and in Obsidian save together;
changes to the same event require a choice. A flagged day names the file and line and
provides `Edit in Obsidian` and `Show file`.

Draft comparisons align unchanged lines. Their highlights and counts
show actual changes. **Next difference** moves between separate change groups and
stays hidden when there is only one group.

Registering a copied package keeps its warning above the name form. **Cancel** closes the form without
writing and keeps the typed name and use choice. Help below **Choose for this campaign** follows its check.
World's unreadable-note count opens the first note's repair and keeps other repair drafts.

Creature, spell and item files use readable tables. The reader shows
each set fact once and hides machine lines. A creature's CR supplies missing XP
and PB; mismatches are flagged unless they describe a lair. Printed creature stats
stay in a folded quote as a record; edit the table above it to change the values.
A creature's `Habitat` row holds its 2024 Monster Manual habitat, such as
`Underdark, Underwater`. Built-in creatures get it from
`scripts/srd_inputs/habitats.yaml`, because the SRD does not print it.

Library's Creatures section, Library search results and the encounter creature
search have **Filter**. It filters by habitat, type, size, alignment, challenge,
movement, senses, languages, resistances, immunities and vulnerabilities. Each
chosen value shows as a chip; click a chip to remove it. Filtered creature lists show the number of matches. Search also finds these
values: `darkvision`, `habitat:forest`, `type:dragon`, `cr:5-10`, `cr:5+`,
`immune:fire` or `lang:"deep speech"`. A creature with habitat `Any` is found
under every habitat.

Unread roster rows show their participant, file line and repair links. They block Start fight, including plan starts. A resumed setup keeps the warning and blocks Begin combat until the row is fixed or removed. Removing a group member offers **Undo remove member**, which restores its count and custom changes without replacing other draft edits. Group names, members and notes keep their form measures in wide panes. When sync is off for every linked participant, the run header says **Sync off for the party** once.

A live initiative change says `acts next round` when its place has passed, and
`acts again this round` when a row that already acted moves below the current turn. Setup counts
rows below the visible list, including missing initiatives and ties between independent rolls. Shared rolls do not produce a tie warning. Session note boxes
keep their edges visible while the text scrolls. Saved runs keep the last change in the
same words as the close dialog.

Run cards show current and maximum HP and one compact **Add by hand** summary.
Live run rows keep their first condition and shorten change bonuses first. Setup
reserves the place column from the start and shows places once every present participant has initiative,
even while the last field has focus. Only the tie column waits for blur. Hidden-condition counts keep
the word **more**, including beside sync state. Recording recovery
keeps its state in the session list; a missing input offers **Choose an input**.
**Process again** reuses the raw transcript and names what it sends in its confirmation.
**Continue processing** also names what it sends. With a complete transcript, it says
that the audio is not sent again. With a partial transcript, it names the remaining
audio parts. Recording silence has no elapsed time in its status word. The Session
page shows the recorded time in the detail; the rail keeps it on hover. On Recordings,
the live row owns the sound state, recorded time and input help.
Session folder is read-only while a run is open. Campaign forms keep sentence-case field labels, their fields,
Browse and save actions on one right edge. The session folder uses its heading
as its visible label and keeps its save action on the field’s right edge. A dirty details form offers Discard and
Undo discard. Missing speech-key links focus that key in Settings. The speech service turns
recordings into text; **Transcript and summary** chooses the service that checks
those words and writes the summary. Custom speech services show their API requirement.
Timeline and the Session plan use the same fight state words. An open fight's Timeline round links
back to the run and does not say **Paused**. Processing says **Processing**, with
the full step on hover. Completed jobs say **Processed** in the rail, and the session list shows the
same check icon and green **Processed**. Warned jobs
keep their warning icon and full words on hover. Note conflicts say **Notes changed**. Long rest starts the next new fight at full from both Session
and Timeline. Every closed long-rest row shows **Everyone at full** or **Not at full**, with an icon, in the
ruling's colour. Both pages let you turn the ruling off or on, with Undo, and the message says what the next
fight starts with.
Both pages name only saved, unfinished fights past Setup as keeping their saved HP.
A saved Setup resumed after the rest starts at full. That fill is not an Undo step: **Use the saved HP** puts
back its saved HP and says **Party HP set to the saved values.**, with **Undo use saved HP** to go back to full.

Library has **New entry** in its header. Its form opens in the reader column and closes after creation. World has **New ▾** for a plain note, a note with a type, an NPC, a location, or a generated NPC. Type headings use capitals and a count. World folder headings keep normal-case paths. Page headers show state or nonzero counts beside the title, with a rule and an `L` gap below. Counts use thousands separators.

A broken or deleted `Package.md` keeps the package's entries available. The manifest
never counts as an entry, even when unreadable; its warning stays on About. Campaign
names the manifest fault and opens an unreadable manifest in Library for repair.
SRD copies no longer warn about hidden citation links. World lists table faults
and offers its repair count in the header. New entries start in their kind folder.
Named NPC search results say **In roster** after Add. An NPC with a missing stat block stays in search, names the missing block and offers **Open in World**. Add stays blocked until repaired.

An unreadable package creature offers **Open in Library** from the encounter roster.
Library keeps a deleted file's draft until you choose **Save my draft again** or
**Remove entry and lose draft**, with **Undo remove**. A moved file's draft saves to its new path. Creature
copies use the same front-matter stats as a new creature and keep the book's prose.
Each stat takes one line, in book order. Library warns on both files when their
identities match. Save changes only the open file. Each repair action names its
file and lists any encounters that will change source. A renamed copy comes first,
or else the newest file does;
**Open in Library** from the roster opens that file. The repair has Undo.
Encounters wait for that repair. A roster edit updates any failed start check at once.
An empty fight **Prep** tab offers **Edit encounter** while the encounter exists.

World checks Part of on disk before every group change, including during a run. If it cannot read the
field, it keeps the file, names the source line and offers **Edit in Obsidian**. Other form fields can
still save without changing the unread group text. **Overwrite with my draft** keeps unread disk groups
and saves the other changed fields. Empty optional links say **not set** in Edit.

Moving a typed World note keeps its stored identity. Plain notes use temporary
path identities; their name links still work after a folder move. Duplicate names
open a chooser, including an NPC's **In** link in World and Lookup. Each choice pairs its file
with **Always use this one**. Until you choose, the note's World row says **Place has 2 matches**
and the header's **notes need fixing** link counts it. An empty World note says **Nothing written yet.**
Its file line keeps the one **Edit in Obsidian** link.

With no campaigns, the switcher says **No campaign** and offers **Add campaign…**.
New entry and Copy entry use **Destination** and show a short path, with the full path
on hover. Sharing required campaign entries lists the copies before **Copy both**.

The `gallery` scenario captures every design token and component, including page headers, blocked reasons, full rows with menus, save rows, discard recovery, the Add strip, paused runs and message actions. Each scroll capture adds a section to the evidence. The `missing-glyph` check rejects painted characters missing from the app fonts. The
`clipped` check also catches text partly covered by a later opaque pane. `src/design/lint.rs` fails on raw
colours, text sizes and raw icon text outside `src/design/`, and on a straight apostrophe in a UI string, and `src/design/tokens.rs` tests WCAG contrast.

Scenario labels are at the top of each scenario in `src/visual_tests/scenarios.rs`, and the
ones they share are in `steps.rs`. Goals use the test IDs in `docs/ux-intent.md` section 8.

## Local layout

Splat reads `splat_schema: 2` and `timeline_version: 2` only. A file with any other
version is an unreadable file. It stays on disk unchanged, and Splat names its path
and the reason. The reader never shows front matter as prose.

The existing binary also has a command that opens no window:

```sh
devenv shell -- cargo build --locked --bin splat
devenv shell -- ./target/debug/splat check /path/to/folder
```

The folder can be a vault, campaign or package. `check` uses the schema-2 readers,
prints flagged or unreadable paths with their line and reason, and prints counts.
Exit status is 1 if any file is flagged or unreadable. Invalid command arguments or
a folder that cannot be opened return status 2.

Creatures use a `Stat block` section with `Stat | Value` and `Ability | Score | Mod | Save`
tables. Spells and items use a `Field | Value` table in `Spell` or `Item`.
These tables supply the stats in schema 2. A bad cell stays in the file and gets a
line flag; its value is not set. Printed modifiers and passive initiative are checked.
Rejected vital text stays in its reader row with a warning.
Stat edits keep formulas, spacing, and other text. New entries use this layout.
Printed stat lines stay unchanged inside a folded quote callout and are not read as
stats. Table shape hints preserve compound identities and text ranges; explicit spell
ritual and concentration rows preserve both true and false values.
Timeline uses event lists. Campaign packages use task items. Groups and encounter
rosters use tables. Character copies use their character's name once,
keep their identity after a rename, and put imported data in a foldable end section.
A provider refresh writes again a character copy that Splat cannot read, when its generated
text still matches its checksum, and keeps the personal notes. With a change in that text, the
refresh stops and keeps the file, and Replace archives it first.

Session plans accept task items such as `- [ ] [[Bridge Ambush]]`. Empty session
titles have no property. Obsidian's core Templates plugin also fills `{{date}}`
and `{{title}}`. For run JSON files to sync through Obsidian Sync, enable
**other file types**.

With Splat open on two computers that share a vault, one computer at a time saves the
D&D Beyond sheets from automatic reads. If both saved a sheet at once, Obsidian Sync
would merge the two saves and repeat their lines. `Characters/Sheet writer.md` names
that computer. A new claim waits one minute before it saves, so that a claim from the
other computer can arrive through Obsidian Sync first. Splat renews the note while it
runs, and another computer takes over about 10 minutes after it stops, if the two clocks
agree. A computer whose saves fail gives up its turn. The other computers read the
sheets and show **Sheets save on <computer>** on Party. **Refresh party** and force
reads save on any computer. Splat never replaces a note at that path that is not its
own; it warns on Party and saves nothing automatically until the note is moved. The
computer's identity is in Splat's config folder, outside the vault.

Code lives in `~/Projects/splat`. The vault link is
`~/Documents/vault/D20 D&D/00-09 System/09 Tools/splat`.
Campaign data remains in the vault. A read-only content verifier and the complete
[built-in SRD 5.2.1 source package](docs/srd-import.md) are available; campaign
package selection and local entry editing are available in **Campaign**. The
[execution proof](docs/execution-proof.md) runs converted SRD mechanics from Markdown.

The link is listed in Obsidian's excluded files. This reduces visibility in
search and related views but is not a complete indexing or filesystem exclusion.
Build artifacts stay in the code repository's ignored `target/` directory.

Keep D&D Beyond session cookies and tokens outside Git. Authentication is available
in Settings; character sheets with combat sync are available in **Party**. Characters now have persistent local copies in the shared creature schema.
The encounter builder uses these local character copies. Manual DM runs are available.
Encounter and group searches also find NPCs with a stat block. Add keeps the NPC's
name through the existing encounter override. Shared NPC initiative rolls stay quiet;
only equal totals from different rolls need a tie ruling. Setup shows each participant’s
place beside initiative and names who goes first. A tied arrival offers **Move before**,
with Undo. Change fields show the printed value and result. Changing the run folder
warns about saved runs and offers **Move runs and save** or **Save without moving**.
Close the open run before saving run settings.

The run panel groups participant tabs and fight tabs. `Correct` changes a participant's
values. A blocked Undo shows **Nothing to undo**. A run file from before Splat saved Undo steps shows
**Nothing to undo since reopening**.
`Add to fight` opens the Arrivals panel for a creature or a prepared group; a blank initiative total rolls it.
Search shows five creatures; **Show all** lists every match without more typing.
Group Add buttons share a fixed slot, so all Initiative fields align.
`Ctrl+G` opens the same panel in one step.
Unsaved runs offer **Save as a new run and close** in the close dialog.
Automatic combat resolution and simulation integration remain later work.

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

This is an internal D&D Beyond endpoint, not a supported public API. The Party viewer fetches and caches character sheets. Campaign details are fetched into memory, with linked names cached in the campaign manifest. Authentication tests use a local mock server;
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

After connecting, expand **Add from D&D Beyond** and select a campaign. Its
name and ID fill the form under that row. Then choose its data folder with **Browse…**
or enter a path, and select **Add campaign**. No local campaign is required first.
The Folder help explains that Splat keeps notes, encounters and runs there; a folder in an Obsidian vault works. Add and edit forms keep Cancel at the left and the commit action at the right.
You can also choose an existing folder and select **Add campaign** without connecting. Campaigns stay on the left of Settings, with the account on the right. On a narrow window, campaigns come first unless a run is open; then the account comes first. **Connect…** opens the account form. The Campaign page also has the add form when no campaign exists. Removal is in each row’s **More actions** menu.

Splat creates or discovers the campaign manifest in that folder. It stores the
name and D&D Beyond link there, alongside existing custom paths and content
settings. Opening a folder preserves its UUID, settings, and notes. Existing
registrations load their manifest settings on startup or **Reload**. App settings
retain the folder registration and cached metadata. Credentials stay in the keyring.
The remote link uses `splat_dndbeyond_campaign_id` and
`splat_dndbeyond_campaign_name`; character cache storage is unchanged.

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
name is cached as `splat_dndbeyond_campaign_name` and shown while offline or refreshing.
Failed requests retain the cache. Replacing or removing a link clears its name;
late responses cannot apply an old name to a new link.

## Sessions and the timeline

Timeline saves keep unknown properties and event fields. Comments outside the event list stay;
comments inside the rewritten list are not guaranteed. If the body changed outside Splat, saving
opens the draft beside the current file at the first difference, with disk-conflict choices,
before it can replace those words. A repaired Timeline file is read
again, and the open fight's latest result is applied.

Session storage remembers each known note path in `.splat-session-paths.json`, and writes it only when a
path changes, so an idle campaign's files keep their times. A copied session
shows both file names and offers **Make <file> a separate entry**, with **Undo separate entry**. A moved draft can save to its new path
with the same session ID. If a session or run folder is gone, choose its folder before making new
files. Recording results stay listed as **Audio removed** when their WAV is gone.

Encounter, group and creature drafts use the disk comparison when a file changes. A move follows
the entry's ID. Multi-file saves stage all files before replacement and report any rollback failure.
Splat uses an advisory lock to keep a second window from restoring and saving the same open run.

On a wide page, the plan takes 40% of the available width, between 330 and 480 px.
The recording count is part of the Recordings tab. Timeline puts its save state,
Copy link and menu on the title line when they fit, and below it when they do not.

**Session** is the page for tonight. Beside the note, a plan lists what has happened in this session, then the fights
still to come. **Fight** adds a saved encounter to the plan, or starts a new one, as a line such as `- [[Bridge Ambush]]`
under `Plan for tonight` in the note, so the file holds the plan. **Start fight** goes straight to setup, and **Close run**
comes back to the page the run began on. A row opens in place: a fight shows its final HP and log, and any event has its
title, length, place and notes. A paused fight on the plan offers Resume even when it came from a timeline event.
The Add strip stays directly below Tonight, above messages and plan rows. Fight,
Group and Long rest stay visible. When all actions do not fit on one line, More holds
Day, Short rest, Travel and Major event. The menu uses a compact `⋯⌄` button named `More Add actions`; hover text names its actions. The strip stays on one line. Timeline
uses the same width rule. At the top, **Last time** links to the previous session and shows the first line of its `What happened`,
else of its processed recording's recap, else its first written line. While the session has no event and no run,
**Carry over** at the end of that line adds the last session's recap as a link, its plan lines that were not
played and its open threads to this note, in one save with `carried_from` and `carried_parts`, and `Undo carry
over` puts the note back exactly. Read shows such an embed as the recap's words under `From <session>`. The title and date share one line. A **Playing** status marks the session being played.
**Return to the session being played** opens that note from another session. If both sessions have the same
number, the warning names both note files.
In a narrow window, the plan and the note have a tab each. **Read** and **Write** turn the note between its two forms, and
Read leaves out headings with nothing under them and shows note links in place. **Templates** opens from the page header; **← Sessions** returns to the notes.

A fight resumed under a later session also counts as part of that session. During a session, a new run starts each player from where the fights so far left them, and setup says so. A player who
missed the last fight stands where an earlier one left them. If a fight's run cannot be read, setup says that too. **Use the sheets'
HP** goes back to the sheets, as one step of undo. Adding **Long rest** records your ruling that the next fight
starts everyone at full HP. One **Undo** removes the rest and that ruling. Open the rest and choose **Mark not at full** to keep the rest without the ruling. Party shows the same HP, and says where it came from. A starting HP written in the encounter
wins over both. With sync on, a first read still takes the sheet's value for HP from a fight. Full HP from a rest is your ruling, so a sheet that differs is a conflict to settle. Conditions do
not carry over.

**Session** holds session notes and session templates, as Markdown in the campaign's session folder (`Sessions`
by default; change it under **Campaign → Link and folder**). A folder change counts what stays behind and offers **Move files and save** or **Save without moving**. Moving keeps note text, templates, events, audio and processing files, and refuses to replace an existing file. **Timeline** holds every encounter, rest, travel interval, day and other
major event in story order, in `Timeline.md` in the same folder. It is also the campaign's one clock, in hours and minutes, counted
from the first **Day**. A **Day** moves to the start of the next day. A rest adds 8 hours or 1 hour unless you say otherwise,
and any event adds its length when it reads as a number and a unit, such as `3 days` or `an hour`. A length it cannot read
moves nothing, and the row says so. A fight does not move the clock. The list in its frontmatter is the record, and Splat
writes the body from it. An encounter run adds its own event when combat begins. New events and runs go to the newest
session unless you choose another on the Timeline page. **Notes** (Ctrl+J) opens that session's note beside any page.

Type `/` at the start of a line or a word in a session note to open its commands. Type more letters to narrow them, and
use the arrows and Enter, or click. With only the `/` typed, Enter still starts a new line. **Generate NPC** rolls an NPC
as in Lookup, and **Save NPC** (or Enter) writes it to `NPCs/`. **Search** finds an entry, a world note or a heading, as
Lookup does. Each puts a link such as `[[Verori]]` where the `/` was. Escape in the list closes it and keeps what you
typed. Escape after you choose a command goes back to the note without a link. A slash inside a word, as in `and/or`,
stays a slash.

A planned fight with a saved run offers **Resume**. A legacy run uses the same saved-name matching rule as Encounters, and only one planned fight can have that name. **Resume** is the primary action for the first waiting fight when no run is open. A run before combat says **Setup**; a closed unfinished run adds **Paused**. Its menu holds **Start a new run** and **Delete the saved run**, with undo for deletion. Every planned fight also has **Remove from tonight**: it removes one link from the plan and offers **Undo remove** below the row. The open fight has a status link in the plan; the run strip keeps **Back to run**. A played fight shows its result and shared run state once, such as **Held the bridge · Ended · Round 1**. It opens a grid of final party HP with first names, then **Log**, **Whole run** and **Timeline** links. Plan summaries use the width below the row action. Removing a planned group keeps its message and Undo in that row’s place.

Session and Timeline use the same quiet **Add** strip: **Fight**, **Group** (plan only), **Day**, **Long rest**, **Short rest**, **Travel**, **Major event**. A new plan event stays closed and uses its kind as its title. Open its row for **Title** and **Notes**, or follow **Timeline** for **How long** and **Where**. A new event on Timeline opens with **Title** focused. On Timeline, **Adds to** chooses the session. The detail column opens only when an event is selected. Its title, save state, **Copy link** and row menu stay above the fields as they scroll. **Place after** sets the event order. Opening Timeline or changing a filter resets the detail fields to the top. A Day shows its start time only.

The note conflict view opens at the first line that differs, with a line count. **Next difference** appears when more than one line differs. **Record** remains available during a note conflict; recording writes its own file. **Reload and lose my draft** is red and separate from **Overwrite with my draft**. Starting a session during a fight offers **Move it here** to move its timeline event in one step. A message confirms the move and offers **Undo move** until the next note or plan edit.

Read view leaves out a plan section that only repeats the fights in the adjacent plan.
An otherwise empty note says **Nothing written yet.** and offers **Write**.
When the plan has its own tab, Read keeps the fight count.

Each event has a reference such as `[[Timeline#^t-1a2b3c4d5e|Rest at the mill]]`. Paste it in any note: it is a block
link in Obsidian, and Splat lists the notes that link to an event. Renaming an event offers **Update links in notes** below its Title field. The update has one Undo in the Timeline list. That Undo clears at the next note or event edit or when you leave Timeline.

In the Reader, bold words such as `**Docket**` are links to Lookup. A unique matching name opens
directly. Otherwise, Lookup shows a search for those words. Bold text inside an explicit link
keeps that link's destination.

### Recording

**Record** on a session uses the host's recorder (`pw-record`, then `parecord`, then `arecord`) to write 16 kHz mono WAV
under `Sessions/Recordings/<session id>/`. Tell the table first. The recorder is a child process that stops if Splat
dies, and the file stays if it does. When the recorder stops by itself with an error Splat knows, the row says it
in plain words (for example, the microphone is no longer connected); hover shows what the recorder said.

### Processing a recording

Nothing leaves the computer until you select **Process** on a recording. Choose the services in **Settings**.
The first recording that needs processing owns the primary **Process** or **Continue processing** action
when no run is open and no enabled recording recovery owns primary. A stopped recording first offers primary
**Choose an input** when its input is missing, otherwise **Record again**. Processing stays secondary during
recovery. The destination text shows below the first available Process action; later actions keep it on hover.
Open runs keep these actions secondary. Session rows with equal titles and dates show their file names.
Recording state replaces the date, and **Playing** stays beside it when it fits.

1. A **speech service** writes `<recording>.transcript.md`. The default is OpenAI's `gpt-transcribe`, and any service
   with the OpenAI `audio/transcriptions` API works. It needs a key, kept in the desktop keyring. Audio goes up in parts
   of at most 10 minutes (the limit is 25 MB), each cut at the quietest moment nearby, and each with the end of the last
   part as its `prompt`. **Continue processing** resumes at the next part after a failure.
2. An **LLM** goes over the transcript twice. The default is `gpt-6.1-sol` through the `codex` command, with your Codex
   sign-in, in a read-only sandbox. Anthropic and any OpenAI-compatible API are options. The Codex CLI takes no audio, so
   step 1 cannot use it.
   - The first pass cleans the raw words and marks events, in `<recording>.cleaned.md`.
   - The second pass checks that against the raw transcript and corrects it. It writes `<recording>.marked.md`, the result,
     `<recording>.review.md`, what it changed and could not verify, and `<recording>.summary.md`.

   Splat then checks the result itself: every time heading must be there, and it must not be far shorter than the raw
   words. The raw transcript never changes. A failed second pass keeps the first, and only the second runs again.

The default instructions for each pass, which you can edit in Settings, ask for one marker before each event:

```
<!-- splat:event kind="long_rest" at="01:12:00" title="Rest at the mill" place="Old mill" duration="8 hours" -->
```

A marker is a suggestion. The recording's row lists them, and only what you add goes on the timeline.
**Add** and **On the timeline** share one column beside the marker text.

## World notes, NPCs and roll tables

**World** lists the Markdown notes in the campaign's folder, with no change to the files. Obsidian stays their editor,
and Splat picks up changes when you return to it. A note is any Markdown file with no `splat_` field, except session
notes, `Timeline.md`, `Trackers.md`, session templates, recordings, and anything in a folder whose name starts with a dot. **Lookup**
(Ctrl+K) searches these notes as well: names first, then each heading, then their words, then session notes and
transcripts. Links such as `[[Verori]]`, `[[Gate Mechanics#Reliability]]` and `![[Map.png]]` work as in Obsidian. A link
to two notes asks which one, and a link to nothing offers to create it. Each note lists what links to it.

Every World note offers **Edit in Obsidian** and **Show file**. A file that cannot be read stays in the
**Cannot be read** group, with the cause below its filename. Open it to repair its text here, or follow **Edit in Obsidian**. Lookup's warning
also opens the file in World. A repair draft stays in memory when you read another note. If its file changes, choose **Overwrite with my draft** or red **Reload and lose my draft**. Reload keeps the old draft for **Undo reload** until the next edit.

Open and answered questions show empty and checked boxes in the reader. The note keeps its Markdown task marks (`[ ]` and `[x]`).

### Table marks

A one-line marker above a table makes it rollable. The first column is a number or a range, such as `2–5`, and the rows
must cover what the dice can roll. A bad table says so, and the rest of the note still reads.

```markdown
<!-- splat:rolls key=gifts dice=1d6 -->

| d6 | The crowd throws |
| --- | --- |
| 1-3 | A knife |
| 4-6 | A potion |
```

**Roll** and a box for a typed result, for a player's own dice, sit under the table. During a run the result goes in the
run's log, and Undo takes it back. Otherwise it only shows.

### House rules and trackers

A **house rule note** counts a house rule's numbers: a race clock, a crowd's favour, seals for each team. It is a
`rule` note with `implementation: splat:house_rules` and its trackers in a `splat:yaml path=splat_members` block;
[the Tournament example](docs/design/world-examples/heralds-tournament.md) shows every field. Turn it on under
**Campaign → Rules and runs**, in **House rules in play**, and save; that writes `splat_in_play` in `Campaign.md`.
A note that is not in play shows nothing anywhere. A note in play that is gone keeps its row there, with its ID and
`not found`, so that you can switch it off.

**Ctrl+T** shows **Trackers** in the Lookup panel, beside any page, and Ctrl+T again goes back to what Lookup
showed. Each tracker has its value and its quick actions; a tracker with `per` has a line for each name. One press
is one act. A value that rises past a mark raises a notice in the **Due** list, with a button for each table it
rolls (with `for:`, one roll for each name), its proposed changes ticked, and **Done**, which applies the ticked
changes, or **Dismiss** in its menu. While a notice is due, the rail says `Due N` on every page. Splat counts and
shows; it never applies a rule by itself.

A **day mark** (`day_marks` in the same block) raises a notice on the campaign's day count, which the Timeline
works out: `at: 721` at the start of Day 721, or `every: 30, from: 2` on Day 32, Day 62 and so on (with no `from`,
the count starts on Day 1). Its proposed changes name their tracker, and `for:` names the line of a tracker with
`per`. Raising a day notice writes nothing. Days of one mark that are due together, after a long journey or while
Splat was closed, are one row (`End of month ×3`) with **Next**, which ends the oldest day, and **Dismiss all** in
its menu. When the clock goes back, for example by the Timeline's Undo, a day no longer reached leaves the Due
list; a day already done stays done. While `Timeline.md` cannot be read, or it changed under a Timeline edit that is
not saved yet, no day notice shows, the Due list says so, and no act on one is recorded; your edit is kept.

Every act is one entry in `Trackers.md` in the campaign's root folder, written at once. The result line names the
last act of the session being played, with **Undo: <label>**, which steps back newest first, also after a
restart, and **Add to Timeline**, which makes a Major event with the act's label. A wrong press seen late is
corrected by the opposite quick action. An act reads the file and the house rule notes as they are on disk when you press, so an entry you edit
in Obsidian counts at once and is never written over; an act on a definition or an entry that changed on disk writes
nothing and says so. A file that cannot be read (`Trackers.md`, `Campaign.md` or a house rule note in play) blocks every act and
shows `Trackers not read`. A save that fails records nothing and keeps nothing: the bottom of Trackers says
`Trackers.md was not saved` with the reason, and you press again. Entries for a tracker that no note in play has are kept and not
counted; the end of Trackers names them, and so it names a value that an edit forced below its `min`. Each
tracker's history is in its house rule note's reader (open the note in Lookup): fold open `<NAME> HISTORY` to see
each entry and the value it left. To take back an entry of an older session, use **Remove** in that row's menu: it
takes the entry, the others of its press and the entries of any notice it raised, and **Undo remove** puts them
back where they were. Remove refuses when a later notice's crossing depends on the entry, and names that notice's
entry to remove first. In a fight the run shows no tracker part: use Ctrl+T, press, and
go back.

### Loot

The encounter editor keeps roster actions, loot and notes within the list width, with **Add participants** directly beside them.
An encounter's **Loot** list, under its roster, holds what the party finds when the fight is over. Type in **Add loot**
and press Enter to add the words as a line, such as `30 gp`. While you type, the field offers the campaign's roll
tables and Library items; the arrow keys and Enter, or a click, choose one. **Roll on a table** lists every table.
A roll line has a count and its table, and a key that no table has says **Cannot roll** before play. Each line's
**⋯** menu has **Remove**, with Undo. The note keeps the list as `## Loot`, which Obsidian shows too:

```markdown
## Loot

- 30 gp
- [[Potion of Healing]]
- Roll 2× gems
```

One creature can carry loot of its own. Open **Changes** on its roster row or group member, and use the same list
under **Loot**. The table's Changes cell keeps it last, so its words can hold commas:
`HP +5, Loot Iron key; Roll goblin-pockets`. A line there cannot hold `;`. The row's Changes link says **Loot**, and
names any table key that cannot roll. Each creature that an encounter group adds carries its member's loot.

In a run, a creature's card shows its loot in one line. When the creature drops to 0 HP, its row says **Loot** and its
card lists the loot with a **Roll** button for each roll. When the fight ends, the ended view lists the encounter's loot
and the loot of each creature at 0 HP, with how many rolls are left. **Roll all** makes every roll as one step.
A result takes the place of its button, the saved run keeps it, and Undo takes it back. Rolls still work after the
end, because that is when the party searches. **Copy loot** copies the list, with the results, as Markdown for the
session note. Loot on creatures still standing is under **more creatures carry loot**.

**New NPC** and **New location** in Lookup create a note from the typed words, in `NPCs/` or `Locations/`. The name keeps
the letters you typed. When you typed no capital, only its first letter becomes one: `toll at the old mill` gives
`Toll at the old mill`, and `McTavish` stays `McTavish`. The note's saved name and its file name keep the same letters.
**Make world
entry** gives an existing note an identity, and its text does not change. An entry has a `type` (`npc`, `location`, or a
word of your own), and can have `aliases`, `status`, `in` (the place it is in) and `creature` (a stat block):

```yaml
splat_kind: world
splat_world:
  type: npc
  aliases: [Champion of the Red Seat]
  status: alive
  creature: {key: dnd2024:creatures/goblin_boss}
```

Set World values have a **Change** picker. Read mode hides empty values, except an NPC's **Stat block** row: its **Change** action stays available to assign the first stat block in three interactions.
Choose by name, or create a location from that picker. Each change edits only that key and offers **Undo**.
The rest of the file, including comments and spacing, stays as it was. `in: "[[Ashfall Keep]]"` also works,
including a location's aliases. A place loop stays in the list and shows a warning.
The **Edit** action beside a campaign entry's name, in World or Lookup, opens one form with **Status**, **Stat block**,
**In**, **Part of** and the entry's text, without its title line. This form includes empty fields. If **Part of** could not be read, its **Change** action is blocked
with a message that quotes the value it could not read, gives its line, and says to write each group as a link, such as `[[Arena Guild]]`. Other fields stay editable, and saving them keeps the stored group text unchanged. **Save changes** (Ctrl+S) writes them together and offers
**Undo changes**. During a live fight, **Save changes** uses a secondary border so **Next turn** stays the primary action.
Results and their Undo, such as **Stat block set to Guard.**, sit at the bottom of the World or Lookup pane, so the note
you read does not move when a result comes or goes.
The form keeps its state and quiet **Discard** at the left, with **Save changes** at the right edge of both text fields.
**Discard** writes nothing and offers **Undo discard**. The rail shows **World draft** for an unsaved edit,
or **World repair draft** once if a repair is also unsaved. If the note changed on disk, Splat writes
nothing until you choose. **Overwrite with my draft** saves only the values you changed over the disk version.
For **Part of**, it adds and removes only the groups you changed. A group that Obsidian spelled another way,
such as `[[Factions/Arena Guild|Guild]]` for `[[Arena Guild]]`, is still the same group. If the note already
holds every change, Splat writes nothing, offers no Undo, and says **Your changes are already in the note.**
**Reload and lose my draft** loads the disk version and offers **Undo reload**. A form with changes stays while you
read other entries, and Splat asks before it closes or changes campaign.
A campaign creature in Library offers **Make NPC note**, which opens its new NPC note in World.
**New ▾ → Note…** creates a file in `Notes/`; its type is optional. The note opens with **Edit in Obsidian**.

**Add to fight** on an NPC puts them in the open fight under their own name, on their stat block. It does not change the
encounter, and Undo removes them. A location lists what is in it, and World shows locations as a tree.

**Generate NPC** in Lookup rolls a name and three attributes: look, manner and want. **Reroll** rolls them all again,
and nothing is written until **Save NPC**, which creates the note and opens it. The card says where: the folder that
holds most of the campaign's NPCs, or `NPCs/` when there are none. **Discard** lets it go.
The card names the campaign tables it uses, or says that it uses built-in lists. Without a names table, Lookup says
**Built-in names.** and offers **Use your own names**, which opens the same NPC on the World page. The World page's card
says to add a table of names to a note and paste the marker on the line above the table. **Copy the marker** copies
the marker line for you; then change `1d6` in it to the number of names.

On the World page, an unsaved NPC stays until you save it or choose **Discard**, and **Undo discard** brings it back.
While you look at other notes, Things to check or a repair, one row above them says **Unsaved NPC: <name> · Show**.
The rail says **Unsaved NPC** on every page; select it to see the NPC's card on World.

The built-in lists suit no world in particular, so a campaign can replace any of the four. Mark a table with the key
`npc-name`, `npc-look`, `npc-manner` or `npc-want` in any world note. The first table in path order wins, session notes
don't count, and a part with no table uses the built-in list. A bad table says so beside the result, and the list stands in:

```markdown
<!-- splat:rolls key=npc-name dice=1d4 -->

| d4 | Name |
| --- | --- |
| 1 | Tally |
| 2 | Blot |
| 3-4 | Rook |
```

The design and what is still only proposed are in [World and house rules](docs/design/world-and-house-rules.md).

## Library workspace

Select **Library** to work on reusable packages without changing the active campaign.
The built-in D&D 2024 SRD, your packages, and the active campaign's own entries are one
catalogue. It opens on what this campaign can use, and the menu at the top shows everything,
one package, or **Campaign entries**. The built-in SRD is read-only. Browse Creatures,
Spells, Items, Character options, Rules, Glossary, and Encounters in the section
sidebar. Sections use subject metadata as well as entry kinds, so spell references
appear with spells. Other entries includes package notes and invalid notes for repair.
Search covers all sections by name, path, or section; clear it to return to the
selected section. Entries share stat cards, headings, and compact detail rows.
Structured details and source identifiers remain expandable. **What Splat applies**
sits inside **Source and identifiers**, separate from structural validity. Unreadable
files omit these sections. Markdown source and local editing remain available in a
fixed frame; its text scrolls inside the frame.

The built-in reader reflows printed PDF lines into paragraphs and keeps page
references under **Source and identifiers**. Raw definitions stay under collapsed
**Technical details**. Classes show separate traits, equipment choices, and
feature links in level order. Display changes do not alter saved content.

A D&D Beyond definition goes to the enabled package note that lists its ID in
`splat_dndbeyond_ids`. Other definitions go to the campaign's **Campaign Package** folder. A
definition keeps its place until you approve a move: **Campaign → Packages** lists what can move
when packages are added or removed, suggests notes with the same words, and adds the ID when you
apply a change. The sidebar shows how many changes wait. See
[Imported character content](docs/design/character-content.md).

Under **Packages → Manage packages…**, register an existing folder containing exactly one package
manifest, or enter a name to create a package. The form shows the new folder path. **Browse…** changes its parent folder. After creation, the first entry form opens. Package
registrations are stored in user settings; package notes remain in their own folders.
New packages write their manifest as `Package.md`. Existing `Module.md` files keep their name;
repair messages for existing files and Undo actions name the file. A deleted manifest uses
**The package manifest was deleted** and **Write the manifest again**, because its old file name is not stored.
Its title uses the registered package name. The owner line says **Manifest deleted** and offers
**Show folder**. Plain prose without front matter or `splat_` lines in `Package.md` or `Module.md`
stays a README. Writing a missing manifest uses the first free name, `Package.md` then `Module.md`.
It stops if both names hold notes. The result names the file written. README bytes stay unchanged.
A copied folder offers **Register as a new package** beside its warning. **Add folder**
stays blocked until you choose another folder.
Unregistering a package leaves its files in place. Campaign keeps its last known name and offers **Remove from this campaign** or **Find the folder…** if it becomes unavailable. Package IDs stay under **Technical details**. Package and campaign folders must
be separate. Register My Homebrew in your preferred location for shared homebrew.

Local packages support new entries and Markdown editing. **About <package name>** opens the manifest and selects its About row. **New entry** shows the folder, kind and path before it writes a note, then opens its editor. Kind templates include a reader skeleton. A new subfolder is created when the note is saved.

**Choose for this campaign** starts on for new entries, new packages and copies to packages. It saves the package choice too, so the content can be found in the encounter editor. The reader says **Chosen for this campaign** or **Not chosen**, and offers the same action in its toolbar when it is not.
**Choose every entry** shows its result and **Undo package choice** in the reader's result place. **Save** checks note
structure, keeps existing UUIDs and kinds, rejects duplicate identities, and checks
for external edits before replacing a file. The header validation count opens messages for unresolved
references; a package-only check can report references to other packages as unresolved.
Invalid entry notes remain available for repair. Edit Markdown scrolls to the error line and puts the caret there.
The owner line says **Cannot be read** once. The error below the title offers **Fix in Edit Markdown**.
The repair error sits directly below Save, above the source. Manifest repair opens the package's warning
About row and shows a bordered replacement action with its consequence below it. Empty or malformed
manifest files stay in repair. A deleted manifest keeps its missing-file warning and repair action. Saving a repair restores the campaign-use state at once. If the file changed on disk, the save stops
and shows disk and draft side by side, with marked lines and **Next difference**. Choose **Overwrite with my draft** or **Reload and lose my draft**.
**Discard** takes one press and offers **Undo discard** until the next edit or disk change. If the file was deleted, **Overwrite with my draft** creates it again. Changing
pages keeps the draft in memory.
Save before closing the app; drafts are not persisted.

Every reader accepts Obsidian links: `[[Name]]`, `[[Name#Heading]]` and `[[Name|text]]`. Names resolve to World notes and content this campaign can use. **Alt+Left** returns through Library history. Duplicate names open a chooser. A missing name has a dotted underline and **Not found in this campaign** on hover. Existing reference keys still work. A package entry that refers to campaign-only content warns before sharing. The copy
preview names each additional entry and explains which source links to it.

Copied notes show **Same entry in two files**. In Library, World, Encounters and Sessions,
**Make <file> a separate entry** keeps both files and offers **Undo separate entry**.

In **Campaign → Packages**, enable a whole package or select individual
entries. Required dependencies are included separately with their reasons, and rule
groups include all members. Whole-package selections include future additions;
individual selections do not. When all current entries are selected, **Include future additions** switches to the whole package. **Save selections** writes `splat_packages` into the
campaign manifest while preserving unrelated fields, comments, and note text. A new campaign gets `Campaign.md` with its UUID and the default SRD selection. If that file is deleted, **Write Campaign.md again** restores the held text only when you select it. **Undo write Campaign.md** removes that restored file if it has no later edits. Missing packages and unresolved references remain visible.
A package that other entries need but that is not registered shows once, as **Package not
found**. The issue uses the package's name when its folder, with a `Package.md`, is beside a
registered package, and its namespace otherwise. An entry missing from a loaded package also
shows once, with the entries that need it.

Use **Copy to…** on a library or campaign entry to make an independent copy in a
registered package or campaign. Review the destination, name, retained dependencies,
and Markdown before **Create copy**. Copies get a new UUID, lose the original resource
key, and retain source attribution and optional origin identity. Existing references
are not rewritten. When an entry needs a campaign entry, **Copy both** copies both into the package and reconnects the new references. The app checks source, dependency,
and destination changes before writing. Saving can create new subfolders.

Clean destinations refresh after copying, and an open encounter draft sees the copy at once. **Campaign → Campaign entries** provides a
search, the shared reader and a Markdown editor for local entries, plus copying back to packages. Pending
campaign edits survive workspace switches and block sidebar campaign changes until
saved or discarded. Invalid campaign notes have a raw repair editor. Repair them before
saving campaign selections. Raw notes can also be edited externally; reload after external changes.

Moving entries between owners, automatic link repair, and running rules from the reader
remain outside this UI. New entries are drafts, not executable rules. D&D Beyond links
are in Settings and in **Campaign → Link and folder**.

## Character sheets

Open **Party** to read the linked D&D Beyond party. Select a character
and browse Overview, Actions, Spells, Equipment, and Features. **Refresh party**
gets the latest sheets. When signed out, **Connect to refresh** in the page header opens Settings with the cookie field focused. An empty Party uses one **Connect D&D Beyond** action instead. Saves and skills use two or three columns. Skills show **Proficient**, **Expertise** or **Half** in words. A sheet list gets a filter only when it has more than eight entries. Cantrips use one-line rows, and coins appear below the Equipment heading. **Bloodied** uses the same heart icon and colour as the run. **Technical details** keeps source and rule checks out of the main sheet. Computed totals come directly from D&D Beyond. Local
creature records remain available offline, and open sheets use the event stream
with polling fallback. No character changes are sent to D&D Beyond. See [viewer details and streaming findings](docs/character-viewer.md).

If you edit a generated campaign note, Splat keeps your edit and stops updating that character's notes. To
get updates again, use **Show file** to inspect it or choose **Replace with D&D Beyond sheet** on the warning. Splat reads that character again and replaces its edited notes. Your version stays beside the note in a `.splat-archive` file. The row says **Local edits** until replacement. The result names the character and has **Show file** for the kept version and **Dismiss**. When signed out, the warning has **Connect D&D Beyond**, which takes you to the connection. While that warning shows, the header shows only **Not connected** and when the sheets were fetched; it offers **Connect to refresh** again when no warning has its own Connect. Personal notes carry over when Splat can still find them.

## Encounter builder

Open **Encounters**, select **New encounter**, and type a name.
Use **Add party**, or search the campaign's available creatures. Each result shows
the CR, HP and AC it has. A source appears only when it differs from the main source, and campaign-owned creatures say **Campaign** in results and the roster. Select its name to open its stat block in Lookup, beside the draft.
Add a creature for each participant and assign its side. The row’s **⋯** menu has
**Move up**, **Move down**, **Duplicate**, **Make it yours**, and **Remove**. Optional starting HP and temporary HP apply to that
participant only. With a value off, the creature's starting state applies.
The side menu offers **Party**, **Enemies**, and **Other…** for a side of your own.
The default sides, Party and Enemies, are hostile to each other. The sync and
repeat options sit below the roster. **Party in roster** means all party members are present.
You can edit or create an encounter while a run is open. Close the run before you start another fight.
An encounter with a paused run offers **Resume** in its editor and shows the paused round beside the save state. **Start another fight** stays in its menu. An empty roster cannot start.

A roster slot can also build in changes to its creature: a display name, maximum HP,
AC, initiative bonus, attack bonus, damage bonus, speed, starting conditions, and
a note. The slot's row sets its side, and a note can also override it. Open them with the slot's **Edit** button. Each one appears under the slot with the
value it replaces, and nothing is written back to the creature, a package or a campaign
copy. An invalid change is refused with a message that names the slot. See
[Overrides](docs/design/campaign-schema.md#overrides).

**Save encounter** writes a Markdown template under `Encounters/` in the campaign
folder. Select a saved encounter to reopen it. Names can change without changing
file or participant identities. A new campaign gets a manifest on first save.
Existing notes and unexposed encounter data are retained. External file changes
block saving; discard the draft and reload to read them. Unsaved encounters
survive page changes and content refreshes, and block campaign switching, with a warning on app close.

Party entries are local character copies from the linked D&D Beyond campaign;
refresh **Party**, then **Reload** on Encounters to see new copies. This first
editor saves direct creature references in campaign-local templates. Shared
`party_slot` bindings still need the Markdown editor. Phases, assumptions,
completion rules, and advanced starting state also remain Markdown fields.
Saving a template does not execute rules or start a combat run.

### Encounter groups

A session's plan can assemble a group in place: choose **Group**, then **New group**. The editor uses
the full column. The note stays behind its **Note** tab. The saved group is linked in the plan. **Edit** on its row opens it again. While a fight is open, the row has **+ Add to fight…**, the run's words, which goes to the Arrivals
panel in the run. Its **⋯ → Remove from tonight** deletes an isolated plan link and keeps words inside a sentence. **Undo remove** restores the link.

An encounter group is a reusable, named set of creatures that you prepare once and add to a
running encounter when it arrives. It is a note with `splat_kind: encounter_group`, saved under
`Encounter Groups/` in the campaign folder, with its members and counts, a default side, and your
notes as its text. **New group** on Encounters opens its editor, which searches creatures as the
encounter editor does. Any creature from this campaign or a package it uses can be a member, and
each member can carry the same changes as a roster slot. A one-line **Changed** summary opens those changes. An empty group or an unavailable
creature is refused with a message.

**Belongs to** chooses who the group is for. With **Any encounter**, a run of any encounter offers
it. With one encounter, only runs of that encounter offer it, and the Encounters page keeps it in a
closed **Groups** section. Open it to see one heading for each encounter, with groups for any encounter first and groups whose encounter was deleted last. Saved group rows have **Edit** and **⋯ → Remove…**, with **Undo remove group** after removal. A partial group names only the members that will join beside **Add 1**, such as **Will add: 1 × Goblin Warrior**. Excluded members stay under **Cannot join**, with their reason and repair link. The list, editor and Arrivals use the same XP total for joining members. A partial summary says **1 of 3 can join · XP 50 · Will add: 1 × Goblin Warrior**. **+ unknown** applies only to a joining member with no XP.

The encounter editor has a frame for each of its own groups, with its name, side, members and notes.
The frame’s **⋯ → Make available to any encounter** makes a saved group available for any encounter on the next save. **Undo move** restores its owner before that save. **Move to…** lists named encounters only.
**New group for this encounter** starts one there, before or after the encounter is saved. **Add to**,
above the search, chooses whether the search adds to the roster or to a group. **Save encounter**
saves the encounter and its groups together, and writes nothing if any of them cannot be saved. See [the schema](docs/design/campaign-schema.md#encounter-groups).

### Running an encounter as DM

Choose **Start fight** beside a saved encounter. The run takes
an independent copy of each creature's stats and starting state. Refreshing Party
or editing content does not change an existing run. Repeated creatures have
numbered names and separate HP, conditions, and notes.

Setup, live play and the ended view are separate. In setup, type initiative totals or
roll each participant using its supplied bonus. Rows hold still while you type, and
each tied total has **Tie** and two fixed arrow slots below it. The setup header warns about ties even while the panel is open. Fill in
unknown HP values through the row menu's **Correct…** action, then choose **Begin combat**. **Next turn** advances through present participants and counts
rounds. Absent participants keep their state and initiative when they return.
In combat, click a row's initiative, type the new total, and press Enter; Escape
leaves it as it was. The row moves to its new place as one step of undo, and the
current turn stays. A tie keeps its order, or roster order when the campaign uses it.
The current participant keeps its turn and its row until **Next turn**: then it moves,
so no one is skipped or acts twice. A lower total acts later this round, as a delay;
a higher one acts next round.
Participants at zero HP stay in turn order until you mark them absent.

Click a row, type an amount, and press Enter to apply final damage. **Heal** and
**Temp HP** are beside it. Ctrl+click adds more rows for one amount, as one step of undo, and the mark on each of those rows
gives it half, rounded down, for a save. **Conditions** lists the official conditions, one click
each, and a condition's word on a row opens its rule in Lookup. The side panel holds the stat block or sheet, **Conditions**, **Correct** (HP and initiative corrections, notes, sync), **Log**, and **Prep**. **Add to fight** opens Arrivals beside the run. At zero HP, party members show editable death-save counts and **+ Success** and **+ Failure** below the tabs.
Other participants show them only when a success or failure is already recorded. The tabs keep their place. A spell named in a stat block opens
in Lookup. Damage uses temporary HP first; healing stops at maximum HP. You resolve
attacks, damage modifiers, condition effects, Concentration checks, death and recovery,
phases, and completion conditions. The dice field beside the amount accepts expressions
such as `2d6 + 3`, and shows the total beside it. The roll also goes in the log. It is not an Undo
step, so **Undo** after a roll takes back the change before it, and the roll stays in the log.

**Effect** (`E`) in the dock pins words to the selected row, such as `Bless 10 rounds`; the Conditions tab's
**Other effect…** field reads them the same way. A time at the end counts: `10 rounds` or `1 minute` goes down
by one when that participant's turn ends, and says `last round`, then `time is up`; `on its turn` repeats on
each of its turns. On that participant's turn the run header repeats these reminders where it has room. `3
uses` is spent from the row, and `recharge 5–6` is marked spent and rolled from the row. An official condition
with a time keeps one entry with its count, and a sheet that removes it removes both. Splat removes nothing by
itself, and each act is one named Undo step. Words with no time, and the words of an encounter or a run saved
before effects, stay as words. After damage to a row marked **Conc**, the row shows its save DC until the next
step.

| Key | Command |
|---|---|
| `N` | Next turn |
| `Enter`, `Shift+Enter` | Damage or heal by the typed amount |
| `U`, `Ctrl+Z` | Undo |
| `↑`, `↓` | Select another row |
| `Ctrl+click` | Add a row to the targets of one amount |
| `Escape` | Select the current turn again |
| `A` | Focus the amount |
| `C` | Conditions |
| `E` | Effect: words for the selected row |
| `Ctrl+K` | Open or close Lookup. `Escape` also closes it |
| `Ctrl+T` | Show Trackers in Lookup, and go back. The run's letter keys stay off beside it |

**Add to fight** opens the panel for creatures and groups that join this fight. To add one creature, find it in the campaign,
or type a new name and its HP to make one up. A value you type replaces the creature's own for this
fight, and the card shows the change. A creature you make up stays in this run, and has no note. Its card shows current and maximum HP and the initiative total you typed, with no printed values. A tied arrival offers **Move before** with the tied creature or shared group's name. Shared rolls stay together; a move before the current turn says **acts next round**.
Below that, the panel lists this encounter's own groups, then the groups tonight's plan links, then
the other groups for any encounter, with their members and total XP. Drag a group's
name onto the initiative list, or press its **Add** button. Type a total beside it to use that for every
member, as players give theirs. An invalid total stays in the field and blocks Add and drop; clear it to roll. Groups with missing, unread or invalid members keep their usable members available. The panel says how many can join and keeps each excluded member's reason and repair link. When the field is blank, the campaign's rule decides: one shared roll with the first
member's bonus (identical NPCs, or the encounter's initiative groups), a roll for each member
(individually), or the initiative its side already has (one group per side). This works in setup and in
combat, and the same group can be added again. Each member is a new participant at full HP, with its
own key and numbered name. In combat the group takes its place on that total and the current turn stays
current. It joins the saved run and the log, and **Undo** takes back the whole group until later
steps stand in the way. The arrival notice gives the group and its initiative totals. The Log keeps member names and roll details.

The row and card of a creature with changes say `Changed`, and the card lists each base and changed
value. AC, maximum HP and initiative changes appear once, in the stat block's value trio with the printed value below. An NPC's row says which stat block it uses, such as `as Bandit Captain`; its name alone does not count as a stat change. The run applies name, maximum HP, AC, initiative bonus, side and conditions. Attacks and speed
are stat block text, so their changes stay on the card and on the row during combat, for you to add. These changes are saved with the run, so editing the file later does not
change it.

The combat log records changes. **Undo** names the last action and restores up to 100
prior actions, including turn changes, initiative and table rolls, and manual endings. The run file keeps these
steps, so **Undo** names and takes back the same step after **Close run** and **Resume**, and after the app starts again.
**End encounter…** records a manual ending and your outcome. It shortens to **End…** before it moves into a menu, and stays visible while it fits. Runs save automatically
after changes. **Close run** returns at once to the page the run began on when the run is saved and nothing is unsent. Otherwise it asks for a choice. A setup closed with nothing done leaves no saved run. The app reopens an open
run after a restart, on the current turn, and turns its sync back on. **In progress** on Encounters has **Resume** for an interrupted run,
with **Start another fight** and **Delete the saved run** in the row’s **⋯** menu. Saved times use relative time for today and yesterday, then a short date. Deletion has one step of undo.
An encounter with an unfinished run appears only in **In progress** and returns to **Ready to run** when it ends.
**Keep in Ready during a run** also keeps its Ready row, with **Start fight** blocked and a reason.
Empty list sections show one muted line and hide the zero count.
The rail keeps a paused run in view and resumes it in one click. Its detail wraps below the Run label. **History** holds dated, ended runs under their session.
History rows show **Open** or **Close**, with **Start fight** and **Edit** in their menu. An ended run opens read-only in place, with its final state and log; the list scrolls so that its row and whole final state are in view, and keeps them there when the window changes size, until the DM scrolls it. It leaves the live run open. Run changes never
write to templates or character copies.

A fight that ended by mistake can go on. In the ended view, **Undo: end encounter** takes the ending away while
the ending is the newest step. After a loot roll, or when the file has no steps, **Continue fight** is beside
**Close run**. A closed, ended run has **Continue fight** under **Read-only** when you open it in History; the
session plan's **Whole run** leads there. Continue fight is one named step: play returns to the turn the fight ended
on, the run's loot results stay, and **Undo: continue fight** puts the ending back with its outcome. When you
choose **End encounter…** again, the outcome field starts with that outcome, selected: Enter keeps it, and typing
replaces it. The outcome comes from the run file, so it is the same after a close and a restart. Timeline, the
session plan and the Encounters lists follow the run each way.

Continue fight is for the fight that was played last. Splat offers it when it can read `Timeline.md`, and no
fight and no long rest has a later place in the order of play than this fight's event. A long rest counts whatever
it restored, because you can still change that ruling. For
that fight, neither Continue fight nor its **Undo** changes which fight was played last, so the next fight still
starts each player from the right HP, and Timeline has no order to put back. For any other ended run the control
is not offered, and one sentence stands in its place: **A later fight or rest was played. Start a new run to play
this one again.** While `Timeline.md` cannot be read, the sentence is **Splat cannot read Timeline.md. Fix that on
the Timeline page first.** For a fight whose event was removed from Timeline, it is **This fight is not on the
Timeline. Start a new run to play it again.** **Undo: end encounter** has no such rule. It takes the ending away
for any open run while the ending is the newest step, as before.
With another run open, Continue fight is blocked and says **Close the open run first.**

### Run storage and campaign initiative

**Campaign → Rules and runs** sets the run folder. It defaults to `Encounter Runs`
inside the campaign, and accepts another relative folder or an absolute path.
Saved JSON files contain the run state, combat log, initiative rule, random state,
and frozen creature definitions. Saves use atomic replacement and detect external
edits. If the file changed, saving stops until you choose **Replace the file with this run** or **Save as a new
run**. Recovery copies keep the encounter name. Their detail starts with **Copy 1**, **Copy 2**, and so on.
**Resume** is primary on the run closed last. Each saved row shows its last change. A file preserved after a disk
conflict says **Changed on disk**, after its copy number if it has one. Other failed saves keep the current run
in memory and show an error. Changing the folder does not move old runs.

The run file also holds the Undo steps, in its `undo` field. Each step is saved as its differences from the state
after it, so 100 steps add little to the file. When the steps would take more than 16 MiB of the file, or more
than 256 MiB of memory when read, the oldest stay out and the newest are kept, with no message. A saved step
changes what a participant has now and never what it is: its name, side, AC, stat block, starting values and
changes come from the run, whatever a step in the file says. **Replace the file with this run** writes this run's steps, and
**Save as a new run** gives the new file this run's steps and leaves the other file's steps as they are. A file
from before this field loads with no steps. If Splat cannot read a file's steps, for example because a later
version wrote them, the run still opens. It says once that Undo starts again from there, the Log keeps that
line, and the next save leaves the unread steps out. Two things are not in the file: after a restart, Undo of a
save's damage restores the HP and does not arm the save again, and **Use the saved HP** is offered only in the
session that filled the party's HP after the long rest.

The same settings select how the campaign rolls NPC initiative: in groups of
identical NPCs (the D&D default), individually, in one group per side, or in the
encounter's initiative groups. **Roll NPC initiative** in run setup uses that
choice and leaves player totals alone. An encounter group added during setup stays
one roll under the identical-NPC rule, and joins neither its like nor another addition. The arrow beside it uses another choice for
one roll, and the campaign's choice stays the default. An arrow choice for the
encounter's initiative groups shows only when an NPC has a group. Side and initiative groups
use the first NPC in roster order for their bonus and roll mode. Normal, advantage, and disadvantage rolls are
available; use disadvantage for surprise. Ties use the campaign's roster-order
setting or the DM/player ordering controls. NPC status can be changed before
combat begins. Run folder settings use `splat_run_storage.directory`; initiative
is a campaign-local rule selected through `splat_rule_choices`.

### D&D Beyond combat sync

Combat sync starts when Splat connects to D&D Beyond and the active campaign is linked. It then
runs in the background on every page. Splat reads the party's sheets when the campaign's event stream
reports a change, and reads each sheet about once a minute. Party's panel can turn off its own sync of
the selected character. The background reads continue.

An encounter has the option **Sync party with D&D Beyond**, on by default. It shows only in a linked
campaign, in the roster of the encounter editor. With the option on:

- The roster shows what the party has now: HP, temporary HP, AC, conditions and spell slots for each
  row, and the party's total, so that you can plan the encounter around them.
- The party starts the run with the values on their sheets. Starting HP overrides for those rows wait,
  unused, in case you turn the option off.
- The run starts sync by itself, and **your changes to HP, death saves and conditions go to the
  player's sheet on D&D Beyond** with no other step.

With the option off, the party starts from their saved copies and the starting values you set, and
nothing in that encounter reaches D&D Beyond unless you select **Sync party** in the run. Use it for a flashback, a training fight, or a test of an
idea. The note records `combat_sync: false`, and a note without it has sync on.

**Sync party** in a run starts or stops sync for every linked player, and **Record** has the panel for
one. A stop lasts for that run, and the run saves it. A run saved before this option does not start by
itself. A finished run does not sync, so a run that you open to read keeps its results. **Continue fight** makes
it a live fight again, as **Undo** of the ending does: sync starts again by itself if **Sync party** was on, and
its first read follows the rule below. A value that the sheet changed after the fight ended replaces the run's
value, the notice and the Log say so, and **Undo: continue fight** does not put the older value back.

If D&D Beyond reports that this session cannot edit a character, the row says `Sync read-only`. Your
changes for that character stay in the run and nothing is sent. Changes on the sheet still come in, and
if you both changed a value, you choose which to keep. Each linked row shows its state, including **Sync off** after you stop it. Conflict words are underlined links to review. A conflict shows on the row and the strip without stopping
the table. Each connection first reads the live sheet. Saved runs do not automatically push old values.
The session must have edit permission for the character.

A value you recorded is not replaced without your say. If a session ends during a run, keep
recording: a row with a change that was not sent says `Not sent`, the others say `Sync stopped`, and
closing the run tells you who has unsent changes. When you connect again, sync starts again by itself
if **Sync party** was on. On a first read, by **Sync party**, after a reconnect or after a
restart, Splat compares each value with the last one D&D Beyond confirmed, or with what the participant
began the run with if there is none. A value you changed stays in the run, and if the sheet differs
that is a conflict, settled from the panel: use the sheet's value, or keep yours and send it. An open
conflict is found again after a close, an app close or a crash. A value you did not change takes
the sheet's.

While sync reads a sheet, that sheet decides Maximum HP, and the field says so. Stop sync to change it
in the run. A maximum set while sync was off stays until the next first read, where the notice and the
log say what it became. A message from sync stays beside the conflict in the header when it fits, or under the list in a narrow layout, until you dismiss it.

A save from an earlier version can hold unsent values as its last match, so on its first read every
difference asks, once. Its new `started` field is optional, and earlier saves load. Spell slots and
limited uses are not in the saved run: an unsent one asks at the next first read in the same session,
and closing says it will be lost. A sheet change ends undo only for the character it changes. A conflict choice in the sync panel is a named undo step. The notice and the Log say what changed, from what to what.

Sync covers current and temporary HP, death saves, official conditions,
ordinary and pact spell slots spent, and supported action/spell limited uses.
Concentration, custom effects, inventory charges, initiative, and notes stay local.
Changes are sent after a short pause; incoming values are checked every 15 seconds.
Each write is followed by a read to verify the result. Conflicts keep local edits
visible and offer a choice of local or remote values. Unconfirmed changes are not
retried without review. The service has no verified atomic compare-and-set API,
so another client can still change values between a read and a write.

Health or condition values taken from a sheet remove undo steps for that character. Other steps stay. Steps
read from the run file follow the same rule: a first read after a restart removes the saved steps of each character
whose sheet changed, and Undo before that first read makes the older value yours, which is a conflict if the sheet differs. If an atomic step makes older steps unsafe, the Log names the removed steps. In a small
window with a full roster, the list can leave no room for that message, so only the Log records the change. Wait for sync
or disable it before closing a run with pending changes. Disabling sync cancels
queued work but cannot recall a request already sent. A change made on Party is still sent after you
leave its page. Party sync stops only when the connection ends, and the sidebar says so until it starts
again. Run sync remains active while that run is open, until the encounter ends.

Transport and conflict handling are tested with simulated responses. Live writes
have not been verified with an authenticated D&D Beyond character. Endpoint
sources and limits are recorded in [the research notes](docs/research/dndbeyond-combat-writes.md).

Recording failures and stops remain on each recording’s row after you dismiss a banner. A stopped recording links to the microphone input in Settings. **Continue processing** resumes kept work. An active job hides the old cancellation or failure reason. The result viewer keeps its back link, date and Add to notes on one line. View choices sit below them, as a segmented control or a dropdown when space is short. The rail’s recording status opens the recording in one click.

Lookup keeps up to five results above the reader and marks the open one. When the only result is open, its result list closes. With more matches, **Show all** lists every match in place, in a short list that you can scroll, Tab through or walk with the arrow keys. New NPC and New location include the proposed name, and appear only when no exact match exists. A copy keeps the original name by default and offers Edit the copy after saving. Wide reader tables keep their first column and header in view. World rolls mark their matched row.

Campaign repair shows invalid notes in place. The campaign-wide Needed by your entries list opens entries in Lookup. A session note conflict shows your draft and the disk text side by side, with changed lines marked.

Library keeps its search when you change scope, and hides empty categories. Draft guidance stays in its save row. New World notes show their Obsidian file and a Show file link. Lookup keeps Back and Open in Library on its title line.

Campaign puts Needed by your entries below the packages in the left column. Select it to search dependencies and open one in Lookup. Invalid YAML marks its error line and says what to fix. Package and run settings keep their save state in the save row.

Session Read view keeps each link's written words. When a Timeline event is renamed, **Update links in notes** offers to replace only aliases that still use the old title, with **Undo update links** for the batch. Custom aliases stay as written. Plan row actions keep Undo beside their row. Templates use the same styled Markdown editor as notes. During autosave, notes show **Saving…**.

Settings imports a D&D Beyond campaign directly below its row and suggests a new folder beside the existing campaigns. Saved API keys name their slot, such as **Remove speech key (OpenAI)**. The quiet remove action sits below the key state, at the column’s left edge. Failed recording jobs use **Continue processing**; **Process again** replaces existing results after a separate confirmation.

Library shows **Back** with **Alt+Left** whenever an earlier entry is available. **Alt+Right** goes forward again to an entry left by Back, until a new link starts a new trail. The mouse's Back and Forward buttons do the same. Both ordinary and wiki links open in place. The draft and disk comparison opens at the first changed line, with two context lines above it; a missing line says **No line**. Lookup has **Clear** beside search. An NPC read beside a run shows linked **Fights as** and **In** values. New packages start beside the last registered package. **Folder** accepts a new folder or suggests an existing one.

The visual suite includes `npc-in-lookup`, `package-rename` and `package-about`. `run-continue` covers Undo after a
close and a restart, Continue fight and the sentence that stands in its place for a fight that was not played last,
and unread Undo steps, each with Lookup closed and open. `encounter-difficulty` covers the difficulty word on
the Ready to run rows, in the editor, on the plan's rows and their opened line, with Lookup closed and open, and
the encounters whose XP or party is not known, whose opened line says whose XP is not known. `run-effects` covers
effects and turn reminders: the dock's **Effect**, a count through a turn and a restart, the row's targets, and Lookup
open. `share-to-package` checks the new references after **Copy both**, and Library navigation checks the dependency copy action and its Undo.

On window focus, Splat checks campaign notes and every registered package for disk changes. Library, Lookup and encounter search reload changed content. A changed draft opens the disk comparison. A package's About page counts its chosen entries and offers **Choose every entry** with Undo. Removing an unavailable package also offers Undo and keeps links to the encounters that need its creatures.

If a package creature is deleted on disk, its encounter row keeps the last known name or says **Missing creature**. **Replace…** chooses a replacement with CR, HP and AC in view. **Replace all** is on by default for rows that use the same missing entry. The change keeps their names and starting values. A message names the result, such as `2 Cinder Wolf rows replaced with Guard.`, with **Undo replacement**. The row menu keeps **Edit** and can remove a row. The NPC **Add to fight** form labels its side choice **Side** and uses the roster's side names.

New creature entries start with speed and six ability scores in the Stat block tables. HP and AC stay unset until you enter them. Size, type, alignment and CR are fields too. Library, Lookup, encounter search and XP use one CR rule. The reader shows each vital once. If a visible AC or HP line disagrees, the editor offers to use its value for the fight, with Undo. A renamed copy also renames its first H1 when it matches the source name. If the first heading differs from the entry name, Library offers to fix that heading, with Undo. Package creation shows a **Folder** field and the destination path after you type a name. `package-file-changed`, `package-delete-entry`, `book-to-package` and `world-note-deleted` cover these disk and copy paths.

Library keeps its reader toolbar on one line. At narrow widths, **More** holds Copy and Make NPC note. Lookup supports **Alt+Left** for Back and **Alt+Right** for Forward, as do the mouse's Back and Forward buttons. World lists broken place and stat block links beside each note and counts affected notes in its header. Generated NPCs show their save path before **Save NPC**.

The run header keeps two lines after each action. Undo uses the space after the notice.
A short label keeps the action, amount and copy number; hover gives the full action.
An empty Undo says why. Recording silence stays ahead of connection notices, while a sync conflict
comes first. Keys keeps its word beside each panel. When several unfinished copies of
one encounter exist, the save word and the rail hover identify the open copy.

Recording and processing keep one linked rail slot above the account state. An open
recording row owns its error or stall message. Other pages show the banner.
Opening the session clears its unread completion notice. The rail keeps **Processed** and its warning
until a later recording or processing replaces the result. Generated drafts ask you to check them against your memory. A live row has
**Stop recording** before **Process**. A processed row keeps **Process again…** in its
menu above **Delete…**, with confirmation before it replaces results. The Recordings tab
keeps Stop recording on the row. Its list has a width limit; result readers keep the full pane.

Package repair keeps entries available when the manifest breaks or is deleted.
**Write the manifest again** restores a missing manifest. **Replace with a basic Package.md** or
**Replace with a basic Module.md** names the existing file and says which fields it drops.
For an unreadable manifest, Campaign puts bordered **Open in Library** first, followed by replacement and its consequence.
Replacing a broken file offers **Undo write Package.md** or **Undo write Module.md** for the file written.
If the file changes after the write, the result and its Undo go away at the next reload. The same applies to
every Library and Campaign result with Undo that wrote files, such as **Undo separate entries**. The copy
registration message stays, because it has another action; its Undo is turned off and says why. Copied World notes stay
visible in World and Lookup with a duplicate warning and file-specific identity
repair. Each repair has Undo. It also uses the file name for the copy's name and matching
first heading. A copied package folder offers **Register as a new package** under
its warning. This writes new package and entry identities, keeps links within the
copy, and has one Undo that restores the files. A run's Sheet uses its live death-save counts once.

Copy registration has **Choose for this campaign**. It starts off when the campaign uses the original. **Use the copy in encounters** changes encounter and group sources while keeping their names, counts and changes, with Undo. Duplicate repair actions name the folder when filenames match and can repair a whole copied folder at once. In the reader, the choices follow the entry's title and scroll with the entry.

A Library file that Splat cannot read at all, such as a note saved in another text encoding, says why and how to
fix it, with **Show file** and **Try again**. It offers no editor link and no repair in Edit Markdown until the file
reads again, and Splat never changes it. If an open entry's file stops reading, your draft stays until you choose.

A copied campaign folder registers with **Register as a new campaign**. Its result names the new campaign, with
**Undo register campaign** on the same message and **Build an encounter** below it.

After **Unregister**, Library offers **Undo unregister**. Campaign keeps the last folder and offers **Register it again**. A missing folder shows its last path. When one matching folder is nearby, one warning names the old and new
folders and the affected encounters. **Use this folder** comes first on the action line. Folder recovery has Undo.

## Obsidian file layouts

Splat reads schema 2 and Timeline version 2 only. Other versions are unreadable
files, flagged with their path. Encounter and group comparisons retain outside tags
and prose.

- Timeline version 2 keeps days and events in the body. Day IDs sit in `%% splat id=… %%`
  below the heading. Event IDs stay at the end of the list item.
- Campaign schema 2 uses flat properties, package task items, and a Rule/Choice table.
  Clear a package task in Obsidian to turn that package off. Bullets `-`, `*` and `+`,
  tab indents, and commas inside links are accepted. A rule choice uses the visible
  rule name or link; its next save updates the stored id. Unknown choices have a line warning.
- World type names ignore case. Unique stat-block names and vault requirement links
  stay readable in Properties. Character payloads and package routes use folded
  Splat data callouts.
- Encounter groups use a member table. Saved encounters use a roster table, with slot
  keys in comments in the Note cell. New files use the entry name.
  Creature cells accept a unique name or `[[Name]]`; save writes a named link.
  Copying a roster row creates another slot with only its visible values; a copied party member is flagged.
  Wiki-link pipes in table cells use `\|`. Both forms are read. The visible link wins
  over a stale ID; the ID is a fallback for a moved file. Note text on both sides of
  machine comments is kept. Group sides travel with their row as `Side rivals`.
  Starting HP is visible as `Start HP 12` in roster Changes. Speed accepts `40`,
  `40 ft`, or `40 ft.` and saves as `Speed 40 ft.`.
  A group with an unread row shows how many members can join in Groups, Arrivals,
  and the plan. Edit opens the normal change form and keeps the name and note.

Bad rows stay in the file and have a line warning. Saves keep other prose and properties.
Run files stay JSON. Turn on Obsidian Sync's **other file types** option to sync them.

Duplicate-file repair names the file that becomes a separate entry and offers **Undo separate entry** beside its result.
