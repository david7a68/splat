# The session cycle

Status: proposed, round 3. Nothing here is built, except F8 (pull request 232) and where a line says "today".
This document fits nine features (F1 to F9) into the app as one workflow: prep at the desk, play at the table,
the wrap-up, and the next prep. It follows [the brief](../ux-intent.md), [the design
system](../design-system.md)
and the words in [`CONTEXT.md`](../../CONTEXT.md). The yardstick is the measured evening in
[`artifacts/dm-audit/`](../../artifacts/dm-audit/README.md): 13 page transitions and 231 interactions.

Evidence: `file:line` refers to `main` at `a4ca1e9c`, and to pull request 232 at `8d3d2de6` where it says so.
"Today" means that source. "New" means that this design adds it, and section 6 names the pull request.

## Decisions in brief

No feature adds a page, a panel, a tab, a strip or a dialog.

**Counts, by hand against `steps.tsv`** (section 2.1): the 41 measured tasks go from 13 page transitions and
231 interactions to 12 and 225, or 227 when the DM opens **More** twice on the Add strip. Seven records that
were on paper cost 15 more, and 2 more, once, when the campaign has several travel tables. The audit script
has drifted from `steps.tsv`, so P1a measures `main` again before any feature lands.

## 1. Rules for every feature

### 1.1 Eight rules

1. **No new page, panel, tab, strip or modal.** Each feature joins a row, a line or a view that exists. The one
   new view is Trackers, and it is a view of the Lookup panel.
2. **The plain evening is a contract.** A DM who uses no new control sees no new notice, count, heading,
   status, step or line. Two things are allowed. **Passive words** at the end of a line that exists: they
   answer the question that line already answers, want no answer, and do not show when their inputs are not
   known. **A quiet action** in a place that already holds actions for that thing, never primary.
   Both obey one **fit rule**: a new word or action takes only room that is spare today. It never adds a line,
   changes a row's height, or takes width that a title or a name uses. When there is no spare room, an action
   goes to a menu that exists in that place today; when there is no such menu, it does not show there.
   Section 2.5 lists every difference with its band in the source, and it is the full list.
3. **One act writes one file, as a patch of the file on disk now.** After the write, the fragment is plain
   Markdown or a frontmatter key in that file. Nothing outside the file remembers that the act happened.
   Section 4.1 has one row for each new write.
4. **Every act has a named Undo where the DM acted.** The run's Undo and the tracker ledger's Undo survive a
   restart. A change to a note has Undo until the next change of that note's text, with no time limit. After
   that the DM removes the words as any words. Section 4.1 holds the rule.
5. **An identity is an ID, a reference or an offset, never a label.** A session is its ID, a fight is its
   Timeline reference, a table is its key, a travel check is its offset from the journey's start. A name or a
   heading is for reading.
6. **One file owns each fact.** A second place shows it by reading the owner. Section 4.2 names each owner.
7. **Splat prompts. The DM rules** (principle 7). A count goes down, a notice shows, a roll is offered. Splat
   never removes an effect, spends a use, ends a fight, plans a fight or starts one by itself.
8. **`Next turn`, `Amount` and the current row never move, wait or lose focus** because of a feature. A prompt
   needs no action to go away.

**The session being played** is one term for four features. `docs/ux-intent.md` (W14) defines it: a session is
played when a fight is open, a recording is live, or the current session is not dated before today. Today the
test is inside `party_bar_shows` (`src/sessions/page.rs:1952-1958`), and `current_link` (`:1929`) uses a
narrower one (dated today). New in P1c: one function, `played_session`, holds W14's test and returns the
current session (`current`, `:1918`) or nothing. `party_bar_shows` and `current_link` call it, and so do the
ledger's Undo (F4), both Log stamps (F6) and `Add to Timeline` (F4).

### 1.2 The side panels: today's rule, and the rule for features

| Thing | Opens with | Rule today | Evidence |
|---|---|---|---|
| **Lookup** | Ctrl+K, a link, a condition word | One panel beside the page at a time. When Lookup opens, Notes give way, and they come back when Lookup closes. Ctrl+K with Lookup open closes it, and Lookup keeps its search words and its reader | `src/lib.rs:1229-1238`, `:2457-2460`; `src/lookup.rs:1575-1582` |
| **Notes** | Ctrl+J | When Notes open, Lookup closes. The Session page shows no Notes panel, because it has the note | `src/lib.rs:1240-1243`, `:2469` |
| **The run's own panel** (Stat block, Conditions, Correct, Log, Prep, Arrivals) | A row, `C`, **Add to fight** | Part of the Run page. It stays beside Lookup or Notes only when the roster keeps 560 px. With less it hides, and the current stat block's action tiles go in one line above the dock | `src/lib.rs:1888-1892`; `src/encounter_run/view.rs:665`, `:924-932` |
| A run task the DM asks for (Conditions, Correct, Arrivals, Log, Keys, Review) | A click or a key | Lookup closes. **Review** also selects the conflict's row. **Add to fight** opens Arrivals and closes Lookup | `src/lib.rs:1915-1925`; `src/encounter_run.rs:2433-2441`; `src/encounter_run/ad_hoc.rs:242-245` |
| Keys in a run | | A field with focus takes its own keys. With Lookup or Notes open, the letter keys are off, and Enter and the arrow keys work only while `Amount` has focus. A button shows its key letter only while the key works | `src/encounter_run/view.rs:405-419`, `:444` |

**The rule for F1 to F9: no feature adds a contender for this space, and no feature adds a key rule.**

- A feature that needs room beside a page is a view of Lookup. There are two: an entry's reader (F3, F6) and
  **Trackers** (F4).
- **Lookup's history holds places** (new, P1c). Today it holds entries only (`src/lookup.rs:1662-1666`). A
  place is what Lookup shows: an entry, a note or an event found by search (`written_reader`, `:2212`), or
  the Trackers view. So Back and Ctrl+T return to any of them.
- **Ctrl+T goes there and back.** With Lookup closed, it opens Lookup on Trackers, and a second press closes
  Lookup. With Lookup on an entry, it goes to Trackers, and a second press goes back to that entry.
- **Keys beside Trackers are the keys beside any Lookup view**, as the table says. A tracker's typed amount
  holds focus until Enter or Escape. `U` is the run's key and never takes back a tracker change.
- A feature opens Lookup only when the DM clicks or presses a key, and it uses Lookup's own open path
  (`open_position`, `src/lookup.rs:1619`). No feature closes Lookup, except where a row above says that the
  same act closes it today.
- **One repair.** While Lookup holds an unsaved generated NPC, its card hides any entry that opens
  (`src/lookup.rs:2167-2169`). Lookup takes the World page's rule: one row above the reader says
  `Unsaved NPC: <name> · Show`, and the opened entry shows below it.
- **The reader's header** is one row: the title, and for an entry of the campaign `Edit`, one `L` after the
  measured title (`src/world.rs:1733-1768`). Lookup and the World page draw this one reader
  (`src/lookup.rs:2195-2203`); the Library page has its own header and does not change. `Add to Log` (F6)
  and `Start fight` (F3) follow `Edit`, as quiet actions. The title keeps the width that it has today. An
  action shows its word while the row has room; then its icon only, as an `icon_button` with its name on
  hover (the component of a plan row's `More actions`, `src/sessions/page.rs:4380`); then it does not show,
  and the DM has today's route. `Start fight` gives way first. In Lookup at case D (about 322 px inside)
  both icons show while the title's widest line has up to about 13 letters; a wider line leaves no room
  (T18.2).

### 1.3 Widths, and what never hides

The widths come from `RAIL_WIDTH` (136), `side_panel_width` (36% of the window, between 340 and 560) and the
Session page's `PLAN_BESIDE_NOTE_FROM` (720) and `TWO_COLUMNS_FROM` (640). Each wireframe names a case.

| Case | Window | Lookup | Page width | Session page | Run page |
|---|---|---|---:|---|---|
| A | 1200x800 | Closed | 1040 | Session list, plan (330) and note side by side | Roster 596, run panel 432 |
| B | 1200x800 | Open (432) | 608 | No session list. **Plan** and **Note** are tabs | Roster 608. Run panel hidden. Tiles above the dock |
| C | 960x640 | Closed | 800 | Session list. **Plan** and **Note** are tabs | Roster about 446, run panel 346 |
| D | 960x640 | Open (346) | 454 | No session list. Tabs. The title has its own line | Roster 454. Run panel hidden. Tiles above the dock |

The party bar is a band above the page and above Lookup, so Lookup does not narrow it (`src/lib.rs:1492`).

| Never hidden, in any case | Where it is | What gives first |
|---|---|---|
| Each reminder for the current turn (F2) | The current row | Section 1.4 |
| That something is due (F4) | The rail's `Due N`. In a run, the bell at the end of the notice line | Section 1.4 |
| A scene's done box (F3) | The start of its row | The words wrap, as a plan row's title wraps today |
| `Undo` in a run | Run header, line two, right | Section 1.4: the feature parts give way first |

### 1.4 The run's notice line and a row's effect words

**The notice line** is the left part of the run header's second line (`live_header`,
`src/encounter_run/view.rs:955`); `Undo` is at its right. Today it holds a state notice or `Next: <name>`,
and then the sheet-change summary. The features add parts 3 to 5. Parts show left to right in this order.

| # | Part | Shows when | Words |
|---|---|---|---|
| 1 | As today: a state notice (a sync conflict first, recording silence, another sync notice, `No participants are present`; `view.rs:1090`, `:1109`), or else `Next: <name>` | As today | As today (`Conflict: <name>`, `:1041`) |
| 2 | As today: the sheet-change summary with its `Log` link (`:1183-1199`) | As today | As today |
| 3 | The current turn's reminders (F2) | The current participant has an effect with a time | `⚑ Wis save DC 15 (Hold Person) · Bless 9`, or the count `⚑ 3`, whose hover and accessible name hold each whole |
| 4 | A new Due notice (F4) | For one turn: from the act that raised it to the next `Next turn` | `Due: Round 2 · Fight minute` and its action, `Done` |
| 5 | The bell (F4) | While a notice is due | `🔔 Due 1`, a link to Trackers. Its shortest form is `🔔 1` |

**When the line is short,** the parts give way in this order. The bell never leaves.

1. Part 2 goes below the roster, and `Next:` goes: today's rules.
2. Part 3 becomes its count.
3. Part 5 takes its shortest form.
4. Part 4's words shorten from their end (`design::words_that_fit`), down to `Due:` and one word. `Done` stays.
5. `Undo` shortens its target, then drops its key letter, then is the bare word: today's rule
   (`docs/design-system.md`, the live run's header; `view.rs:1154-1168`).
6. Part 4 leaves. The notice is in Trackers.
7. Part 3's count leaves. The current row has each reminder.
8. The name in part 1 shortens as the dock's name does (`dock_name`, `:6108`). The icon, `Conflict` and
   `Review` stay.

- **`Undo` and the features.** The notice is measured first and `Undo` takes the rest, as today. Steps 2 to 4
  come before step 5, so on a turn with a reminder `Undo` still names its step. One cost is stated: beside a
  state notice at case D, a reminder's count and the bell take the room in which `Undo` keeps a short target
  today, so it is the bare word, with its full name on hover.
- **No part asks for an answer.** Part 4's `Done` is optional; the notice waits in Trackers.
- **The ended view has no notice line.** There the rail's `Due N` is the sign.
- **Sums at case D** (454 px; estimates that T17.4 tests). With a conflict on `Kestrel Dunmoor` (about 246
  px), `⚑ 3` (40), `🔔 1` (40) and two 12 px gaps: 350 px. `Undo` has 88 px after its 16 px gap and is the bare
  word (about 70). With no conflict, `Undo: next turn` takes about 152 px, and the line has about 286:
  `Due: Round 2 · Fight…  Done` (about 180), `⚑ 1` and `🔔 1`.

```text
Case D, 454 px: a conflict, three reminders, one notice
┌──────────────────────────────────────────────────────────┐
│ Round 3 · Cinder Wolf 2                    [ Next turn N ]│
│ ⚠ Conflict: Kestrel Dunmoor  Review · ⚑ 3 · 🔔 1     Undo │
├──────────────────────────────────────────────────────────┤
│▶ Cinder Wolf 2                                   HP 11/26 │
│  Wis save DC 15 (Hold Person) ·                           │
│  Wis save DC 13 (Fear) ·                                  │
│  Wis save DC 17 (Banishment) · Poisoned 2                 │
└──────────────────────────────────────────────────────────┘
```

**A row's effect words.** They sit on the row's second line with its conditions and `Conc`, as today. That
line wraps before the HP column (`view.rs:6446`): it has about 267 px at case D.

| Rule | What it means |
|---|---|
| An effect with a time comes before a plain condition, and it is never folded into `N more` | Plain conditions fold first, as today |
| Each timed effect shows whole. The row takes the lines that its timed effects need | The wireframe: three reminders that start with the same words take three lines, and no reminder is cut |
| A row's height comes from its own effects, never from being the current row | No row grows or shrinks at `Next turn` |

## 2. The evening, end to end

Each journey uses the step fields of `artifacts/workflow-score/journeys.md`. **Count** uses the kinds of the
audit: `Nav` (go to a page), `Sub` (a tab in a page), `Panel` (open or close a side panel), `Find` (search or
select), `Do` (the work). A page transition is marked **T**. "Must remember" is **nothing** unless a row
says so. Every count is a hand count.

### 2.1 J1: the measured evening

Window 1200x800 (case A, and case B while Lookup is open). `steps.tsv` has 41 tasks. Thirty-seven keep their
actions and counts. F1 only adds its word to lines that the DM reads in tasks 5 and 8. These four change:

| `steps.tsv` task | Surface | Action after | Count after | Before | File |
|---|---|---|---|---:|---|
| A player asks who Captain Varra is | Session plan, the scene's row; Lookup opens (case B) | Click `Captain Varra` in the row; close Lookup | Panel 2 | 3 | None |
| A wolf frightens Oren for 10 rounds (F2) | Run, dock | Select Oren; `E`; type `Frightened 10 rounds`; Enter | Find 1, Do 3 = 4 | 3, with no count | The run file: the condition with its count |
| A fight nobody planned, with a Bandit from the book, then talk (F3) | Session, Lookup, Run | Ctrl+K; type `bandit`; `Start fight`; `Close run`; close Lookup | Panel 2, Find 1, Nav 2 = 5, **T T** | 9, **T T T** | An encounter `Fight with Bandit` stays in **Ready to run**, as the measured task's encounter stays today |
| To link the fight in the note | | Nothing. `Add rewards` writes the fight's link | 0 | 2 | |

- **Why the second row costs one more:** the DM types the number of rounds, and no design can take a number
  with no interaction. A DM who wants no count uses **Conditions** as today, for 3.
- **The stray encounter** of the third row can go with `Delete encounter`: 2 more, on the Encounters page.
  Today it cannot go in Splat at all.

**Seven records that were on paper, or had no route.**

| The DM wants | Surface | Action | Count | File |
|---|---|---|---|---|
| **F5.** What the last session left | Session, the **Last time** line | `Carry over` | Do 1 | The new note: one embed line, the plan lines, the open threads, `carried_from`, `carried_parts` |
| **F3.** To plan a talk with Captain Varra and a secret | The note beside the plan | Click under `## Plan for tonight`; type `- [ ] [[Captain Varra]]`; type `- [ ] The miller is the informer` | Do 3 (by the pointer, with `Scene` in **More**: 7) | The note: two task lines. The plan shows two scenes |
| **F3.** To mark the talk done | The scene's row | Click the box | Do 1 | The note: `- [x] [[Captain Varra]]` |
| **F9.** To check for an encounter on the road | The Travel row | The row's menu; `Roll for encounters`; `Roll` | Find 1, Do 2 = 3. With several marked tables and no earlier choice, 2 more, once | `Timeline.md`: the table's key and one check line |
| **F7.** The first fight's rewards, with XP | The played fight's row | `Add rewards`; `Add XP` on the result, because the notes have no rewards yet | Do 2 | The note: a block under `## Loot and rewards` |
| **F7.** The second fight's rewards | The played fight's row | `Add rewards`. It follows the first block | Do 1 | A second block. The result says `XP tonight` |
| **F6.** To put what Varra said in her note | The done scene's row; Lookup opens (case B) with the Log field open | `Add to Log`; type; Enter; close Lookup | Panel 2, Do 2 = 4 | `NPCs/Captain Varra.md`: one line under `## Log` |

Both fights' titles leave room for `Add rewards` on the row at 330 px. A long title costs 1 more: open the row.

| J1, by hand against `steps.tsv` | Transitions | Interactions | Nav | Sub | Panel | Find | Do |
|---|---:|---:|---:|---:|---:|---:|---:|
| `steps.tsv`, 41 tasks | 13 | 231 | 14 | 7 | 12 | 49 | 149 |
| The same tasks, after | 12 | 225 | 12 | 6 | 14 | 47 | 146 |
| The same, when the DM opens **More** before `Travel` and before `Short rest` | 12 | 227 | 12 | 6 | 14 | 49 | 146 |
| Seven new records | 0 | 15 | 0 | 0 | 2 | 1 | 12 |
| **After, in all** | **12** | **240, or 242** | 12 | 6 | 16 | 48 or 50 | 158 |

- **The More clicks.** At 330 px the Add strip does not show `Travel` or `Short rest`; `steps.tsv` counts each
  as one click. The app of today needs the same two clicks, so they are not a cost of this design.
- **Overhead for the measured tasks:** 82 before; 79 after, or 81 with the More clicks. **Table time:** the
  180 measured interactions become 176.
- **The audit script has drifted from `steps.tsv`.** Its task for a played fight clicks an `Add to notes`
  that a played fight has not had since pull request 39 (`src/sessions/page.rs:4388`). So this document
  gives no total for `main`. P1a repairs the script, freezes it, and measures `main`.

### 2.2 J2: the improvised evening

Only the steps where a feature acts, at case A. Setup and turns cost what they cost today on both sides.

| # | The DM wants | Surface | Action | Count | Today | File |
|---|---|---|---|---|---:|---|
| 2 | **F3.** To play the planned NPC | The scene's row | Click `Captain Varra`. Lookup opens on her page | Panel 1 | 2: Ctrl+K, type | None |
| 3 | **F3.** The party attacks her | Lookup, her reader; then Setup | `Start fight`; close Lookup, to see her stat block | Nav 1, Panel 1 = 2, **T** | 8, **T T** | An encounter note `Fight with Captain Varra`, and the run file, as the editor's `Start fight` saves them today |
| 4 | **F6.** She gave up, and the party holds her | Ended view; Lookup | Ctrl+K, which shows her page again; `Add to Log`; type; Enter; close Lookup | Panel 2, Do 3 = 5 | 6: Ctrl+K, `Edit`, click the text, type, `Save changes`, close | `NPCs/Captain Varra.md`: one Log line |
| 5 | **F7.** To close, keep the rewards, and add loot nobody planned | Run, then the plan and the note | `Close run`; open the row (its title is long); `Add rewards`; click under the new block; type `Varra’s signet ring` | Nav 1, Find 1, Do 3 = 5, **T** | 4: Notes, type, close Notes, `Close run`. The words go under another heading | The note: the fight's block, and one line under it |
| 6 | **F9.** Two days overland | The Travel row | `More Add actions`; `Travel`; the row's menu; `Roll for encounters`; click **How long**; type `2 days`; `Roll all 2` | Find 2, Do 5 = 7 | 12, **T T** | `Timeline.md`: the length, the table's key and two check lines |
| 7 | **F9.** The second day's result names Wolf Pack | The Travel row, the plan, then Run | `Plan “Wolf Pack”`; `Start fight` on the new row, which is the first to come and the primary | Do 1, Nav 1 = 2, **T** | 3, **T** | The note: `- [[Wolf Pack]]` above the skipped fight. The run file |

| J2, these steps | Transitions | Interactions | Nav | Panel | Find | Do |
|---|---:|---:|---:|---:|---:|---:|
| Today | 6 | 35 | 6 | 8 | 7 | 14 |
| After | 3 | 22 | 3 | 4 | 3 | 12 |

- **Today's 35 leaves a smaller record.** The loot is prose under another heading, the second road roll
  replaces the first on screen, and neither roll is saved.
- **W7.** The fight that nobody planned costs one click, as a planned one does, and leaves the same record.
- **A wrong click** on `Start fight` costs `Close run`: 1. The encounter stays in **Ready to run**, and
  `Delete encounter` removes it for 2 (F3).
- **Step 3 with no stat block** costs 3: `Start fight`, her HP, Enter (F3).

### 2.3 J3: the broken evening, at 960x640 with Lookup open (case D)

The start: fight 2, round 2. Lookup shows a session note found by the search `miller`. Sync is on. A tracker
has one notice due. The current and selected row is Cinder Wolf 2.

| # | What happens | What the DM sees | Recovery | Count | File |
|---|---|---|---|---|---|
| 1 | The DM ends the fight by mistake | The ended view, 454 px wide. The header says `Undo`; its hover and accessible name say `Undo: end encounter`. The rail's `Due 1` is the sign of the notice | Click `Undo`. `U` is off while Lookup is open. Play returns to the same turn | Do 1 | The run file is at round 2 again |
| 2 | The DM pins `Poisoned 3 rounds` to the wolf (`Effect`, type, Enter). The app stops in round 3 | After a restart the app opens on the run, on the current turn. The row says `Poisoned 2`. `Undo` names the last saved step. The line ends with the bell, read from the ledger | Nothing for the run | 0 | The run file holds the effect, its count and the Undo steps |
| 3 | Lookup is closed after the restart (a limit, section 7) | The run, with no Lookup | Ctrl+K; type `miller` | Panel 1, Find 1 = 2 | None |
| 4 | A sync conflict opens on Kestrel Dunmoor. The wolf's reminder is due, and the notice is still due | The notice line: `⚠ Conflict: Kestrel Dunmoor  Review · ⚑ 1 · 🔔 1`, and `Undo` (section 1.4). The wolf's row says `Poisoned 2` | `Review`; choose. **Review** closes Lookup and selects Kestrel, as today. Then: select the wolf; Ctrl+K, which shows the note again (1 more when Lookup returns to its result list) | Do 2, Find 1, Panel 1 = 4 | The run file. The ledger does not change |
| 5 | To finish the notice | The bell, then Trackers at its Due row | Click `🔔 1`; `Done`; Ctrl+T, back to the note | Panel 2, Do 1 = 3 | `Trackers.md`: one `done` entry. After a restart here, `Undo: Bell done` is on the result line |
| 6 | The DM edits the session note in Obsidian: a tick, a new plan line | On the Run page nothing opens. Splat held no unsaved words, so it takes the file's text within a second | Nothing | 0 | The note is as Obsidian wrote it |
| 7 | The same, while Splat holds unsaved typed words for that note | The rail says `Notes changed`. A plan act says `The note changed on disk. Choose in the note first.` and does nothing | Ctrl+J, which closes Lookup; one choice; Ctrl+K. It keeps one version of the lines that both sides changed (a limit of today) | Panel 2, Do 1 = 3 | The note holds the chosen version |

| J3, recovery | Transitions | Interactions | Panel | Find | Do |
|---|---:|---:|---:|---:|---:|
| Steps 1 to 4 and 6, the clean case | 0 | 7 | 2 | 2 | 3 |
| Today with pull request 232, the same steps | 0 | 7 | 2 | 2 | 3 |

Today has no count on `Poisoned`, no reminder and no bell. Steps 5 and 7 add 3 each. Nothing in this journey
is modal, and nothing takes focus from `Amount`.

### 2.4 J4: the race

The Tournament of [the example](world-examples/heralds-tournament.md), as one whole walk. P10 makes two
additions to the example's note: `Fight minute` becomes a mark with `every: round`, and the bells' rival
table gets `for:`. The start: Session page, Lookup closed, the clock at 30, Favor 2, activation 29. The fight
is prepared. During the fight Lookup shows the operator's stat block. **Today** is the same walk with the
paper sheet: each mark, tick, note or erasure on paper is one act (**p**).

| # | The DM wants | Action in the design | Count | Today |
|---|---|---|---|---|
| 1 | To open the numbers | Ctrl+T | Panel 1 | 0. The sheet is on the table |
| 2 | The party enters a room. The clock reaches 40, and a bell is due | `Enter room` | Do 1 | 1 p |
| 3 | To roll for each rival and for wandering | On the Due row: `Roll Rival Track` three times; `Roll Wandering` | Do 4 | 4 p |
| 4 | To finish the bell | `Done` | Do 1 | 1 p |
| 5 | The Old Guard takes its first seal | `Take seal` on the Old Guard's line. A notice is due: a rival uses its group | Do 1 | 1 p |
| 6 | The party touches the Ring's controls | `Touch controls`. Activation reaches 30, and a notice is due | Do 1 | 1 p |
| 7 | The party enters the arena | `Enter room` | Do 1 | 1 p |
| 8 | To start the fight and read the operator | Ctrl+T; `Start fight`; Setup, 11; Ctrl+K; type the stat block's name | Panel 2, Nav 1, Find 1, Do 11 = 15, **T** | 14, **T** |
| 9 | The Ring opens on the operator's next turn | Select the operator; `Effect`; type `Ring opens 2 rounds`; Enter. Must remember: a count starts with the current turn | Find 1, Do 3 = 4 | 1 p |
| 10 | To play round 1 | `Next turn` six times. On the operator's turn the notice line says `⚑ Ring opens · last round` (a count at case D; the row has the words) | Do 6 | 6 |
| 11 | One minute passes each round | The notice line: `Due: Round 2 · Fight minute  Done`. Click `Done` | Do 1 | 1 p |
| 12 | The crowd loves a move, then throws a gift | Ctrl+T; `Crowd loves it`; `Throw a gift`; Ctrl+T, which shows the stat block again | Panel 2, Do 2 = 4 | 2 p |
| 13 | The rival group arrives | `Add to fight`; `Add` on the group in Arrivals; Ctrl+K, which shows the stat block again: today's route | Sub 1, Do 1, Panel 1 = 3 | 3 |
| 14 | A wrong press on `Throw a gift`, then a right one | Ctrl+T; `Throw a gift`; `Crowd loves it` | Panel 1, Do 2 = 3 | 2 p |
| 15 | To take back the wrong press, seen one press late | `Undo: Crowd loves it`; `Undo: Throw a gift`; `Crowd loves it` | Do 3 | 1 p |
| 16 | To read the stat block again | Ctrl+T | Panel 1 | 0 |
| 17 | `Collect soul` takes the clock past 60: the second bell. The Ring's and the seal's notices are also due | Ctrl+T; `Collect soul`; three rival rolls; the wandering roll; `Done` on the bell, on the Ring's notice and on the seal's notice; Ctrl+T | Panel 2, Do 8 = 10 | 8 p |
| 18 | To go on | `Next turn`. The operator's row now says `Ring opens · time is up` | Do 1 | 1 |
| 19 | To remove the reminder | Click `time is up` on the row | Do 1 | 1 p |

| J4, the whole walk | Transitions | Interactions | Nav | Sub | Panel | Find | Do |
|---|---:|---:|---:|---:|---:|---:|---:|
| Today: Splat, and 25 acts on paper | 1 | 49 | 1 | 1 | 2 | 1 | 19, and 25 p |
| After | 1 | 62 | 1 | 1 | 10 | 2 | 48 |

- **The design costs 13 more acts on this walk.** Eight are Ctrl+T presses between the stat block and
  Trackers (steps 1, 8, 12, 14, 16 and 17): Trackers is a view of Lookup by ruling, so the two share one
  panel. With Trackers left in view they go, and the stat block is out of view. Three are the reminder's
  typed words against one note on paper (step 9). Two are the repair (step 15): the ledger's Undo steps back
  in order, and paper erases one mark.
- **What they buy:** Splat raised the bells and the notices, not memory; each roll names its rival; every act
  is in `Trackers.md`, with Undo after a restart.
- **Each tracker act costs one press, as its paper mark costs one mark:** 25 presses against 23 marks; the
  two more are the repair. `Take seal` is one press on the instance's line (step 5).
- **At case D the walk is 65.** The example's pinned trackers fill the view (about 480 px of the area's 460).
  So three steps cost 1 more each: in steps 3 and 17 a click on the heading's `Due N` brings the Due rows
  into view, and step 6 needs a scroll to the last pinned tracker. A DM who pins the tracker in use first
  saves the scroll.

### 2.5 J5: the plain evening, and the contract

The DM of J1 uses no new control. The audit's script runs unchanged and must give the counts that it gives
before each feature (G6). One frozen plain campaign must give captures that differ from the reference only
by the rows below (G7, section 5.3).

**The contract** (rows 1 to 10, 12 and 13). Each row is passive words or a quiet action (rule 2). Each names
the band that it joins
today and its fit rule. No row adds a line, changes a row's height or takes width that a title or a name uses.

| # | Screen | What differs | Band today | Fit rule |
|---|---|---|---|---|
| 1 | A planned fight's row, on the plan and in **Ready to run** | The summary ends with one word, such as `· Low` (F1), when the XP, the levels and the budget table are known | The summary: `src/sessions/page.rs:4447`; `src/encounters/list.rs:553-558` | Only when the summary keeps its lines and words |
| 2 | An opened planned fight | The line of `Read encounter` ends with `· Low · XP 550 of 750` (F1) | `page.rs:4484-4487` | The line holds one link today. The numbers go first, then the word |
| 3 | The encounter editor | `· XP 550 of 2,500 · Low` (F1) at the end of the `Roster` heading, or of the tab row below 760 px | `src/encounters.rs:4381`; `:3932-3955` | One line. The numbers go first, then the word |
| 4 | Arrivals | A group's XP is followed by `→ Moderate` (F1) | `src/encounter_run/view.rs:3468-3520` | Only where the totals and the status words are in the heading today (`:3479-3492`) and stay there |
| 5 | A row marked `Conc` that takes damage | The row's second line ends with `Conc save DC 10` until the next step (F2) | `view.rs:6476-6486` | Only when the line keeps its lines and folds nothing that shows today. Else in the hover and accessible name only |
| 6 | Run dock, line two | A quiet `Effect`, left of **Conditions** (F2) | `view.rs:2073-2150` | F2: the word, then the icon, then not shown, before the name loses a word or its size |
| 7 | The Add strip's **More** menu | Its last line is `Scene` (F3) | `src/design/components.rs:2339`; `src/timeline/page.rs:89` | A menu line. The strip's visible actions are as today |
| 8 | A campaign World entry's reader | `Add to Log` after `Edit` (F6) | The header: `src/world.rs:1733-1768` | Section 1.2: the word, then the icon, then not shown |
| 9 | A creature's or an NPC's reader, with no run open | A quiet `Start fight` after `Add to Log` (F3) | The same header. A package entry's header holds no action today; there it is the first | The same. It gives way before `Add to Log` |
| 10 | An ended fight's plan row | `Add rewards` (F7) | The action band: `page.rs:4344-4346`; the opened row's links: `:4918-4926` | On the row only when the title keeps its lines. Else the first link of the opened row |
| 12 | A Travel row's menu | One more line, `Roll for encounters`, when the campaign has a marked table (F9) | `page.rs:4379-4387`; `src/timeline/page.rs:875-877` | A menu line |
| 13 | A **Ready to run** row's menu | One more line, `Delete encounter` (F3) | `src/encounters/list.rs:560-565` | A menu line |

**Ruled exceptions and repairs.** These are not of the two kinds. Each comes from a ruling, and each is a row
that G7 allows.

| # | Screen | What differs | From |
|---|---|---|---|
| 11 | The **Last time** line | `Carry over` at its end, while the last session holds a part to carry (F5). The preview is cut to its room today (`page.rs:4106-4115`), so the action takes about 90 px from the preview, which stays whole on hover | Rule H of round 1: carry-over lives on this line |
| 14 | The plan, when the DM's note has a task line under `## Plan for tonight` | The line is a scene: a row with a box, counted in **Tonight**, and not repeated in Read (F3). The measured evening has none | The ruling on scenes: it is the DM's own plan |
| 15 | The **Conditions** tab's `Other effect…` field | Words that end with a time, a count of uses or a recharge are read as F2 says: `Bless 10 rounds` counts down, and `Frightened 10 rounds` becomes that condition with a count. The field, its label, its `Add` button and its place are as today | Round 3: the field stays, so `Effect` has a way when the dock has no room |
| 16 | The **Last time** line | Its preview reads the last session's `What happened`. Today it reads that note's own recap first (`page.rs:915`) | Rule H: the preview and the carry-over read one source |
| 17 | Lookup, while it holds an unsaved generated NPC | An entry that opens shows under one row, `Unsaved NPC: <name> · Show`. Today the card hides it (`src/lookup.rs:2167-2169`) | The accepted repair of Lookup |
| 18 | Read, for an embed of a session note's heading | The heading's text shows under a muted line that links the session (F5). Today Read shows the embed as a link (`src/world.rs:380-390`) | Round 2, E30: draw by the target |
| 19 | A **Change** of status, place or group | It writes its Log line when a session is being played by W14's test. Today the test is "dated today" (`page.rs:1929-1937`). So a change during a fight or a recording on a note with another date, and a change in prep for a session dated later, now write a line | Rule M: one definition of the session being played |

Nothing else differs. A free-words effect from an encounter or an older run gives no reminder. A Travel row
shows no checks and no new field until the DM asks. The rail says nothing about trackers unless a notice is
due. The empty plan's words (`page.rs:4545`), the run's **Keys** list and the loot editor's table menu are as
today. **Campaign → Rules and runs** lists house rule notes only when
the campaign has one. No file changes that the DM did not act on. F8 is the app's baseline once pull request
232 merges.

### 2.6 J6: the week between

The campaign's notes already use XP, and the last session's note has words under `What happened`.

| # | The DM wants | Surface | Action | Count | Today | File |
|---|---|---|---|---|---:|---|
| 1 | The party searches the last fight | Run, ended view | `Roll all 2` | Do 1 | 1 | The run file |
| 2 | Verori died here | Ended view, then Lookup | Ctrl+K; type `verori`; **Change** on Status; type `dead`; Enter | Panel 1, Find 1, Do 3 = 5 | 5 | `NPCs/Verori.md`: the status, and `- [[Session 12]], Day 3: Status alive → dead.` |
| 3 | **F6.** He gave the gate key away first | Lookup, his reader | `Add to Log`; type `Gave the gate key to Iselle.`; Enter; close Lookup | Do 3, Panel 1 = 4 | 5: `Edit`, click the text, type, `Save changes`, close | One more Log line, with the same stamp |
| 4 | To go back to the evening | Run, then Session | `Close run` | Nav 1, **T** | 1 | None |
| 5 | **F7.** Rewards for each of three fights | The played rows | `Add rewards` three times. Each follows the newest block, which has XP | Do 3; 1 more for each title with no room | No route | The note: three blocks, each with its XP line |
| 6 | A week later: a new session | Session | `New session`; title; `Create session` | Do 3 | 3 | A new note |
| 7 | **F5.** What the last session left | **Last time** line | `Carry over` | Do 1 | By hand | The new note: the recap embed, the plan lines, the open threads, `carried_from`, `carried_parts` |
| 8 | **F3.** Three scenes | The note beside the plan | Click under `## Plan for tonight`; type three task lines | Do 4 | 4, with no rows | The note: three `- [ ]` lines. The first key ends `Undo carry over` |
| 9 | **F1.** Is each of the two fights right for this party? | Session plan | Read the word on each row; open a row whose summary has no room | 0 to 2 (`Find`) | By hand, with the rule open | None |
| 10 | To start | Session plan | `Start fight` | Nav 1, **T** | 1 | The run file |

Totals: **2 transitions and 23 to 25 interactions (Nav 2, Panel 2, Find 1 to 3, Do 18).** Today's app has no
route for steps 5, 7 and 9; the steps that it has cost 20 and leave less record. The carried HP of today
still starts the fight (T12.2). Nothing is typed twice. After step 2 the reader and any scene row that links
Verori say `dead`.
## 3. The features

Each part has the same order: the question, the place, the wireframes, the exact words, the states, the file,
Undo, keys, what stays with the DM, and what is not built. A table at the end gives what a reviewer checks on
paper. Every placement follows the fit rule (rule 2), every write follows section 4.1, and **a state that a
table does not list is as today**: sync, a run that is open, a file that changed on disk or cannot be saved.

The examples of F1 use one party: 5 characters at level 3. Their budgets are Low 750, Moderate 1,125 and High
2,000.

### F1. Encounter difficulty

**Question.** "Is this fight too much, or too little, for this party?"

**Where the numbers come from.** The rule entry
`dnd2024:rules/combat_encounters_step_2_determine_your_xp_budget` has a typed table at
`splat_rule/parameters/per_character_xp`, with `level`, `low`, `moderate` and `high` for levels 1 to 20. Splat
reads it from the rules that the campaign uses, as it reads the inspiration rule
(`src/encounter_settings.rs:421`). No table is written in Rust.

**One computation** (new module `src/difficulty.rs`, one function, every screen calls it).

- **The party:** the characters on the party's side in the roster; in a run, those who are not absent. A
  roster with no character uses the campaign's party. A level comes from the saved sheet.
- **The budget:** for each difficulty, the sum of the table's value for each character's level.
- **The XP:** the sum for every creature that is not on the party's side, from the function that groups use
  today (`creature_xp`, `src/encounters/groups.rs:535`).
- **The word:** `Low`, `Moderate` or `High` when the XP is at most that budget, and `Above High` for more.
- **When.** The answer is kept with the revisions it came from (the encounter note, the saved sheets, the rule
  entry) and is computed again when one of them changes, never each frame.

**Where it shows.** Passive words at the end of a line that exists.

| Place | The line with F1 |
|---|---|
| Encounter editor, 760 px or wider: the `Roster` heading | `ROSTER 8 · XP 1,050 of 1,125 · Moderate` |
| Encounter editor, narrower: the tab row, right of its two tabs | `Roster 8 │ Add participants      Moderate · XP 1,050 of 1,125` |
| A planned fight's summary, on the Session plan and in **Ready to run** | `5 party · 2 × Ember Cultist · Low` |
| The opened planned fight: the line of `Read encounter` | `Read encounter · Low · XP 550 of 750` |
| A group's line in Arrivals | `Enemies · 2 members · XP 400 → Moderate` |

Rows 1 to 4 of section 2.5 name each band in the source and its fit rule.

- In the editor, `of 1,125` is the budget of the word. For `Above High` it reads `XP 2,400 · Above High`.
- **Hover** on the word, and its accessible description, give the rest: `Budget for 5 at level 3: Low 750 ·
  Moderate 1,125 · High 2,000`. For mixed levels: `for 5 at levels 3 to 4`. With no character in the roster:
  `for the party of 5 at level 3`.
- In Arrivals the word is what the fight becomes when the group joins. Its accessible name says `adds 400 XP;
  the fight becomes Moderate`.
- **A plan row with no room** (case A, two kinds of creature) shows no word. The DM opens the row, by a click
  or Enter on its title, and the word ends the line of `Read encounter`. That is 1 `Find`, with the keyboard.
- It does not show on a running or played fight's row, on Setup's roster, in History or on Timeline.
  Arrivals shows it in Setup too.

```text
Case A, the plan at 330 px                         Case D, the editor at 454 px
▸ Bridge Ambush                  [▷ Start fight]   Roster 8 │ Add participants   Moderate · XP 1,050 of 1,125
  5 party · 2 × Ember Cultist · Low                1. Brannoch Vale                              Edit   ⋯
▾ Mill Road                      [▷ Start fight]      Party · HP 58/62 · AC 19
  5 party · 2 × Ember Cultist ·
  1 × Cult Fanatic
    Read encounter · High · XP 1,150 of 2,000
```

**Exact words.** `Low`, `Moderate`, `High`, `Above High`. `XP 1,050 of 1,125`. The hover lines above.

| State | What shows |
|---|---|
| The XP, every level and the budget table are known | The words above |
| A creature has no XP and no CR; a level is not known; there are no sheets; the rules have no table; there is no creature against the party | Nothing. The line is as today |
| Run open | The open fight's plan row shows its status link, not a word. Arrivals shows the live word |

**What the file stores.** Nothing. **Undo.** None is needed. **Keyboard.** Nothing takes focus.

**What stays with the DM.** Everything. Splat blocks nothing, warns about nothing, and gives the word no
colour, because red or green would be a verdict.

**Not built, and why.** A line of budgets or a link to the rule (a new line or control on a plain screen). A
slider to scale a fight. The 2014 thresholds (no typed table in the SRD 5.1 data).

| Check | F1 |
|---|---|
| Primary action in view | The screen's own, unchanged. F1 adds no action |
| Components and tokens | The word: the line's own type, in `text`. Numbers: the same type in `muted`, with `number` digits. No icon and no state colour |
| In view at 960x640 | Lookup closed and open: the editor's line, and each summary that has room; else the opened row |
| States | Words, or nothing |
| Keys and targets | No new target |

### F2. Effects and turn reminders

**Question.** "What must I remember on this creature's turn, and how long does it last?"

**The idea.** An **effect** is words that the DM pins to one participant in a run. Today that is
`Other effect…`. An effect can now carry a time, a count of uses, or a recharge. **A reminder exists only when
the DM gave the effect a time: a count of rounds, or `on its turn`.** Words with no time are a label, as today.

| Need | The DM types | The row says, and the notice line repeats on its turn |
|---|---|---|
| A recurring save | `Wis save DC 15 (Hold Person) on its turn` | `Wis save DC 15 (Hold Person)` |
| A duration | `Frightened 10 rounds`, `Bless 1 minute` | `Frightened 10`, later `Frightened · last round`, then `Frightened · time is up` |
| Recharge | `Fire Breath recharge 5–6` | `Fire Breath ready`, or `Fire Breath spent ⚄`. No reminder |
| Legendary actions | `Legendary actions 3 uses` | `Legendary actions 2 of 3`. No reminder |
| A label | `Hexed` | `Hexed`. No reminder, as today |

**The row is the place to act. The notice line only repeats the row's words** (section 1.4).

**Where the DM types it: one field, in two places that exist.**

- **The Conditions tab** has the field today (`Other effect…`, `src/encounter_run/view.rs:2421-2445`). It
  stays, with its label and its place. It now reads a time when the DM types one.
- **The dock** gets the quick way, `Effect` (key `E`): a quiet action on the dock's second line, left of
  **Conditions**. That line is laid out from the right (Absent, Conc, Conditions), and the name takes the rest
  (`view.rs:2073-2150`). **The fit rule for `Effect`, stated once:** it shows with its word while the selected
  name keeps its size and its words; with its icon only while the name keeps both beside the icon; and not at
  all otherwise. `Death saves` and a damage total, which that line holds at times (`view.rs:2103-2122`), keep
  their places first. With several rows selected, the line is one sentence today (`:2170-2187`) and has no
  `Effect`. The field in the Conditions tab is then the way, as today.
- `E` shows on the button while the key works, as each dock key does. The run's **Keys** list does not change.

**Wireframe: case A, on Oren's turn in round 3.**

```text
Round 3   Oren Thistlewick’s turn                    [+ Add to fight]  [> Next turn  N]
⚑ Frightened 9 · Wis save DC 15 (Hold Person)                        [↶ Undo: next turn  U]
──────────────────────────────────────────────────────────────────────────────
 16 ■  Oren Thistlewick                    AC 18           45/45          ← current row
 TURN  Frightened 9 · Wis save DC 15 (Hold Person)
…
[Amount] [Damage  Enter]  Heal  🛡 Temp HP                      [1d20] [Roll]
Oren Thistlewick                  ⚑ Effect E   Conditions C   ✧ Conc   Absent
```

**The dock while the DM adds an effect.** Line one does not change. Line two becomes one field. At most four
suggestions open upward, over the lowest roster rows, and close on Enter or Escape.


- **Enter adds the words, closes the field and puts focus in `Amount`.** Escape clears and closes the field
  and puts focus in `Amount`, as the loot field's Escape clears today (`src/loot_editor.rs:533`).
- **The field belongs to the row it opened for.** Its placeholder names that row.
- **While the field has focus, a letter is text.** A click on `Next turn` still advances the turn. The field
  then closes, and its words wait for that row.

**Case D (roster 454 px, the run's panel hidden).** The notice line and the row are the wireframe of section
1.4, with three reminders that start with the same words. The dock:

```text
Dagger +6   Fire Bolt +7                       ← the tile line of today
[Amount] [Damage] Heal 🛡Temp HP        [1d20] [Roll]
Cinder Wolf 2             ⚑  Conditions  ✧ Conc  Absent
```

With Lookup or Notes open, `E` is text, as every letter is, and `Effect` takes a click. Case B is case D with
more room. Case C is case A with the run's panel at 346 px.

**How a time works.**

- **Rounds.** The count is the number of this participant's turns that the effect still covers, with the
  current turn. It goes down by one when that participant's turn ends, in the same step as `Next turn`, so
  Undo of `Next turn` gives it back. At 1 the words say `last round`. At 0 they say `time is up`, and the
  effect stays until the DM removes it. An absent participant's count does not go down. To time an effect by
  its caster's turns, pin it to the caster.
- **On its turn.** The words show on each of that participant's turns, until the DM removes the effect.
- **A condition's name.** Words that are an official condition, such as `Frightened 10 rounds`, add that
  condition with a count. Words that are already on the row change their time and add nothing.
- **A recharge roll** uses the run's dice. The result words say `Rolled 5 · ready again` or `Rolled 2 · still
  spent`.
- **Concentration.** When a row marked `Conc` takes damage, the row's second line ends with `Conc save DC 12`
  until the next step: 10, or half the damage if that is more, and at most 30. With no room on that line, the
  words are in the row's hover and accessible name only. Splat does not roll the save or clear `Conc`.

**What a click does.** One act for each target. Hover and the accessible name say it: `Remove Frightened`,
`Spend one`, `Roll 1d6. 5–6 makes it ready`.

| Target on the row | A click |
|---|---|
| An official condition's name, with or without a count | Opens its rule in Lookup, as today |
| A count that runs (`10`, `last round`), or words with `on its turn` | Opens **Conditions**, where the effect is removed |
| `time is up` | Removes the effect, also when it is an official condition |
| `Legendary actions 2 of 3`; at `0 of 3` | Spends one; gives all back |
| `Fire Breath ready`; `Fire Breath spent ⚄` | Marks it spent; rolls the recharge |
| A label with no time | As today (`view.rs:6787-6869`) |

A count is its own target, 24 px wide or more, so the name beside it keeps its act.

**What Splat reads at the end of the words.** `10 rounds` or `1 round`. `1 minute` or `2 minutes`, as 10
rounds each. `on its turn`. `3 uses` or `1 use`. `recharge 6`, `recharge 5–6` or `recharge 5-6`. Anything else
stays words: `Bless for a while`, `Mage Armor 8 hours`.

**The suggestions** (dock field only). `stat block` ones come from the creature's typed data: a feature's
`recharge` block and its `legendary_actions`. `used before` ones are the last effects of this fight.

**Exact words.** `Effect`. Dock field: `Effect on <name>, such as Bless 10 rounds`. On the row and the notice
line: `Bless 9`, `Bless · last round`,
`Bless · time is up`. The Conditions tab's words are as today.

| State | What shows |
|---|---|
| No effect with a time | The notice line and every row are as today |
| A sheet removes a condition that has a count | The condition and its count go together. The sync notice and the Log line name what went: `Frightened, 7 rounds left`. A sheet's change is not an Undo step, as today |
| A sheet adds a condition | It arrives with no count |
| Nobody selected | `Effect` is blocked: `Select a participant first.` |
| The run has ended | Effects show as words on the Final state rows. Nothing counts, and a click does nothing |
| Setup | No `Effect`. Starting conditions come from the encounter, and their words are labels, as today |

**What the run file stores.** One owner for each time: the condition's own entry. A plain condition or label
stays a string. An effect with a time, uses or a recharge is one object in the same list. A run file from
before holds only strings, and loads.

```json
"conditions": [
  "Poisoned",
  {"name": "Frightened", "rounds": 10, "left": 9},
  {"name": "Wis save DC 15 (Hold Person)", "on_turn": true},
  {"name": "Fire Breath", "recharge": [5, 6], "spent": true},
  {"name": "Legendary actions", "uses": 3, "left": 2}
]
```

Every act that changes a condition changes its one entry: the DM's click, **Conditions**, a sheet's read and
Undo. So a count cannot outlive its condition. P4 greps every reader of `conditions` (sync, Setup, the
summaries, and the saved Undo steps of pull request 232, whose `Step::weight` and row check read strings).

**Undo.** Each act is one named step of the run's Undo, which the run file keeps (F8): `Undo: effect Bless on
Oren`, `Undo: Fire Breath spent`, `Undo: recharge roll for Cinder Drake`, `Undo: remove Bless from Oren`. The
Log has a line for each, and a line when a count reaches 0.

**Keyboard.** `E` opens the dock field for the selected row; when `Effect` has no room, it opens **Conditions**
with focus in its field. Enter adds. Escape closes. The targets on a row are Tab stops, as condition words
are, and Enter on one is a click.

**What stays with the DM.** When an effect starts and ends. Every save. Whether a use was spent and when it
comes back. Whether concentration holds. Splat counts, shows and rolls when asked.

**Not built, and why.** A time on words typed in prep (an encounter's `Other effect` stays a label, so no old
text starts to count). A link from a caster's concentration to its effects. A reminder for uses or a recharge.

| Check | F2 |
|---|---|
| Primary action in view | `Next turn`, unchanged and alone |
| Components and tokens | `Effect`: `Action::quiet` with `.key("E")`, dock density, icon `icons::PIN`. Field: `add_field`. Row targets: `link` at the run scale's condition size in `participant.condition`; digits tabular; `icons::DICE` after a spent recharge. `time is up`: `result.warning` icon, words in `text`. Notice part: `Tone::Info`, icon `icons::PIN` |
| In view at 960x640 | Lookup closed and open: round, turn, `Next turn`, the current row with each timed effect, the dock. No scroll |
| States | Counting: `Bless 9`. Last round: `last round`. Ended: warning icon and `time is up`. Spent: `spent` and the dice. Ready: `ready` |
| Keys and targets | `E`, Enter, Escape, Tab. Each target is at least 24 px each way, with an accessible name such as `Spend one legendary action of Adult Red Dragon` |

### F3. Plan items that are not fights

**Question.** "What did I plan for tonight besides the fights, and what is done?"

**The name.** A plan item that is not a fight is a **scene**: a talk, a person to meet, a place, a secret to
reveal. `Item` already means loot and equipment.

**Where it lives.** In the plan that exists. A task line under `## Plan for tonight` is a scene: a row with a
done box, in the note's own order. Obsidian shows the same line with the same box. This is true of a task
line that the DM typed before this feature: it is the DM's plan (section 2.5, row 14).

- **Not a Timeline event,** because a planned scene has not happened. **Not a new list or tab.**
- **The order of the plan:** what has happened in this session, as today; then the fights and scenes still to
  come, in the note's order; then the groups, as today. A done scene stays in its place, so no row jumps.

| A line under `## Plan for tonight` | Row |
|---|---|
| A link to an encounter, alone or with a task box or other words | A fight, as today. Its box, if it has one, is not shown |
| A link to an encounter group, anywhere in the note | A group, as today, and never a scene |
| A task line, `- [ ] words` or `- [x] words`, that links no encounter | A scene |
| A plain list line, a numbered line, a paragraph, or a line nested under another | No row. It stays text in the note, as today |

**Wireframe: the Session page, case A.**

```text
TONIGHT 3                                 │ # Session 2: The Crossing
Add  ⚔ Fight  ⚇ Group  ⌂ Long rest  ⋯⌄    │
▸  Bridge Ambush        [▷ Start fight] ⋯ │ ## Plan for tonight
   5 party · 2 × Cinder Wolf · Moderate   │
☐  Captain Varra                        ⋯ │ - [[Bridge Ambush]]
   NPC · alive · Ashfall Keep             │ - [ ] [[Captain Varra]]
☑  The miller is the informer           ⋯ │ - [x] The miller is the informer
▸  Shrine of Cinders    [▷ Start fight] ⋯ │ - [[Shrine of Cinders]]
```

**Adding a scene: two routes.**

- **Type it in the note,** in Splat or in Obsidian: one click for the caret and one typed line for each scene.
- **`Scene` in the Add strip's More menu.** The strip puts an action in **More** only when it does not fit
  (`src/design/components.rs:2339`, `src/timeline/page.rs:89`). `Scene` is the strip's last action and the
  first to go there. On a plan in cases A to D (330 to 608 px) the strip never shows it,
  because seven actions need about 640, so no action that shows today moves. Where today's seven actions fit
  and the menu button does not, the strip is as today and has no `Scene`.
  `Scene` opens one field at the end of the plan, with focus:

```text
+  [Plan a scene: words, a person or a place                ]
   Captain Varra · NPC
   Ashfall Keep · Location
```

  Enter adds the words as a task line and keeps the field open for the next line. In a note with no `## Plan
  for tonight`, the first scene adds that heading at the end of the note. A suggestion (at most four World
  entries that match) adds a scene that links that entry. Escape clears and closes the field.

**A row.**

- **The box** is at the start of the row, where a fight has its caret. A click or Space ticks it. The note
  changes by one character.
- **The words** are drawn by the Reader and wrap as a plan row's title wraps today. A link opens its entry in
  Lookup, beside the page; Back returns to what Lookup held.
- **The second line** shows only when the words link exactly one World entry: its kind, status and place
  (`NPC · alive · Ashfall Keep`).
- **The action place** is empty until the scene is done. Then, when the words link exactly one World entry, it
  holds the quiet `Add to Log` of F6.
- **The menu** holds `Remove from tonight`, with `Undo remove` under the row, as for a fight.

**A fight that nobody planned: `Start fight` in a reader's header.** It sits with the header's actions, after
`Add to Log`, by the rule of section 1.2 (the reader's header), as a quiet action with the run icon. It shows
for a creature and for an NPC while
no run is open. While a run is open it does not show: an NPC's reader has **Add to fight** then, as today
(`src/world.rs:3160`), and **Arrivals** adds a creature.

- **What it does.** What the editor's `Start fight` does today for a new encounter, in one click: it saves an
  encounter named `Fight with <name>`, with the party and that creature, and it starts the run, which saves at
  once. If the name is taken, it is `Fight with Captain Varra 2`. The name cannot be read as a link to the NPC,
  so a scene line that links the NPC stays a scene.
- **The plan.** Nothing is written in the note. The fight is a row of tonight's plan from `Begin combat`, when
  the run adds its Timeline event, as every run does today.
- **An NPC with no stat block.** The encounter holds the party. The run opens in Setup with the form that
  **Add to fight** has today for a creature that the DM makes up (`add_creature_form`,
  `src/encounter_run/ad_hoc.rs:258`; it works in Setup, `:237`), with the name filled in and focus in HP. The
  DM types the HP and presses Enter. The NPC lives in the run, as a made-up creature does today (`:95`). The
  form is in Arrivals, so Lookup closes, as **Add to fight** closes it today (`:242-245`). Cost: 3.
- **A wrong click has no clean-up of its own.** `Close run` on a Setup that nobody touched removes the run
  file, as today (`untouched_setup`, `src/encounter_run.rs:1048-1053`). The encounter `Fight with <name>` stays
  in **Ready to run**, as an encounter that the DM starts by mistake stays today.
- **New: `Delete encounter`.** Today no action in Splat deletes an encounter: a **Ready to run** row's menu
  holds only `Edit encounter` (`src/encounters/list.rs:560-565`). P3 adds the line `Delete encounter` to that
  menu. It removes the encounter's file and says `Deleted “Fight with Captain Varra”.` with `Undo delete
  encounter`, as
  `Delete the saved run` does (`src/encounters.rs:3237-3262`). It is blocked with today's words while the
  encounter
  has an open or unfinished run (`list.rs:539-545`). Cost: the menu and the line = 2.

**Cases B, C and D.** The plan has its own tab and the full page width. The row is the same. In case D,
Lookup on an entry and the plan are both in view; the note is behind its tab, so the typed route costs one
tab change.

**Exact words.** `Scene`. Field: `Plan a scene: words, a person or a place`. The empty plan's words are as
today. The box's accessible names: `Done: Captain Varra` and `Open: Captain Varra`. `Remove from tonight`.
`Undo remove`. Result of the field: `Scene added to tonight.` with `Undo scene`. `Start fight`, with the
accessible name `Start a fight with Captain Varra`. `Delete encounter`. The count in `TONIGHT 3` is the rows
still to come with the events of the session, as today; a done scene does not count.

| State | What shows |
|---|---|
| No scene | No row. The plan is as today |
| A link to nothing; a link to two notes | As in every reader: the dotted underline and `Not found in this campaign`; Lookup's chooser |
| The note changed on disk | Section 4.1. With unsaved words, a plan act says `The note changed on disk. Choose in the note first.` |
| Run open | Scenes work. A reader has no `Start fight` |
| Read view | Read hides `Plan for tonight` when every line of it is a row beside the note, as it hides link-only plans today |
| An older session | Its scenes show with their boxes. A tick writes that session's note |

**What the note stores.**

```markdown
## Plan for tonight

- [[Bridge Ambush]]
- [ ] [[Captain Varra]]
- [x] The miller is the informer
```

**Undo.** `Undo scene` after the field adds one, in the plan's message place below the Add strip. A tick is
its own Undo: click again. Removing gives `Undo remove`. `Delete encounter` gives `Undo delete encounter`.
Section 4.1 says
when each ends.

**Keyboard.** Tab reaches the box, then the links in the words, then the menu. Space ticks. In the add field,
Enter adds, the arrow keys choose a suggestion, and Escape closes.

**What stays with the DM.** What a scene is, when it is done, and whether it happened at all.

**Not built, and why.** Editing a scene's words in its row (the note holds them). Dragging rows. A plan link
for an improvised fight (the run's own event is its row). Removal of the improvised encounter by `Close run`
(cut in round 3: after a restart nothing remembers which click made it).

| Check | F3 |
|---|---|
| Primary action in view | The first upcoming `Start fight` of the plan, as today. A scene never takes it. `Start fight` in a reader is quiet |
| Components and tokens | `Scene`: a line of the `More Add actions` menu, icon `icons::CHECK_SQUARE`. Field: `add_field`. Box: the theme's checkbox, 24 px. Words: the Reader's inline text with `link`. Second line: `Type::Small` in `muted`. `Start fight` in a reader: `Action::quiet` with `icons::RUN`. `Delete encounter`: `menu_action` with `icons::REMOVE`. Messages: `undo_message` |
| In view at 960x640 | Lookup closed and open: the Add strip and the rows. Nothing scrolls sideways |
| States | Open: an empty box. Done: a checked box, and the words in `muted` |
| Keys and targets | As above. The box, each link and the menu are 24 px or more |

### F4. Trackers and the Due list

**Question.** "What is due, and what are the numbers?"

The data model of [World and house rules](world-and-house-rules.md) stands: house rule notes, in play,
trackers, quick actions, marks, the Due list and the ledger `Trackers.md`. This pull request changes that
document in place: Trackers is a view of Lookup, the ledger is the Undo, a round is a mark, a mark's table
can name what it rolls for (`for:`), and each instance has its own line.

| Surface | What shows | When |
|---|---|---|
| The rail's status slot, above Lookup | A bell and `Due 2` | Only while a notice is due. A click opens Trackers at its Due rows |
| The Lookup panel | The **Trackers** view | Ctrl+T, or a click on `Due 2` or on the notice line's bell |
| The run's notice line | Parts 4 and 5 of section 1.4: a new notice for one turn with its action, and the bell | In a run |

**Wireframe: Trackers in Lookup at case D (the panel is 346 px, about 322 inside), opened from the bell, with
three notices and four rolls.** The view is one scroll area, as Lookup's reader is
(`src/lookup.rs:2195-2203`). Its heading holds the Due count, and the result line keeps its place at the
bottom edge.

```text
🔍 [Search…                             ]  ← Back  ×
Trackers · 🔔 Due 3
DUE 3
Bell. The Cistern floods. Announce the standings.
Roll each rival. Check d6 for wandering.
  [⚄ Roll Rival Track]
  [⚄ Roll Wandering, 0:40 to 1:19]
  Old Guard: 4 · Takes a seal.
  Keyring: 2 · Loses its way in the Kitchens.
  Convicts: 6 · Takes a seal and wounds a rival.
  Wandering: 3 · Two Sand Wraiths at the next door.
                                          Done ⋯
Old Guard: First seal. The party reaches level 2.
A rival uses its 1-2 seals group.         Done ⋯
Round 2 · Fight minute
  ☑ Elapsed race time +1                  Done ⋯
▸ MORE TRACKERS 1
─────────────────────────────────────────────────────
✓ Collect soul: Elapsed race time 61, Souls 1. Due: Bell.
  ↶ Undo: Collect soul              Add to Timeline
```

- **The order in the scroll area:** the pinned trackers in the note's order, then the Due rows, then `MORE
  TRACKERS`. So a new notice moves no pinned quick action. Ctrl+T opens the view at its top. The bell, the
  rail's `Due N` and the heading's `Due 3` open or scroll it to the first Due row, as drawn.
- **What fits at case D.** A roll button that is wider than half the panel has a line of its own, and `Done`
  ends the notice's last line. By estimate the three notices above take about 410 px of the area's 460 in a
  run, and of 424 on the Session page, where the party bar shows. T19.4 tests it. When they do not fit, the
  area scrolls, as a reader does. From Ctrl+T the example's five pinned trackers fill the area, so the
  heading's `Due 3` is the way to the Due rows (1 more; section 2.4).
- **A pinned tracker's line.** Its first two quick actions follow its value on the same line when they fit;
  the others wrap below. **A tracker with instances** (`per`) has one line for each instance: its name, its
  value and the tracker's quick actions. One press, with no menu.

```text
Seals
  Party        1   [Take seal] [Lose seal]
  Old Guard    2   [Take seal] [Lose seal]
```

- **A Due row** has its words, then its actions, and ends with `Done` and the row's menu (`Dismiss`).
  Proposed changes show with a box each, ticked; `Done` applies the ticked ones and finishes the notice, as
  one entry. A table's `Roll` lists its result under the row, and each further press adds one. With `for:`,
  each press rolls for the next name; after the last name the button is off: `Each name has its roll.` The
  button names the table by its heading (section 4.4). A notice's short name is the first sentence of its
  words (`Bell`).
- **A notice from `every: round`** names its round: `Round 2 · Fight minute`. Round 1 raises none; each later
  round of an open run raises one when it begins. A tracker has at most one such mark. It is an ordinary
  notice: it stays until `Done` or `Dismiss`, and it shows here.
- **On the notice line** a new notice has the same action, `Done`.
- **A quick action** is one press. An action that asks for a number (`add: ask`) puts a number field in its
  own place; the field holds focus until Enter or Escape.
- **The result line** names the last act and has `Undo` and `Add to Timeline`. After `Add to Timeline` it says
  `Added to the Timeline.` with `Undo add`.
- **A tracker's history,** with **Remove**, and the list of problems show in the house rule note's reader.
- **Cases A and C** are the page before Ctrl+T: nothing shows unless a notice is due. **Case B** is the same
  view at 432 px.

**Exact words.** Rail: `Due 1`; hover `1 notice is due. Open Trackers (Ctrl+T).` View: `Trackers`, `PINNED`,
`MORE TRACKERS 1`, `DUE 1`. `Roll <table’s name>`. `Done`. `Dismiss`. Result: `<label>: Favor 2.`; with a
table `… Rolled 4 on 1d6: A healing
potion.`; with a new notice `… Due: Bell.`; after `Done`, `Bell done.`; after a dismissal `Bell dismissed.`
`Undo: <label>`. `Add to Timeline`. Notice line: `Due: <the mark's words>`; `🔔 Due 1`. A blocked action:
`Favor cannot go below 0.` With no note in play, Ctrl+T opens the empty state `No house rules are in play.`
with `Turn one on in Campaign → Rules and runs.`

| State | What shows |
|---|---|
| No house rule note in play; or in play with nothing due | Nothing in the rail |
| A notice is due | Rail: bell and `Due 1`, in the text colour |
| `Trackers.md` cannot be read | Rail: error icon and `Trackers not read`. The view names the file, says the history is safe, and offers `Try again` and `Show file`. Quick actions are blocked with that reason |
| `Trackers.md` changed on disk | Separate changes merge with a quiet notice, as `Timeline.md` does. The same entry needs a choice |
| The fight of a round's notice is closed or ended | The notice stays, with its round. `Done` and `Dismiss` work. When the run's file is deleted, its notices go |

**What the files store.** Definitions are in the house rule note's Splat data, as before, with two new
optional words: `every: round` on a mark, and `for:` on a mark's table. The ledger is one ordered list. Each
act is one entry. A notice's state is not stored: a notice is open until an entry finishes or dismisses it.
A crossing's notice is worked out from the changes. A round's notice is worked out from the run file's round,
so Splat writes nothing when a round begins. An entry holds what it replaced where that cannot be worked out:
`was` on a `set`. `why` holds the action's label, so the result line and `Undo: <label>` name the act after
a restart. The changes of one press share a `use` and undo together.

```yaml
changes:
- {id: c-5e6f7a8b, tracker: race_clock, add: 10, why: Enter room, session: 070eca36-…}
- {id: c-6f7a8b9c, notice: {change: c-5e6f7a8b, at: 40}, roll: {table: rival_track, for: Old Guard, value: 4, row: Takes a seal.}, session: 070eca36-…}
- {id: c-7a8b9c0d, notice: {change: c-5e6f7a8b, at: 40}, done: true, session: 070eca36-…}
- {id: c-8b9c0d1e, tracker: favor, add: -1, why: Throw a gift, roll: "1d6: [4]", row: A healing potion, session: 070eca36-…}
- {id: c-9c0d1e2f, notice: {tracker: race_clock, run: 4f1c2a9e-…, round: 2}, done: true, tracker: race_clock, add: 1, session: 070eca36-…}
```

**Undo.** `Undo: <label>` on the result line removes the newest entry that has the stamp of the session being
played (section 1.1), or no stamp when no session is played. The label is the action's own (`Undo: Throw a
gift`), or `Undo: Bell done`, `Undo: Bell dismissed`, `Undo: Rival Track roll`. The line then names the entry
before it, so the DM steps back, newest first. The ledger is a file, so the steps are there after a restart.
Undo of an entry that raised a notice removes the notice and its entries. An entry with another stamp leaves
by **Remove** in the tracker's history, as the companion document says. `Add to Timeline` is a second act: it
makes a Major event with the action's label, in the session being played, with the Timeline's own Undo
(section 4.1). `U` is the run's key.

**Keyboard.** Ctrl+T, there and back. Tab walks the trackers' actions, then the Due rows. Enter presses an
action. Escape closes Lookup. The run's keys are as beside any Lookup view.

**What stays with the DM.** Awarding Favor, moving a rival, what a bell does on the map, and every rule that a
tracker's number stands for. A proposed change applies only when the DM presses `Done`.

**Not built, and why.** The parts in [World and rules: later](world-and-rules-later.md). Quick actions in the
run's dock or footer. Due rows in the plan. A word in the rail when nothing is due. A link of its own for each
round (cut in round 3: the round is a mark). An action on a notice for a group that its words link (cut in
round 3: a mark of a tracker with instances cannot name one instance's group; **Add to fight** is the route,
as today). A `Make house rule note` action (no band on a World note's reader holds it today; the DM sets the
note's properties, as the companion shows).

| Check | F4 |
|---|---|
| Primary action in view | None in the view. The page's own primary stays |
| Components and tokens | Rail: the status slot with `status_link`; icon `icons::BELL`. View title: Lookup's title line, with a `link` for the count. `PINNED`, `DUE`: `group_heading`. Quick actions and `Roll`: `Action::secondary` at Read density; `Roll` has `icons::DICE`. `Done`: `Action::quiet`. Proposed change: the theme's checkbox. Values: `Type::Numeral`, right-aligned with `number`. Result: `undo_message` with a second quiet action, above a hairline. Notice parts: the header's notice with a `link` |
| In view at 960x640 | The rail's `Due N` on every page. In the view from the bell: the result line and three notices with four rolls. From Ctrl+T: the result line and the first pinned trackers |
| States | Due: bell and `Due 1`. Not read: error icon and `Trackers not read`. Blocked: the reason beside the action |
| Keys and targets | Ctrl+T, Tab, Enter. Every action is 28 px high (Read) |

### F5. Carry-over between sessions

**Question.** "What did the last session leave for this one?"

**Where it lives.** On the **Last time** line of the Session page, which already links the last session and
shows the first line of its recap (`last_time_block`, `src/sessions/page.rs:4066`). One quiet action,
`Carry over`, ends that line while a part is still to carry. The preview gives it room and is whole on hover,
as today, so the line stays one line.

- **Not automatic at Create session,** because a note would then change with no act of the DM (rule 2).

**Three parts.** Each is done once.

| Part | From the last session | To this note | Copied text or a link | Done when |
|---|---|---|---|---|
| Recap | The prose under `## What happened` | `## Recap of last time` | **A link:** one embed line, `![[Session 1 The Road#What happened]]`. One file owns what happened, and a later correction shows here | The embed is added. Or this note already has the DM's own prose under the heading: it is kept, and the part counts as done |
| Recap, second source | A processed recording's **Recap**, only when `What happened` has no prose | `## Recap of last time` | **Copied text,** under the line `> This draft was written automatically. Check it against your memory.` | The same. Later, when the DM has written `What happened` and has not changed the draft, `Carry over` shows again and puts the embed in the draft's place |
| Plan | Each row still to come under `## Plan for tonight`: a fight that was not played, a fight with a paused run, a group, a scene that is not done | `## Plan for tonight` | **Copied lines,** links kept, boxes empty | The lines are added, also when the DM deletes them later |
| Threads | Each line under `## Open threads` that is not ticked | `## Open threads` | **Copied lines** | The same |

- **A paused fight** carries as its encounter's link. Its row in the new plan has `Resume`, as a planned fight
  with a saved run has today (`src/sessions/page.rs:4432`, `:4462-4466`).
- **The recap can come later.** When the last session has no prose and no processed recording yet, the plan
  and the threads carry, and the recap stays to carry.
- **When the DM already wrote something.** Carry over never replaces the DM's words and never reorders. It
  adds below what a heading holds, and it skips a line that this note already has, compared without the task
  mark. A heading that this note does not have is added at the end of the note.
- **Which session.** The session before this one, by number. Once `carried_from` is set, `Carry over` reads
  only that session.
- **When the line reads the last session.** When that note's file changes and when a recording of that session
  is processed, never each frame.
- **The preview and the carry-over read one source.** The **Last time** line previews the last session's
  `What happened`, then its processed recap. Today it reads that note's own `Recap of last time` first, which
  tells the session before it (`recap_preview`, `src/sessions/page.rs:915`).

```text
Case A, before and after
Last time:  Session 1: The Road — Hobb the miller says the toll collectors …    Carry over

Last time:  Session 1: The Road — Hobb the miller says the toll collectors …
✓ Carried over from Session 1: the recap, 2 plan lines and 2 open threads.   ↶ Undo carry over
```

**An embed in Read.** Read draws an embed of a session note's heading as that heading's text, under a muted
line, `From Session 1: The Road`, which links that session. The rule is by the embed's target, not by who
wrote the line, so an embed that the DM typed is drawn the same way (section 2.5, row 18). Today Read shows it
as a link. In Write it is the one line of source.

**Cases B, C and D.** `Carry over` never takes a line of its own. The result message is capped at the list
measure and wraps. Carry over does not use Lookup.

**Exact words.** `Carry over`. Hover says what it will add: `Add the last session’s recap, 2 plan lines and 2
open threads to this note.` Result: `Carried over from Session 1: the recap, 2 plan lines and 2 open
threads.` It names only what it added. When this note has a recap: `… Your recap was kept.` When the recap is
not there yet: `… The recap is still to come.` When the embed takes the draft's place: `The recap now follows
your notes.` `Undo carry over`. `Update the link`, with `Undo link`.

| State | What shows |
|---|---|
| No earlier session; no part can be carried now; every part is done | No action |
| The last note cannot be read | The action is blocked: `Splat cannot read Session 1’s note.` |
| This note changed on disk and Splat holds unsaved words | The act does nothing and says `The note changed on disk. Choose in the note first.` |
| The embed's note was renamed outside Obsidian | Read finds the session by the ID in `carried_from` and shows its text, with the quiet `Update the link`, which writes the note's new name into the embed line |
| The embed's note or its heading is gone | Read says `The recap’s source is gone: Session 1 The Road › What happened.` in the embed's place. The line stays in the note |

**What the note stores.** The lines, and two flat properties in the same save. Obsidian's Properties view
shows both as it shows `session_id`.

```markdown
---
session_id: 9b1f0c1e-52a7-4c3b-8f13-0f6f4f4f2a10
session_number: 2
carried_from: 070eca36-331c-46cc-b5a6-32a6e35265da
carried_parts: [recap, plan, threads]
---

## Recap of last time

![[Session 1 The Road#What happened]]

## Plan for tonight

- [[Shrine of Cinders]]
- [ ] [[Captain Varra]]

## Open threads

- [ ] Who burned the ropes?
```

**Undo.** `Undo carry over` removes exactly the lines that the act added, when they still stand as written,
and takes its parts out of `carried_parts`. Section 4.1 says when it ends. After that the DM removes a carried
line as any line, and the part stays done.

**Keyboard.** Tab reaches `Carry over` after the **Last time** link. Enter presses it.

**What stays with the DM.** Which threads are still open, and what the recap says aloud.

**Not built, and why.** A recap that an LLM writes for reading aloud. Carrying from sessions before the last.
Ticking a thread in the old note when it is ticked here (two files for one act).

| Check | F5 |
|---|---|
| Primary action in view | The plan's `Start fight`, as today |
| Components and tokens | `Carry over`: `Action::quiet`. Result: `undo_message`, success tone. The embed in Read: the Reader's prose under a `Type::Small` muted line with `link`. A source that is gone: `small_message`, warning tone |
| In view at 960x640 | The **Last time** line and the action, above the tabs, with Lookup closed or open |
| States | Blocked: the reason beside the action in `Small` |
| Keys and targets | Tab, Enter. A Read control, 28 px |

### F6. Wrap-up to World

**Question.** "How does what happened to this person or place get into its own note?"

**Today.** A change of Status, place or group writes the value, and also a Log line when a session is being
played, in one step with one Undo (`src/world.rs:2280-2305`; the stamp is `log_as`, `src/lib.rs:1187-1194`).
The session's Read view lists those lines under **World changes**, from each entry's Log. No action writes
the DM's own words.

| The DM means | The act | What the file gains |
|---|---|---|
| A fact that a field holds: "Verori died" | **Change** on Status, as today | The status, and the line `Status alive → dead.` |
| A fact that no field holds: "She told the party who burned the ropes" | **Add to Log**, new: a quiet action in the reader's header, after `Edit` (section 1.2) | One line, in the DM's words |

A death is a status. `Add to Log` does not change a status, and Splat proposes no "died". When the DM does
both, the Log has two lines for two facts.

**One stamp for both acts.** Each line starts with the session being played (section 1.1) and the campaign
day. With no session being played, `Add to Log` writes the day only, because the DM asked for a line; **Change**
writes no Log line then, as today. The muted start of the field shows the stamp before Enter.

**The ways in.** A done scene that links one World entry has `Add to Log` in its row's action place (F3): it
opens Lookup on that entry with the Log field open. From any other place the DM opens the entry, by a link
or by Ctrl+K and the name, and then clicks `Add to Log`.

**Wireframe: Lookup on an entry at case D (346 px), beside the ended view, with a long line.**

```text
🔍 [Search…                             ]  ← Back  ×
Captain Varra   ✎ Edit  ＋  ▷
NPC · Campaign · alive
  Session 12, Day 3:
  [Told the party who burned the ropes at the ford, and  ]
✓ Added to Captain Varra’s Log.   ↶ Undo Log line
Status       alive                             Change
Log
• Session 12, Day 3: Told the party who burned the ropes
  at the ford, and asked for safe passage north.
```

- `Add to Log` opens one add field under the kind line, with the line's stamp before it in muted text. Enter
  adds the line and closes the field. Escape clears and closes it. At case B the stamp and the field share a
  line, and the header shows `＋ Add to Log` with its word.
- **From a done scene** the field starts with the scene's words that are outside its link, selected, so that
  typing replaces them and Enter keeps them.
- **The typed words belong to the entry and the session.** They wait while Lookup shows another entry, and a
  restart loses them, as it loses every held form (section 7).

**Lookup.** This feature uses Lookup. When Lookup shows another entry, the click goes to the new entry, and
Back returns. While the same entry has an edit form with changes, `Add to Log` is blocked, so that Splat never
makes its own "changed on disk" warning.

**Exact words.** `Add to Log`. Field's accessible name: `Log line for Captain Varra`. Placeholder: `What
happened?`. Result: `Added to Captain Varra’s Log.` `Undo Log line`.

| State | What shows |
|---|---|
| The entry has no Log | The first line makes `## Log` as the last heading |
| The entry's edit form has changes | Blocked: `Save or discard your edit first.` |
| The entry belongs to a package | No action. Its Log would belong to every campaign that uses the package |
| The file cannot be read | Blocked: `Splat cannot read NPCs/Captain Varra.md. Repair the file first.` |
| The file changed on disk | Splat reads the file again and adds the line to what is there |
| The write fails | An error in the message place names the file. The field stays open with its words |

**What the note stores.** One more line, and nothing else:

```markdown
## Log

- [[Session 11]], Day 2: In [[Ashfall Keep]] → [[The Ford]].
- [[Session 12]], Day 3: Told the party who burned the ropes.
```

The session's Read view then lists `Captain Varra  Told the party who burned the ropes.` under **World
changes**, with no second copy in the session note.

**Undo.** `Undo Log line` removes that one line, after a check that the file still holds it. It ends when the
entry's text changes or Lookup shows another entry (section 4.1).

**Keyboard.** Tab reaches `Add to Log` after `Edit`. Enter opens the field; Enter adds; Escape closes.

**What stays with the DM.** The words, and whether a status changed.

**Not built, and why.** A link from a fight's ended row to the entry (a new control on a plain screen). A Log
for player characters. Editing a Log line in Splat (Edit and Obsidian do it).

| Check | F6 |
|---|---|
| Primary action in view | None in Lookup |
| Components and tokens | `Add to Log` in a reader: `Action::quiet` with `icons::ADD`. On a scene's row: `Action::quiet` in the row's action slot. Field: `add_field`. Stamp: `Type::Small` in `muted`. Result: `undo_message` |
| In view at 960x640 | The header with `Edit` and `Add to Log`, the field and the result, above the entry's text |
| States | Blocked: the reason in `Small` after the action |
| Keys and targets | As above. Every target is 24 px or more |

### F7. Rewards record

**Question.** "What did the party get tonight, and is it in the notes?"

**The note owns the rewards.** `Add rewards` writes one block in the session note: a list line that starts
with the fight's Timeline reference, and its loot lines under it. The run stores nothing about it. **A fight
has its rewards in the notes when the note has a list line that starts with its reference, anywhere in the
note.** The heading `## Loot and rewards` is only where a new block goes; a DM who renames it or moves the
block loses nothing.

**Where it lives.** On the Session page, on the plan row of a fight that has ended. That row's action band
holds only the menu today (`src/sessions/page.rs:4344-4346`), and its title has the rest of the line.

`Add rewards` is a quiet action in that band, left of the menu, only when the title keeps the lines that
it has today; it is no wider than `Add to notes`, which the band's reserve counts (`action_width`, `:4313`).
When the title uses the width, it is the first of the opened row's links: `Add rewards · Log · Whole run ·
Timeline` (`:4918-4926`).

After the act, the same place says `In notes`: the status that an event's row has today (`:4392`), or muted
words in the opened row. `Copy loot` stays on the ended view, for a chat or another program.

```text
Case A, the plan at 330 px
▸ Shrine of Cinders             Add rewards  ⋯
  Ended in round 1
▾ Ashfall Crossing at the Old Toll Bridge     ⋯
  Held the ford · Ended · Round 3
    Add rewards · Log · Whole run · Timeline
✓ Rewards added under Loot and rewards. XP 400, as in your last
  rewards. XP tonight: 1,650 for 2 fights.
  ↶ Undo rewards   Leave XP out
```

**What it writes.**

- The fight's link, as events write it: `[[Timeline#^t-08a07eb0ec|Ashfall Crossing]]`. A played fight has no
  `Add to notes` today (`:4388`), so this is the act that links it.
- **Loot,** as lines under it: the encounter's loot and the loot of each creature at 0 HP. A roll that was
  made has its result. A roll that was not made is written with `(not rolled)`, as `Copy loot` writes it
  today, and the result says how many.
- **XP,** on the fight's line, when the campaign's notes use XP: the XP of the creatures that ended at 0 HP,
  also those marked absent at 0 HP, and each character's share. F1's function gives the numbers. The line
  names what it counted and what still stood, so the DM can change it. With nobody at 0 HP it says `XP 0 for 0
  at 0 HP`, so a block in a campaign that uses XP always has the XP word.

**XP is a choice about one block.** There is no setting and no stored mark. For a new block Splat looks for
the newest rewards block: in this note first, then in the note of each earlier session, newest first. A note
with no block is passed. A note that cannot be read stops the search, and Splat never looks past it.

| The search finds | `Add rewards` writes | The result says | Switch |
|---|---|---|---|
| A block with an XP line | The link, the XP and the loot | `XP 400, as in your last rewards.` | `Leave XP out` |
| A block with no XP line | The link and the loot | `No XP, as in your last rewards.` | `Add XP` |
| No block | The link and the loot | `No XP: your notes have no rewards yet.` | `Add XP` |
| A note that cannot be read | The link and the loot | `No XP: Splat cannot read Session 11’s note.` | `Add XP` |

The switch is one quiet action on the result line. It changes **only the block just written**: `Add XP` adds
the computed XP words to it, and `Leave XP out` takes them from it. Splat never rewrites a block that the DM
did not just add, so an XP line that the DM corrected by hand stays. `Undo rewards` removes the new block
whatever the switch did.

**A limit.** Splat does not watch the block. When the fight changes later (a roll is taken back, or the fight
goes on and ends again), the DM changes the block by hand, or deletes its line: the row then says `Add
rewards` again.

**Loot that nobody planned.** The DM types it under the block, in the note beside the plan: one click and one
line.

**The session's XP.** Splat stores no total. The result says the sum of the first `XP` number after each
fight's reference in this note, as the note stands, and how many fights it sums.

**Cases B, C and D.** The plan has the page's width, so more titles leave room. The result message wraps, and
its actions start at its words. The act does not use Lookup.

**Exact words.** `Add rewards`. `In notes`. Result: `Rewards added under Loot and rewards.`, then one line of
the table above; with XP, `XP tonight: 1,650 for 2 fights.`; with rolls that were not made, `2 loot rolls were
not made.` `Undo rewards`. `Add XP`. `Leave XP out`.

| State | What shows |
|---|---|
| The fight is not ended | No action. The row has its status link |
| The fight is ended, and the note has no line with its reference | `Add rewards` |
| The note has a line with the fight's reference | `In notes` |
| The run file cannot be read | Blocked: `Splat cannot read this fight’s run, so its rewards are not known.` |
| The note changed on disk and Splat holds unsaved words | The act does nothing and says `The note changed on disk. Choose in the note first.` |
| The note has no `## Loot and rewards` | The heading is added at the end of the note |
| An older session's fight | The block goes to that session's note |

**What the note stores.**

```markdown
## Loot and rewards

- [[Timeline#^t-08a07eb0ec|Ashfall Crossing]] · XP 400 for 2 at 0 HP · 80 each for 5 · 1 still stood (XP 200)
  - 120 gp in a locked coffer
  - A smoky quartz (50 gp), from 2× gems
  - Varra’s signet ring
- [[Timeline#^t-5c1d2e3f4a|Shrine of Cinders]] · XP 1,250 for 3 at 0 HP · 250 each for 5
```

The last loot line of the first block is one that the DM typed. The sum reads only the first `XP` number
after each reference, so `(XP 200)` is not counted.

**Undo.** `Undo rewards` removes the block that the act added, when it still stands as Splat left it (section
4.1). **Keyboard.** Tab reaches `Add rewards` in the row or among the opened row's links, and the result's
actions after its words.

**What stays with the DM.** Who earned XP, whether a creature that fled counts, how loot is shared, and when
the party levels up. A milestone is a line that the DM writes.

**Not built, and why.** `Update rewards`, and any record in the run of what was written (two owners for one
fact). Switches that change XP in every block (cut in round 3: they could replace an XP line that the DM had
corrected). A status link to the block (cut: the row has `In notes`). A stored total. A party inventory.

| Check | F7 |
|---|---|
| Primary action in view | The next `Start fight` |
| Components and tokens | `Add rewards`: `Action::quiet` in the plan row's action band, or `word_link` in the opened row. `In notes`: `design::status` with `icons::OK` in `document.saved`. Result: `undo_message` with one more quiet action |
| In view at 960x640 | The played rows; the action on the row, or one click away in the opened row |
| States | Not added: `Add rewards`. Added: check and `In notes`. Blocked: the reason in `Small` |
| Keys and targets | As above. Read controls, 28 px; the opened row's links are 24 px high |

### F8. Undo after a restart, and Continue fight

**Question.** "The app closed, or I ended the fight by mistake. Can I go on?"

F8 is built in [pull request 232](https://github.com/david7a68/splat/pull/232), read here at `8d3d2de6`. Its
body is the specification; this design adopts it whole and adds no surface. P0 is its merge.

| What the DM gets | As built |
|---|---|
| Undo after **Close run**, **Resume** and a restart | The run file keeps the Undo steps (at most 100) in an optional `undo` field. `Undo` names and takes back the same step. `src/encounter_run/undo.rs` owns the saved form |
| A fight that ended by mistake | While the ending is the newest step: `Undo: end encounter`, and no second control. After a loot roll, or with no steps: `Continue fight`, a secondary action beside `Close run` on the ended view, and under **Read-only** in Encounters → History. It is one named step, `Undo: continue fight`. Play returns to the turn the fight ended on, with the loot results kept |
| The outcome at a second ending | The End step's outcome field starts with the outcome of the ending that Continue removed, selected |
| Steps that this version cannot read | The run loads and drops only the steps. A warning says so once, with `Dismiss`: `Splat could not read this run's saved Undo steps, so Undo starts again from here. The run itself is complete.` |
| `Continue fight` while another run is open | Blocked: `Close the open run first.` |

**Limits, as the pull request states them.** A sheet that changed after the ending wins at the first read
after `Continue fight`, as for any resumed run. After a restart, Undo of a save's damage does not arm the save
again, and `Use the saved HP` is offered only in the session that made the long rest's step. When the steps
do not fit 100 steps, 16 MiB in the file or 256 MiB in memory, the oldest are left out with no message. Text
that the DM typed in the outcome field wins over the offered outcome. A closed, ended fight goes on in 2
interactions from History and 3 from the plan.

**One exception to rule 8, from that build.** The unread-steps warning pushes the roster down by its two
lines. It shows only after a file that this version cannot read, and `Next turn` works while it shows.

**What this design changes in it.** One thing, in P4: a condition with a time is an object in the row's
`conditions` list (F2). A saved step holds rows as they are, so the step's form does not change and its
`version` stays 1; a step saved before P4 holds strings and reads as labels. P4 proves this with a file that
was saved before it.

| Check | F8 |
|---|---|
| Primary action in view | `Next turn` in play. On the ended view, `Close run`, as today |
| Components and tokens | As built: `Continue fight` is `Action::secondary` with `icons::RUN`; the warning is the run's own message with `Dismiss` |
| In view at 960x640 | Scenario `run-continue` of the pull request captures each surface with Lookup closed and open |
| States | The table above |
| Keys and targets | `U`, as today. No new key |

### F9. Travel and random encounters

**Question.** "The party travels for two days. Does anything find them?"

**Where it lives.** In the Travel event's own row, in the session's plan and in the detail column on
Timeline. A Travel row is as today until the DM chooses `Roll for encounters` in the row's menu, which exists
today (`src/sessions/page.rs:4379-4387`; Timeline has its own event menu). That one act puts the block
**Encounter checks** on that row: the row's length field, a table, and one line for each day or watch.

- It is not a new kind of event (Travel already moves the clock), it is not on every Travel row (a DM who
  only records a journey asked for no checks), and it is not in Lookup (a roll there is not kept).

**The table marker.** The marker gets one optional word, `per`. Nothing else changes.

```markdown
## On the road

<!-- splat:rolls key=road dice=1d20 per=day -->

| d20 | On the road |
| --- | --- |
| 1–14 | Nothing stirs. |
| 15–17 | [[Wolf Pack]] pick up the trail. |
| 18–20 | A toll gang blocks the way: [[Bridge Ambush]]. |
```

- `per` says how often to roll: `day`, which is the value when there is none; `watch`, which is 8 hours; or a
  number of hours, such as `4h`. A word that Splat cannot read gives the warning `Interval not read; using one
  check each day.` on that table. `Cannot roll` stays the words for a fault in the dice or the rows.
- **A table's name** in this block and in Trackers is its heading (`On the road`), from the one table
  descriptor of section 4.4. A table's identity is its key (`road`).
- A row's words can link an encounter or an encounter group. This needs no marker.

**Which table the block starts on.** Splat never picks by a key's name or by file order.

| The campaign has | The block starts on | Cost |
|---|---|---:|
| One marked table | That table | 0 |
| Several, and an earlier journey with checks | The table of the newest such journey | 0 |
| Several, and no earlier journey with checks | No table. The menu says `Choose a table`, in the order of the tables' names, and `Roll` is blocked: `Choose a table first.` | 2, once |
| An earlier choice whose table is gone | The same as no earlier choice | 2 |

The table menu's last line is `No checks`. It takes the block off the row while no result stands; with a
result it is blocked: `Clear the results first.` So a wrong `Roll for encounters` can go at any time.

**Wireframe: the Travel row after `Roll for encounters`, case A (the plan is 330 px), with long results.**

```text
▾  Road to the ford by the old quarry     Add to notes ⋯
   Title      [Road to the ford by the old quarry ]
   Encounter checks    On the road ⌄ · 1d20 each day
   How long   [2 days                ]
              Day 3, 08:00 to Day 5, 08:00
     Day 3   12 · Nothing stirs.                    Clear
     Day 4   16 · Wolf Pack pick up the trail
             near the quarry gate.                  Clear
             Plan “Wolf Pack”
     Day 5   [⚄ Roll] [Enter a roll]
   Timeline
✓ 2 travel checks rolled.   ↶ Undo roll
```

- **How long** is the Timeline's field. It is part of the block, so a plain Travel row does not gain a line.
- **The lines follow the length as the DM types it,** with no Enter. An empty length, or one that the clock
  cannot read, gives one line, `This journey`. `Roll all 2` shows under the lines while two or more are not
  rolled.
- **A check belongs to its offset** from the journey's start: 0 hours, 24 hours, and so on. Its name is read
  from the clock: `Day 3` for a day, `Day 3, 08:00` for a watch or for hours. **A result is never deleted by
  a change of length or of table.** A result whose offset is now outside the journey, or between the new
  table's lines, stays under the muted line `Outside this journey now`, until the DM clears it.
- **A result's line ends with the quiet `Clear`,** which gives the line its buttons back. It works at any
  time, also after a restart, and whatever the words now say.
- **A result that links an encounter or a group offers `Plan “<name>”`** for each, at most three. One click
  writes the link under `## Plan for tonight` as the first line still to come. So the fight is the next plan
  row and has the primary `Start fight`. Then the line says `In the plan`, for as long as the note holds that
  link: Splat reads this from the note and stores no mark.
- More than 31 lines are not listed. The row says `This journey has 240 checks. Split it into shorter
  journeys to roll them here.`
- **Cases B, C and D.** The plan has the page's width, which is wider than 330 px, so nothing more gives.
- **On Timeline** the event's menu has the same `Roll for encounters`, and the detail column shows the same
  block under its own **How long**.

**Lookup.** The table's name is a link. It opens the table's note at its heading in Lookup, beside the page.


**Exact words.** Menu: `Roll for encounters`. Label: `Encounter checks`. `<table’s name> · 1d20 each day`, or
`each watch`, or `every 4 hours`. `Choose a table`. `No checks`. `Roll`. `Enter a roll`. `Roll all 2`. `This
journey`. `Clear`. `Plan “Wolf Pack”`. `In the plan`. `Outside this journey now`. Results: `Encounter checks
added to this journey.` `Undo checks`; `2 travel checks rolled.` `Undo roll`; `Check cleared.` `Undo clear`;
`“Wolf Pack” is in tonight’s plan.` `Undo plan line`.

| State | What shows |
|---|---|
| The campaign has no marked table | No menu line. The row is as today |
| The DM has not chosen `Roll for encounters` on this row | The row is as today |
| The table's rows do not cover the dice | `Cannot roll`, and why |
| The table was renamed, moved or deleted after a roll | Each result keeps its words. The menu says `Table not found` and lists the other tables |
| A check line that Splat cannot read after an edit in Obsidian | The line shows its words as they are, with `Clear` |
| Run open | The same. A travel check is not a run step and uses its own dice |

**What `Timeline.md` stores.** The file is a list that the DM can edit in Obsidian (version 2,
`src/timeline/formats.rs:393-424`). The event gains one line for its table and one line for each result.
An older Splat keeps lines that it does not know and writes them back (`:413-423`).

```markdown
- Travel: **Road to the ford** ^t-5c1d2e3f4a
  - Duration: 2 days
  - Session: [[Session 2 The Crossing]]
  - Checks table: road
  - Check at 0h: 12 · Nothing stirs.
  - Check at 24h: 16 · [[Wolf Pack]] pick up the trail.
  %% splat session=070eca36-331c-46cc-b5a6-32a6e35265da %%
```

- **What Splat reads back:** the table's key; from each check line, the offset in its label, the number
  before `·` and the words after it. It shows the words as the file has them.
- **An edit in Obsidian, then `Clear`.** The DM changes the second line to `16 · Two wolves, driven off.` Splat
  shows those words, and `Plan “Wolf Pack”` goes, because the words link nothing now. `Clear` on that line
  removes the line `Check at 24h`, found by its label. `Undo clear` puts back the line with the DM's words.

**Undo.** `Undo checks` takes the block off the row while it has no result. `Undo roll` removes the results
of the last press, one roll or all of `Roll all`. `Clear` has `Undo clear`. `Plan “…”` has `Undo plan line`.
Section 4.1 says when each ends.

**Keyboard.** Tab walks the fields, the table menu, each `Roll` and `Enter a roll`, then `Roll all`. Enter
presses. A number and Enter in `Enter a roll` records a player's own roll.

**What stays with the DM.** Whether a result happens, where and when on that day, and what it means. Splat
does not plan a fight or start one.

**Not built, and why.** Checks on every Travel row, and a table found by its key's name. Weather, pace,
getting lost and foraging. A table chosen by terrain. A result that rolls its own dice, such as `2d4 wolves`.

| Check | F9 |
|---|---|
| Primary action in view | The plan's `Start fight`. No roll is primary |
| Components and tokens | Menu lines: the row menu's and the table menu's quiet items. **How long**: the Timeline's field. Label: `small_field`. Table: `menu_button`. `Roll`, `Roll all`: `Action::secondary` with `icons::DICE`. `Enter a roll`: `number_field`. Result: `Type::Small`, the number in `text`, the words in `muted`. `Clear`: `Action::quiet`. `Plan “…”`: `Action::quiet` with `line_start`. `In the plan`: `status`. Messages: `undo_message`; the interval warning: `small_message` |
| In view at 960x640 | The open row scrolls with the plan, as open rows do today. The Add strip stays above it |
| States | Not rolled: the buttons. Rolled: the number and words. Planned: check and `In the plan`. Fault: warning icon and `Cannot roll` |
| Keys and targets | As above. `Roll` and `Enter a roll` are 28 px high and at least 24 px wide |

## 4. Cross-feature rules

### 4.1 Every new write

One act writes one file, as a patch of the file on disk now. There are five write paths. One function is
new, and one save is new; the table says which.

| Path | Today | For a new act | After a restart | A failed save says |
|---|---|---|---|---|
| **Session note** | A plan act changes the draft (`insert_in_plan`, `src/sessions.rs:1071`); the save comes 2 seconds later (`SAVE_AFTER`, `:26`). `Draft::save` writes only when the file is as Splat last read it (`:1282`). `follow_disk` takes the file's text when Splat holds no unsaved words, on a poll each second (`:1423`) | **New in P1c: one function, `Draft::apply`.** It follows the disk, patches one fragment, saves now through `Draft::save`, and keeps the Undo. Every note act of this design calls it, and today's two plan acts move onto it (`add_to_plan`, `src/sessions/page.rs:3942`; the removal, `:4570`) with no change that the DM sees | The note holds what the act wrote. The DM removes it as any words | `Notes not saved`, as today. The fragment waits in the draft |
| **World note** | A change reads the note, changes it and writes it with `write_note`, which writes only when the file still holds the text that was read (`src/world.rs:1033`) | The same call. An edit form with changes for the same entry blocks the act | The same | An error that names the file |
| **`Timeline.md`** | `Book::save` writes the file atomically, checks for an outside change, and merges by event (`src/timeline.rs:924`, `:774`) | The act changes the lines of one event. Separate events merge with a quiet notice; the same event needs a choice, as today | The lines are in the file | `Timeline not saved`, as today |
| **`Trackers.md`** | No code today | **New in P9: the ledger's save.** It shares `Book`'s atomic write and outside-change check, which P9 moves to a shared place, and it merges by entry ID with its own code, because the Timeline's merge is written for events | The entries, and so their Undo, are in the file | `Trackers not saved`. The entry waits |
| **Run file** | The run saves after each step and has its own choices when the file changed | As today | The step and its Undo are in the file (F8) | The run's footer, as today |

**What `Draft::apply` does with an outside edit.** It calls `follow_disk` first, so an edit made in Obsidian a
moment before is in the text that it changes. When Splat holds unsaved typed words and the file changed, the
act is not applied, its place says `The note changed on disk. Choose in the note first.`, and the comparison
of today decides. An act on a line that exists (a tick, a removal) finds the line by its whole text, nearest
its old place; when the line is gone or changed, it writes nothing and says `That line changed on disk.`

**When an Undo ends.**

- **A note act:** its Undo shows while the note's text is the text that the act left, as the plan's `Undo
  remove` of a fight does today (`removed_fight`, `src/sessions/page.rs:4658-4680`). It has no time limit. Any
  other change of the text, in Splat or on disk, ends it, and so does a change of session. A switch on the
  act's own result (F7) keeps it.
- **A World note act:** until the entry's text changes or the reader shows another entry.
- **A Timeline act of this design** (`Undo checks`, `Undo roll`, `Undo clear`): until that event changes again
  or the DM leaves the page. It has no time limit. Today's `Undo add` of an event keeps its 30 seconds
  (`UNDO_ADD_FOR`, `src/sessions/page.rs:40`, `src/timeline/page.rs:13`).
- **`Undo delete encounter`:** until the DM leaves the Encounters page or deletes another encounter. It
  writes the file back only when no file has that path.
- **A run step and a ledger entry** stay, also after a restart.
- **An Undo shows where the DM acted,** also when the act changed a file that is not in view.

Each row is one new write, with the fragment as the file holds it. Longer fragments are in section 3.

| Act | The one file | The fragment | Undo |
|---|---|---|---|
| `Scene` (F3) | The session note | `- [ ] [[Captain Varra]]` under `## Plan for tonight` | `Undo scene` |
| Tick or untick a scene (F3) | The session note | `- [x] [[Captain Varra]]`: one character | The box, again |
| `Remove from tonight` on a scene (F3) | The session note | That line, out | `Undo remove` |
| `Start fight` in a reader (F3) | The encounter note; then the run starts and saves, the two saves of the editor's `Start fight` today | `Encounters/Fight with Captain Varra.md`; a run file | `Close run` removes an untouched Setup's run file, as today. The encounter stays |
| `Delete encounter` (F3) | The encounter note | The file, removed | `Undo delete encounter` |
| An effect: add, spend, give back, mark spent, recharge roll, remove (F2) | The run file | `{"name": "Frightened", "rounds": 10, "left": 9}` in the participant's `conditions` | The run's `Undo: <step>` |
| A tracker press, roll, `Done`, `Dismiss` (F4) | `Trackers.md` | One entry at the end of `changes` (F4). A press that changes two trackers writes two entries that share a `use`, in one save | `Undo: <label>`, step by step |
| `Add to Timeline` after a tracker act (F4) | `Timeline.md` | `- Major event: **Throw a gift** ^t-7d2c9a41be`, with its `Session` line | Today's `Undo add`, on the Trackers result line |
| `Carry over` (F5) | This session's note | The embed line or the copied recap, the copied lines, and `carried_from` with `carried_parts`, in one save (F5) | `Undo carry over` |
| `Update the link` (F5) | This session's note | The embed line | `Undo link` |
| `Add to Log` (F6) | The World entry's note | `- [[Session 12]], Day 3: Told the party who burned the ropes.` at the end of `## Log` | `Undo Log line` |
| `Add rewards`, and its switch (F7) | The session note | One list line that starts with the fight's reference, and its loot lines (F7) | `Undo rewards` |
| `Roll for encounters`; `No checks` (F9) | `Timeline.md` | `  - Checks table: road` in the event | `Undo checks`; later `No checks` |
| `Roll`, `Roll all`, `Enter a roll`, `Clear` (F9) | `Timeline.md` | `  - Check at 24h: 16 · [[Wolf Pack]] pick up the trail.` | `Undo roll`, `Undo clear`; later `Clear` |
| `Plan “<name>”` (F9) | The session note | `- [[Wolf Pack]]`, before the first line still to come | `Undo plan line` |

- **No act writes two files of the DM's.** `Start fight` saves the encounter and then starts the run, as the
  editor's `Start fight` does today; this design adds no second save to it.
- **No hidden state.** Each "is it done?" is read from the owner: `carried_from`; the line with the fight's
  reference; the plan's link; the newest block; the `Checks table` line; the notice's entry.
- **Only two stores have Undo after a restart:** the run (F8) and the tracker ledger (F4). A travel result is
  not words in a note, so it has `Clear`, and its block has `No checks`.

### 4.2 One fact, one owner, one place to act

"Acts on it" is the one place where the DM changes the fact. "Also shows" reads the same owner.

| Fact | Owner | The DM acts on it | Also shows, read-only |
|---|---|---|---|
| A fight's difficulty (F1) | None. Computed from the encounter note, the saved sheets and the rule entry | Nowhere. It follows the roster | The editor, the **Ready to run** row, the plan row, Arrivals |
| An effect, its time and its count (F2) | The run file, in the participant's one `conditions` entry | The effect field to add; the target on the row; **Conditions** to remove | The notice line on its turn; the Final state rows |
| A scene and its done mark (F3) | The session note, under `## Plan for tonight` | The plan row's box; the note | Obsidian |
| A tracker's value; a Due notice, its rolls and its end (F4) | `Trackers.md`, as entries. A round's notice also reads the run's round | The Trackers view; a new notice's action on the notice line | The rail's count; the bell; the house rule note's page |
| The recap of the last session (F5) | The last session's note, `## What happened` | That note | This note's `## Recap of last time`, by an embed; the **Last time** line |
| Which parts a note carried (F5) | The note's `carried_from` and `carried_parts` | `Carry over`, `Undo carry over` | Whether the **Last time** line has the action |
| A Log line (F6) | The World entry's note, `## Log` | `Add to Log`, or a **Change** | The session's **World changes** |
| A fight's rewards in the notes (F7) | The session note: the line with the fight's reference | `Add rewards`; then the note | The row's `In notes` |
| Whether the next block has XP (F7) | The newest rewards block in the session notes | The switch on the result; the note | What the next `Add rewards` writes |
| A fight's result and its Undo steps (F8) | The run file. `Timeline.md` holds the result line | `End encounter…`, `Undo`, `Continue fight` | The plan row, Timeline, History |
| A journey's length, table and checks (F9) | `Timeline.md`, in the event's lines | The Travel row, in the plan or on Timeline | The clock; Obsidian |
| That a check's fight is planned (F9) | The session note: the link under `## Plan for tonight` | `Plan “…”`; `Remove from tonight` | `In the plan` on the check |

### 4.3 Pairs that could show one fact twice, and which one wins

| Pair | The rule |
|---|---|
| F2 reminder and F4 Due notice | The notice is the campaign's fact, and the bell stays until `Done`. An effect that the DM types on a row is the DM's own second note. Splat never makes one from the other |
| F2 effect and an official condition | Words that are a condition's name become that condition, with the time. One entry holds the condition and its count, so a sheet that removes one removes both |
| F2 the row and the notice line | The row is the place to act. The notice line repeats its words |
| F2 the dock's field and the Conditions tab's field | One reader of the words, and both write the same entry. The Conditions tab's field keeps its `Add` button, as today (`view.rs:2430`) |
| F3 scene and a Timeline event | The scene is the plan. The event is what happened. A done scene with one entry offers `Add to Log`, not an event |
| F3 a scene that links an NPC and the fight with that NPC | The improvised encounter is named `Fight with <name>`, so the NPC's own name is never an encounter's name |
| F6 `Add to Log` and a Status change | A fact that a field holds is a **Change**, which writes its own Log line. `Add to Log` is for what no field holds |
| F4 quick action and a Timeline event | The ledger owns the change. `Add to Timeline` is a second, chosen act |
| F4 table roll and the run's Log | The ledger owns a quick action's roll and a notice's rolls. A roll from a note's own **Roll** button stays a run step, as today |
| F4 `Undo: <label>` and the run's `Undo` | Each names its own step and takes back only its own file's change. `U` is the run's |
| F4 a notice on the notice line and in Trackers | One notice, read from the ledger. `Done` in either place writes the same entry |
| F5 recap and the recording's summary | The embed of `What happened` wins. The summary is copied only when `What happened` is empty, and the embed can take its place later |
| F7 the rewards block and the run | The note owns the rewards after `Add rewards`. Splat does not compare them again. A stated limit |
| F7 XP and F1 XP | One function. F1 counts every creature against the party. F7 counts those at 0 HP and names the rest |
| F9 travel check and a roll in Lookup | The Travel row keeps its results in `Timeline.md`. A roll from Lookup's **Roll** button is a run step when a run is open, and otherwise only shows, as today |

### 4.4 One pattern for one act

Each act has one control, one set of words and one key, with Lookup open or closed. "Today" names where the
pattern is already in the app.

| Act | Control and words | Key | Today | Used by |
|---|---|---|---|---|
| Add a thing of a kind | A quiet icon-and-word action of the Add strip, or a line of its **More** menu: `Fight`, `Scene`, `Travel` | Tab, Enter | Fight, Group, rests, events | F3 `Scene` |
| Add typed words to a list | The add field: one field with no button. Enter adds. A suggestion adds by an arrow key and Enter, or a click. Escape clears and closes. The field stays open after Enter only where several lines follow each other | Enter, Escape | `Add loot` in the encounter editor | F2 the effect field, F3 scenes, F6 `Add to Log` |
| Put a record in the session note | A quiet action on the plan row, `Add …`. Then the status `In notes` in its place | Tab, Enter | `Add to notes` and the status `In notes` on event rows (`src/sessions/page.rs:4392-4394`) | F7 `Add rewards` |
| Start a fight | `Start fight` with the run icon: a bordered button on a row and in the editor, a quiet action in a reader's header | Tab, Enter | Plan, Encounters, editor | F3 in a reader |
| Mark a plan line done | A box at the start of the row; accessible name `Done: …` or `Open: …` | Space | New for a plan row | F3 scenes |
| Finish a notice | A quiet `Done` at the end of the notice's actions. It applies the ticked proposed changes | Tab, Enter | New | F4, in Trackers and on the notice line |
| Roll | A control with the dice icon. A result takes its place when the thing rolls once, and is listed under it otherwise. `Roll all N`; `Enter a roll` for a player's own roll | Enter | Tables in the Reader, loot, the dock | F2 a spent recharge, F4 notices, F9 checks |
| Name a table outside its own note | Its heading, from one descriptor for a marked table: source note, heading, key, dice, interval | | New. The Reader's own button says `Roll 1d20` and the loot editor's menu is as today | F4, F9 |
| Undo in a run | The header's one button, `Undo: <step>` | `U`, Ctrl+Z | Every run step | F2, F8 |
| Undo of a tracker act | The result line's first action, `Undo: <label>` | Tab, Enter | New | F4 |
| Undo anywhere else | A message that says what happened, with a quiet `Undo <noun>` on its line, where the DM acted. Section 4.1 says when it ends | Tab, Enter | Plan, Timeline, World, Library | `Undo scene`, `Undo carry over`, `Undo Log line`, `Undo rewards`, `Undo checks`, `Undo roll`, `Undo clear`, `Undo plan line` |
| Change a fact that a field holds | **Change** beside the value | Tab, Enter | Status, place, group, stat block | F6 uses it for a death |
| Remove | A line of the row's menu, with an Undo message | Tab, Enter | Fights and groups in the plan; `Delete the saved run` | F3 scenes and `Delete encounter`, F4 `Dismiss`, F9 `No checks` |
| Open a thing beside the page | A link with the thing's name | Tab, Enter; Alt+Left returns | Reader links, condition words | F3 scene links, F4 `Due N` and the bell, F9 the table's name |

## 5. Consistency

### 5.1 New controls and their components

No feature adds a colour or a type size. Three icons join `design::icons`: `PIN`, `BELL` and `CHECK_SQUARE`.
Section 3 names the component and token of each control in its check table. The shared parts land before the
features, in P1c, so that every user has one copy:

- `add_field`: the loot editor's add field with suggestions, with today's behaviour. Its two callers of today
  are checked against it.
- One descriptor for a marked table: source note, heading, key, dice and interval. Today `rolls_marker` gives
  only the key and the dice (`src/roll_table.rs:66-78`).
- Lookup's history holds a place: an entry, a found note or event, or the Trackers view.
- `Draft::apply` (section 4.1) and `played_session` (section 1.1).

### 5.2 Proposed text for `docs/design-system.md`

This pull request does not edit that file. Each pull request of the build plan adds its own lines there, and
the rule in this document is the text.

- **The live run** (Header, Roster, Dock, Panels): section 1.4 and F2. **The encounter ledger and editor:** F1.
- **The session's plan; Notes and templates:** F3, F5, F7, F9 and section 4.1. **Widgets:** the Add strip
  (F3) and `add_field` (section 4.4).
- **World; Lookup and copies; the rail:** section 1.2, F4 and F6.
- **Icons.** `PIN`: an effect with a time. `BELL`: Due. `CHECK_SQUARE`: Scene.

### 5.3 Proposed text for `docs/ux-intent.md`

**Changed and new workflows.** Pull request 232 has its own changes to W4 and to assumption A8.

| Workflow | Text to add | Done well |
|---|---|---|
| W5 | "The editor says how the roster compares with the party's XP budget, from the rules the campaign uses." | T5.7 |
| W11 | "A journey that the DM asks to check rolls its encounter checks in its own row, and a result becomes a planned fight in one step." | T11.9, T11.10 |
| W14 | "The plan also holds scenes, with a done mark. A creature's or an NPC's page starts a fight that nobody planned." | T12.10 to T12.12 |
| Section 3, Reachable from anywhere | "**Trackers:** Ctrl+T opens the trackers and the Due list in the Lookup panel, and goes back." | T19.1 |
| W15, new: Keep track of what lasts | "On each turn the DM sees what they asked to be reminded of for that creature, and how long it lasts, with no answer needed. Splat counts and prompts. The DM rules." | T17.1 to T17.6 |
| W16, new: Close the evening and open the next | "Rewards, what changed in the world, the recap and the open threads are where the next step needs them, with nothing typed twice." | T18.1 to T18.4 |
| W17, new: Run a house rule's numbers | "One press records a change, a due notice stays in view, and the history is a file." | T19.1 to T19.5 |

**New rows for section 8.**

| ID | Observable test | Scenario |
|---|---|---|
| T5.7 | The editor's roster line ends with the XP and a word from the rules data, wide and narrow. With no table, sheets or XP it is as today | encounter-difficulty **new** |
| T11.9 | A two-day journey rolls two checks in 1; each is a line of the event in `Timeline.md`. A shorter length deletes none. A result edited in Obsidian shows its words, and `Clear` removes it | travel-checks **new** |
| T11.10 | A result that names an encounter is planned in 1, and its `Start fight` is next. With several tables and no earlier choice, `Roll` waits for a choice. `No checks` removes the block | travel-checks **new** |
| T12.10 | A task line is a scene with a box. A tick changes one character. A plain list line gets no row. The empty plan is as today | session-plan-scenes **new** |
| T12.11 | A scene's link opens its entry in Lookup in 1, and Back returns | session-plan-scenes **new** |
| T12.12 | A reader starts a fight in 1. An NPC with no stat block opens the made-up form with its name and focus in HP. After `Close run`, also after a restart, the encounter is in **Ready to run**; `Delete encounter` and `Undo` remove and restore it | session-plan-scenes **new** |
| T17.1 | An effect with a count joins a row in ≤ 4, with the run's panel hidden; focus returns to Amount. With the longest name, the dock has no `Effect`, the name is whole, and the Conditions tab's field adds the effect | run-effects **new** |
| T17.2 | On its turn the effect's words show on the notice line and the row. A label shows on no notice. `Next turn` takes 1 and does not move | run-effects **new** |
| T17.3 | A count of 10 says 10, then 9, says `time is up` at 0, and removes nothing. Undo of `Next turn` gives it back, also after a restart | run-effects **new** |
| T17.4 | Case D, with a conflict on the longest name, three reminders that start with the same words, and a notice: the line holds the conflict, `⚑ 3`, the bell and `Undo`; the row holds each reminder whole; no row changes height at `Next turn`. Two more captures: the sheet-change summary; `No participants are present` | run-effects **new** |
| T17.5 | A spent recharge rolls in 1 and a use is spent in 1, on the row. A counted condition's name opens its rule | run-effects **new** |
| T17.6 | Damage to a `Conc` row adds the save DC at the end of its second line, with no action, notice or new line | run-effects **new** |
| T18.1 | A fight's loot reaches the note in 1, with its reference and one Undo. The row says `In notes`, also after the heading is renamed. The XP follows the newest block; the switch leaves an edited XP line in an older block. A title that fills the row has the action in the opened row, at the same row height | session-plan |
| T18.2 | A line reaches an entry's Log in ≤ 4 from a done scene, stamped as a **Change** at that moment. At 346 px both header actions are icons beside a 13-letter title, and every fixture title keeps its width | world-facts |
| T18.3 | `Carry over` adds the recap link, the plan lines (a paused fight has `Resume`) and the threads in 1, below the DM's words. A written recap is kept. A copied draft becomes the embed later. A deleted carried line stays out. Two flat properties | session-carry-over **new** |
| T18.4 | A note the DM did not act on does not change. A new session with no carry-over is the template | session-carry-over **new** |
| T19.1 | Ctrl+T opens Trackers in Lookup, and Ctrl+T again shows the entry that was open | trackers **new** |
| T19.2 | A quick action takes 1, also on an instance's line. `Undo` steps back through the acts of the session being played, also after a restart past midnight with the run open | trackers **new** |
| T19.3 | A value that passes a mark raises a Due row, the rail's count and the bell. No pinned quick action moves | trackers **new** |
| T19.4 | Case D, from the bell: three notices with four rolls and the result line are in view with no scroll | trackers **new** |
| T19.5 | A mark with `every: round` raises one notice at each new round: on the notice line for one turn with `Done`, then in Trackers. `Done` applies its change in 1 | trackers **new** |
| G6 | The unchanged evening of the audit gives the counts it gave before | dm-audit |
| G7 | The plain campaign differs from its reference only by rows of section 2.5 | plain-evening **new** |

**G7, the plain-campaign check.** One frozen campaign, with old task lines, old free-words effects, a marked
table and long names, is played by a script that uses no new control, at 1200x800 and 960x640, with Lookup
closed and open. For each screen the check compares four things with the reference: each control's and
text's **rectangle**, each row's **height**, the **words**, and the campaign's **files**. A difference passes
only when it is a row of section 2.5 that the pull request owns. The reference lands in P1a. Each feature's
pull request proves that the check can fail, with four mutations: the difficulty word forced onto a line of
its own (a height); `Add rewards` forced onto a row whose title then wraps (a rectangle); a line `E` added to
the run's **Keys** list (words); `carried_from` written at `Create session` (files).

### 5.4 Proposed terms for `CONTEXT.md`

- **Encounter difficulty**: the word that compares an encounter's XP with the party's XP budget: Low,
  Moderate, High or Above High. Splat shows it and blocks nothing. _Avoid_: Rating, challenge.
- **XP budget**: the XP a party can face at one difficulty, from the rules the campaign uses.
- **Effect**: words that the DM pins to one participant of a run. It can have a time (a count of rounds, or
  on its turn), a count of uses, or a recharge. It stays in that run. _Avoid_: Custom condition, timer.
- **Reminder**: the words of an effect that has a time, on the run's notice line during its participant's
  turn. It asks for nothing.
- **Scene**: a task line of a session's plan that is not a fight or a group, with a done mark. _Avoid_: Plan
  item, task, to-do.
- **Carry-over**: the act that adds the last session's recap link, plan lines that were not played and open
  threads to a session's note.
- **Rewards**: what the party received in a fight: its loot results and, where the notes use XP, its XP, in
  the session note. _Avoid_: Treasure.
- **Travel check**: one roll on a marked table for one day or watch of a Travel event, kept with that event
  by its offset from the journey's start.
- **Trackers**: the view of the Lookup panel that holds the trackers and the Due list.
- **Session being played**: the current session, while a fight is open, a recording is live, or its note is
  not dated before today.
- **Session plan**, change to: "The fights, groups and scenes a session's note lists under its plan, with
  what has happened so far." **Log**, add: "The DM can also add a line in their own words."
- The tracker terms of [World and house rules](world-and-house-rules.md) stand.

## 6. Build plan

Each pull request merges alone and leaves `main` working. `src/sessions/page.rs` and
`src/encounter_run/view.rs` are shared, so the plan first moves the parts that several features touch into
files of their own, and first builds the shared parts. **No feature merges before P1a,** because P1a holds
the reference of G7.

**Rules for every pull request.**

- Its visual-test scenarios cover each changed surface **with Lookup closed and with Lookup open, at
  960x640**, and once at 1200x800, with the long content of its wireframes. A new scenario goes at the end of
  `SCENARIOS`. The author looks at each new or wider menu and suggestion list in the 960 px capture.
- It extends an existing scenario or test when that one has the same setup, and it breaks the code once for
  each new check and says so.
- For each result that stays on screen and each Undo, it tests an act A, an unrelated act B, then Undo of A.
- For each new write it tests the path of section 4.1: an edit on disk a moment before, unsaved words with a
  changed file, a failed save, and a restart.
- It runs G6 and G7, with its mutation of G7.
- It adds its own lines to `docs/design-system.md`, `docs/ux-intent.md`, `CONTEXT.md` and `README.md`, from
  section 5, and its task to the evening in `src/visual_tests/dm_audit.rs` as a **new** task.

| PR | Scope | Files and modules | Depends on | Scenarios |
|---|---|---|---|---|
| **P0** | F8: merge pull request 232 at its head | As built | None | `run-continue`, as built |
| **P1a** | The audit. Repair the script (drop the task that clicks `Add to notes` on a played fight; keep setup acts apart from counted acts). Freeze it and measure `main`. Add the plain campaign, its script and the G7 reference | `src/visual_tests/dm_audit.rs`, `scenarios.rs`, `fixture.rs`, `artifacts/dm-audit/` | P0 | New `plain-evening` |
| **P1b** | No change in behaviour. Move the plan pane to `src/sessions/plan.rs`, and the run's dock and notice line to `src/encounter_run/dock.rs` and `notice.rs` | Those files | P0 | None new. `visual-compare` shows no difference |
| **P1c** | The shared parts of section 5.1. Today's two plan acts move onto `Draft::apply`. `current_link` and `party_bar_shows` call `played_session` (section 2.5, row 19). An entry that opens while an NPC is held shows under its row (row 17) | `src/sessions.rs`, `src/sessions/plan.rs`, `src/loot_editor.rs`, `src/design/components.rs`, `src/roll_table.rs`, `src/lookup.rs` | P1a, P1b | Extend `session-plan`, `run-loot`, `roll-table-author`, `npc-in-lookup`, `world-facts` |
| **P2** | F1 | New `src/difficulty.rs`. `src/encounter_settings.rs`, `src/encounters.rs`, `src/encounters/list.rs`, `src/sessions/plan.rs`, `src/encounter_run/view.rs` (Arrivals) | P1a, P1b | New `encounter-difficulty`. Extend `encounter-build`, `session-plan`, `encounter-groups` |
| **P3** | F3: scenes, `Scene` in **More**, `Start fight` in a reader's header, the made-up form's entry from it, `Delete encounter` | `src/sessions.rs`, `src/sessions/plan.rs`, `src/design/components.rs`, `src/timeline/page.rs`, `src/world.rs`, `src/encounters.rs`, `src/encounters/list.rs`, `src/encounter_run/ad_hoc.rs` | P1a, P1b, P1c | New `session-plan-scenes`. Extend `session-notes`, `npc-in-lookup`, `encounter-list` |
| **P4** | F2. One entry type for a condition: grep every reader of `conditions` (sync, Setup, the summaries, `undo.rs`) | `src/encounter_run.rs`, `src/encounter_run/undo.rs`, `dock.rs`, `notice.rs` (parts 1 to 3 of section 1.4), `view.rs`, `src/rolls.rs` | P1a, P1b, P1c | New `run-effects`. Extend `encounter-run`, `run-long-names`, `run-sync-conflict` |
| **P5** | F9 | `src/timeline.rs`, `src/timeline/formats.rs` (the event's new lines), `src/timeline/page.rs`, `src/sessions/plan.rs`, `src/roll_table.rs` (`per`) | P1a, P1b, P1c | New `travel-checks`. Extend `session-notes`, `roll-table-author` |
| **P6** | F6 | `src/world.rs` (the header band and the field), `src/world/lines.rs` (reuse `add_log`, `:459`), `src/sessions/plan.rs` | P1c, P3 | New `log-from-play`. Extend `world-facts` |
| **P7** | F7 | `src/sessions.rs` (the block, the search for the newest block), `src/sessions/plan.rs`, `src/encounter_run/loot.rs`, `src/difficulty.rs` | P2, P3 | Extend `run-loot`, `session-plan` |
| **P8** | F5, with rows 11, 16 and 18 of section 2.5 | `src/sessions.rs`, `src/sessions/page.rs` (`last_time_block`, `recap_preview`), `src/markdown_view.rs` | P3 | New `session-carry-over`. Extend `session-plan` |
| **P9** | F4, part one: house rule notes, in play, the ledger with its own save and its Undo, quick actions, instances, the Trackers view, Ctrl+T | New `src/trackers.rs`, `src/trackers/ledger.rs`, `src/trackers/view.rs`. `src/lookup.rs`, `src/lib.rs`, `src/campaign_content.rs`, `crates/core` | P1a, P1c | New `trackers`. Extend `run-lookup`, `nav-state` |
| **P10** | F4, part two: marks, `every: round`, the Due rows, the rail's count, parts 4 and 5 of the notice line, `Add to Timeline`, and the two additions to the Tournament example | `src/trackers.rs`, `src/trackers/view.rs`, `src/encounter_run/notice.rs`, `src/lib.rs` | P9, P4 | Extend `trackers`, `run-effects` |
| **P11** | F4, part three: day marks | `src/trackers.rs`, `src/timeline.rs` | P10 | Extend `trackers` |
| **P12** | The audit, again. Extend the evening to use every feature, measure it, and add the scripts for J2, J3, J4 and J6. Run G6 and G7 on the whole | `src/visual_tests/dm_audit.rs`, `artifacts/dm-audit/` | All | The audit's captures, and `plain-evening` |

**What can be built at the same time.**

- **First:** P0; then P1a and P1b at once, which share no file; then P1c.
- **Wave A, three at once:** P2, P3, P5. They meet in `src/sessions/plan.rs`, `src/timeline/page.rs` and
  `src/encounters/list.rs`, each in its own function.
- **Wave B, three at once:** P4, P6, P9. P4 and P9 share no file.
- **Wave C, two at once:** P7 and P8. Both call `Draft::apply`, so neither adds a save of its own.
- **Then:** P10, P11, P12, in order. The full gate runs on merged source after each wave.

## 7. Limits

Each is a limit that this design keeps and states.

- **An overlapping edit is not merged** (today). When Splat holds unsaved typed words for a note and the file
  changed, the comparison of today offers one version or the other. A feature's act does nothing until the
  DM has chosen (section 4.1).
- **Lookup's place is not kept across a restart** (today), and a held form is lost with it. J3 counts the
  search. What Lookup's reader shows for a note that changed on disk is as today; P1c records it in a test,
  so that no feature changes it.
- **The rewards block does not follow the fight** after it is written (F7).
- **A reader's header has no room for its two actions** at case D beside a title whose widest line has more
  than about 13 letters. The DM then has today's routes (section 1.2).
- **The race costs 13 more acts than paper** on the walk of section 2.4 (16 more at case D), for a saved
  history and raised notices.
- **A closed, ended fight goes on in 2 or 3 interactions,** and the other limits of F8.
- **The typed route to a scene costs a tab change** where the note is behind its tab (cases B, C and D).

**Two rulings of round 2, as this document reads them.**

1. **Rule L counts a click on a row's `Delete`.** The app has no action that deletes an encounter. This design
   adds the menu line `Delete encounter` (F3), in a menu that exists.
2. **Rule I reads "the newest rewards block".** This design passes a note that has no block, and stops at a
   note that cannot be read. So a campaign that uses XP does not choose again in each session, and Splat
   never looks past a note that it cannot read.

No question is open. While a run is open, a reader has no `Start fight` action.
