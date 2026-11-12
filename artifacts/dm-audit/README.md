# DM audit: one evening of play

## The frozen evening (2026-11-11)

The script `src/visual_tests/dm_audit.rs` runs as the scenario `dm-audit` in every visual run, and it is
G6: it plays the evening and fails when a task's page transitions, interactions or path differ from
`counts.json`, when a frozen task is missing, when a task is new or out of order, or when a counted act finds
no control.

**G6 runs in every gate.** When a change makes a task cost more or fewer acts on purpose, its author runs
`devenv shell -- audit-freeze counts`, which writes `steps.tsv`, `summary.tsv` and `counts.json` from this
source, and says in the pull request why the task costs more or less. A feature that adds acts of its own adds
them as **new** tasks and runs `devenv shell -- audit-freeze add`: that writes one row for each new task and
leaves every frozen row as it is; it refuses, and writes nothing, when a frozen task's transitions,
interactions or path differ, or when a frozen task is missing or out of order. The scenario takes no captures,
so the capture names in the sections below refer to the first audit's run.

**Set-up is apart, and it is only what a DM does not do in the session.** Each is a `Setup` row in
`steps.tsv` and counts for nothing:

| Set-up act | Why it is not counted |
|---|---|
| Play last week's session, then quit and open the app | It makes the fixture: last week happened before this evening |
| Work at 1200x800 | The window size of the measurement, not an act |
| Save, quit and open the app at the table; and after the session | Time passing between prep, play and wrap-up. Starting the app is outside it |
| A microphone is there | A simulated device: the test has no sound card |
| The recorder hears the table | A simulated signal from that device |
| The recording service has its keys | Settings made once, long before the evening; the test has no network |

The Undo of the Fireball was set-up before; it is a key press the DM makes in the session, so it is now the
counted task "The table rules that the Fireball missed: undo it" (1).

**Measured on `main`** at `1040ddff`, with pull request 232:

| Phase | Tasks before | Tasks now | Transitions before | Transitions now | Interactions before | Interactions now |
|---|---:|---:|---:|---:|---:|---:|
| Before | 7 | 7 | 5 | 5 | 34 | 36 |
| During | 27 | 28 | 7 | 7 | 180 | 183 |
| After | 7 | 7 | 1 | 5 | 17 | 21 |
| Total | 41 | **42** | **13** | **17** | **231** | **240** |

By kind, before and now: Nav 14 and 17, Sub 7 and 6, Panel 12 and 12, Find 49 and 53, Do 149 and 152.

**Each difference from 13 and 231.** The old `steps.tsv` was measured on 2026-11-02 (#25). The script and
the app changed after it, and the script was not run again.

| Task | Before | Now | Why |
|---|---|---|---|
| Recall what happened last session | 1: `open Last time` | 2: the `Last time` link to the last session, then tonight's session again | Since #39 the line is a link with a one-line preview, and the script stopped clicking. The preview reads only `Recap of last time` or `What happened`. Last week's words are under another heading, so the line shows none of them (a fact in `summary.tsv`). Reading them takes the link and the way back |
| Read last session's recap aloud | 1 | 2 | The same |
| The party travels for half a day | 1 | 2: `More Add actions`, `Add Travel` | Since #136 the strip keeps Fight, Group and Long rest in view, and Travel is in **More** at 1200x800 |
| Record a short rest | 1 | 2 | The same, for Short rest |
| Build tonight's encounter | 15 | 16 | `Duplicate` is a line of the row's menu: 2, was 1 |
| Write the read-aloud text and tactics | 3 | 4 | Since #247 the `Read-aloud and notes` field starts below the editor's visible part at 1200x800, so the DM scrolls to it first. The driver counts one scroll into view as one interaction. G6 caught this move when #247 merged |
| Make the boss your own | 6 | 5 | `Make it yours` is a line of the row's menu: 2, was 1. The HP field has focus, so the number is typed in 1, was 3 |
| The table rules that the Fireball missed: undo it | set-up, not counted | 1 | A key press the DM makes in the session |
| A fight you did not plan | 9 | 8 | `Close run` closes the empty run at once. `Close now` is no longer asked |
| Say where the fight happened | 4, no transition | 4, **T T** | The `Details` tab went in #39. The place is now the `Where` field of the Timeline page: the row's `Timeline` link, type, then `Session` again |
| Link the fight in the note | 2: `Add to notes` | 6, **T T** | `Add to notes` is drawn only for an event with no run (`src/sessions/page.rs`, the plan row's action band), so on a played fight that act was blocked. The DM's goal still has a route: the row's `Timeline` link, `Copy link` in the Timeline's detail of the fight, `Session`, then click in the note, Ctrl+End and Ctrl+V. The note then holds the fight's `[[Timeline#^…]]` link (a fact in `summary.tsv`) |
| Process the recording | 4 | 4 | Its tab is named `Recordings, 1 recording` now. The count is the same |

Sum: +1, +1, +1, +1, +1, +1, −1, +1, −1, 0 and +4 interactions give 240. The four transitions are the two
Timeline trips. One fact also changed with the app and not the count: "final HP reads in place" looked for a
`FINAL HP` heading, which the opened row no longer has; it now looks for the HP lines and `Log`, and is true.

**What G6 does not hold.** The order of acts inside one task is not frozen: two acts that swap places keep
the task's counts and path.

### The plain campaign (G7)

`Seed::Plain` in `src/visual_tests/fixture.rs` is the campaign of the measured evening plus what the
session-cycle features touch: old task lines under `## Plan for tonight`, words typed as conditions
(`Bless 10 rounds`, `Hexed until the hag dies`), a marked roll table, two played fights in one session (one
with loot and a title longer than its row), NPCs with and without a stat block, a session with a recap and one
without, a Travel event, the participant name `Ashfang, Eldest Wolf of the Southern Ridge`, and 12
participants in one fight. Its runs have IDs made from their campaign and name, so their files are the same
on every run of the test.

The scenario `plain-evening` plays it with today's controls (`src/visual_tests/plain_evening.rs`). `SCREENS`
and `HOVERS` in that file fix its 142 views in code; a missing or extra view always fails, and no row of
`allowed.json` can excuse one. After every act the screen must equal a view that G7 records, or the act is in
`PASSING` with its reason (setting the stage, one `Next turn` of twelve, the editor scrolled to its loot row);
so a form or panel that the evening opens between two views cannot go unseen:

- **Pages, at 1200x800 and 960x640 with Lookup closed and open:** tonight's plan; its Travel row open; its
  planned fight open; last session with its played fight open; the ended view of the fight that was played
  last (with `Continue fight`) and of an older one (with the reason it cannot go on); Encounters; the
  encounter editor; the 12-person run; its **Keys** list; its **Conditions** tab with `Other effect`; its
  Arrivals panel; the run after a whole round of `Next turn`; Timeline; the World page's reader; a new session.
- **Lookup's readers, at both sizes:** the six readers whose titles section 1.2 sums, the marked roll table
  `Road Encounters`, and the package creature `Cinder Wolf`.
- **Open menus, at both sizes:** the Add strip's **More**, a Travel row's menu, a planned fight's menu, an
  encounter row's menu, and the loot editor's table menu. Each view must hold the menu's own items.
- **Screens as they stand, at both sizes:** the page the app opens on; last session before and after each
  played fight opens; Encounters after each ended run closes; a roster row selected; Encounters after the
  run closes; the Session page under the readers; the World page's list; the new-session form, empty and
  with its title; the new note written, and saved.
- **Hovers, at both sizes (`HOVERS`, each with its reason):** the **Last time** link, a planned fight, a Travel
  row's menu button, a played fight's long title, `Continue fight`, the older fight's reason, an encounter
  row's menu button, a roster row's condition words, the dock's **Conditions**, the Undo label, a reader's
  `Edit`, and a package creature's art. A control with no hover today is recorded too, so a hover it gains
  is a difference.

For each view G7 records every named control in view and every painted text, with its rectangle, and the
part a person sees when that is less. Every input of the evening is an **act** (113 of them): for each, the
words that any frame painted and the frame before did not, the words that went, and each campaign file that
was written or changed, by digest. A test hook reads every frame, also the frames that one test step runs,
so a notice shown for one frame is in the record, and so is a file written and put back. G7 compares every
campaign file at the start and the end by its **bytes**. A run's file must open as a run (`Run::open`); in
it, and nowhere else, the test folder inside `splat_run_source_path` becomes `<world>`, and the World page's
path label loses the same folder. Each file that the evening changed is kept whole in
`src/visual_tests/plain_evening/files/`, so a new reference shows as a readable diff, also of a saved run.

G7 fails on any difference that a row of `plain_evening/allowed.json` does not name whole: its row of
section 2.5, the screen, the view, the control, text, act or file, and the rectangles, words or lines. A row
that matches nothing fails, a row cannot name a view, and the check fails when the reference or the
evening's record is empty or missing.

**G7 compares only with `SPLAT_FROZEN=1`.** Any change of the user interface fails it by design, so the
default gate runs the scenario and its captures and skips the comparison; the pull requests of the
session-cycle build plan run `SPLAT_FROZEN=1 devenv shell -- audit-gate <label>`. The reference is true only
for the commit of `main` it was written on, which it records in `frozen_at`, and the failure's first line
names it. Such a pull request carries one commit that is the output of `devenv shell -- audit-freeze reference`
on an unchanged commit of `main` (the command refuses other changes in the tree), and makes it again after each
merge of `main`. A feature adds a view of its own surface to `SCREENS` (a hover to `HOVERS`) and the acts that
reach it, one row in `allowed.json` for each difference it makes to an old record, and runs
`devenv shell -- audit-freeze add`: that inserts the new views and acts in their places and leaves every old
record as it is, so `git diff --diff-algorithm=patience` of the reference shows added lines only; it refuses,
and writes nothing, while any other difference has no row. A reviewer checks out the commit in `frozen_at`,
copies in the pull request's frozen files, and runs `devenv shell -- audit-freeze verify`: it writes the
reference into a scratch folder and fails unless the tree's files are byte for byte the same.

**What G7 does not hold.** Content scrolled out of view: a feature whose change is below the fold adds a
screen that shows it. A control's behaviour, colour or imagery. A key binding with no visible control: its
feature tests it. Pages the plain evening does not visit, such as Settings. A same-bytes rewrite of
`Sessions/.splat-session-paths.json`, which the app makes on a one-second poll of real time; a change of its
bytes is recorded. A file created outside the campaign folder, a file created and deleted inside one act, a
change of permissions with the same bytes, one extra frame of delay, and a corrupt-and-restore of
`.splat-session-paths.json` inside one act. The `why` of an allowed row: the tool does not check it, so a
reviewer reads the whole `difference`.

### The reader's title, measured

`titles.tsv` holds the measurement, from the same scenario. Lookup's reader, with the title in `Display`
(26 px):

| Title | Width | Case B (1200x800, Lookup open), inside 406: room after `Edit` | Case D (960x640, Lookup open), inside 320: room after `Edit` | With art (a campaign creature, no `Edit`): B and D |
|---|---:|---:|---:|---:|
| `Captain Varra`, 13 letters | 160.1 | 176.8 | 90.8 | 173.9 and 87.9 |
| `Oren Thistlewick`, 16 | 196.5 | 140.4 | 54.4 | 129.5 and 43.9, scrolling |
| `Captain Varra Black`, 19 | 229.5 | 107.4 | 21.4 | 96.5 and 10.9, scrolling |

`Edit` is 61.1 px and starts 8 px after the title. In Lookup a campaign creature has no `Edit` (it offers
`Open in Library`), and its art, 64 px, starts 8 px after the title. The two creatures of 16 and 19 letters
have long stat blocks, so their readers scroll: the scroll bar starts 8 px inside the search field's edge at
both sizes (at 1179 and 939.4), so a reader that scrolls has 8 px less. The 16-letter row reads 129.5 and
43.9 for that reason. Section 1.2 of `docs/design/session-cycle.md` estimated 82, 47 and 12 at case D, and
168, 133 and 98 at case B. The verdicts at case D stay. At case B the 16-letter title now has room for
`Start fight` and the icon, with 0.4 px to spare on the estimated widths of actions that do not exist yet; in a
reader that scrolls it has `Start fight` alone. Section 1.2 and section 7 now carry the measured numbers, and
the limit "about 10 letters" for `Start fight` at case D is now about 11.

## The other journeys, measured (J2, J3, J4, J6)

The workflow score has six journeys (`journeys.md` of the score). J1 is the evening above and J5 the plain
evening (G7). The other four are now played on the real app too, by `src/visual_tests/dm_audit/journeys.rs`,
with this file's driver, tasks and counting: each act has its kind, the driver sees each page change, and a
set-up act counts for nothing. Each journey is a sequence of tasks in the DM's words, step by step as section 2
of `docs/design/session-cycle.md` walks it (2.2 for J2, 2.3 for J3, 2.4 for J4, 2.6 for J6). Where a task is not
in the design's walk (a fight's set-up and end, which the design says cost what they cost today), it is measured
too, and the tables below say so.

**One scenario for each journey:** `journey-j2`, `journey-j3`, `journey-j4` and `journey-j6`. Each has its own
window size (J3 is at 960x640 with Lookup open), its own campaign state and its own dice (from the scenario's
name), and they share the fixture work through helpers in one file; separate scenarios also run side by side in
the suite and a failure names its journey. The fixtures are those that exist: the audit's campaign with last
week's session (J2, J3, J6), and for J4 the Tournament note of `docs/design/world-examples/heralds-tournament.md`
with its tables, as the `trackers` scenario writes it. Each journey adds what its prep would have made: copies of
the fixture's encounters under new names (`Wolf Pack`, `Ember Watch`, `Gate Watch`, `Ford Patrol`), a road table,
the treasure table, and the World NPCs `Warden Ostrel` (J2's planned NPC: a World NPC whose stat block is the
fixture's creature Captain Varra, with no portrait, as section 2.2 assumes) and `Verori` (J6). Then, as J1 does,
the app is saved, closed and opened at the table.

**G6 holds them as it holds J1.** Each journey has `counts.json`, `steps.tsv` and `summary.tsv` under
`journeys/J2/` and so on; `summary.tsv` gives each task's transitions and interactions by kind, its path and the
totals. The same code compares them in every gate (`measure` in `src/visual_tests/dm_audit.rs`), so a change
that makes a journey's task cost more or less fails it. `devenv shell -- audit-freeze journeys` writes every row
of the four; `devenv shell -- audit-freeze add` adds new tasks to J1 and to each journey and refuses when a frozen
task changed. A refused freeze writes nothing: each scenario writes into a staging folder, the command
copies it into the tree only when every scenario passed, and on a refusal it checks that no file under
`artifacts/dm-audit/` or `src/visual_tests/plain_evening/` changed. The copy itself is file by file: when it
fails, the command fails and keeps the staged files, and git restores the tree. `journeys/FINDINGS.md` lists what the
scripts found, as app faults, limits the design accepts, and tasks the design does not walk.

**Each outcome is checked on the saved files.** After the act, the script reads what Obsidian and the next
session read: the session note (its plan, rewards blocks, carried parts, each line once), a World entry's Log,
`Timeline.md` (each event and its day, by the app's own day count), `Trackers.md` (by the app's own ledger
type), the run file, and the encounter. A wrong record fails the journey before its counts
(`Audit::outcome`).

The tables put each step's measured count beside the count that section 2 promises, and beside "today" (the cost
before the features) where the design gives it. **T** is a page transition.

### J2, the improvised evening (1200x800)

| Design step | The DM wants | Measured | Designed | Today |
|---|---|---|---:|---:|
| 2 | Play the planned NPC: read Warden Ostrel | Panel 1 = **1** | 1 | 2 |
| 3 | The party attacks him: start the fight | Nav 1, Panel 1 = **2**, T | 2, T | 8, T T |
| — | Roll and begin the fight | Do 11 = **11** | as today | 11 |
| — | He gives up: end the fight | Do 3 = **3** | as today | 3 |
| 4 | A line in his Log | Panel 2, Do 3 = **5** | 5 | 6 |
| 5 | Close, keep the rewards, add loot nobody planned | Nav 1, Find 1, Do 3 = **5**, T | 5, T | 4 |
| 6 | Two days overland, with the checks rolled | Find 2, Do 4 = **6** | 6 | 12, T T |
| 7 | The second day names the Wolf Pack: plan it, start it | Nav 1, Do 1 = **2**, T | 2, T | 3, T |
| — | Roll and begin the Wolf Pack fight | Do 11 = **11** | as today | 11 |
| — | End the Wolf Pack fight | Nav 1, Do 2 = **3**, T | as today | 3, T |
| — | The Wolf Pack fight's rewards in the note | Do 1 = **1** | — | no route |
| — | Mark the talk with him done, and log what happened | Panel 2, Do 4 = **6** | — | — |
| | **Steps 2 to 7** | **21, 3 T** | **21, 3 T** | **35, 6 T** |
| | **The whole journey** | **56, 4 T** (Nav 4, Panel 6, Find 3, Do 43) | | |

Step 5's `Add rewards` is a line of the row's menu: beside the note at 1200x800 the plan is too narrow for it on
the row or as the opened row's last link (the fit rule of section 2.5, row 10). The menu costs the Find that the
design counts for opening the row. One of the last task's four Do is the scroll that brings the done scene's row
into view.

Checked on the saved files: each Log line once in `NPCs/Warden Ostrel.md`, stamped with the session and its
day; one rewards block for each fight, with its XP line and the ring under Ostrel's; the fight with Ostrel on
Day 1 and the journey of 2 days with one check for each day, the second naming the Wolf Pack; the Wolf Pack
once, above the skipped fight, and its fight on Day 2, the day of the check that named it (since pull request
284; FINDINGS J2-2); the talk ticked done once.

### J3, the broken evening (960x640, Lookup open)

The start, checked: fight 2 in round 2, Lookup open on last week's note found by `miller`, sync on, and one
tracker notice due from before the fight (the first bell), so the rail says `Due 1`.

| Design step | The DM wants | Measured | Designed | Today |
|---|---|---|---:|---:|
| 1 | The fight was ended by mistake (the two acts of the mistake) | Do 2 = **2** | not counted | — |
| 1 | Take the ending back: `Undo: end encounter` | Do 1 = **1** | 1 | — |
| 2 | Pin `Poisoned 3 rounds` to a wolf | Find 1, Sub 1, Do 3 = **5** | not counted (the walk implies 4) | no count |
| — | Play on into round 3 | Do 8 = **8** | as today | 8 |
| 2 | After the crash in round 3: back to the fight | **0** | 0 | — |
| 3 | Lookup is closed after the restart: find the note again | Panel 1, Find 1 = **2** | 2 | — |
| 4 | A sync conflict on Kestrel Dunmoor: `Review`, `Use D&D Beyond values`, select the wolf, Ctrl+K, the note again | Panel 1, Find 2, Do 2 = **5** | 5 | — |
| 5 | The note changed in Obsidian (a tick and a new plan line) | **0** | 0 | — |
| — | End the second fight | Nav 1, Do 2 = **3**, T | as today | 3, T |
| 6 | The same, while Splat holds unsaved words: a plan act is refused | Do 1 = **1** | in the 4 | — |
| 6 | Choose a version, and the plan act again | Do 2 = **2** | in the 4 | — |
| W6-D1 | Take a scene off tonight's plan (the scenes are folded: `3 scenes`) | Find 2, Do 1 = **3** | — | — |
| W6-D1 | Begin the fight before it | Nav 1 = **1**, T | — | — |
| W6-D1 | The scene is wanted after all: **Session**, `Undo remove` | Nav 1, Do 1 = **2**, T | — | — |
| | **Steps 1 to 5 (recovery)** | **8** | **8** | **8**, with pull request 232 |
| | **Step 6** | **3** | **4** | — |
| | **The whole journey** | **36, 3 T** (Nav 3, Sub 1, Panel 2, Find 6, Do 24) | | |

The sync conflict is simulated, as the scenario `run-sync-conflict` does (`simulate_sync_conflict`): a test
build reads no live sheet. Its `Review` and choice are the app's own. Checked on the saved files: the run file
is in round 2 again after `Undo`; after the restart it is in round 3, the wolf holds `Poisoned` with its count,
`Undo` names the step it named before the crash, and `Trackers.md` still holds the bell's notice, not done, so
the rail says `Due 1`; after the conflict, Kestrel's HP in the run file is the one the row shows; the refused
act leaves the note byte-equal to Obsidian's; the chosen version holds the DM's line and the first Obsidian
edit, and the rewards once; after `Undo remove`, the plan is byte-equal to the plan before the removal.

Step 6 costs 3, not 4: the design walks it in the run, where the choice is behind Ctrl+J. Played after the
fight, the Session page shows the comparison in the plan's place, with its choices, and Lookup stays open
(FINDINGS J3-3).

### J4, the race (1200x800)

The start, checked on `Trackers.md`: the clock at 30, Favor 2, no seals. The prize day of the Tournament's
note comes due on Day 2.

| Design step | The DM wants | Measured | Designed | Paper |
|---|---|---|---:|---:|
| 1 | Open the numbers | Panel 1 = **1** | 1 | 0 |
| 2 | Enter a room: the clock reaches 40, a bell is due | Do 1 = **1** | 1 | 1 |
| 3 | The bell: each rival's roll and the wandering roll | Do 4 = **4** | 4 | 4, 5 dice |
| 4 | The Old Guard takes its first seal | Do 1 = **1** | 1 | 1 |
| — | Put the seal on the Timeline (`Add to Timeline`) | Do 1 = **1** | — | — |
| 5 | Finish the bell | Do 1 = **1** | 1 | 1 |
| 6 | Keep the seal's notice in mind | **0** | 0 | 1 |
| 7 | The party enters the arena | Do 1 = **1** | 1 | 1 |
| — | Start the arena fight | Nav 1, Panel 1 = **2**, T | no tracker act | — |
| — | Roll and begin the arena fight | Do 11 = **11** | as today | — |
| — | Round 1 | Do 7 = **7** | as today | — |
| — | In the fight, the crowd loves a move: Ctrl+T, press, Ctrl+T | Panel 2, Do 1 = **3** | ruled by the owner | 1 |
| — | The seal's notice is due: the Old Guard's group joins (`Add to fight`, the group's `Add`) | Find 1, Do 1 = **2** | as today | — |
| — | Round 2 | Do 10 = **10** | as today | — |
| — | Round 3 | Do 9 = **9** | as today | — |
| — | End the arena fight | Nav 1, Do 2 = **3**, T | as today | — |
| 8 | Three minutes passed in the fight | Panel 1, Do 3 = **4** | 4 | 3 |
| 9 | The rivals came: finish the seal's notice | Do 1 = **1** | 1 | 1 |
| 10 | The crowd loved a move, then threw a gift | Do 2 = **2** | 2 | 2, 1 die |
| 11 | A wrong press on `Throw a gift`, corrected by the opposite action | Do 3 = **3** | 3 | 3 |
| 12 | A soul is collected: the clock passes 60 | Do 1 = **1** | 1 | 2 |
| 13 | The second bell | Do 5 = **5** | 5 | 5, 5 dice |
| — | A new day between sessions: the prize day comes due | Nav 1, Panel 2, Do 2 = **5**, T | — | — |
| | **Steps 1 to 13** | **25** (Panel 2, Do 23) | **25** | **25** |
| | **The fight (as today, with the ruled tracker act and the arrival)** | **47, 2 T** | — | — |
| | **The whole journey** | **78, 3 T** (Nav 3, Panel 7, Find 1, Do 67) | | |

The fight runs three rounds: the Old Guard's 1-2 seals group (three goblin warriors, side `rivals`, made in prep)
joins at the start of round 2, while the rail says `Due 1` for the seal's notice, and the DM ends the fight in
round 3. Checked on the saved files: the seal is a Day 1 event of `Timeline.md`; the ledger holds the four bell
rolls and the seal; the run file ends in round 3 with the goblin warriors in it, and `Timeline.md` says
`round 3` on Day 1; after the fight, three `Fight minute` changes, the seal's notice done, the presses of steps
10 and 11 in order with Favor at 4 and a gift roll on each `Throw a gift`; the clock at 63 and one soul after
the second bell; on Day 2, the prize day's notice done and Favor at 5.

### J6, the week between (1200x800)

| Design step | The DM wants | Measured | Designed | Today |
|---|---|---|---:|---:|
| 1 | The party searches the last fight (`Roll 2 × gems`) | Do 1 = **1** | 1 | 1 |
| 2 | Verori died here | Panel 1, Find 1, Do 5 = **7** | 5 | 5 |
| 3 | He gave the gate key away first | Panel 1, Do 3 = **4** | 4 | 5 |
| 4 | Back to the evening | Nav 1 = **1**, T | 1, T | 1 |
| 5 | Rewards for each of the three fights | Do 3 = **3** | 3 | no route |
| 6 | A week later: a new session | Do 3 = **3** | 3 | 3 |
| 7 | Carry over what the last session left (the recap, two fights and a scene, one open thread) | Do 1 = **1** | 1 | by hand |
| 8 | Three scenes | Do 4 = **4** | 4 | 4 |
| 9 | Is each fight right for this party? | **0** | 0 to 2 | by hand |
| — | Make the Gate Watch harder in the encounter editor | Nav 2, Find 3, Do 2 = **7**, T T | — | — |
| — | Prepare the road: a Travel row with its checks | Find 2, Do 4 = **6** | — | — |
| 10 | Start the first fight | Nav 1 = **1**, T | 1, T | 1 |
| | **Steps 1 to 10** | **25, 2 T** (Nav 2, Panel 2, Find 1, Do 20) | **23 to 25, 2 T** | **20**, with no route for 5, 7 and 9 |
| | **The whole journey** | **38, 4 T** (Nav 4, Panel 2, Find 6, Do 26) | | |

Last week's note holds what `Carry over` reads: prose under `What happened`, two fights and a scene not played,
and two threads, one ticked. Checked on the saved files: `NPCs/Verori.md` holds `status: dead` and each Log line
once, stamped; the note holds one rewards block for each of the three fights; the new note holds, once each,
the recap's embed, the two fights and the scene, the open thread and not the ticked one, `carried_from` and
`carried_parts` `recap`, `plan`, `threads`; then the three scenes; both fights' rows have a word; the saved
Gate Watch holds the Oni and its word changes; `Timeline.md` holds the road of 2 days; the new run holds the
Oni.

### Above the design

| Journey and step | Measured | Designed | The acts, in order | Why |
|---|---:|---:|---|---|
| J6 step 2 | 7 | 5 | Ctrl+K, type `verori`, `Edit Verori`, click Status, select all, type `dead`, `Save changes` | Lookup's reader is compact and has no `Change` on Status (FINDINGS J6-2) |
| J3 step 2 | 5 | not counted; the walk implies 4 | select the wolf, **Conditions**, click `Other effect` and type, Enter | A limit the design accepts (section 7: `Effect` by the pointer costs 5 with Lookup open); the tab takes Lookup's place (FINDINGS J3-1) |
| J4, the arena fight's start | 2 | 1 (no tracker act in the fight) | `Start fight`, Ctrl+T | A limit the design accepts: a side panel stays until the DM closes it (section 1.2), and trackers in a fight are behind Ctrl+T (section 7). FINDINGS J4-1 |

J6 step 2 is the one app fault above the design. The others are limits that section 1.2 or 7 states; their
cost stays here so that a later change can be weighed. J3 step 6 costs less than its walk (3 against 4,
FINDINGS J3-3). The design's own arithmetic is right in all four walks (J2: 21; J3: 8 for steps 1 to 5 and 4
for step 6; J4: 25; J6: 23 to 25), so the design document is not changed.

## Update: what the design changes did

Design changes A, B, D, E and F below are built. C, adding a participant to a live fight, is now built too. The run's Arrivals panel adds one creature, found or made up, and encounter groups add a prepared group. A group can also be assembled in the session's plan. The measured counts below do not include these flows. The same evening,
measured again in the running app with the changes in place:

| Phase | Transitions before | Transitions now | Interactions before | Interactions now |
|---|---:|---:|---:|---:|
| Before | 9 | 5 | 74 | 34 |
| During | 12 | 7 | 212 | 180 |
| After | 6 | 1 | 23 | 17 |
| Total | 27 | 13 | 309 | 231 |

The hand count in section 6 said 11 and 221. It missed a trip from Party back to the session in prep, and one through
Encounters for an improvised fight. `steps.tsv` and `summary.tsv` hold this measured evening, and the tables from section 2
on describe the app before the changes.

Two later audits, by Codex Astra and Codex Sol, each played other journeys on this work. Fixed from their findings: a fight
that goes on under a later session keeps its HP for the next fight; a fight can be planned and started again; Close run and
the session being played survive a restart; the fight picker scrolls and has a search; half for a save is a mark on each
target's row, so any number of targets stays in view; initiative can be corrected during combat; and Exhaustion is one level.

Done since: **Undo after a restart.** The run file keeps the Undo steps, and a fight that ended by mistake goes on
with **Continue fight** while it is the fight that was played last (`README.md`, Running an encounter as DM).

Left for later, because each is a feature of its own:

- **A note pinned to a creature**, with a reminder on its turn, for a recurring save.
- **A way to choose several targets with one hand.** Ctrl+click needs both.

Corrections to the first audit:

- **L15 was wrong.** Lookup already ranked a DM's own entry above the book. The second "Fireball" in that capture is a
  copy from a character sheet, which sorts lower on purpose. It now says "From a character sheet".
- **D5** counted 9 entries from a text scan. A unit test now copies all 9.


I played one full evening in the real app as a DM: prep at the desk, a session at the table, and the
wrap-up. The script is `src/visual_tests/dm_audit.rs`. The driver records each change of top-level page,
so the transition counts are measured and cannot miss one.

**Result.** The evening took **27 page transitions and 309 interactions**. Combat is good: three rounds
took 96 interactions and no page transition. The load is in the gaps between fights and in prep. The
design below brings the transitions to about **11** and the interactions to about **221**. Those two
numbers are hand counts against the design, not measurements.

Files here: `steps.tsv` (each step), `summary.tsv` (each task and fact). Captures are in
`artifacts/screenshots/dm-audit/`, which git ignores. A capture name below means
`dm-audit__<name>__1200x800.png` in that folder.

## 1. What I counted

- **Page transition:** the top-level page changes (Encounters, Sessions, Timeline, Party, Library, Campaign,
  Settings, Run). Lookup and Notes open beside the page, so they do not count. I count them as panel steps.
- **Interaction:** one click, one typed string, or one key press. This is the meaning in `docs/ux-intent.md`.
- **Kinds of interaction:** `Nav` (go to a page), `Sub` (a tab inside a page), `Panel` (open or close a side
  panel), `Find` (search or select to reach something), `Do` (the work). Everything except `Do` is overhead.
- **Set-up:** window 1200x800, the fixture campaign, a party of 5. Each fight had 8 participants, and the first
  fight ran 3 rounds. The DM knows every shortcut, so these counts are a lower limit.

## 2. Measured results

| Phase | Tasks | Page transitions | DM chose the page | An action caused it | Interactions | Nav | Sub | Panel | Find | Do |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Before | 8 | **9** | 9 | 0 | 74 | 9 | 5 | 1 | 22 | 37 |
| During | 28 | **12** | 6 | 6 | 212 | 10 | 14 | 12 | 44 | 132 |
| After | 7 | **6** | 4 | 2 | 23 | 5 | 5 | 0 | 3 | 10 |
| Total | 43 | **27** | 19 | 8 | 309 | 24 | 24 | 13 | 69 | 179 |

**The path through the evening:**

- Before: Encounters > Sessions > Timeline > Sessions > Party > Encounters > Library > Encounters > Library > Sessions
- During: Sessions > Encounters > Run > Sessions > Run > Encounters > Sessions > Encounters > Run > Encounters > Run > Encounters > Sessions
- After: Sessions > Timeline > Sessions > Encounters > Run > Encounters > Party

**Costs and effects per task.** "Model" is the count with the design in section 5.

| Phase | Task | Transitions | Interactions | Model transitions | Model interactions |
|---|---|---:|---:|---:|---:|
| Before | Recall what happened last session | 1 | 2 | 1 | 1 |
| Before | Check where the story clock stands | 1 | 1 | 0 | 0 |
| Before | Start tonight's session note | 1 | 5 | 0 | 3 |
| Before | Compare the party's passive Perception | 1 | 6 | 1 | 1 |
| Before | Build tonight's encounter | 1 | 14 | 1 | 13 |
| Before | Copy the boss, change it, swap it in | 2 | 24 | 0 | 7 |
| Before | Write the read-aloud text and tactics | 1 | 14 | 0 | 3 |
| Before | Put the encounter in the session plan | 1 | 8 | 1 | 1 |
| During | Start recording | 0 | 2 | 0 | 2 |
| During | Read last session's recap aloud, then come back | 0 | 4 | 0 | 1 |
| During | A player asks who an NPC is | 0 | 3 | 0 | 3 |
| During | Note what the party decides | 0 | 3 | 0 | 3 |
| During | Record travel | 0 | 2 | 0 | 1 |
| During | Read the read-aloud text and start the fight | 2 | 15 | 1 | 12 |
| During | Combat: rounds 1 to 3 (24 turns) | 0 | 96 | 0 | 96 |
| During | Read a spell | 0 | 3 | 0 | 3 |
| During | Fireball on three enemies | 0 | 9 | 0 | 6 |
| During | Mark a condition | 0 | 3 | 0 | 3 |
| During | What does the condition do? | 0 | 3 | 0 | 2 |
| During | Mark Concentration | 0 | 2 | 0 | 2 |
| During | Check a player's spell save DC and spells | 0 | 3 | 0 | 3 |
| During | Roll a save for a monster | 0 | 5 | 0 | 3 |
| During | On a player's turn, read a monster's reaction | 0 | 3 | 0 | 2 |
| During | What did the miller say last session? | 2 | 9 | 0 | 3 |
| During | Note a moment beside the fight | 0 | 3 | 0 | 3 |
| During | Take a dead wolf out of the order | 0 | 2 | 0 | 2 |
| During | End the fight | 1 | 6 | 1 | 4 |
| During | Record a short rest | 1 | 3 | 0 | 1 |
| During | Start the next fight | 2 | 2 | 1 | 1 |
| During | Correct one hurt player's HP in the new run | 0 | 6 | 0 | 0 |
| During | Roll and begin the next fight | 0 | 11 | 0 | 11 |
| During | End the next fight | 1 | 3 | 1 | 3 |
| During | Start a fight you did not plan, then close it | 2 | 9 | 2 | 7 |
| During | Stop recording | 1 | 2 | 0 | 1 |
| After | Check the evening on the timeline | 1 | 5 | 0 | 3 |
| After | Process the recording, read the summary | 1 | 6 | 0 | 4 |
| After | Add a suggested event to the timeline | 0 | 1 | 0 | 1 |
| After | Write the wrap-up | 0 | 5 | 0 | 3 |
| After | Link the fight in the note | 0 | 2 | 0 | 1 |
| After | Review the fight: log and final HP | 3 | 3 | 0 | 1 |
| After | Check the party's HP for next time | 1 | 1 | 1 | 1 |
| **Total** | | **27** | **309** | **11** | **221** |

## 3. What works, and must stay

- **The live run.** A turn costs 4 interactions: select the target, type, Enter, Next turn. The amount field has
  focus, the rows stay in place, and the side panel follows the current turn. No round needed a page change.
- **Lookup and Notes beside the run.** Each costs 3 and keeps the roster in view. Words typed in Notes go under
  "What happened". Digits typed there do not become damage.
- **The strip on other pages.** Round, turn, Next turn and Back to run stay in view (`13-during-sessions-with-strip`).
- **Setup.** Roll NPCs, then type and Enter for each player: 11 interactions for 5 players.
- **Timeline.** A fight adds itself, with its outcome.
- **Reopening.** The app reopens on the last page or the open run.

## 4. Findings

### Defects

These cost trust at the table. Fix them first. None needs a design change.

| # | Defect | Evidence | Effect |
|---|---|---|---|
| D1 | **An encounter edit can be lost.** The Library keeps an old copy of a note after Encounters saves it. The Library then says "This note changed on disk", though the DM changed nothing outside Splat. **Overwrite with my draft** removes the roster change. | `dm_audit_stale_library`: the ninth roster row is gone after the overwrite. `04-before-encounter-note-saved` | Lost prep. A false alarm teaches the DM to ignore the real one. |
| D2 | **The Prep tab shows raw YAML.** After the read-aloud text comes the whole roster, with UUIDs (`src/encounters.rs:714`). | `07-during-setup-prep` | Noise at the moment the DM reads to the table. |
| D3 | **Outcome starts with the value "Undecided".** A click puts the caret inside the word, and typing goes there. The result was "UndecidThe party held the forded" in the title, the log and the timeline. A later script uses Ctrl+A, which a DM must know to do. | An earlier run of the script, and `src/encounter_run.rs:348` | A wrong permanent record. |
| D4 | **Current HP is both a checkbox and a number, with one name.** A click on the words sets HP to unknown, and Begin combat turns off. | The script hit this: `HP ?/41`. `src/encounter_run/view.rs:3180` | A one-click trap in setup. |
| D5 | **Copy refuses some entries, and the DM cannot fix it.** "This entry refers to itself." Seen on Werewolf. A text scan finds 9 built-in entries: ochre jelly, shadow, werebear, wereboar, wererat, weretiger, werewolf, black pudding, foresight. | `src/content_workspace.rs:769` | A dead end in prep. The scan is not a count from the app. |
| D6 | **Saved runs say Resume on an ended run, and give an abandoned setup the strongest button.** A closed setup stays as "Round 0 · Preparing". No control removes a run. | `21-after-saved-runs` | The list grows every session, and the wrong item draws the eye. |
| D7 | **Session Read shows `[[Ashfall Crossing]]` as plain text.** Only timeline links work. | `05-before-plan-read` | The plan cannot lead to the fight. |

### Load at the table

| # | Finding | Evidence |
|---|---|---|
| L1 | **The next fight forgets the last one.** Kestrel ended a fight on 7 HP and began the next on 11. The closed run held the true value. One correction costs 6, and each further player 4, so a whole party costs 22. The DM must also remember the numbers. Sync may cover this for a linked party, but I could not test sync here. | `summary.tsv`, `16-during-second-setup`, `17-during-setup-record` |
| L2 | **A live fight could not grow (fixed: see the update above).** No control added a participant to a run when this audit ran (`Command` in `src/encounter_run.rs`). Reinforcements are common at a table, and the domain language has phases for them. Today the only route is a new run, with all HP and initiative typed again. | `summary.tsv` |
| L3 | **No page holds "tonight".** The DM thinks in order: scene, fight, rest, scene. Those live on Sessions, Encounters and Timeline. 10 of the 12 transitions at the table are in the gaps between fights. | Path in section 2 |
| L4 | **Lookup cannot find what the DM wrote.** "miller" gave no note. Leaving the fight to read last week's note cost 2 transitions and 9 interactions, all overhead. | `12-during-lookup-notes` |
| L5 | **A fight ends on the wrong page.** Close run goes to Encounters, a prep page. The DM then needs Sessions to record a rest. | Path in section 2 |
| L6 | **Reading a condition takes a search.** The word Frightened on a row is not a link. The rule is 3 interactions away, and it is asked for often. | `summary.tsv` |
| L7 | **Area damage repeats.** Fireball on 3 targets costs 9. | `steps.tsv` |
| L8 | **The dice are a tab away.** The roll hides the stat block that the roll is for, and costs 5. | `steps.tsv` |
| L9 | **Current and selected can differ.** Reading a monster on a player's turn needs a click, often a tab, then a click back to the current row. | `steps.tsv` |

### Load in prep and wrap-up

| # | Finding | Evidence |
|---|---|---|
| L10 | **Read-aloud text has no field.** It lives in the note's Markdown, below the YAML, on another page. With D1, it cost 14 interactions. | `steps.tsv` |
| L11 | **Changing one monster crosses pages.** Library, copy, edit, back to Encounters, remove, add: 24 interactions, the largest single prep task. | `steps.tsv` |
| L12 | **A new session starts empty.** Its Recap is blank though the last note exists. Read shows six empty headings. The DM goes to Read and to Timeline to recover the thread. | `01-before-recap`, `13-during-sessions-with-strip` |
| L13 | **Party shows HP and AC only.** A question about everyone (passive Perception) means opening 5 sheets and remembering 5 numbers. | `02-before-party` |
| L14 | **The wrap-up takes 6 transitions for one question: what happened tonight?** To read a fight's log, the DM uses **Resume**, which reopens it as the open run. | Path in section 2 |
| L15 | **A customised entry ranks below the original.** "fireball" lists the built-in spell first and the campaign's own copy second. A DM who changed a spell reads the wrong text first. | `10-during-lookup` |

## 5. Design changes

The order is by value at the table. Each obeys `docs/ux-intent.md` section 5.

### A. Make Sessions the place for tonight (L3, L5, L12, L14, D7)

Rename the rail item **Session**. It opens on the session being played. It replaces the sequence of pages with
one list, and the DM works from top to bottom:

```
Session 2: The Crossing                         ● Recording 01:12   Stop
Last time  ▸ The miller said the toll collectors were not paid. Open: who burns the ropes?
────────────────────────────────────────────────────────────────────────
TONIGHT                                       │  NOTES
 ☐ Road to the ford        Travel             │  ## What happened
 ▶ Ashfall Crossing        8 · Start fight    │  They take the ford.
 ✓ Shrine of Cinders       Held the ford ▾    │
    Final HP · Log · Add to notes             │
 + Fight   + Rest   + Travel   + Day   + Event│
```

- **Start fight** goes straight to setup. **Close run** comes back here. Both caused transitions stay, and they are
  not navigation choices, because the DM does not have to think where to go.
- A fight has one row for its whole life: planned, live, ended. An ended row opens its log and final HP in place,
  with no Resume.
- A new session fills **Last time** from the previous note's "What happened" and "Open threads", and from the
  last timeline event. This replaces the trips to Read and Timeline.
- `[[An encounter]]` in a note is a link. This is how a plan written in Obsidian joins the list.
- Merge Write and Read into one view, and hide empty headings.
- **Encounters** stays as the workshop for all encounters. Saved runs move into it as history: **Resume** for a
  fight in progress, **View** for an ended one, and **Remove** for a run nobody wants (D6).
- Events edit in place here, so Timeline becomes the place to read the whole story and to reorder it.

Effect: every transition the DM chooses at the table goes away (6 to 0), and the total goes from 12 to 6.

### B. Keep the party's state from fight to fight (L1)

- During a session, a new run starts each player from the end of the last run. Setup says so in words:
  "HP from Ashfall Crossing", with one click to use the sheet's value.
- Offer **Everyone to full** beside a Long rest, as a choice. It is not automatic, so Splat still records rulings
  and does not make them.
- Show the same values on Party, marked as "from tonight's fights". Define how this works with sync on before
  building it, because sync has its own rules about who wins.
- `CONTEXT.md` already has the term: **encounter sequence**.

### C. Let a live fight grow (L2)

Add **Add to fight** beside Absent. It opens Lookup on creatures, and each result gets **Add**. The DM types an
initiative, and rows take their places without changing whose turn it is. Absent players already return this way.
The undo history and the log record it.

### D. Make Lookup find what the DM wrote (L4, L6, L15)

- Add session notes and timeline events to Lookup, and read them in the same panel. The miller question goes from
  2 transitions and 9 interactions to none and 3.
- Rank the campaign's own copy above the original, and mark it.
- Make each condition on a row a link that opens its rule. 3 becomes 2, with no typing.

### E. Remove prep's page changes (L10, L11, L13, D1, D2)

- Add a **Read-aloud and notes** field to the encounter editor. It writes the note's body. The DM does not see YAML.
- On a roster row, add **Make it yours**. It copies the creature to the campaign, swaps it in, and opens its text
  beside the roster: 24 to about 7.
- Show only the read-aloud text on Prep. Put the YAML in Technical details, closed.
- Show passive Perception, Insight and spell save DC as columns on Party.
- Start a new encounter with the party in it, as it is the usual case.

### F. Small changes to the run (L7, L8, L9, D3, D4)

- **Several targets:** Ctrl+click selects rows, and one Enter applies to each. A **½** chip on a row halves it for a
  save. The DM still decides each result. 9 becomes 6.
- **Dice in the dock**, beside Amount, so the stat block stays. 5 becomes 3.
- **Escape returns the selection to the current turn.** 3 becomes 2.
- **Outcome is a hint, not a value.** It is empty until the DM types, and the field has focus when it opens.
- **Current HP** has one control. A separate word, **Unknown**, sets it to unknown.

## 6. What the model predicts

| Phase | Transitions today | Model | Interactions today | Model |
|---|---:|---:|---:|---:|
| Before | 9 | 4 | 74 | 29 |
| During | 12 | 6 | 212 | 178 |
| After | 6 | 1 | 23 | 14 |
| Total | 27 | 11 | 309 | 221 |

96 of the interactions at the table are the turns themselves, which I did not change on purpose. Without them,
the table goes from 116 to 82.

## 7. Limits of this audit

- **The model numbers are hand counts** against a design that does not exist. Only "today" is measured.
- **One order of tasks.** A different order changes the counts, and prep split across two evenings adds transitions.
  The pattern holds, because the page pairs stay the same.
- **Sync and live D&D Beyond were not tested.** Test builds have no route to them. L1 may be smaller for a linked party
  with Sync party on. Whether the next run reads the sent values is not verified.
- **The recording pipeline is simulated.** Its steps are real, but no audio or service was used.
- **Counts do not measure time or thought.** A Find on a known name is cheap, and a Find from memory is not. I did not
  test this with a real DM at a real table. One observed session would check the priorities here.
- **`docs/ux-intent.md` section 2 describes the app before the redesign.** Its "Today" counts are out of date. Its
  section 8 tests are current.
