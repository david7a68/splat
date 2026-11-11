# The session cycle

Status: proposed, round 2. Nothing here is built, except F8 (pull request 232) and where a line says "today".
This document fits nine features (F1 to F9) into the app as one workflow: prep at the desk, play at the table,
the wrap-up, and the next prep. It follows [the brief](../ux-intent.md), [the design system](../design-system.md)
and the words in [`CONTEXT.md`](../../CONTEXT.md). The yardstick is the measured evening in
[`artifacts/dm-audit/`](../../artifacts/dm-audit/README.md): 13 page transitions and 231 interactions.

Evidence: `file:line` refers to `main` at `20fd453a`, and to pull request 232 at `b305c085` where it says so.

**How to read this.** Section 1 holds the rules that every feature obeys. Section 2 walks six evenings step by
step. Section 3 has one part for each feature. Section 4 holds the tables that bind the features together:
each new write, each fact's owner, each pair that could show a fact twice, and one pattern for each act.
Sections 5 to 7 hold the proposed text for other documents, the build plan and the limits.

## Decisions in brief

| # | Feature | Where it lives | What the DM does |
|---|---|---|---|
| F1 | Encounter difficulty | A word at the end of a line that exists: the editor's roster heading, a planned fight's summary, a group's line in Arrivals. One function computes it from the rules data | Nothing. It is in view |
| F2 | Effects and turn reminders | Words on the roster row, where the DM also acts on them. The notice line echoes the ones that have a time. One dock control, `Effect` | Select, `E`, type `Bless 10 rounds`, Enter |
| F3 | Plan items | A task line under `## Plan for tonight` is a plan row with a done box: a **scene**. `Start fight` on a creature's or an NPC's reader | Type the line in the note. Click the box when it is done |
| F4 | Trackers and the Due list | One view of the Lookup panel, **Trackers** (Ctrl+T, and Ctrl+T back). The rail says `Due N`. The run's notice line ends with the bell and the count | Click a quick action. Tick a notice |
| F5 | Carry-over | `Carry over` on the Session page's **Last time** line, while a part is still to carry | One click adds the recap link, the unplayed plan lines and the open threads |
| F6 | Wrap-up to World | `Add to Log` among the reader's header actions, beside `Edit` | Click, type the words, Enter |
| F7 | Rewards | `Add rewards` on a played fight's plan row. The note owns the rewards | One click writes the fight's loot under `## Loot and rewards` |
| F8 | Saved Undo and Continue | Built in pull request 232: the run's Undo button, and `Continue fight` on the ended view | `U` after a restart. One click to go on with a fight |
| F9 | Travel checks | The Travel row, after `Roll for encounters` in its menu | Type `2 days`, `Roll all 2`, `Plan “Wolf Pack”` |

No feature adds a page, a panel, a tab, a strip or a dialog.

**Counts, by hand against `steps.tsv`** (section 2.1): the measured tasks go from 13 page transitions and 231
interactions to 12 and 226. Seven records that were on paper before cost 15 more. The audit script has
drifted from `steps.tsv`, so the app's own numbers for `main` and for each feature are to be measured in P1
and P12.

## 1. Rules for every feature

### 1.1 Eight rules

1. **No new page, panel, tab, strip or modal.** Each feature joins a row, a line or a view that exists. The one
   new view is Trackers, and it is a view of the Lookup panel.
2. **The plain evening is a contract.** A DM who uses no new control sees no new notice, count, heading,
   status, step or line. Two things are allowed. **Passive words** at the end of a line that exists, which
   answer the question that line already answers, move and cut nothing, want no answer, and do not show when
   their inputs are not known. **A quiet action** in a place that already holds actions for that thing, never
   primary, with nothing moved. Section 2.5 lists every such difference, and it is the full list.
3. **One act writes one file, as a patch of the file on disk now.** After the write, the fragment is plain
   Markdown or one frontmatter key in that file. Nothing outside the file remembers that the act happened.
   Section 4.1 has one row for each new write.
4. **Every act has a named Undo where the DM acted.** The run's Undo and the tracker ledger's Undo survive a
   restart. A change to a note has Undo until the next edit of that note or until the DM leaves, as note acts
   have today. After that the DM removes the words as any words.
5. **An identity is an ID or an offset, never a label.** A session is its ID, a fight is its Timeline
   reference, a travel check is its offset from the journey's start. A name is for reading.
6. **One file owns each fact.** A second place shows it by reading the owner. Section 4.2 names each owner.
7. **Splat prompts. The DM rules** (principle 7). A count goes down, a notice shows, a roll is offered. Splat
   never removes an effect, spends a use, ends a fight, plans a fight or starts one by itself.
8. **`Next turn`, `Amount` and the current row never move, wait or lose focus** because of a feature. A prompt
   needs no action to go away.

### 1.2 The side panels: the rule today, and the rule for new features

Three things want the space beside a page. This is how the app decides today.

| Thing | Opens with | Rule today | Evidence |
|---|---|---|---|
| **Lookup** | Ctrl+K, a link, a condition word | One panel beside the page at a time. When Lookup opens, Notes give way, and they come back when Lookup closes. Ctrl+K with Lookup open closes it | `src/lib.rs:1229-1238`, `:2457-2460`; `src/lookup.rs:1575` |
| **Notes** | Ctrl+J | When Notes open, Lookup closes. On the Session page the panel does not show, because the page has the note | `src/lib.rs:1240-1243`, `:2469` |
| **The run's own panel** (Stat block, Conditions, Correct, Log, Prep, Arrivals) | A row, `C`, **Add to fight** | It is part of the Run page. It stays beside Lookup or Notes only when the roster keeps 560 px. With less, it hides, and the current stat block's action tiles go in one line above the dock | `src/lib.rs:1888-1892`, `src/encounter_run/view.rs:665`, `:924-932` |
| A run task the DM asks for (Conditions, Correct, Arrivals, Log, Keys, Review conflict) | A click or key | Lookup closes. Notes close only when they are what hides the run's panel. **Review** also selects the conflict's row | `src/lib.rs:1915-1925`, `src/encounter_run.rs:2364-2372` |
| Keys in a run | | A field with focus takes its own keys. With Lookup or Notes open, the letter keys (`N`, `U`, `A`, `C`) are off, and Enter and the arrow keys work only while `Amount` has focus. The buttons still work, and a button shows its key letter only while the key works | `src/encounter_run/view.rs:411-419`, `:444`, `:2115` |
| After a restart | | Lookup's search, entry and Back history are not kept, and a held form is lost (`docs/ux-intent.md`, section 3). This design does not change that | |

**The rule for F1 to F9: no feature adds a contender for this space, and no feature adds a key rule.**

- A feature that needs room beside a page is a view of Lookup. There are two: an entry's reader with
  **Add to Log** (F6), and **Trackers** (F4). The old design's separate tracker panel is cut.
- **Lookup's history holds places.** Today it holds entries only (`src/lookup.rs:1662`). A place becomes an
  entry or the Trackers view, so that Back can return to either. This lands before the features that need it
  (P1c in the build plan).
- **Ctrl+T goes there and back.** With Lookup closed, it opens Lookup on Trackers, and a second press closes
  Lookup. With Lookup on an entry, it goes to Trackers, and a second press goes back to that entry.
- **Keys beside Trackers are the keys beside any Lookup view.** The run's letter keys are off, as the table
  says. A tracker's typed amount holds focus until Enter or Escape. `U` is the run's key and never takes back
  a tracker change. In a fight the DM seldom needs Trackers open: the bell, the count and the action for each
  round are on the run's notice line (section 1.4).
- A feature opens Lookup only when the DM clicks or presses a key. It uses Lookup's own open path
  (`open_position`, `src/lookup.rs:1619`), so Back returns to what the DM had open, and an unsaved edit form
  stays with its entry (`src/world.rs:1476`).
- **One repair.** While Lookup holds an unsaved generated NPC, its card hides any entry that opens
  (`src/lookup.rs:2178`). Lookup takes the World page's rule: one row above the reader says
  `Unsaved NPC: <name> · Show`, and the opened entry shows below it. The NPC stays until Save or Discard.

### 1.3 Widths, and what never hides

These widths come from `RAIL_WIDTH` (136), `side_panel_width` (36% between 340 and 560) and two constants of
the Session page, `PLAN_BESIDE_NOTE_FROM` (720) and `TWO_COLUMNS_FROM` (640). Every wireframe in section 3
names one of these four cases.

| Case | Window | Lookup | Page width | Session page | Run page |
|---|---|---|---:|---|---|
| A | 1200x800 | Closed | 1040 | Session list, plan (330) and note side by side | Roster 596, run panel 432 |
| B | 1200x800 | Open (432) | 608 | No session list. **Plan** and **Note** are tabs | Roster 608. Run panel hidden. Tiles in one line above the dock |
| C | 960x640 | Closed | 800 | Session list. **Plan** and **Note** are tabs | Roster about 446, run panel 346 |
| D | 960x640 | Open (346) | 454 | No session list. Tabs. The title has its own line | Roster 454. Run panel hidden. Tiles above the dock |

The party bar is a band above the page and above Lookup, so Lookup does not narrow it (`src/lib.rs:1492`).

| Never hidden, in any case A to D | Where it is | What gives first |
|---|---|---|
| `Next turn`, round and current turn | Run header, line one | The turn heading drops `’s turn`, then shortens the name (today's rule) |
| `Amount`, `Damage`, the dock's first line | Dock | Nothing new is on that line |
| Each reminder for the current turn (F2) | The current row, whole or in its short form | Section 1.4 |
| That something is due (F4) | The rail's `Due N`. In a run, the bell and the count at the end of the notice line | Nothing. Every other part of the notice line gives first |
| A scene's done box (F3) | The start of its row | The words wrap to at most three lines |
| `Undo` in a run | Run header, line two, right | The target shortens, then the `U` letter drops (today's rule) |

The difficulty word (F1) is passive words, so it gives way before it cuts anything that shows today. Where a
summary line has no room for it, the line's hover says it.

### 1.4 The run's notice line and a row's effect words: one order

The notice line is the left part of the run header's second line; `Undo` is at its right. Several features
want it. This table is the one rule. Parts show left to right in this order.

| # | Part | Shows when | Words | When the line is short |
|---|---|---|---|---|
| 1 | A sync conflict, recording silence, another sync notice | As today. A conflict comes first | As today | It never gives room to a later part. When two of these meet, they take the short forms that they take today |
| 2 | The current turn's reminders (F2) | The current participant has an effect with a time | `⚑ Oren: Frightened, 9 rounds left · Wis save DC 15 (Hold Person)` | First each count takes its short form (`Frightened 9`). Then each later reminder keeps its first two words. Then all become a count, `⚑ 2`, whose hover and accessible name hold each reminder whole. The row has each one |
| 3 | A Due notice's words (F4) | For one turn: from the act that raised it until the next `Next turn` | `Due: Bell. The Cistern floods.` | It loses words from its start and keeps its last sentence. Then it goes, and part 6 stays |
| 4 | The action for each round (F4) | On the first turn of a new round, while a tracker in play has a quick action marked `each: round` | The link `Round 3: Fight minute`, while the ledger has no entry for this run and round. After the click, the words `Fight minute: 43` until the next `Next turn` | It drops `Round 3:`. Then it goes; the action is in Trackers |
| 5 | `Next: <name>` | As today | As today | It is the first to go, as today |
| 6 | The bell and the count (F4) | While a notice is due | `🔔 1`, a link to Trackers | It never gives |

- **Order of giving:** 5, then 3, then 4, then 2 down to its count. Parts 1 and 6 never give. `Undo` gives up
  its target before part 2 shortens, as it does beside a state notice today.
- **No part has a control that asks for an answer.** Part 4 is one optional click. Parts 2 and 3 only say.
- **The ended view has no notice line.** There the rail's `Due N` is the sign that something is due.
- **Proof at case D with a conflict.** The roster is 454 px. `Undo` with no target takes about 70 px, so the
  notice line has about 368 px. `⚠ Conflict: Kestrel  Review` takes about 190, `⚑ 2` about 40 and `🔔 1`
  about 40, with two 12 px gaps: 294 px. Parts 3, 4 and 5 have gone. The current row holds each reminder, so
  nothing that the DM must do this turn is out of view. The build plan tests this capture (T17.4).

**A row's effect words.** They sit on the row's second line, with its conditions, as today.

| Rule | What it means |
|---|---|
| An effect with a time comes before a plain condition and is never folded into `N more` | Plain conditions fold first, as today |
| Each timed effect shows whole while the row's timed effects fit on two lines | `Bless 9 · Wis save DC 15 (Hold Person) on its turn` |
| When they do not fit on two lines, each takes its short form: a count keeps its name and number, and other words keep their first two words and `…` | `Bless 9 · Wis save… · Poisoned 2`. Hover and the accessible name keep each whole |
| A row's height comes from its own effects, and never from being the current row | No row grows or shrinks at `Next turn` |
| Three reminders in short form fit one line at 454 px | The tested overflow case (T17.4) |

## 2. The evening, end to end

Each journey below uses the step fields of `artifacts/workflow-score/journeys.md`. **Count** uses the kinds of
the audit: `Nav` (go to a page), `Sub` (a tab in a page), `Panel` (open or close a side panel), `Find` (search
or select), `Do` (the work). A page transition is marked **T**. "Must remember" is **nothing** for every row
unless the row says so. Every count is a hand count from this document and the source.

### 2.1 J1: the measured evening, with each feature where a DM would use it

Window 1200x800 (case A, and case B while Lookup is open). Rows with a feature label are new or changed. All
other rows are the 41 measured tasks of `steps.tsv`, with the same actions and counts.

**Before: prep at the desk**

| # | The DM wants | Surface | Action | Count | `steps.tsv` | File |
|---|---|---|---|---|---:|---|
| 1 | To start tonight's note | Rail, then Session | `Session`; `New session`; type the title; `Create session` | Nav 1, Do 3 = 4, **T** | 4 | A new session note from the template |
| 2 | **F5.** What the last session left | Session, the **Last time** line | `Carry over` | Do 1 | New | The new note: one embed line, the unplayed plan lines, the open threads, and `carried_from` |
| 3 | To recall last session | Session, **Last time** | Open **Last time** | Find 1 | 1 | None |
| 4 | To compare passive Perception | Party | `Party` | Nav 1, **T** | 1 | None |
| 5 | To build tonight's encounter. **F1:** the roster heading ends with `· XP 550 of 2,500 · Low` as the roster grows | Session, then the encounter editor | As measured | Nav 2, Find 6, Panel 1, Do 6 = 15, **T T** | 15 | The encounter note |
| 6 | To make the boss their own | Editor | As measured | Do 6 | 6 | The creature copy and the encounter |
| 7 | To write the read-aloud text | Editor | As measured | Do 3 | 3 | The encounter note |
| 8 | To plan both fights. **F1:** each row's summary ends with its word | Session plan | As measured | Nav 1, Find 1, Do 2 = 4, **T** | 4 | The note: two links under `## Plan for tonight` |
| 9 | **F3.** To plan a talk with Captain Varra and a secret | Session, the note beside the plan | Click under `## Plan for tonight`; type `- [ ] [[Captain Varra]]`; type `- [ ] The miller is the informer` | Do 3 | New. On paper: 2 lines | The note: two task lines. The plan shows two scenes |

Row 9 by the pointer costs 7: `More Add actions`; `Scene`; type `varr`; ↓; Enter; type the second line; Enter.

**During: at the table**

| # | The DM wants | Surface | Action | Count | `steps.tsv` | File |
|---|---|---|---|---|---:|---|
| 10 | To start recording | Session | As measured | Do 2 | 2 | The recording |
| 11 | To read the recap aloud | Session | Open **Last time** | Find 1 | 1 | None |
| 12 | **F3.** A player asks who Captain Varra is | Session plan, the scene's row. Lookup opens (case B) | Click `Captain Varra` in the row; close Lookup | Panel 2 | 3 | None |
| 13 | To note what the party decides | Session, the note | As measured | Panel 1, Do 1 = 2 | 2 | The note |
| 14 | **F3.** To mark the talk done | Session plan, the scene's row | Click the box | Do 1 | New. On paper: 1 tick | The note: `- [x] [[Captain Varra]]` |
| 15 | To record half a day of travel | Session plan | `Travel` | Do 1 | 1 | `Timeline.md` |
| 16 | **F9.** To check for an encounter on the road | Session plan, the Travel row | The row's `⋯`; `Roll for encounters`; `Roll` | Find 1, Do 2 = 3 | New. Today in Lookup: 4 | `Timeline.md`: the journey's table and one check |
| 17 | To read the read-aloud text and start the fight | Session, then Run setup | As measured | Nav 1, Do 11 = 12, **T** | 12 | The run file. The timeline event |
| 18 | Round 1 | Run | 8 turns: select, type, Enter, `Next turn` | Find 8, Do 24 = 32 | 32 | The run file |
| 19 | To read Fireball | Run, Lookup | As measured | Panel 2, Find 1 = 3 | 3 | None |
| 20 | Fireball on three enemies | Run | As measured | Find 3, Do 3 = 6 | 6 | The run file |
| 21 | **F2.** A wolf frightens Oren for 10 rounds | Run, dock | Select Oren; `E`; type `Frightened 10 rounds`; Enter | Find 1, Do 3 = 4 | 3, with no count | The run file: the condition with its count |
| 22 | To read what Frightened does | Run | Click `Frightened` on the row; close Lookup. The name still opens its rule | Panel 2 | 2 | None |
| 23 | To mark Concentration on Iselle | Run | As measured | Find 1, Do 1 = 2 | 2 | The run file |
| 24 | Round 2. **F2:** on Oren's turn the notice line says `Oren: Frightened, 10 rounds left`. When Iselle is hit, her row says `Conc · save DC 10` | Run | 8 turns, as measured | Find 8, Do 24 = 32 | 32 | The run file |
| 25 | To check Iselle's spell save DC | Run | As measured | Find 2, Sub 1 = 3 | 3 | None |
| 26 | To roll a save for a monster | Run, dock | As measured | Do 4 | 4 | The run's Log |
| 27 | To read a monster's reaction | Run | As measured | Find 2 | 2 | None |
| 28 | What the miller said | Run, Lookup | As measured | Panel 2, Find 1 = 3 | 3 | None |
| 29 | To note a moment | Run, Notes | As measured | Panel 2, Do 1 = 3 | 3 | The note |
| 30 | Round 3 | Run | 8 turns, as measured | Find 8, Do 24 = 32 | 32 | The run file |
| 31 | To take a dead wolf out of the order | Run | As measured | Find 1, Do 1 = 2 | 2 | The run file |
| 32 | To end the fight | Run, then Session | As measured | Do 3, Nav 1 = 4, **T** | 4 | The run file. The timeline result |
| 33 | **F7.** To put the fight's rewards in the note, with XP | Session plan, the played fight's row | `Add rewards`; `Add XP` on the result, once for the campaign | Do 2 | New | The note: a block under `## Loot and rewards`, with the fight's link |
| 34 | To record a short rest | Session plan | As measured | Do 1 | 1 | `Timeline.md` |
| 35 | To start the next fight | Session, then Run | As measured | Nav 1, **T** | 1 | The run file |
| 36 | To roll and begin | Run setup | As measured | Do 11 | 11 | The run file |
| 37 | To end it | Run, then Session | As measured | Do 2, Nav 1 = 3, **T** | 3 | The run file |
| 38 | **F3.** A fight nobody planned, with a Bandit from the book, then talk | Session, Lookup, Run | Ctrl+K; type `bandit`; `Start fight`; close the run again, as measured; close Lookup | Panel 2, Find 1, Nav 3 = 6, **T T** | 9, **T T T** | None. The Setup that nobody touched closes to nothing (F3) |
| 39 | To stop recording | Session | As measured | Do 1 | 1 | The recording |

**After: the wrap-up**

| # | The DM wants | Surface | Action | Count | `steps.tsv` | File |
|---|---|---|---|---|---:|---|
| 40 | To review the fight | Session plan | As measured | Find 1 | 1 | None |
| 41 | To say where it happened | Session | As measured | Sub 1, Do 3 = 4 | 4 | `Timeline.md` |
| 42 | To process the recording | Session, Recordings | As measured | Sub 3, Do 1 = 4 | 4 | The transcript and summary |
| 43 | To add a suggested event | Session, Recordings | As measured | Do 1 | 1 | `Timeline.md` |
| 44 | To write the wrap-up | Session, the note | As measured | Sub 1, Do 3 = 4 | 4 | The note |
| 45 | To link the fight in the note | | Nothing more. `Add rewards` in rows 33 and 46 writes the fight's link | 0 | 2 | |
| 46 | **F7.** To add the second fight's rewards | Session plan, the played fight's row | `Add rewards` | Do 1 | New | The note: a second block. The result says `XP tonight` |
| 47 | **F6.** To put what Varra said in her note | Session plan, the done scene's row. Lookup opens (case B) on her page, with the Log field open | `Add to Log`; type `Told the party who burned the ropes.`; Enter; close Lookup | Panel 2, Do 2 = 4 | New | `NPCs/Captain Varra.md`: one line under `## Log` |
| 48 | To check the party's HP | Party | As measured | Nav 1, **T** | 1 | None |

**Totals for J1, by hand against `steps.tsv`**

| | Transitions | Interactions | Nav | Sub | Panel | Find | Do |
|---|---:|---:|---:|---:|---:|---:|---:|
| `steps.tsv`, 41 tasks | 13 | 231 | 14 | 7 | 12 | 49 | 149 |
| The same tasks, after | 12 | 226 | 13 | 6 | 14 | 47 | 146 |
| Seven new records (rows 2, 9, 14, 16, 33, 46, 47) | 0 | 15 | 0 | 0 | 2 | 1 | 12 |
| **After, in all** | **12** | **241** | 13 | 6 | 16 | 48 | 158 |

- **Four measured tasks change.** Row 12 falls from 3 to 2: the planned NPC is one click from the plan. Row 21
  rises from 3 to 4. **Reason:** the DM types the number of rounds, and no design can take a number with no
  interaction. A DM who wants no count uses **Conditions** as today, for 3. Row 38 falls from 9 to 6 and
  loses a page transition. Row 45 falls from 2 to 0.
- **Overhead does not grow for the measured tasks:** 82 before, 80 after. The new records add 3 overhead
  interactions and 12 of work.
- **Table time:** the 180 measured interactions become 177. The DM adds 6 for three records (rows 14, 16, 33).
- **Against paper.** Rows 2, 16, 33, 46 and 47 cost less than their paper or copy-and-paste form. Rows 9 and
  14 cost what they cost on paper, plus one click to put the caret in the note.
- **The audit script is not `steps.tsv`.** On `main` the script no longer clicks **Last time** (rows 3 and 11,
  `src/visual_tests/dm_audit.rs:162`, `:279`). It clicks `More Add actions` before `Travel` and `Short rest`
  (rows 15 and 34, `:297`, `:422`). It reaches `Duplicate` and `Make it yours` through menus (`:184`). Its
  task for row 45 clicks `Add to notes` on a played fight, and a played fight has had no such control since
  pull request 39 (`src/sessions/page.rs:4387`). So this document gives no total for `main`. **To be measured
  in P1**, after the script is repaired and frozen.

### 2.2 J2: the improvised evening

Only the steps where a feature acts. Setup and turns cost what they cost today: 28 interactions for the two
Setups, one recorded hit and one ending, on both sides.

| # | The DM wants | Surface | Action | Count | Today | File |
|---|---|---|---|---|---:|---|
| 1 | To skip the first planned fight | Session plan | Nothing. The row stays | 0 | 0 | None. F5 can carry the unplayed line to the next session |
| 2 | **F3.** To play the planned NPC | Session plan, the scene's row | Click `Captain Varra`. Lookup opens on her page | Panel 1 | 2: Ctrl+K, type | None |
| 3 | **F3.** The party attacks her | Lookup, her reader; then Run setup | `Start fight`; close Lookup, to see her stat block | Nav 1, Panel 1 = 2, **T** | 8, **T T**: close Lookup, `Fight`, `New encounter`, name, to the search, search, add, `Start fight` | An encounter note `Fight with Captain Varra`, and the run file, as the editor's `Start fight` saves them today |
| 4 | **F6.** She gave up, and the party holds her | Run, ended view; Lookup | Ctrl+K, which shows her page again; `Add to Log`; type `Gave up at the ford. The party holds her.`; Enter; close Lookup | Panel 2, Do 3 = 5 | 7: Ctrl+K, type, `Edit`, click the text, type, `Save changes`, close | `NPCs/Captain Varra.md`: one Log line |
| 5 | **F7.** To close, keep the rewards, and add loot nobody planned | Run, then Session plan and the note beside it | `Close run`; `Add rewards`; click under the new block; type `Varra’s signet ring` | Nav 1, Do 3 = 4, **T** | 4: Notes, type, close Notes, `Close run`. The words go under another heading | The note: the fight's block, and one line under it |
| 6 | **F9.** Two days overland | Session plan, the Travel row | `More Add actions`; `Travel`; the row's `⋯`; `Roll for encounters`; click **How long**; type `2 days`; `Roll all 2` | Find 2, Do 5 = 7 | 12, **T T**: `More Add actions`, `Travel`, open the row, `Timeline`, click, type, `Session`, Ctrl+K, type, `Roll` twice, close | `Timeline.md`: the length, the table, two checks and their rows |
| 7 | **F9.** The second day's result names Wolf Pack | The Travel row, then the plan, then Run | `Plan “Wolf Pack”`; `Start fight` on the new row, which is the first to come and the primary | Do 1, Nav 1 = 2, **T** | 3, **T**: `Fight`, pick, `Start fight` | The note: `- [[Wolf Pack]]` above the skipped fight. The run file |

| J2, these steps | Transitions | Interactions | Nav | Panel | Find | Do |
|---|---:|---:|---:|---:|---:|---:|
| Today | 6 | 36 | 6 | 8 | 8 | 14 |
| After | 3 | 21 | 3 | 4 | 2 | 12 |

- **Today's 36 leaves a smaller record.** The loot is prose under another heading, the second road roll
  replaces the first on screen, and neither roll is saved.
- **W7.** The fight that nobody planned costs one click, as a planned one does, and leaves the same record.
  **A wrong click** on `Start fight` costs one more: `Close run` on a Setup that nobody touched removes the
  run, as today, and with it the encounter that this click made (F3).
- **Step 3 with no stat block.** The NPC joins Setup as a made-up creature with her name. Setup says that
  she needs HP, as it does today, and the DM types it there. No question comes before Setup.
- **Step 4 in general** costs one more: Ctrl+K, then the name, when Lookup's last entry is another one.

### 2.3 J3: the broken evening, at 960x640 with Lookup open (case D)

The start: fight 2, round 2. Lookup shows a session note found by the search `miller`. Sync is on. A tracker
has one notice due. The current and selected row is Cinder Wolf 2.

| # | What happens | What the DM sees | Recovery | Count | File |
|---|---|---|---|---|---|
| 1 | The DM ends the fight by mistake | The ended view, 454 px wide. The header says `Undo`; its hover and accessible name say `Undo: end encounter`. No `Continue fight` shows, because Undo does the same | Click `Undo`. `U` is off while Lookup is open. Play returns to the same turn of round 2 | Do 1 | The run file is at round 2 again. The timeline result says `Round 2` |
| 2 | The DM pins `Poisoned 3 rounds` to the wolf. The app stops in round 3 | After a restart the app opens on the run, on the current turn. The row says `Poisoned 2`. On the wolf's turn the notice line says it. `Undo` names the last saved step | Nothing for the run | 0 | The run file holds the effect, its count and the Undo steps. A step whose save did not finish is lost; while saving fails, the footer says so, as today |
| 3 | Lookup is closed after the restart, and its place is not kept (a limit, section 7) | The run, with no Lookup | Ctrl+K; type `miller` | Panel 1, Find 1 = 2 | None |
| 4 | A sync conflict opens on Kestrel. The wolf's reminder is due, and the notice is still due | The notice line: `⚠ Conflict: Kestrel  Review · ⚑ 1 · 🔔 1`. The wolf's row says `Poisoned 2`. Kestrel's row says `Conflict` | `Review`; choose. **Review** closes Lookup and selects Kestrel, as today. Then: select the wolf; Ctrl+K, which shows the note again | Do 2, Find 1, Panel 1 = 4 | The run file. The ledger does not change |
| 5 | The DM edits the session note in Obsidian: a tick, a new plan line | On the Run page nothing opens. Splat held no unsaved words, so it takes the file's text. The plan shows the tick and the new line when the DM next looks | Nothing | 0 | The note is as Obsidian wrote it |
| 6 | The same, while Splat holds unsaved typed words for that note | The rail says `Notes changed`. On the Session page the comparison of today takes the note's place, and the plan waits. A plan act says `The note changed on disk. Choose in the note first.` and does nothing | One choice on the Session page, or in the Notes panel beside the run (Ctrl+J), which closes Lookup. It keeps one version of the lines that both sides changed (a limit of today, section 7) | Nav 2, Do 1 = 3, **T T** | The note holds the chosen version |

| J3, recovery | Transitions | Interactions | Panel | Find | Do |
|---|---:|---:|---:|---:|---:|
| Steps 1 to 5, the clean case | 0 | 7 | 2 | 2 | 3 |
| Today, the same steps | 0 | 7 | 2 | 2 | 3 |

Today has no saved Undo on `main`, no count on `Poisoned` and no reminder; pull request 232 gives the first.
Step 6 adds 3 on both sides. Nothing in this journey is modal, and nothing takes focus from `Amount`.

### 2.4 J4: the race

The Tournament of [the example](world-examples/heralds-tournament.md). Its note is in play. Between fights the
DM is on the Session page with Trackers open (case B). In the fight, Lookup shows a stat block.

| # | The DM wants | Surface | Action | Count | Must remember | On paper today |
|---|---|---|---|---|---|---|
| 1 | To open the numbers | Session | Ctrl+T | Panel 1 | Nothing | Find the sheet of paper |
| 2 | The party enters a room | Trackers | `Enter room` | Do 1 | Nothing | 1 tally |
| 3 | The clock passes 40. A bell is due | Trackers: a Due row above the result line. No quick action moves. The rail says `Due 1` | Nothing | 0 | Nothing | Must remember the bell |
| 4 | To roll for each rival and for wandering | The Due row | `Roll Rival Track` three times; `Roll Wandering, 0:40 to 1:19` | Do 4 | Nothing. Each result starts with its rival's name | Find two tables: 8 or more |
| 5 | To finish the bell | The Due row | Tick its box | Do 1 | Nothing | 1 tick |
| 6 | The Old Guard takes its first seal | Trackers | `Take seal`; `Old Guard`. A Due row says `First seal. … A rival uses its 1-2 seals group.` | Do 2 | Nothing | 1 tally |
| 7 | In a fight: one minute passes each round | Run, the notice line: `Round 3: Fight minute` | Click the link | Do 1 | Nothing. The link shows at each new round | 1 tally, and the memory |
| 8 | The crowd loves a move. Then the crowd throws a gift | Run; Lookup on a stat block | Ctrl+T; `Crowd loves it`; `Throw a gift`; Ctrl+T, which shows the stat block again | Panel 2, Do 2 = 4 | Nothing. The result line names the gift until the next act | 2 tallies and a table |
| 9 | The rival group arrives on its mark | Run. The notice line ends with `🔔 1` | Ctrl+T; `Add to fight` on the Due row, which links the group; tick the row; Ctrl+T | Panel 2, Do 2 = 4 | Nothing. The row names the group | The same 2 as today, and the memory |
| 10 | The Practice Ring opens on the operator's next turn | Run, dock | Select the operator; click `Effect`; type `Ring opens on its turn`; Enter | Find 1, Do 3 = 4 | Nothing after this | 1 note |
| 11 | A wrong click on `Throw a gift`, seen one press late | Trackers, the result line | Ctrl+T; `Undo: Crowd loves it`; `Undo: Throw a gift`; `Crowd loves it`; Ctrl+T | Panel 2, Do 3 = 5 | Nothing | Erase |
| 12 | In one round: the reminder `Ring opens` and the bell at 60 | Run. The notice line: `⚑ Operator: Ring opens · Fight minute: 60 · 🔔 1` after the click | Click the link; Ctrl+T; four rolls; tick; Ctrl+T | Panel 2, Do 6 = 8 | Nothing. The bell and its count end the notice line | Must remember both |

Totals for steps 1 to 11: **no transition; 27 interactions (Panel 7, Find 1, Do 19)** with a stat block in
Lookup during the fight. With Trackers left in view for the whole race, the three pairs of Ctrl+T in steps 8,
9 and 11 go: **21**. A reviewer's play of round 1 gave 28 and 21 for the same acts, with step 11 as one
press. Here step 11 is three presses, because the wrong press is seen one press late.

### 2.5 J5: the plain evening, and the contract

The DM of J1 uses no new control. The audit's script runs unchanged and must give the counts that it gives
before each feature (G6). The build plan also keeps one frozen plain campaign: its captures may differ from
today's only by the rows of the two tables below (G7).

**The contract.** Each row is passive words at the end of a line that exists, or a quiet action in a place
that already holds actions. None adds a line or moves a thing.

| # | Screen | What differs | Kind |
|---|---|---|---|
| 1 | A planned fight's row, in the plan and in Encounters | The summary line ends with one word, such as `· Low` (F1). It shows only when the XP, the levels and the budget table are all known, and only where the line has room | Passive words |
| 2 | The encounter editor | The roster heading ends with `· XP 550 of 2,500 · Low` (F1), under the same condition | Passive words |
| 3 | Arrivals | A group's XP is followed by `→ Moderate` (F1), under the same condition | Passive words |
| 4 | A row marked `Conc` that takes damage | The row's second line ends with `Conc save DC 10` until the next step (F2). With no room on that line, the words are in the row's hover only. Never the notice line | Passive words |
| 5 | Run dock, line two | A quiet `Effect`, left of **Conditions**, so that **Conditions**, **Conc** and **Absent** keep their places (F2). The run's **Keys** list has one more line, `E` | Quiet action |
| 6 | The Add strip's **More** menu | Its last line is `Scene` (F3). The strip's visible actions are as today at 1200 and 960 | Quiet action |
| 7 | A creature's or an NPC's reader, with no run open | `Start fight`, in the place where an NPC's reader has **Add to fight** during a run, and bordered as that is (F3) | Quiet action, never primary |
| 8 | A campaign World entry's reader | `Add to Log` after `Edit`, on the title line (F6) | Quiet action |
| 9 | An ended fight's plan row | `Add rewards` in the row's action place, which is empty today (F7) | Quiet action |
| 10 | The **Last time** line | `Carry over` at its end, while the last session holds a part to carry (F5). The preview gives it room and is whole on hover, as today | Quiet action |
| 11 | A Travel row's `⋯` menu | One more line, `Roll for encounters`, when the campaign has a marked table (F9) | Quiet action |

**Ruled exceptions and repairs.** These are not of the two kinds. Each comes from a ruling of round 1, and
each is a row that G7 allows.

| # | Screen | What differs | From |
|---|---|---|---|
| 12 | The plan, when the DM's note has a task line under `## Plan for tonight` | The line is a scene: a row with a box, counted in **Tonight**, and not repeated in Read (F3). The measured evening has none | The ruling on scenes: it is the DM's own plan |
| 13 | The **Conditions** tab | `Other effect…` is gone, because `Effect` in the dock takes its place, and the help sentence is the one of F2 | The ruling on `Effect`: one home |
| 14 | The **Last time** line | Its preview reads the last session's `What happened`. Today it reads that note's own recap first, which tells the session before it (`src/sessions/page.rs:914`) | Rule H: the preview and the carry-over read one source |
| 15 | Lookup, while it holds an unsaved generated NPC | An entry that opens shows under one row, `Unsaved NPC: <name> · Show`. Today the card hides it (`src/lookup.rs:2178`) | The accepted repair of Lookup |

Nothing else differs. In particular: a free-words effect from an encounter or an older run gives no reminder
and keeps its place on the row; a Travel row shows no checks and no new field until the DM asks for them; the
rail says nothing about trackers unless a notice is due; the empty plan says what it says today; a creature's
reader during a run is as today; an embed in a note is drawn as today unless `Carry over` wrote it; and no
file changes that the DM did not act on. F8 is pull request 232: its changes are the app's baseline once it
merges.

### 2.6 J6: the week between

The campaign's notes already use XP, as J1 left them, and the last session's note has words under `What
happened`.

| # | The DM wants | Surface | Action | Count | Today | File |
|---|---|---|---|---|---:|---|
| 1 | The party searches the last fight | Run, ended view | `Roll all 2` | Do 1 | 1 | The run file |
| 2 | Verori died here | Ended view, then Lookup | Ctrl+K; type `verori`; **Change** on Status; type `dead`; Enter | Panel 1, Find 1, Do 3 = 5 | 5. F6 adds nothing: a death is a status | `NPCs/Verori.md`: the status, and `- [[Session 12]], Day 3: Status alive → dead.` |
| 3 | **F6.** He gave the gate key away first | Lookup, his reader | `Add to Log`; type `Gave the gate key to Iselle.`; Enter; close Lookup | Do 3, Panel 1 = 4 | 5: `Edit`, click the text, type, `Save changes`, close | One more Log line |
| 4 | To go back to the evening | Run, then Session | `Close run` | Nav 1, **T** | 1 | None |
| 5 | **F7.** Rewards for each of three fights | Session plan, the played rows | `Add rewards` three times | Do 3 | No route. `Copy loot` is on an ended run only while it is open (`src/encounter_run/loot.rs:400`) | The note: three blocks under `## Loot and rewards` |
| 6 | A week later: a new session | Session | `New session`; title; `Create session` | Do 3 | 3 | A new note |
| 7 | **F5.** What the last session left | **Last time** line | `Carry over` | Do 1 | By hand | The new note: the recap embed, the unplayed plan lines, the open threads, and `carried_from` |
| 8 | **F3.** Three scenes | The note beside the plan | Click under `## Plan for tonight`; type three task lines | Do 4 | 4, with no rows | The note: three `- [ ]` lines |
| 9 | **F1.** Is each of the two fights right for this party? | Session plan | Read the word on each row. Hover gives the three budgets, and the word itself on a row whose summary has no room | 0 | By hand, with the rule open | None |
| 10 | To start | Session plan | `Start fight` | Nav 1, **T** | 1 | The run file |

Totals: **2 transitions and 23 interactions (Nav 2, Panel 2, Find 1, Do 18).** Today's app has no total for
this journey: steps 5, 7 and 9 have no route in it. The steps that it has cost 20 and leave less record. The
carried HP of today still starts the fight (T12.2). Nothing is typed twice. After step 2 the reader and any
scene row that links Verori say `dead`.

## 3. The features

Each part has the same order: the question, the place, the wireframes for the four cases of section 1.3, the
exact words, the states, the file, Undo, keys, what stays with the DM, and what is not built. A table at the
end of each part gives what a reviewer checks on paper. Every write follows section 4.1.

The examples of F1 use one party: 5 characters at level 3. Their budgets are Low 750, Moderate 1,125 and High
2,000.

### F1. Encounter difficulty

**Question.** "Is this fight too much, or too little, for this party?"

**Where the numbers come from.** The rules data holds the table. The rule entry
`dnd2024:rules/combat_encounters_step_2_determine_your_xp_budget` has a typed table at
`splat_rule/parameters/per_character_xp`, with `level`, `low`, `moderate` and `high` for levels 1 to 20.
Splat reads that table from the rules that the campaign uses, as it reads the inspiration rule
(`src/encounter_settings.rs:421`). No table is written in Rust.

**One computation.** A new module, `src/difficulty.rs`, has one function. Every screen calls it.

- **The party:** the characters on the party's side in the roster; in a run, those who are not absent. A
  roster with no character uses the campaign's party. A level comes from the saved sheet.
- **The budget:** for each difficulty, the sum of the table's value for each character's level.
- **The XP:** the sum of the XP of every creature that is not on the party's side, from the function that
  groups use today (`creature_xp`, `src/encounters/groups.rs:535`).
- **The word:** `Low`, `Moderate` or `High` when the XP is at most that budget, and `Above High` for more.
- **When.** The answer is kept with the revisions it came from: the encounter note, the saved sheets and the
  rule entry. It is computed again when one of them changes, never each frame.

**Where it shows.** It is passive words at the end of three lines that exist. It adds no line.

| Place | The line today | The line with F1 |
|---|---|---|
| Encounter editor, the roster heading | `ROSTER 8` | `ROSTER 8 · XP 1,050 of 1,125 · Moderate` |
| A planned fight's summary, in the plan and in Encounters | `5 party · 2 × Ember Cultist` | `5 party · 2 × Ember Cultist · Low` |
| A group's line in Arrivals | `Enemies · 2 members · XP 400` | `Enemies · 2 members · XP 400 → Moderate` |

- In the editor, `of 1,125` is the budget of the word. For `Above High` it reads `XP 2,400 · Above High`.
- **Hover** on the word, and its accessible description, give the rest: `Budget for 5 at level 3: Low 750 ·
  Moderate 1,125 · High 2,000`. For mixed levels: `for 5 at levels 3 to 4`. With no character in the roster:
  `for the party of 5 at level 3`.
- In Arrivals the word is what the fight becomes when the group joins. Its accessible name says `adds 400 XP;
  the fight becomes Moderate`.
- **It gives way first.** The word and the XP never cut or move what the line shows today. Where a summary
  has no room, the summary's hover holds the word, as it holds a shortened summary today.
- It does not show on a running or played fight's row, in Setup, in History or on Timeline.

**Wireframe: the encounter editor, case A.** Cases B, C and D differ only in the column's width; the heading
is one line in each.

```text
← [Bridge Ambush                                         ]   [Save encounter] [▷ Start fight]  ⋯
✓ No changes
────────────────────────────────────────────────────────────────────────────────────────────────
ROSTER 8 · XP 1,050 of 1,125 · Moderate                 │ ADD PARTICIPANTS
1. Brannoch Vale                         Edit   ⋯       │ Party members come from the sheets …
   Party · HP 58/62 · AC 19                             │ [🔍 Find a creature by name or detail…  ]
```

**Exact words.** `Low`, `Moderate`, `High`, `Above High`. `XP 1,050 of 1,125`. The hover lines above.

**States.**

| State | What shows |
|---|---|
| The XP, every level and the budget table are known | The words above |
| A creature has no XP and no CR; a level is not known; there are no sheets; the rules have no table; there is no creature against the party | Nothing. The line is as today |
| The encounter file cannot be read, or changed on disk | The warnings of today. The word follows the draft on screen |
| Run open | The open fight's plan row shows its status link, not a word. Arrivals shows the live word |
| Sync on, Lookup open or closed | No effect |

**What the file stores.** Nothing. **Undo.** None is needed. **Keyboard.** Nothing takes focus.

**What stays with the DM.** Everything. Splat shows numbers and a word. It blocks nothing, warns about
nothing, and gives the word no colour, because red or green would be a verdict.

**Not built, and why.** A line of budgets, a link to the rule, and a sentence that says why no word shows
(each would be a new line or control on a plain screen; the hover holds the numbers). A slider to scale a
fight. The 2014 thresholds (no typed table in the SRD 5.1 data).

| Check | F1 |
|---|---|
| Primary action in view | The screen's own, unchanged. F1 adds no action |
| Components and tokens | Heading: `group_heading` with its muted `Small` parts. The word: `Type::Small` in `text`. Numbers: `Type::Small` in `muted`, with `number` digits. No icon and no state colour |
| In view at 960x640 | Lookup closed and open: the heading's one line, and each summary line |
| States | Words, or nothing |
| Loss and failure | Nothing to lose |
| Keys and targets | No new target |

### F2. Effects and turn reminders

**Question.** "What must I remember on this creature's turn, and how long does it last?"

**The idea.** An **effect** is words that the DM pins to one participant in a run. Today that is
`Other effect…`. An effect can now carry a time, a count of uses, or a recharge. **A reminder exists only when
the DM gave the effect a time: a count of rounds, or `on its turn`.** Words with no time are a label, as today,
and remind nobody.

| Need | The DM types | The row says | On its turn the notice line says |
|---|---|---|---|
| A note for a recurring save | `Wis save DC 15 (Hold Person) on its turn` | `Wis save DC 15 (Hold Person)` | `Oren: Wis save DC 15 (Hold Person)` |
| A duration on a condition | `Frightened 10 rounds` | `Frightened 10` | `Oren: Frightened, 10 rounds left`, later `last round`, then `time is up` |
| A duration on any effect | `Bless 1 minute` | `Bless 10` | The same |
| Recharge | `Fire Breath recharge 5–6` | `Fire Breath ready`, or `Fire Breath spent ⚄` | Nothing |
| Legendary actions | `Legendary actions 3 uses` | `Legendary actions 2 of 3` | Nothing |
| A label | `Hexed` | `Hexed` | Nothing, as today |
| Concentration | Nothing. `Conc` exists | `Conc`; after damage the line ends with `Conc save DC 12` | Nothing |

**The row is the place to act. The notice line only echoes it** (section 1.4), and it has no control.

**Where it lives, and why not elsewhere.**

- **Entry: the dock.** One quiet control, `Effect` (key `E`), on the dock's second line, left of
  **Conditions**. So **Conditions**, **Conc** and **Absent** keep their places. The dock stays in view when
  Lookup or Notes hides the run's panel. `Other effect…` leaves the **Conditions** tab: a run has one field
  for free words. **Conditions** keeps the 15 conditions, Exhaustion, and the list where an effect is removed.
- **Result: the row and the notice line.** No panel view, no tab and no strip is added.
- **Not the run's panel:** the user ruled that run features stay off it. **Not a dialog:** nothing is modal.

**Wireframe: the live run, case A, on Oren's turn in round 3.**

```text
Round 3   Oren Thistlewick’s turn                    [+ Add to fight]  [> Next turn  N]
⚑ Oren: Frightened, 9 rounds left · Wis save DC 15 (Hold Person)     [↶ Undo: next turn  U]
──────────────────────────────────────────────────────────────────────────────
 18 ■  Brannoch Vale                       AC 19   🛡+5    58/62
       Bless 9
 17 ◆  Cinder Drake                        AC 13           40/52
       Fire Breath spent ⚄ · Legendary actions 2 of 3
 16 ■  Oren Thistlewick                    AC 18           45/45          ← current row
 TURN  Frightened 9 · Wis save DC 15 (Hold Person)
 14 ■  Kestrel Dunmoor                     AC 15           11/41  ♡ Bloodied
 NEXT  Poisoned
…
[Amount] [Damage  Enter]  Heal  🛡 Temp HP                      [1d20] [Roll]
Oren Thistlewick                    Effect E   Conditions C   ✧ Conc   Absent
Close run   End encounter…                                Sync party  Keys  ✓ Saved
```

**The dock while the DM adds an effect.** Line one does not change. Line two becomes one field. At most four
suggestions open upward, over the lowest roster rows, and close on Enter or Escape.

```text
                 ┌──────────────────────────────────────────┐
                 │ Fire Breath recharge 5–6    · stat block │
                 │ Legendary actions 3 uses    · stat block │
                 │ Bless 10 rounds             · used before│
                 └──────────────────────────────────────────┘
[Amount] [Damage  Enter]  Heal  🛡 Temp HP                      [1d20] [Roll]
⚑ [Effect on Cinder Drake, such as Bless 10 rounds                         ]
```

- **Enter adds the words, closes the field and puts focus in `Amount`.** Escape clears and closes the field
  and puts focus in `Amount`, as the loot field's Escape clears today.
- **The field belongs to the row it opened for.** Its placeholder names that row. A click on another row does
  not change it.
- **While the field has focus, a letter is text.** `N` does not advance the turn; a click on `Next turn` does.
  The field then closes, and its words wait for that row: the next `E` on that row shows them.
- An arrow key and Enter, or a click, on a suggestion adds it, as the loot field's suggestions do.

**Case D (960x640, Lookup open): the roster is 454 px and the run's panel is hidden.**

```text
Round 3   Oren…                         [+ Add]  [> Next turn]
⚑ Oren: Frightened 9 · Wis save…                       [↶ Undo]
────────────────────────────────────────────────────────
 16 ■  Oren Thistlewick            AC 18        45/45
 TURN  Frightened 9 · Wis save DC 15 (Hold Person)
…
Dagger +6   Fire Bolt +7                       ← the tile line of today
[Amount] [Damage] Heal 🛡Temp HP        [1d20] [Roll]
Oren Thistlewick      ⚑ Effect  Conditions  ✧ Conc  Absent
```

- **The notice line and the row** follow section 1.4.
- **The dock's second line.** `Effect` never cuts the selected name. Where the line has no room for its word,
  it keeps its icon, with its name on hover. The icon always shows, so the field has a pointer route in every
  case; a long name beside it shortens by whole words, as it does today.
- **Case B** is case D with more room. **Case C** is case A with the run's panel at 346 px.
- **With Lookup or Notes open,** `E` is text, as every letter is, and `Effect` takes a click.

**How a time works.**

- **Rounds.** The count is the number of this participant's turns that the effect still covers, with the
  current turn. It goes down by one when that participant's turn ends, in the same step as `Next turn`, so
  Undo of `Next turn` gives it back. An effect of 10 rounds added before Oren's turn says 10 during that turn
  and 9 during the next. At 1 the words say `last round`. At 0 they say `time is up`, and the effect stays
  until the DM removes it. An absent participant's count does not go down. To time an effect by its caster's
  turns, pin it to the caster.
- **On its turn.** The words show on each of that participant's turns, until the DM removes the effect.
- **A condition's name.** Words that are an official condition, such as `Frightened 10 rounds`, add that
  condition with a count. Words that are already on the row change their time and add nothing.
- **Several rows.** With rows added by Ctrl+click, one effect goes to each, as one step of Undo.
- **A recharge roll** uses the run's dice. The row's result words say `Rolled 5 · ready again` or `Rolled 2 ·
  still spent`.
- **Concentration.** When a row marked `Conc` takes damage, the row's second line ends with `Conc save DC 12`
  until the next step: 10, or half the damage if that is more, and at most 30. The words move nothing and add
  no line: with no room on the line, they are in the row's hover and accessible name only. Splat does not
  roll the save and does not clear `Conc`.

**What a click does.** One act for each target. Hover and the accessible name say it.

| Target on the row | A click | Hover |
|---|---|---|
| An official condition's name, with or without a count | Opens its rule in Lookup, as today | `Read the rule for Frightened`, as today |
| A count that runs, after any name (`10`, `last round`) | Opens **Conditions**, where the effect is removed | `Open Conditions to remove Frightened` |
| `time is up` | Removes the effect, also when it is an official condition | `Remove Frightened` |
| Words with `on its turn` | Opens **Conditions**, where the effect is removed | `Open Conditions to remove this effect` |
| `Legendary actions 2 of 3` | Spends one | `Spend one` |
| `Legendary actions 0 of 3` | Gives all back | `Back to 3` |
| `Fire Breath ready` | Marks it spent | `Mark Fire Breath spent` |
| `Fire Breath spent ⚄` | Rolls the recharge | `Roll 1d6. 5–6 makes it ready` |
| A label with no time | As today: nothing for short words; long words and `N more` open **Conditions** (`src/encounter_run/view.rs:6787-6869`) | As today |

A count is its own target, 24 px wide or more, so the name beside it keeps its act. To remove an effect with
uses or a recharge, the DM selects the row and opens **Conditions** (`C`), which lists every effect.

**What Splat reads at the end of the words.** `10 rounds` or `1 round`. `1 minute` or `2 minutes`, as 10
rounds each. `on its turn`. `3 uses` or `1 use`. `recharge 6`, `recharge 5–6` or `recharge 5-6`. Anything else
stays words: `Bless for a while`, `Mage Armor 8 hours`.

**The suggestions.** `stat block` ones come from the creature's typed data: a feature's `recharge` block and
its `legendary_actions`. `used before` ones are the last effects of this fight.

**Exact words.** `Effect`. Field: `Effect on <name>, such as Bless 10 rounds`; with several rows, `Effect on 3
participants, such as Bless 10 rounds`. On the row: `Bless 9`, `Bless · last round`, `Bless · time is up`. On
the notice line: `<name>: Bless, 9 rounds left`, `<name>: Bless, last round`, `<name>: Bless, time is up`,
`<name>: <words>`. The help under the **Conditions** grid: `You rule on saves and removal. Round counts advance
with turns. Only official conditions sync; their counts stay here. Add other words with Effect in the dock.`

**States.**

| State | What shows |
|---|---|
| No effect with a time | The notice line is as today |
| A conflict, recording silence or a sync notice at the same time | That notice keeps its place, and the reminders follow it or become a count (section 1.4). The current row has each one |
| A sheet removes a condition that has a count | The condition and its count go together. As today, a sheet's change is not an Undo step and removes that character's earlier steps. The sync notice and the Log line name what went: `Frightened, 7 rounds left` |
| A sheet adds a condition | It arrives with no count |
| Nobody selected | `Effect` is blocked: `Select a participant first.` |
| The run has ended | Effects show as words on the Final state rows. Nothing counts, and a click does nothing |
| The run's panel is hidden | Every act of the click table works. Where it opens **Conditions**, Lookup closes, as today |
| Run file cannot be saved, or changed on disk | The run's own messages of today |
| Sync on | An official condition syncs. Its count, and every effect, stay in the run |
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
Undo. So a count cannot outlive its condition. The saved Undo steps of pull request 232 hold rows as they
are, so they hold this too; its row comparison (`src/encounter_run/undo.rs`) must read the new entries.

**Undo.** Each act is one named step of the run's Undo, which the run file keeps (F8): `Undo: effect Bless on
Oren`, `Undo: Fire Breath spent`, `Undo: recharge roll for Cinder Drake`, `Undo: legendary action of Adult Red
Dragon`, `Undo: remove Bless from Oren`. The Log has a line for each, and a line when a count reaches 0.

**Keyboard.** `E` opens the field for the selected row. Enter adds. Escape closes. The targets on a row are
Tab stops, as condition words are, and Enter on one is a click.

**What stays with the DM.** When an effect starts and ends. Every save. Whether a use was spent and when it
comes back. Whether concentration holds. Splat counts, shows and rolls when asked.

**Not built, and why.** A time on words typed in prep (an encounter's `Other effect` stays a label, so no old
text starts to count). A link from a caster's concentration to its effects. A reminder for uses or a recharge.

| Check | F2 |
|---|---|
| Primary action in view | `Next turn`, unchanged and alone |
| Components and tokens | `Effect`: `Action::quiet` with `.key("E")`, dock density, icon `icons::PIN`. Field: `add_field`, with an accessible name. Row targets: `link` at the run scale's condition size in `participant.condition`; digits tabular; `icons::DICE` after a spent recharge. `time is up`: `result.warning` icon and words in `text`. Notice parts: the header's notice with `Tone::Info`, icon `icons::PIN`, or `icons::HOURGLASS` for a count |
| In view at 960x640 | Lookup closed and open: round, turn, `Next turn`, the current row with each timed effect, the dock. No scroll |
| States | Counting: hourglass and `Bless 9`. Last round: `last round`. Ended: warning icon and `time is up`. Spent: `spent` and the dice. Ready: `ready` |
| Loss and failure | Every step has the run's named Undo, also after a restart. Words that Splat cannot count stay as words. A blocked `Effect` says why |
| Keys and targets | `E`, Enter, Escape, Tab. Each target is at least 24 px each way, with an accessible name such as `Spend one legendary action of Adult Red Dragon` |

### F3. Plan items that are not fights

**Question.** "What did I plan for tonight besides the fights, and what is done?"

**The name.** A plan item that is not a fight is a **scene**: a talk, a person to meet, a place, a secret to
reveal. The app uses that one word. `Item` already means loot and equipment.

**Where it lives.** In the plan that exists. A task line under `## Plan for tonight` is a scene: a row with a
done box, in the note's own order. Obsidian shows the same line with the same box. This is true of a task
line that the DM typed before this feature: it is the DM's plan.

- **Not a Timeline event,** because a planned scene has not happened.
- **Not a new list or tab,** because the DM reads one plan from top to bottom.
- **The order of the plan:** what has happened in this session, as today; then the fights and scenes still to
  come, in the note's order; then the groups, as today. A done scene stays in its place, so no row jumps.

**Which line is which row.**

| A line under `## Plan for tonight` | Row |
|---|---|
| A link to an encounter, alone or with a task box or other words | A fight, as today. Its box, if it has one, is not shown: a fight is done when it is played |
| A link to an encounter group, anywhere in the note, with or without a task box | A group, as today, and never a scene |
| A task line, `- [ ] words` or `- [x] words`, that links no encounter | A scene |
| A plain list line, a numbered line, a paragraph, or a line nested under another | No row. It stays text in the note, as today |

**Wireframe: the Session page, case A.**

```text
Session 2: The Crossing  2026-01-01                       ✓ Notes saved  Details  [◎ Record]
Last time:  Session 1: The Road — Hobb the miller says the toll collectors have not been paid.
Plan and notes   Recordings                                                          Read
TONIGHT 3                                 │ # Session 2: The Crossing
Add  ⚔ Fight  ⚇ Group  ⌂ Long rest  ⋯⌄    │
>  Bridge Ambush        [▷ Start fight] ⋯ │ ## Plan for tonight
   5 party · 2 × Cinder Wolf · Moderate   │
☐  Captain Varra                        ⋯ │ - [[Bridge Ambush]]
   NPC · alive · Ashfall Keep             │ - [ ] [[Captain Varra]]
☑  The miller is the informer           ⋯ │ - [x] The miller is the informer
>  Shrine of Cinders    [▷ Start fight] ⋯ │ - [[Shrine of Cinders]]
   5 party · 2 × Ember Cultist · Low      │
```

**Adding a scene: two routes.**

- **Type it in the note.** A task line typed under `## Plan for tonight`, in Splat or in Obsidian, is a scene.
  This is the cheap route: one click for the caret and one typed line for each scene. The empty plan says so.
- **`Scene` on the Add strip.** It is the last action of the strip and the first to go into **More**. On a
  plan the strip is never wide enough to show it, so it is the last line of the **More** menu. At 1200 and at
  960 that menu exists today in all four cases (the plan is 330, 608, 568 and 454 px wide, and seven actions
  need about 640), so no action that is visible today moves. In a window where today's seven actions fit and
  the menu button does not (a plan of 640 to 683 px), the strip is as today and `Scene` is not on it.
  `Scene` opens one field at the end of the plan, with focus:

```text
+  [Plan a scene: words, a person or a place                ]
   Captain Varra · NPC
   Ashfall Keep · Location
```

  Enter adds the words as a task line and keeps the field open for the next line. In a note with no `## Plan for
  tonight`, the first scene adds that heading at the end of the note. An arrow key and Enter, or
  a click, on a suggestion adds a scene that links that entry. Escape clears and closes the field. At most
  four World entries that match show under it.

**A row.**

- **The box** is at the start of the row, where a fight has its caret. A click or Space ticks it. The note
  changes by one character.
- **The words** are drawn by the Reader. A link opens its entry in Lookup, beside the page.
- **The second line** shows only when the words link exactly one World entry: its kind, status and place, as
  Lookup's compact reader says them (`NPC · alive · Ashfall Keep`).
- **The action place** at the right is empty until the scene is done. Then, when the words link exactly one
  World entry, it holds the quiet `Add to Log` of F6.
- **The menu** holds `Remove from tonight`, with `Undo remove` under the row, as for a fight.

**A fight that nobody planned: `Start fight` on a reader.** An NPC's reader has **Add to fight** while a run
is open, as today (`Action::secondary`, `src/world.rs:3150`). With no run open, the same place says
`Start fight`, in the same style. A creature's reader, in Lookup and in the Library, gets `Start fight` with
its other actions while no run is open. While a run is open it has no such action, as today: **Arrivals**
adds a creature to an open fight.

- **What it does.** It does what the editor's `Start fight` does today for a new encounter, in one click: it
  saves an encounter named `Fight with <name>`, with the party and that creature, and it starts the run,
  which saves at once. The name cannot be read as a link to the NPC, so a scene line that links the NPC stays
  a scene. If the name is taken, it is `Fight with Captain Varra 2`.
- **The plan.** Nothing is written in the note. The fight is a row of tonight's plan from `Begin combat`,
  when the run adds its timeline event, as every run does today.
- **A wrong click.** `Close run` on a Setup that nobody touched removes the run file today
  (`untouched_setup`, `src/encounter_run.rs:979`). It then also removes the encounter that this click made,
  when the file still holds what Splat wrote. So one click takes back all of it. After one step in Setup,
  the run stays as a paused Setup with `Delete the saved run`, and the encounter stays in **Ready to run**,
  as any encounter the DM starts today.
- **An NPC with no stat block** joins Setup as a made-up creature with its name. It is in the run from the
  start, so an untouched Setup still closes to nothing. Setup says that it needs HP, as today.

**Cases B, C and D.** The plan has its own tab and the full page width. The row is the same. The words wrap to
at most three lines, then shorten with the full text on hover. In case D, Lookup on an entry and the plan are
both in view; the note is behind its tab, so the typed route costs one tab change.

**Lookup.** A click on a link in a row opens that entry in Lookup. If Lookup shows another entry, it goes to
this one, and Back returns. An unsaved edit form stays with its own entry, and an unsaved NPC stays under its
row (section 1.2). Nothing opens Lookup without a click.

**Exact words.** `Scene`. Field: `Plan a scene: words, a person or a place`. The empty plan's words are as today. The box's accessible names: `Done: Captain
Varra` and `Open: Captain Varra`. `Remove from tonight`. `Undo remove`. Result of the field: `Scene added to
tonight.` with `Undo scene`. `Start fight`, with the accessible name `Start a fight with Captain Varra`. The
count in `TONIGHT 3` is the rows still to come with the events of the session, as today; a done scene does not
count.

**States.**

| State | What shows |
|---|---|
| No scene | No row. The plan is as today |
| A link to nothing | The words keep the dotted underline and `Not found in this campaign`, as in every reader |
| A link to two notes | The link opens Lookup's chooser, as today |
| The note cannot be read | The plan shows the problem that it shows today |
| The note changed on disk | Section 4.1: Splat takes the file's text first when it holds no unsaved words. With unsaved words, the comparison of today decides, and a plan act says `The note changed on disk. Choose in the note first.` |
| Run open | Scenes work. An NPC's `Start fight` is **Add to fight** |
| Read view | Read hides `Plan for tonight` when every line of it is a row beside the note, as it hides link-only plans today |
| An older session | Its scenes show with their boxes. A tick writes that session's note |

**What the note stores.**

```markdown
## Plan for tonight

- [[Bridge Ambush]]
- [ ] [[Captain Varra]]
- [x] The miller is the informer
- [[Shrine of Cinders]]
```

**Undo.** `Undo scene` after the field adds one, in the plan's message place below the Add strip. A tick is
its own Undo: click again. Removing gives `Undo remove`. Each ends at the next edit of the note or the plan.

**Keyboard.** Tab reaches the box, then the links in the words, then the menu. Space ticks. In the add field,
Enter adds, the arrow keys choose a suggestion, and Escape closes.

**What stays with the DM.** What a scene is, when it is done, and whether it happened at all.

**Not built, and why.** Editing a scene's words in its row (the note holds them). Dragging rows to reorder.
Scenes with a time or a place (that is a Timeline event). Nested lines. A plan link for an improvised fight
(the run's own event is its row).

| Check | F3 |
|---|---|
| Primary action in view | The first upcoming `Start fight` of the plan, as today. A scene never takes it. `Start fight` on a reader is secondary |
| Components and tokens | `Scene`: a line of the `More Add actions` menu, icon `icons::CHECK_SQUARE`. Field: `add_field`, with suggestions in `Type::Small` and a muted kind. Box: the theme's checkbox, 24 px. Words: the Reader's inline text with `link`. Second line: `Type::Small` in `muted`. `Start fight`: `Action::secondary` with `icons::RUN`, `.blocked(reason)`. Messages: `undo_message` |
| In view at 960x640 | Lookup closed and open: the Add strip and the rows. Nothing scrolls sideways |
| States | Open: an empty box. Done: a checked box, and the words in `muted` |
| Loss and failure | `Undo scene`, `Undo remove`. A tick is one character. A failed save shows the note's own `Notes not saved` |
| Keys and targets | As above. The box, each link and the menu are 24 px or more |

### F4. Trackers and the Due list

**Question.** "What is due, and what are the numbers?"

The data model of [World and house rules](world-and-house-rules.md) stands: house rule notes, in play,
trackers, quick actions, marks, the Due list and the ledger `Trackers.md`. That design is older than the
Session page, the plan, the rail and the saved Undo. This table lists what changes. The same pull request
changes that document in place.

| The old design said | Now |
|---|---|
| A tracker panel opens beside any page with Ctrl+T, as a third panel | **Trackers** is a view of the Lookup panel. Ctrl+T goes there and back |
| The run strip and the panel's button show the Due count | The rail says `Due 2` while a notice is due, and nothing else. The run's notice line ends with the bell and the count |
| A tracker action in a run also puts a step on the run's undo stack | A tracker action is never a run step. The ledger is its Undo |
| The panel's Undo takes back the newest change | Each press, tick, dismissal and roll is one ledger entry. `Undo` steps back through this session's entries, newest first, also after a restart |
| A quick action's table roll goes to the run's Log on the run's dice | The ledger records the roll and its row. The run does not |
| A notice's roll results stay until the app closes | The ledger keeps each roll as an entry |
| The DM presses a tally for each round of a fight | A quick action can say `each: round`. The run's notice line then offers it once at each new round |
| A mark's tables roll with no name | A mark's table can say `for: [Old Guard, Keyring, Convicts]`. Each result starts with its name |
| **Done** and **Dismiss** are two buttons | A row with no proposed changes has a box, as a scene has. A row with proposed changes has `Apply and finish`. `Dismiss` is in the row's menu |

**Where it shows.**

| Surface | What shows | When |
|---|---|---|
| The rail's status slot, above Lookup | A bell and `Due 2` | Only while a notice is due. A click opens Trackers |
| The Lookup panel | The **Trackers** view | Ctrl+T, or a click on `Due 2` or on the notice line's bell |
| The run's notice line | Parts 3, 4 and 6 of section 1.4: a new notice's words for one turn, the link for each round, the bell and the count | In a run |
| The Session page | Nothing of its own. The rail is on this page too | |

**Wireframe: Trackers in Lookup, case B (the panel is 432 px), beside the plan or a run.**

```text
🔍 [Search…                                          ]   ← Back   ×
Trackers
PINNED
Elapsed race time        42 / 120   [Enter room] [Fight minute]
  [Cistern or Sands] [Reach Kitchens seal] [Alcove rest] [Collect soul]
Crowd Favor                     3   [Crowd loves it] [Throw a gift]
Souls in the jar                0
Seals   Party 1 · Old Guard 2 · Keyring 0 · Convicts 0
  Old Guard: Coming for the party      [Take seal ⌄] [Lose seal ⌄]
Practice Ring activation   0 / 30   [Touch controls] [Missed twice or Ring reset]
> MORE TRACKERS 1
                 (the list above scrolls; what follows keeps its place)
DUE 1
☐  Bell. The Cistern floods. Announce the standings.
   Roll each rival. Check d6 for wandering.              ⋯
   [⚄ Roll Rival Track]   [⚄ Roll Wandering, 0:40 to 1:19]
   Old Guard: 4 · Takes a seal.
──────────────────────────────────────────────────────────
✓ Throw a gift: Favor −1. Rolled 4 on 1d6: A healing potion.
  ↶ Undo: Throw a gift     Add to Timeline
```

- **Pinned trackers keep the note's order,** so the DM puts the one in use first. A tracker's first two
  quick actions follow its value on the same line when they fit; the others wrap below.
- **Due rows sit above the result line and grow upward,** as the rail's status slot does. A new notice moves
  no quick action.
- **A Due row with no proposed changes** starts with a box. A tick means done, and the row goes. **A row with
  proposed changes** lists them with a box each and ends with the quiet `Apply and finish`, which applies the
  ticked ones. The row's menu holds `Dismiss`, which applies nothing.
- **A table's `Roll`** lists its result under the row, and each further press adds a result. With `for:`, each
  press rolls for the next name. The button names the table by its heading (section 4.4).
- **A notice whose words link an encounter group** offers `Add to fight` on its row during a run, and
  `Plan “<name>”` outside one. `Add to fight` adds the group as **Arrivals** does with an empty Initiative
  field: one step of the run's Undo. `Plan “<name>”` adds the group's link to tonight's plan.
- **An act in this view that changes another file** shows its result here too. After `Plan “<name>”` the Due
  row says `In the plan`, and the result line says `Added to tonight’s plan.` with the plan's `Undo add`.
  After `Add to Timeline` it says `Added to the Timeline.` with the Timeline's `Undo add`.
- **A quick action** is one press. A tracker with instances (`per`) opens a menu of them. An action that asks
  for a number (`add: ask`) puts a number field in its own place; the field holds focus until Enter or Escape.
- **The result line** keeps its place at the panel's bottom edge. It names the last act and has `Undo` and
  `Add to Timeline`.
- **Case D (960x640):** the panel is 346 px. A tracker's actions wrap below its value, two to a line. The
  five pinned trackers of the example are taller than the list, so the list scrolls; the tracker in use is
  first, and the Due rows and the result line are in view.
- **Case A and C** are the page before Ctrl+T: nothing shows unless a notice is due.

**Lookup's own rules** (section 1.2). Trackers is a place in Lookup's history. Ctrl+T from Trackers goes back
to the place that was in view, or closes Lookup when Lookup was closed before. Ctrl+K closes Lookup, as on
every view; a click in the search field starts a search, and Back returns to Trackers. An unsaved edit form
or NPC in Lookup stays held. Opening **Add to fight** or **Conditions** in a run closes Lookup, as today.

**Reminders and Due notices** are two things with two owners. A reminder belongs to one participant of one
run, is in the run file, and ends with the effect or the fight. A Due notice belongs to the campaign, is in
`Trackers.md`, and ends when the DM ticks or dismisses it. A mark never makes a reminder, and a reminder never
writes the ledger. When a mark's words name a turn, the notice says it, and the bell stays on the notice line
until the DM ticks it. An effect that the DM types on that row is a second fact: the DM's own note.

**Exact words.** Rail: `Due 1`; hover `1 notice is due. Open Trackers (Ctrl+T).` View: `Trackers`, `PINNED`,
`MORE TRACKERS 1`, `DUE 1`. `Roll <table’s name>`. `Apply and finish`. `Dismiss`. `Add to fight`.
`Plan “<name>”`. Result: `<label>: Favor −1.`; with a table `… Rolled 4 on 1d6: A healing potion.`; after a
tick `Bell done.`; after a dismissal `Bell dismissed.` `Undo: <label>`. `Add to Timeline`. Notice line:
`Due: <the mark's words>`; `Round 3: Fight minute`; `🔔 1`. A blocked action: `Favor cannot go below 0.` No
note in play, after Ctrl+T: the empty state `No house rules are in play.` with `Turn one on in Campaign →
Rules and runs.`

**States.**

| State | What shows |
|---|---|
| No house rule note in play | Nothing in the rail. Ctrl+T opens the empty state above |
| In play, nothing due | Nothing in the rail. Ctrl+T opens the view |
| A notice is due | Rail: bell and `Due 1`, in the text colour |
| `Trackers.md` cannot be read | Rail: error icon and `Trackers not read`. The view names the file, says the history is safe, and offers `Try again` and `Show file`. Quick actions are blocked with that reason |
| `Trackers.md` changed on disk | Separate changes merge with a quiet notice, as `Timeline.md` does. The same entry needs a choice |
| `Timeline.md` cannot be read | Day marks stop, and the Due list says why (the old design's rule) |
| A definition cannot be read | The tracker says `Definition unavailable, history is safe` |
| Run open | The same view. No tracker act is a run step, except `Add to fight` on a Due row, which is the run's own act |
| Sync on | No effect |

**What the files store.** Definitions are in the house rule note's Splat data, as before, with two new
optional words: `each: round` on a quick action, and `for:` on a mark's table. The ledger is one ordered
list. Each act is one entry. A notice's state is not stored: a notice is open until an entry ticks or
dismisses it. An entry holds what it replaced where that cannot be worked out: `was` on a `set`. An entry
from the link for each round holds its run and its round, so the link shows once for each.

```yaml
changes:
- {id: c-5e6f7a8b, tracker: race_clock, add: 10, label: Enter room, session: 070eca36-…}
- {id: c-6f7a8b9c, notice: {change: c-5e6f7a8b, at: 40}, roll: {table: rival_track, for: Old Guard, value: 4, row: Takes a seal.}, session: 070eca36-…}
- {id: c-7a8b9c0d, notice: {change: c-5e6f7a8b, at: 40}, done: true, session: 070eca36-…}
- {id: c-8b9c0d1e, tracker: favor, add: -1, label: Throw a gift, roll: "1d6: [4]", row: A healing potion, session: 070eca36-…}
- {id: c-9c0d1e2f, tracker: race_clock, add: 1, label: Fight minute, run: 4f1c2a9e-…, round: 3, session: 070eca36-…}
```

**Undo.** `Undo: <label>` on the result line removes this session's newest entry. The label is the action's
own (`Undo: Throw a gift`), or `Undo: Bell done`, `Undo: Bell dismissed`, `Undo: Rival Track roll`. An entry
made with no session being played belongs to the sitting in which it was made. The line then names the
entry before it, so the DM steps back as far as the session's first act. The ledger is a file, so the steps
are there after a restart. Undo of an entry that raised a notice removes the notice and its entries. A change
from an older session leaves by **Remove** in the tracker's history, as the old design says. `Add to
Timeline` is a second act: it makes a Major event with the action's label in the session being played, with
the Timeline's own `Undo add`. `Undo: <label>` leaves that event. `U` is the run's key.

**Keyboard.** Ctrl+T, there and back. Tab walks the trackers' actions, then the Due rows. Space ticks a Due
row. Enter presses an action. Escape closes Lookup. The run's keys are as beside any Lookup view.

**What stays with the DM.** Awarding Favor, moving a rival, what a bell does on the map, and every rule that a
tracker's number stands for. A mark's proposed changes apply only when the DM presses `Apply and finish`.
The link for each round is offered, never pressed for the DM.

**Not built, and why.** The parts in [World and rules: later](world-and-rules-later.md). Quick actions in the
run's dock or footer (the notice line's link covers the act that repeats). Due rows in the plan (a notice
would show in two places). A word in the rail when nothing is due.

| Check | F4 |
|---|---|
| Primary action in view | None in the view. The page's own primary stays |
| Components and tokens | Rail: the status slot with `status_link`; icon `icons::BELL`. View title: Lookup's title line. `PINNED`, `DUE`: `group_heading`. Box: the theme's checkbox. Quick actions, `Roll`, `Add to fight`: `Action::secondary` at Read density; `Roll` has `icons::DICE`. `Apply and finish`, `Plan “…”`: `Action::quiet`. A menu of instances: `menu_button`. Values: `Type::Numeral`, right-aligned with `number`. Result: `undo_message` with a second quiet action, above a hairline. Notice parts: `link` in the header's notice |
| In view at 960x640 | The rail's `Due N` on every page. In the view: the result line, the Due rows above it, and the first pinned trackers |
| States | Due: bell and `Due 1`. Not read: error icon and `Trackers not read`. Blocked: the reason beside the action |
| Loss and failure | `Undo: <label>`, step by step, also after a restart. `Dismiss` is in a menu. An unreadable file blocks writing and says what to do |
| Keys and targets | Ctrl+T, Tab, Space, Enter. Every action is 28 px high (Read) |

### F5. Carry-over between sessions

**Question.** "What did the last session leave for this one?"

**Where it lives.** On the **Last time** line of the Session page, which already links the last session and
shows the first line of its recap (`last_time_block`, `src/sessions/page.rs:4065`). One quiet action,
`Carry over`, ends that line while a part is still to carry.

- **Not automatic at Create session,** because a note would then change with no act of the DM (rule 2).
- **Not in the New session form,** because the act would then have two controls.

**Three parts.** Carry-over has three parts, and each is done once.

| Part | From the last session | To this note | Copied text or a link | Done when |
|---|---|---|---|---|
| Recap | The prose under `## What happened`. It holds the summary when the DM added it with **Add to notes** | `## Recap of last time` | **A link:** one embed line, `![[Session 1 The Road#What happened]]`. One file owns what happened, and a later correction shows here | The embed is added. Or this note already has prose under the heading: the DM's recap is kept, and the part counts as done |
| Recap, second source | A processed recording's **Recap**, only when `What happened` has no prose | `## Recap of last time` | **Copied text,** under the line `> This draft was written automatically. Check it against your memory.` An embed cannot point into a recording's file by a name that a person can read | The same |
| Plan | Each row still to come under `## Plan for tonight`: a fight that was not played, a group, a scene that is not done | `## Plan for tonight` | **Copied lines,** links kept, boxes empty. They are this session's own lines | The lines are added, also when the DM deletes them later |
| Threads | Each line under `## Open threads` that is not ticked | `## Open threads` | **Copied lines** | The same |

- **The recap can come later.** When the last session has no prose and no processed recording yet, the plan
  and the threads carry, and the recap stays to carry. When the recap exists, `Carry over` is on the line
  again and adds it.
- **When the DM already wrote something.** Carry over never replaces and never reorders. It adds below what a
  heading holds, and it skips a line that this note already has, compared without the task mark.
  A heading that this note does not have is added at the end of the note.
- **When the line reads the last session.** It reads that note again when its file changes and when a
  recording of that session is processed, never each frame. So `Carry over` comes back when the recap exists.
- **One act, one file.** The act writes this note only: the lines, and one property, `carried_from`, with the
  last session's ID and the parts done. The last session's note does not change.
- **The preview and the carry-over read one source.** The **Last time** line previews the last session's
  `What happened`, then its processed recap. Today it reads that note's own `Recap of last time` first, which
  tells the session before it (`recap_preview`, `src/sessions/page.rs:914`).

**Wireframe: case A, before and after.**

```text
Last time:  Session 1: The Road — Hobb the miller says the toll collectors …    Carry over

Last time:  Session 1: The Road — Hobb the miller says the toll collectors …
✓ Carried over from Session 1: the recap, 2 plan lines and 2 open threads.   ↶ Undo carry over
```

In Read, the embed shows the last session's text under a muted line, `From Session 1: The Road`, which is a
link to that session. In Write it is the one line of source. Read draws this for the line that `Carry over`
wrote, in a note that has `carried_from`; every other embed is drawn as today.

**Cases B, C and D.** `Carry over` never takes a line of its own. The preview gives it room, and the preview
is whole on hover, as today. The result message is capped at the list measure and wraps.

**Lookup.** Carry over does not use Lookup.

**Exact words.** `Carry over`. Hover says what it will add: `Add the last session’s recap, 2 plan lines and 2
open threads to this note.` Result: `Carried over from Session 1: the recap, 2 plan lines and 2 open
threads.` It names only what it added. When this note has a recap: `… Your recap was kept.` When the recap is
not there yet: `… The recap is still to come.` `Undo carry over`. `Update the link`, with `Undo link`.

**States.**

| State | What shows |
|---|---|
| No earlier session, or no part can be carried now | No action |
| Every part is done | No action |
| The last note cannot be read | The action is blocked: `Splat cannot read Session 1’s note.` |
| This note changed on disk and Splat holds unsaved words | The comparison of today decides first. The act does nothing and says `The note changed on disk. Choose in the note first.` |
| The embed's note was renamed outside Obsidian | Read finds the session by the ID in `carried_from` and shows its text, with the quiet `Update the link`, which writes the note's new name into the embed line |
| The embed's note or its heading is gone | Read says `The recap’s source is gone: Session 1 The Road › What happened.` in the embed's place. The line stays in the note |
| An older session is open | The action is on that session's own **Last time** line and writes that note |
| Run open, sync on | No effect |

**What the note stores.**

```markdown
---
session_id: 9b1f0c1e-52a7-4c3b-8f13-0f6f4f4f2a10
session_number: 2
carried_from: {session: 070eca36-331c-46cc-b5a6-32a6e35265da, parts: [recap, plan, threads]}
---

## Recap of last time

![[Session 1 The Road#What happened]]

## Plan for tonight

- [[Shrine of Cinders]]
- [ ] [[Captain Varra]]

## Open threads

- [ ] Who burned the ropes?
- [ ] Does the miller know about the toll?
```

**Undo.** `Undo carry over` removes exactly the lines that the act added, when they still stand as written,
and takes its parts out of `carried_from`. It belongs to this note and ends at the next edit of the note or
the plan. After that the DM removes a carried line as any line, and the part stays done.

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
| Loss and failure | One Undo for all lines. Nothing is replaced |
| Keys and targets | Tab, Enter. A Read control, 28 px |

### F6. Wrap-up to World

**Question.** "How does what happened to this person or place get into its own note?"

**Today.** A change of Status, place or group in play writes the value and a Log line in one step, with one
Undo (T15.4, `src/world.rs:2280-2298`). The session's Read view lists those lines under **World changes**,
from each entry's Log (`src/sessions/page.rs:3856`). One thing is missing: no action writes the DM's own
words.

**Two acts, in one place.** Both are in the entry's own reader, in Lookup and on World.

| The DM means | The act | What the file gains |
|---|---|---|
| A fact that a field holds: "Verori died" | **Change** on Status, as today | The status, and the line `Status alive → dead.` |
| A fact that no field holds: "She told the party who burned the ropes" | **Add to Log**, new: a quiet action on the title line, after `Edit` | One line, in the DM's words |

A death is a status. `Add to Log` does not change a status, and Splat proposes no "died", because death is
the DM's ruling. When the DM does both, the Log has two lines for two facts.

**The ways in.** The act has one home, the reader's header, so it is in view however the reader opened and
however long the note is.

| From | What the DM does |
|---|---|
| A scene that is done and links one World entry (F3) | `Add to Log` in the row's action place opens Lookup on that entry, with the Log field open |
| A session note, a scene's words, any Reader text | A link to the entry opens it in Lookup. Then `Add to Log` |
| A fight's ended view | Ctrl+K and the name, as today. Then `Add to Log`, or **Change** |

**Wireframe: Lookup on an entry, case B (the panel is 432 px), beside the Session page or the ended view.**

```text
🔍 [Search…                                          ]   ← Back   ×
Captain Varra   ✎ Edit   + Add to Log
NPC · Campaign · alive
  Session 12, Day 3:  [What happened?                              ]
✓ Added to Captain Varra’s Log.   ↶ Undo Log line
Stat block   Bandit Captain                    Change
Status       alive                             Change
Runs the toll at the ford.

Log
• Session 11, Day 2: In Ashfall Keep → The Ford.
• Session 12, Day 3: Told the party who burned the ropes.
```

- `Add to Log` opens one add field under the kind line. The line's start, `Session 12, Day 3:`, shows before
  it in muted text, so the DM sees what the file will say. Enter adds the line and closes the field. Escape
  clears and closes it.
- **From a done scene** the field starts with the scene's words that are outside its link, selected, so that
  typing replaces them and Enter keeps them. A scene that is only a link starts an empty field.
- **Which session.** The one the DM came from: the plan's session or the note's session. From Lookup or World
  with no such start, it is the session being played today. With none, the line starts with the campaign day
  only.
- **The result** takes the reader's one message place: below the file line on World, below the kind line in
  Lookup. It is in view because the field is there too.
- **Case D:** the panel is 346 px. `Add to Log` keeps its icon when the title leaves no room for its words.
  The field takes a line of its own under the muted start.

**Lookup.** This feature uses Lookup. When Lookup shows another entry, the click goes to the new entry, and
Back returns. An unsaved edit form stays with its entry, and an unsaved NPC stays under its row (section
1.2). While the same entry has an edit form with changes, `Add to Log` is blocked, so that Splat never makes
its own "changed on disk" warning.

**Exact words.** `Add to Log`. Field's accessible name: `Log line for Captain Varra`. Placeholder: `What
happened?`. Result: `Added to Captain Varra’s Log.` `Undo Log line`.

**States.**

| State | What shows |
|---|---|
| The entry has no Log | The first line makes `## Log` as the last heading |
| The entry's edit form has changes | Blocked: `Save or discard your edit first.` |
| The entry belongs to a package | No action. Its Log would belong to every campaign that uses the package |
| The file cannot be read | Blocked: `Splat cannot read NPCs/Captain Varra.md. Repair the file first.` |
| The file changed on disk | Splat reads the file again and adds the line to what is there |
| The write fails | An error in the message place names the file. The field stays open with its words |
| Run open, ended or not | The same. A Log line is not a run step |
| Sync on | No effect |

**What the note stores.** One more line, and nothing else:

```markdown
## Log

- [[Session 11]], Day 2: In [[Ashfall Keep]] → [[The Ford]].
- [[Session 12]], Day 3: Told the party who burned the ropes.
```

The session's Read view then lists `Captain Varra  Told the party who burned the ropes.` under **World
changes**, with no second copy in the session note.

**Undo.** `Undo Log line` removes that one line, after a check that the file still holds it. It belongs to
that entry and ends when the DM edits the entry or opens another, as a World change's Undo ends today.

**Keyboard.** Tab reaches `Add to Log` after `Edit`. Enter opens the field; Enter adds; Escape closes.

**What stays with the DM.** The words, and whether a status changed.

**Not built, and why.** A link from a fight's ended row to the entry (a new control on a plain screen; Ctrl+K
is one press). A `/` command for the Log. A Log line on a Location for each fight there. A Log for player
characters. Editing a Log line in Splat (Edit and Obsidian do it).

| Check | F6 |
|---|---|
| Primary action in view | None in Lookup |
| Components and tokens | `Add to Log` in a reader: `Action::quiet` with `icons::ADD`, after `Edit`. On a scene's row: `Action::quiet` in the row's action slot. Field: `add_field`. Start of the line: `Type::Small` in `muted`. Result: `undo_message` |
| In view at 960x640 | The title line with `Edit` and `Add to Log`, the field and the result, above the entry's text |
| States | Blocked: the reason in `Small` after the action |
| Loss and failure | `Undo Log line`. A failed write keeps the typed words |
| Keys and targets | As above. Every target is 24 px or more |

### F7. Rewards record

**Question.** "What did the party get tonight, and is it in the notes?"

**The note owns the rewards.** `Add rewards` writes one block in the session note, under `## Loot and
rewards`, that starts with the fight's Timeline reference. The run stores nothing about it. A fight has its
rewards in the notes when the note has a block with its reference.

**Where it lives.** On the Session page, in the plan row of a fight that has ended: a quiet `Add rewards` in
the row's action place, which is empty today. The DM lands on this page after `Close run`, with the note
beside the plan, so the next step is in view and its result shows where the DM looks. `Copy loot` stays on
the ended view, for a chat or another program.

**What it writes.**

- The fight's link, as events write it: `[[Timeline#^t-08a07eb0ec|Ashfall Crossing]]`. This is the act that
  links a played fight in the note; a played fight has no `Add to notes` today
  (`src/sessions/page.rs:4387`).
- **Loot,** as lines under it: the encounter's loot and the loot of each creature at 0 HP. A roll that was
  made has its result. A roll that was not made is written with `(not rolled)`, as `Copy loot` writes it
  today, and the result of the act says how many.
- **XP,** on the fight's line, when the campaign's notes use XP: the XP of the creatures that ended at 0 HP,
  also those marked absent at 0 HP, and each character's share. F1's function gives the numbers. The line
  names what it counted and what still stood, so the DM can change it.

**XP is the campaign's choice, and the notes hold it.** The choice is not set until the DM picks it once,
where the first rewards are added. There is no setting and no stored mark. The **newest block** is the last
block under the heading in the note of the session with the highest number that has one.

| The campaign's session notes | `Add rewards` writes | The result offers |
|---|---|---|
| Hold no rewards block yet | The link and the loot, with no XP | `Add XP` |
| Their newest block has XP | The link, the XP and the loot | `Leave XP out` |
| Their newest block has no XP | The link and the loot | `Add XP` |

`Add XP` adds the XP words to every block under this note's heading that has none, and `Leave XP out` takes
them from every block that has them. Each changes one file, and each is the other's Undo. So one press picks
for the campaign, in any order of the evening's presses, and each later `Add rewards` follows the newest
block. Splat reads the notes' blocks again when a session note changes, never each frame.

**After the block is written.** The row's action place shows the status link `Rewards in notes`. A click
shows the block in the note; where the note is behind its tab, it changes to the **Note** tab. Splat does not
watch the block. **A limit:** when the fight changes later (a roll is taken back, or the fight goes on and
ends again), the DM changes the block by hand, or deletes it: the row then says `Add rewards` again.

**Loot that nobody planned.** The DM types it under the block, in the note beside the plan: one click and one
line. It is then in the fight's block, where planned loot is.

**The session's XP.** Splat stores no total. The result of each act says the sum of the `XP` numbers that
follow a fight's link under the heading, as they stand in the note, and how many fights it sums.

**Wireframe: the plan, case A.**

```text
>  Ashfall Crossing                         Add rewards   ⋯
   Held the ford · Ended · Round 3
>  Shrine of Cinders                 ✓ Rewards in notes   ⋯
   Ended in round 1
✓ Rewards added under Loot and rewards. XP tonight: 1,650 for 2 fights.   ↶ Undo rewards   Leave XP out
```

**Cases B, C and D.** The plan has the page's width. `Add rewards` keeps its place at the right of the name
line. The result message wraps, and its actions start at its words.

**Lookup.** The act does not use Lookup.

**Exact words.** `Add rewards`. `Rewards in notes`. Result: `Rewards added under Loot and rewards.`; with XP,
`XP tonight: 1,650 for 2 fights.`; with rolls that were not made, `2 loot rolls were not made.` `Undo
rewards`. `Add XP`, with the result `XP added to 3 fights.` `Leave XP out`, with `XP taken from 3 fights.`

**States.**

| State | What shows |
|---|---|
| The fight is not ended | No action. The row has its status link |
| The fight is ended, and the note has no block with its reference | `Add rewards` |
| The note has a block with the fight's reference | `Rewards in notes` |
| The run file cannot be read | Blocked: `Splat cannot read this fight’s run, so its rewards are not known.` |
| The note changed on disk and Splat holds unsaved words | The act does nothing and says `The note changed on disk. Choose in the note first.` |
| The note has no `## Loot and rewards` | The heading is added at the end of the note |
| An older session's fight | The block goes to that session's note |
| Sync on, Lookup open or closed | No effect |

**What the note stores.**

```markdown
## Loot and rewards

- [[Timeline#^t-08a07eb0ec|Ashfall Crossing]] · XP 400 for 2 at 0 HP · 80 each for 5 · 1 still stood (XP 200)
  - 120 gp in a locked coffer
  - A smoky quartz (50 gp), from 2× gems
  - Vault key (Vault Guard)
  - Varra’s signet ring
- [[Timeline#^t-5c1d2e3f4a|Shrine of Cinders]] · XP 1,250 for 3 at 0 HP · 250 each for 5
```

The last loot line is one that the DM typed. The sum reads only the first `XP` number after each fight's
link, so `(XP 200)` is not counted.

**Who holds what.** Splat does not record it. The players keep their sheets on D&D Beyond, and a second list
would go stale. A loot line is words, and the DM can write a name on it.

**Undo.** `Undo rewards` removes the block that the act added, when it still stands as written. `Add XP` and
`Leave XP out` are each other's Undo. Each follows the note rule of section 4.1.

**Keyboard.** Tab reaches `Add rewards` in the row, and the result's actions after its words.

**What stays with the DM.** Who earned XP, whether a creature that fled counts, how loot is shared, and when
the party levels up. A milestone is a line that the DM writes.

**Not built, and why.** `Update rewards`, and any record in the run of what was written (two owners for one
fact). A loot field on the ended view (the note is where the line belongs, and it is beside the plan). A
stored total. A party inventory.

| Check | F7 |
|---|---|
| Primary action in view | The next `Start fight` |
| Components and tokens | `Add rewards`: `Action::quiet` in the plan row's action slot, as `Add to notes` on an event row. `Rewards in notes`: `status_link` with the check icon in `result.success`. Result: `undo_message` with one more quiet action |
| In view at 960x640 | The played rows and their action, with Lookup closed or open |
| States | Not added: `Add rewards`. Added: check and `Rewards in notes`. Blocked: the reason in `Small` |
| Loss and failure | `Undo rewards`. Splat never rewrites a block's loot after it is written |
| Keys and targets | As above. Read controls, 28 px |

### F8. Undo after a restart, and Continue fight

Pull request 232 builds this (head `b305c085`, not merged). This part says what it does, checks it against
the rest of this design, and lists what is still to decide.

**Question.** "I made a mistake, or I ended the fight too soon. Can I still take it back?"

**What is built.**

- **Saved Undo.** The run file keeps the Undo steps in one optional field, `"undo": {"version": 1, "steps": […]}`. `Undo` names and takes
  back the same step after `Close run` and **Resume**, and after the app starts again. One module owns the
  saved form, `src/encounter_run/undo.rs`, and `Run::open` is its one read path.
- **The saved form.** Each step is the state before it, saved as its differences from the state after it: the
  rows that differ, without their creature definitions, and the Log lines that differ. The rest of a state is
  small and is saved whole.
- **Bounds.** At most 100 steps. In the file the steps take at most 16 MiB, and steps read from a file take
  at most about 256 MiB of memory. Over a bound, the oldest steps stay out. With 20 participants and 100
  steps the file grows by about 9%.
- **Old and unread files.** A run file from before this change opens with no steps and says `Nothing to undo
  since reopening`. A file whose steps this version cannot read opens the run and drops only the steps. The
  run says so once, as a warning with `Dismiss`: `Splat could not read this run's saved Undo steps, so Undo
  starts again from here. The run itself is complete.` The Log keeps the line.
- **Continue fight.** An ended fight has one control that takes the ending away. While the ending is the
  newest step, that is `Undo: end encounter`, and no second control shows. After a loot roll, or with no
  steps, it is `Continue fight`: secondary, with the run icon, left of `Close run`. A closed, ended run that
  is read in Encounters → History has it under **Read-only**, blocked with `Close the open run first.` while
  another run is open. It is one named step, `Undo: continue fight`. Play returns to the turn the fight ended
  on, with the loot results kept. The Log says `Fight continued. Round 2: Cinder Wolf 2's turn.`
- **Two faults of today are fixed.** The Timeline saves when the open run closes. A fight whose every step
  since it opened is taken back keeps its old place in the order of play.

**At 960x640 with Lookup open (case D).** The title, the outcome, the save word and `Close run` keep the
first line. With `Undo` and `Continue fight` both in view, the two take a line of their own below, at the
right edge, and `Undo` has its full name. With `Undo` alone, it stays on the first line and says `Undo`, with
the full name in its hover and its accessible name.

**Checks against the rest of the app.**

| Part of the app | What happens |
|---|---|
| The session plan | The fight's row goes from its result to the open fight's status link, and back when it ends again |
| The Timeline | The same event, found by its run. Its result says `Round 2` again. No event is added, so T4.6 holds |
| History | The run leaves History while it is open and comes back when it ends again |
| Ready and In progress | The encounter is In progress while the fight goes on, and returns to Ready when it ends, as the code does today |
| Carried state | A continued fight that ends again is the fight played last, and the next Setup starts from it. A fight whose steps are all taken back is not |
| Loot rolled after the ending | The results stay through Continue fight, and Undo gives them back |
| Sync | Sync starts again as for a resumed run. The first read takes the sheet's value for anything the DM did not change, and that character's steps go; the Log names them. Only a value that the DM changed is a conflict |
| F2 effects | They are part of a row, so the saved steps hold them. The module's row comparison must read the new entries (P4) |
| F4 and F6 | A tracker change and a Log line are never run steps. Each has its own file and its own Undo |
| F7 rewards | The note's block does not follow a continued fight. The DM changes it by hand (the limit in F7) |

**Still to decide.**

1. **The old outcome.** A continued fight that ends again starts `End encounter…` with an empty outcome. It
   should start with the earlier outcome, selected, so that Enter keeps it and typing replaces it. Pull
   request 232 gets this one repair.
2. **Nothing else.** The four changes that round 1 of this document asked for are built or are not needed:
   `Continue fight` shows only when Undo cannot do the same; loot results are kept; steps are saved without
   stat blocks; and no other file's change is a run step.

**Known limits of the build.** After a restart, Undo of a save's damage restores the HP and does not arm the
save again. A closed, ended fight goes on in 2 interactions from History and 3 from the plan row.

**What stays with the DM.** Whether the fight goes on, and what the players' HP means when another fight
began in between.

| Check | F8 |
|---|---|
| Primary action in view | `Close run`, alone |
| Components and tokens | `Undo`: the header's control of today. `Continue fight`: `Action::secondary` with `icons::RUN`, `.blocked(reason)`. The unread-steps notice: `message`, warning tone, with `Dismiss` |
| In view at 960x640 | `Ended · Round 2`, the outcome, `Undo`, `Continue fight`, `Close run` |
| States | The newest step is the ending: `Undo: end encounter` only. A later step, no steps, or History: `Continue fight`. Another run is open: blocked, with the reason |
| Loss and failure | Every step has Undo, also after a restart. Steps that cannot be read are dropped with a message, and the run stays |
| Keys and targets | `U` and Ctrl+Z, as today. `Continue fight` has no letter |

### F9. Travel and random encounters

**Question.** "The party travels for two days. Does anything find them?"

**Where it lives.** In the Travel event's own row, in the session's plan and in the detail column on
Timeline. A Travel row is as today until the DM chooses `Roll for encounters` in the row's `⋯` menu. That one
act puts the block **Encounter checks** on that row, for good: the row's length field, a table, and one line
for each day or watch of the journey.

- **Not a new kind of event,** because Travel exists and already moves the clock.
- **Not shown on every Travel row,** because a DM who only records a journey asked for no checks.
- **Not in Lookup,** where a roll on a table is not kept with the journey.

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
  check each day.` on that table, and the checks roll each day. `Cannot roll` stays the words for a fault in
  the dice or the rows.
- **Which table.** The block's menu lists every marked table. It starts on the table of the newest journey
  that has one. There is no rule about a table's key.
- **A table's name** in this block and in Trackers is its heading (`On the road`), from the one table
  descriptor of section 4.4. The Reader's own button says `Roll 1d20` today and does not change.
- A row's words can link an encounter or an encounter group. This needs no marker.

**Wireframe: the Travel row after `Roll for encounters`, case A. The plan is 330 px.**

```text
⌄  Road to the ford                      Add to notes   ⋯
   Title      [Road to the ford      ]
   Notes      [                      ]
   Encounter checks    On the road ⌄ · 1d20 each day
   How long   [2 days                ]
              Day 3, 08:00 to Day 5, 08:00
     Day 3   [⚄ Roll] [Enter a roll]
     Day 4   [⚄ Roll] [Enter a roll]
     [⚄ Roll all 2]
   Timeline
✓ Encounter checks added to this journey.   ↶ Undo checks
```

After the rolls:

```text
     Day 3   12 · Nothing stirs.
     Day 4   16 · Wolf Pack pick up the trail.
             Plan “Wolf Pack”
✓ 2 travel checks rolled.   ↶ Undo roll
```

- **How long** is the Timeline's field. It is part of the block, so a plain Travel row does not gain a line.
- **The lines follow the length as the DM types it.** Each key press that gives a length the clock can read
  sets the lines at once, with no Enter. An empty length, or one that the clock cannot read, gives one line,
  `This journey`.
- **A check belongs to its offset** from the journey's start: 0 hours, 24 hours, and so on. Its name is read
  from the clock: `Day 3` for a day, `Day 3, 08:00` for a watch or for hours. **A result is never deleted by
  a change of length or of table.** A result whose offset is now outside the journey, or between the new
  table's lines, stays under the muted line `Outside this journey now`, until the DM takes it back.
- **A result's line ends with the quiet `Clear`,** which gives the line its buttons back. So a wrong roll can
  go at any time, also after a restart.
- `Roll for encounters` also opens the row.
- **A result that links an encounter or a group offers `Plan “<name>”`** for each, at most three. One click
  writes the link under `## Plan for tonight` as the first line still to come. So the fight is the next plan
  row and has the primary `Start fight`, and a group is first in Arrivals. Then the line says `In the plan`,
  for as long as the plan holds that link: Splat reads this from the note and stores no mark. A result with
  no link offers nothing, and the DM uses `Fight` on the Add strip, as today.
- More than 31 lines are not listed. The row says `This journey has 240 checks. Split it into shorter
  journeys to roll them here.`
- **Cases B, C and D.** The plan has the page's width, which is wider than 330 px, so nothing more gives. In
  case A a result's words wrap under the day, and `Plan “…”` starts a line at the words' edge.
- **On Timeline** the event's menu has the same `Roll for encounters`, and the detail column shows the same
  block under its own **How long**. `Plan “…”` adds to the plan of the session that the event belongs to.

**Lookup.** The table's name is a link. It opens the table's note at its heading in Lookup, beside the page.
A link in a result opens as every Reader link does. Both go through Lookup's own path, so Back returns and a
draft is kept. A roll on a marked table from Lookup's own **Roll** button is not a travel check: with a run
open it is a step of that run, as today (`src/lib.rs:1220`), and with no run it only shows.

**Exact words.** Menu: `Roll for encounters`. Field label: `Encounter checks`. `<table’s name> · 1d20 each
day`, or `each watch`, or `every 4 hours`. `Roll`. `Enter a roll`. `Roll all 2`. `This journey`.
`Plan “Wolf Pack”`. `In the plan`. `Outside this journey now`. Results: `Encounter checks added to this
journey.` `Undo checks`; `2 travel checks rolled.` `Undo roll`.

**States.**

| State | What shows |
|---|---|
| The campaign has no marked table | No menu line. The row is as today |
| The DM has not chosen `Roll for encounters` on this row | The row is as today |
| The block is on the row, with no length | One line, `This journey`, and no warning |
| The table's rows do not cover the dice | `Cannot roll`, and why |
| The table's `per` cannot be read | The warning above, and one check each day |
| The table was renamed, moved or deleted after a roll | Each result keeps the words it was rolled with. The menu says `Table not found` and offers the other tables |
| `Timeline.md` cannot be read, or changed on disk | The plan's problem block and the Timeline's choices of today. Nothing rolls until they are settled |
| Run open | The same. A travel check is not a run step and uses its own dice |
| Sync on | No effect |

**What `Timeline.md` stores.** The event keeps its table and its checks, in the event's own fields. Splat
writes the body from the list, so Obsidian shows each result under the event. An older Splat keeps these
fields, because unknown event fields stay in the file (`src/timeline.rs:223`).

```yaml
- id: t-5c1d2e3f4a
  kind: travel
  title: Road to the ford
  duration: 2 days
  session: 070eca36-331c-46cc-b5a6-32a6e35265da
  table: road
  checks:
  - {after_hours: 0, roll: 12, row: Nothing stirs.}
  - {after_hours: 24, roll: 16, row: "[[Wolf Pack]] pick up the trail."}
```

**Undo.** `Undo checks` takes the block off the row while it has no result. `Undo roll` removes the results
of the last press, one roll or all of `Roll all`. `Clear` has `Undo clear`. `Plan “…”` gives the plan's own `Undo add`. Each belongs to
the event or the note it changed and ends at that file's next edit, or when the DM leaves the page.

**Keyboard.** Tab walks the fields, the table menu, each `Roll` and `Enter a roll`, then `Roll all`. Enter
presses. A number and Enter in `Enter a roll` records a player's own roll.

**What stays with the DM.** Whether a result happens, where and when on that day, and what it means. Splat
does not plan a fight or start one.

**Maps.** A separate design defines maps. When a Location can give a travel time to another, **How long** can
start with that value. Nothing here needs it.

**Not built, and why.** Checks on every Travel row, and a table found by its key (a DM who marked a table for
another use would get checks without asking). Weather, pace, getting lost and foraging. A table chosen by
terrain. A result that rolls its own dice, such as `2d4 wolves`.

| Check | F9 |
|---|---|
| Primary action in view | The plan's `Start fight`. No roll is primary |
| Components and tokens | Menu line: the row menu's quiet item. **How long**: the Timeline's field (`text_field` with its help line). Label: `small_field`. Table: `menu_button`. `Roll`, `Roll all`: `Action::secondary` with `icons::DICE`. `Enter a roll`: `number_field`. Result: `Type::Small`, the number in `text`, the words in `muted`. `Plan “…”`: `Action::quiet` with `line_start`. `In the plan`: `status`. Messages: `undo_message`; the interval warning: `small_message` |
| In view at 960x640 | The open row scrolls with the plan, as open rows do today. The Add strip stays above it |
| States | Not rolled: the buttons. Rolled: the number and words. Planned: check and `In the plan`. Fault: warning icon and `Cannot roll` |
| Loss and failure | `Undo checks`, `Undo roll`, `Undo add`. A change of length deletes no result. A failed save shows the Timeline's own `Timeline not saved` |
| Keys and targets | As above. `Roll` and `Enter a roll` are 28 px high and at least 24 px wide |

## 4. Cross-feature rules

### 4.1 Every new write

One act writes one file, as a patch of the file on disk now. The app has four write paths today, and each new
act uses one of them. No act has a path of its own.

| Path | The functions of today | What it does with an outside edit |
|---|---|---|
| **Session note** | A plan act today, such as `Fight`, changes the note's draft with `insert_in_plan` and saves through `Draft::save`, which reads the file and writes it with `replace_file` only when the file is as Splat last read it (`src/sessions.rs:1071`, `:1282`, `:663`). `Draft::follow_disk` takes the file's text when Splat holds no unsaved words (`:1423`) | A new act first calls `follow_disk`, so an edit made in Obsidian a moment before is in the text it changes. It then changes one fragment and saves at once. An act on a line that exists (a tick, a removal, an Undo) finds the line by its whole text, nearest its old place; when the line is gone or changed, it writes nothing, says `That line changed on disk.`, and the rows follow the file. When Splat holds unsaved typed words and the file changed, the save stops, as today, and the comparison of today decides. The act is not applied, and its place says `The note changed on disk. Choose in the note first.` |
| **World note** | A Status change reads the note, changes it and writes it with `library::write_note`, which writes only when the file still holds the text that was read (`src/world.rs:1033`, `src/library.rs:1670`) | The act reads the file again and changes that text. An edit form with changes for the same entry blocks the act |
| **`Timeline.md` and `Trackers.md`** | `Book::save` writes the file atomically and checks for an outside change (`src/timeline.rs:924`). The ledger shares this code | Separate events or entries merge with a quiet notice. The same event or entry needs a choice, as today |
| **Run file** | The run saves after each step and offers its own choices when the file changed | As today |

**When an Undo ends.** One rule for each owner, as that owner has today.

- **A note act** (the session note): at the next edit of the note or the plan, by Splat or on disk, or when
  the DM leaves the page. This is the rule of the plan's `Undo add` today. An Undo whose lines no longer
  stand as written is gone.
- **A World note act:** at an edit of that entry, or when the DM opens another, as a World change's Undo
  today.
- **A Timeline act:** at the next note or event edit, or when the DM leaves the page, as the Timeline's Undo
  today.
- **A run step and a ledger entry:** they stay, also after a restart.
- **An Undo shows where the DM acted,** also when the act changed a file that is not in view: a `Plan “…”`
  pressed in Trackers has its `Undo add` on the Trackers result line.

Each row below is one new write.

| Act | The one file | The fragment | Undo | After a restart | A failed save says |
|---|---|---|---|---|---|
| Add a scene with `Scene` (F3) | The session note | One task line under `## Plan for tonight` | `Undo scene` | The line is in the note. The DM removes it as any line | `Notes not saved`, as today. The line waits in the draft |
| Tick or untick a scene (F3) | The session note | One character | The box, again | The mark is in the note | The same |
| `Remove from tonight` on a scene (F3) | The session note | One line out | `Undo remove`, as for a fight today | The line is gone | The same |
| `Start fight` on a reader (F3) | The encounter note; then the run starts and saves, as the editor's `Start fight` does today | A new note `Fight with <name>`; a new run file | `Close run` on a Setup that nobody touched removes both. After a step: `Delete the saved run`, with its Undo, as today | The Setup is a saved run, as today | The editor's and the run's own messages |
| An effect: add, spend, give back, mark spent, recharge roll, remove (F2) | The run file | The participant's `conditions` entry | The run's `Undo: <step>` | The effect, its count and its Undo step are in the file | The run's footer, as today |
| A tracker press, tick, `Apply and finish`, `Dismiss`, roll; the link for each round (F4) | `Trackers.md` | One entry at the end of `changes` | `Undo: <label>`, step by step | The entries and their Undo are in the file | `Trackers not saved`; the entry waits |
| `Add to Timeline` after a tracker act (F4) | `Timeline.md` | One Major event | The Timeline's `Undo add`, on the Trackers result line | The event is in the file | `Timeline not saved`, as today |
| `Add to fight` on a Due row (F4) | The run file | The group's participants | The run's `Undo: <group> joined`, as from Arrivals | As any run step | The run's footer |
| `Carry over` (F5) | This session's note | The embed line or the copied recap, the copied plan lines and threads, and the `carried_from` property | `Undo carry over` | The lines and the property are in the note | `Notes not saved`. One save holds the body and the property, so the act is whole or not at all |
| `Update the link` on a recap whose source was renamed (F5) | This session's note | The embed line | `Undo link` | The line is in the note | `Notes not saved` |
| `Add to Log` (F6) | The World entry's note | One line at the end of `## Log` | `Undo Log line` | The line is in the note | An error that names the file. The field keeps its words |
| `Add rewards` (F7) | The session note | One block under `## Loot and rewards` | `Undo rewards` | The block is in the note | `Notes not saved`. The block waits in the draft |
| `Add XP`, `Leave XP out` (F7) | The session note | The XP words of the fight lines under the heading | Each is the other's Undo | The words are in the note | The same |
| `Roll for encounters` (F9) | `Timeline.md` | The event's `table` | `Undo checks`, while the block has no result | The block is on the row | `Timeline not saved`, as today |
| `Roll`, `Roll all`, `Enter a roll`, `Clear` on a check (F9) | `Timeline.md` | Entries in the event's `checks` | `Undo roll`, `Undo clear`. After they end, `Clear` on a result's line takes it back | The results are in the file | The same |
| `Plan “<name>”` (F9, F4) | The session note | One link line, before the first line still to come | The plan's `Undo add`, as for `Fight` today | The line is in the note | `Notes not saved` |

- **No act writes two files of the DM's.** `Start fight` saves the encounter and then starts the run, as the
  editor's `Start fight` does today for a new encounter; this design adds no second save to it.
- **No hidden state.** That a note carried over is its `carried_from`. That a fight's rewards are in the
  notes is the block. That a check's fight is planned is the plan's link. That a campaign uses XP is its
  newest block. That a journey has checks is its `table`. That a round's action was pressed is its entry.
- **Only two stores have Undo after a restart:** the run (F8) and the tracker ledger (F4). After a restart a
  note keeps what an act wrote, and the DM removes it as any words. A travel result is not words in a note,
  so it has `Clear`.

### 4.2 One fact, one owner, one place to act

"Acts on it" is the one place where the DM changes the fact. "Also shows" is a read-only echo with the same
words, drawn from the same owner.

| Fact | Owner file | The DM acts on it | Also shows, read-only |
|---|---|---|---|
| A fight's difficulty (F1) | None. Computed from the encounter note, the saved sheets and the rule entry | Nowhere. It follows the roster | Editor heading, Encounters row, plan row, Arrivals |
| An effect, its time and its count (F2) | The run file, in the participant's one `conditions` entry | The dock's `Effect` field to add; the target on the row for every other act; **Conditions** to remove | The notice line on its turn; the Final state rows |
| A scene and its done mark (F3) | The session note, under `## Plan for tonight` | The plan row's box; the note | Obsidian |
| A tracker's value (F4) | `Trackers.md` | The Trackers view | The house rule note's page |
| A Due notice, its rolls and its end (F4) | `Trackers.md`, as entries | Its row in the Trackers view | The rail's count; the bell and the count on the run's notice line; its words there for one turn |
| The recap of the last session (F5) | The last session's note, `## What happened` | That note | This note's `## Recap of last time`, by an embed; the **Last time** line |
| A carried plan line or thread (F5) | This session's note | This note | Nowhere. It is this session's own line |
| Which parts a note carried (F5) | The note's `carried_from` property | `Carry over`, `Undo carry over` | Whether the **Last time** line has the action |
| An NPC's status (today) | The entry's note | **Change** in the entry's reader | The reader's kind line; a scene row's second line |
| A Log line (F6) | The World entry's note, `## Log` | `Add to Log`, or a **Change**, in the entry's reader | The session's **World changes**; the pages of entries that the line links |
| What the party found in a fight (today) | The run file | The ended view: `Roll` | The creature's card; `Copy loot` |
| A fight's rewards in the notes (F7) | The session note, `## Loot and rewards` | `Add rewards`; then the note | The row's `Rewards in notes` |
| Whether the campaign's rewards have XP (F7) | The newest rewards block in the session notes | `Add XP`, `Leave XP out`; the note | What the next `Add rewards` writes |
| The session's XP (F7) | None. The sum of the fight lines in the note | Those lines | The result of `Add rewards` |
| A fight's result (today, F8) | The run file. `Timeline.md` holds the result line | `End encounter…`, `Undo`, `Continue fight` | The plan row, Timeline, History |
| The Undo steps of a fight (F8) | The run file | `Undo` in the run's header | Nowhere |
| A journey's length, table and checks (F9) | `Timeline.md` | The Travel row, in the plan or on Timeline | The clock; the Timeline's body in Obsidian |
| That a check's fight is planned (F9) | The session note: the link under `## Plan for tonight` | `Plan “…”`; the plan row's `Remove from tonight` | `In the plan` on the check |

### 4.3 Pairs that could show one fact twice, and which one wins

| Pair | The risk | The rule |
|---|---|---|
| F2 reminder and F4 Due notice | A mark's words name a turn, and both speak of it | The notice is the campaign's fact, and the bell stays on the notice line until it is ticked. An effect that the DM types on a row is the DM's own second note, and Splat never makes one from a notice |
| F2 effect and an official condition | `Frightened` twice on a row | Words that are a condition's name become that condition, with the time. One entry |
| F2 a condition and its count | A count that outlives its condition after a sheet removes it | One entry holds both. Every act that changes the condition changes the entry |
| F2 the row and the notice line | Two places to act on one effect | The row is the place to act. The notice line has no control |
| F3 scene and a Timeline event | A scene as a plan line and as a Major event | The scene is the plan. The event is what happened. A done scene with one entry offers `Add to Log`, not an event |
| F3 scene and a World open question | An unticked task counted in two lists | World counts only World notes' tasks, as today (`src/world/checks.rs:72`). A scene is in a session note |
| F3 a scene that links an NPC and the fight with that NPC | One link read as a scene and as a fight | The improvised encounter is named `Fight with <name>`, so the NPC's own name is never an encounter's name |
| F3 done scene and F6 Log line | The same sentence in the session note and in the entry | The Log line is the entry's fact. The plan line stays the plan. `Add to Log` only offers the scene's words |
| F6 `Add to Log` and a Status change | A death written as words while the status says alive | A fact that a field holds is a **Change**, which writes its own Log line. `Add to Log` is for what no field holds. Every journey here records a death with **Change** |
| F4 quick action and a Timeline event | A change in the ledger and on the Timeline | The ledger owns the change. `Add to Timeline` is a second, chosen act with the Timeline's own Undo |
| F4 table roll and the run's Log | One roll in two records | The ledger owns a quick action's roll and a notice's rolls. A roll from a note's own **Roll** button stays a run step, as today |
| F4 `Undo: <label>` and the run's `Undo` | Two Undo controls in view in a fight | Each names its own step and takes back only its own file's change. `U` is the run's |
| F4 the link for each round and the quick action | One act in two places | Both write the same ledger entry. The link is the quick action, offered once at each new round |
| F5 recap and F5 summary | The summary copied, and embedded | The embed of `What happened` wins. The summary is copied only when `What happened` is empty |
| F5 the preview and the embed | Two sessions told as "last time" | Both read the last session's `What happened` |
| F5 carried line and the old line | The same plan line in two notes | Each note owns its line. Carry over never changes the old note |
| F6 Log line and **World changes** | A change listed in the session note and in the entry | The entry owns it. The session's list is computed, as today |
| F7 the rewards block and the run | The note says what the run no longer holds, after an Undo or `Continue fight` | The note owns the rewards after `Add rewards`. Splat does not compare them again, and the DM changes the block by hand. A stated limit |
| F7 `Add rewards` and `Copy loot` | Loot pasted and added | `Copy loot` writes no file. A pasted copy is the DM's own text |
| F7 XP and F1 XP | Two sums for one fight | One function. F1 counts every creature against the party. F7 counts those at 0 HP and names the rest |
| F8 `Undo: end encounter` and `Continue fight` | Two controls, one effect | `Continue fight` shows only when Undo cannot do the same in one press (built) |
| F9 travel check and a roll in Lookup | One table rolled in two places | The Travel row keeps its results in `Timeline.md`. A roll from Lookup's **Roll** button is a run step when a run is open, and otherwise only shows, as today |
| F9 `In the plan` and the plan | A mark that outlives the plan line | Splat stores no mark. It reads the plan |
| F9 planned fight and the fight's own event | A fight on the plan and on the Timeline | As today: the plan link is the intent, and the run adds the event at Begin |
| F1 word in the plan and in the editor | Two values | One function, called by both |

### 4.4 One pattern for one act

Each act has one control, one set of words, one place and one key, with Lookup open or closed. "Today" names
where the pattern is already in the app.

| Act | Control | Words | Place | Key | Today | Used by |
|---|---|---|---|---|---|---|
| Add a thing of a kind | A quiet icon-and-word action of the Add strip, or a line of its **More** menu | The kind: `Fight`, `Scene`, `Travel` | The strip under **Tonight**, and on Timeline | Tab, Enter | Fight, Group, rests, events | F3 `Scene` |
| Add typed words to a list | The add field: one field with no button. Enter adds. An arrow key and Enter, or a click, on a suggestion adds it. Escape clears and closes. The field stays open after Enter only in a list that takes several lines in a row | A placeholder that says what to type | The end of its list. In the run, the dock's second line. In a reader, under its header | Enter, Escape | `Add loot` in the encounter editor | F2 `Effect`, F3 scenes, F6 `Add to Log` |
| Put a record in the session note | A quiet action in the row's action place. Then a status link in the same place | `Add …`, then `… in notes` | The plan row, right of the name | Tab, Enter | `Add to notes`, `In notes` on event rows | F7 `Add rewards`, `Rewards in notes` |
| Start a fight | One bordered button | `Start fight` | The plan row, the Encounters row, the editor, a reader | Tab, Enter | Plan, Encounters, editor | F3 on a reader |
| Mark done | A box at the start of the row | Accessible name `Done: …` or `Open: …` | Where a fight row has its caret | Space | World's task marks, read-only | F3 scenes; F4 Due rows with no proposed changes |
| Apply proposed changes | A quiet action at the end of the row | `Apply and finish` | The Due row | Tab, Enter | New | F4 |
| Roll | The dice icon marks every control that rolls. In a list or a panel it is a bordered button. On a run row it is the word with the dice after it. A result takes the button's place when the thing rolls once, and is listed under it when it can roll again. `Roll all N` rolls several. `Enter a roll` takes a player's own roll | `Roll`, `Roll all 2`, `Roll <table’s name>` | Beside the thing rolled | Enter | Tables in the Reader, loot, the dock, numbers in a stat block | F2 a spent recharge, F4 notices, F9 checks |
| Name a table | Its heading, from one descriptor for every marked table: source note, heading, key, dice, interval | `On the road` | Menus and roll buttons outside the table's own note | | New. The Reader's own button says `Roll 1d20` and stays | F4, F9, and loot's table menu |
| Undo in a run | The header's one button, which names the step | `Undo: <step>` | Header, line two, right | `U`, Ctrl+Z | Every run step | F2, F4 `Add to fight`, F8 |
| Undo of a tracker act | The result line's first action, which names the act | `Undo: <label>` | The Trackers view, bottom edge | Tab, Enter | New | F4 |
| Undo anywhere else | A message that says what happened, with a quiet Undo on its line. It belongs to one file and ends at that file's next edit or when the DM leaves | `<What happened>.` then `Undo <noun>` | Where the DM acted | Tab, Enter | Plan, Timeline, World, Library | `Undo scene`, `Undo carry over`, `Undo Log line`, `Undo rewards`, `Undo checks`, `Undo roll` |
| Record a fight's result | The ending's field and Enter | `End encounter…` | The run's footer | Enter | Today | F8 adds the way back, `Continue fight` |
| Change a fact that a field holds | **Change** beside the value | `Change` | The entry's reader | Tab, Enter | Status, place, group, stat block | F6 uses it for a death |
| Remove | The row's menu, with an Undo message | `Remove from tonight`, `Dismiss` | The row's `⋯` | Tab, Enter | Fights and groups in the plan | F3 scenes, F4 `Dismiss` |
| Open a thing beside the page | A link with the thing's name | The name | In the words | Tab, Enter; Alt+Left returns | Reader links, condition words | F3 scene links, F4 `Due N` and the bell, F9 the table's name |
| Go to Trackers and back | One key | | | Ctrl+T | New | F4 |
| Say why a control is off | `.blocked(reason)`, with the reason beside a lone control | A short sentence | Beside the control | | Every blocked control | Every state table in section 3 |

## 5. Consistency

### 5.1 New controls and their components

No feature adds a colour or a type size. Four icons join `design::icons`: `PIN`, `HOURGLASS`, `BELL` and
`CHECK_SQUARE`. Section 3 names the component and token of each control in its check table. Three parts move
to a shared place before the features, so that every user has one copy (P1c):

- The loot editor's add field with suggestions becomes the shared part `add_field`, with today's behaviour.
  Its callers of today, the encounter's loot and a creature's loot, are checked against it.
- One descriptor for a marked table: source note, heading, key, dice and interval. The Reader, loot, travel
  and trackers read it. Today `rolls_marker` gives only the key and the dice (`src/roll_table.rs`).
- Lookup's history holds a place: an entry or the Trackers view.

The Timeline's **How long** field also becomes a form part that the Travel row draws (P5).

### 5.2 Proposed text for `docs/design-system.md`

This pull request does not edit that file. Each pull request of the build plan adds its own lines. Where a
line below names a section of this document, that section's rule is the text.

- **Widgets, Add strip.** "On a plan, Scene is the last action and the first to go into `More Add actions`.
  It never takes the place of an action that fits today."
- **Widgets, a new row.** "Add field | `add_field` | One field with no button. Enter adds the words. An arrow
  key and Enter, or a click, on a suggestion adds it. Escape clears and closes. It stays open after Enter in
  a plan and a loot list, and closes after an effect and a Log line. At most four suggestions sit under it,
  or above it at a pane's bottom edge."
- **The encounter ledger and editor.** "A planned fight's summary and the roster heading end with the
  difficulty word, and in Arrivals a group's XP is followed by `→ Moderate`. These are passive words: no new
  line, nothing cut, no colour, no icon, and nothing unless the XP, the levels and the budget table are all
  known. Hover gives the three budgets." (F1)
- **The live run, Header and Roster.** The two tables of section 1.4. And: "An official condition's name
  opens its rule, with or without a count. A count is its own target, 24 px or more. An effect's target says
  in its hover what a click does. A spent recharge ends with the dice icon. After damage to a row marked
  Conc, its second line ends with `Conc save DC 12` until the next step; the words add no line."
- **The live run, Dock and Panels.** "The dock's second line is the selected name, then Effect, Conditions,
  Conc, Absent. Effect never cuts the name: it keeps its icon, then gives up its place. Effect turns the
  second line into one add field for the row it opened for; Enter and Escape put focus in Amount. The
  Conditions tab has no free-words field." With the help sentence of F2.
- **The session's plan.** "A task line under `Plan for tonight` that links no encounter is a scene: a 24 px
  box where a fight has its caret, the words as Reader text, and a second line only when the words link one
  World entry. Rows to come keep the note's order. A done scene stays in place, with muted words. A played
  fight's action place holds quiet `Add rewards`, then the link `Rewards in notes`. A Travel row's menu has
  `Roll for encounters`; after it the open row shows Encounter checks, with How long. A plan act changes the
  file's text as it is on disk and saves at once." (F3, F7, F9, section 4.1)
- **Notes and templates.** "The Last time line ends with quiet `Carry over` while a part is still to carry.
  It never takes a line of its own. The preview reads the last session's What happened. In Read, an embed of
  another note's heading shows that text under a muted `From <session>` link, or says that its source is
  gone." (F5)
- **World.** "A campaign entry's title line has quiet `Add to Log` after `Edit`; at a narrow title it keeps
  its icon. It opens one add field under the kind line, after a muted line start. With no run open, a reader
  says `Start fight` where it says `Add to fight` during a run, in the same style. While Lookup holds an
  unsaved NPC, one row says `Unsaved NPC: <name> · Show`, and the opened entry shows below it." (F6, F3)
- **Lookup and copies.** "Trackers is a Lookup view and a place in Lookup's history. Ctrl+T goes to it, and
  from it back to the place before. Pinned trackers keep the note's order; Due rows sit above the result
  line and grow upward; the result line keeps the panel's bottom edge. No feature adds a panel beside Lookup
  and Notes. Outside its own note, a marked table is named by its heading." (F4)
- **The rail.** "The status slot holds `Due N` with the bell icon while a notice is due, and nothing about
  trackers at other times."
- **Icons.** "`PIN`: an effect with a time and no count. `HOURGLASS`: a count. `BELL`: Due. `CHECK_SQUARE`:
  Scene."

### 5.3 Proposed text for `docs/ux-intent.md`

**Changed workflows.** Pull request 232 has its own changes to W4 and to assumption A8.

- **W5,** add: "The editor says how the roster compares with the party's XP budget, from the rules the
  campaign uses." Done well: T5.7.
- **W11,** add: "A journey that the DM asks to check rolls its encounter checks in its own row, and a result
  becomes a planned fight in one step." Done well: T11.9, T11.10.
- **W14,** add: "The plan also holds what is not a fight: a scene, with a done mark. The note holds it. A
  creature's or an NPC's page starts a fight that nobody planned." Done well: T12.10 to T12.12.
- **Section 3, Reachable from anywhere,** add: "**Trackers:** Ctrl+T opens the trackers and the Due list in
  the Lookup panel, and goes back. A Due count shows in the rail while a notice is due."

**New workflows.**

- **W15. Keep track of what lasts.** "Trigger: a spell, a poison, a breath weapon or a legendary creature.
  Goal: on each turn the DM sees what they asked to be reminded of for that creature, and how long it lasts,
  with no answer needed. Context: table. Splat counts and prompts. The DM rules." Done well: T17.1 to T17.6.
- **W16. Close the evening and open the next.** "Trigger: the last fight ends, or the next session starts.
  Goal: rewards, what changed in the world, the recap and the open threads are where the next step needs
  them, with nothing typed twice. Context: desk." Done well: T18.1 to T18.4.
- **W17. Run a house rule's numbers.** "Trigger: a race, a clock, a count of seals. Goal: one press records a
  change, a due notice stays in view, and the history is a file. Context: table and desk." Done well: T19.1
  to T19.5.

**New rows for section 8.**

| ID | Observable test | Scenario | Type |
|---|---|---|---|
| T5.7 | The roster heading ends with the XP and a word from the rules data. With no table, no sheets or no XP the heading is as today | encounter-difficulty **new** | state |
| T11.9 | After `Roll for encounters`, a two-day journey rolls two checks in 1, and each result is in `Timeline.md`. A shorter length deletes no result | travel-checks **new** | cost, state |
| T11.10 | A result that names an encounter is planned in 1, and its `Start fight` is the next one | travel-checks **new** | cost, state |
| T12.10 | A task line is a scene with a box. A tick changes one character of the note. A plain list line gets no row | session-plan-scenes **new** | state |
| T12.11 | A scene's link opens its entry in Lookup in 1, and Back returns to what was open | session-plan-scenes **new** | cost, state |
| T12.12 | A creature or an NPC starts a fight in 1 from its reader. `Close run` on the untouched Setup leaves no run and no encounter | session-plan-scenes **new** | cost, state |
| T17.1 | An effect with a count joins a row in ≤ 4, with the run's panel hidden. Focus is in Amount after it | run-effects **new** | cost, view |
| T17.2 | On its turn the effect's words show on the notice line and on the row. A label with no time shows on no notice. `Next turn` takes 1 and does not move | run-effects **new** | state, view |
| T17.3 | A count of 10 says 10 on the first turn it covers and 9 on the next, says `time is up` at 0, and removes nothing. Undo of `Next turn` gives it back, also after a restart | run-effects **new** | state |
| T17.4 | At 960x640 beside Lookup, with a sync conflict: the notice line holds the conflict, the reminders' count and the bell; the current row holds three reminders; no row changes height at `Next turn` | run-effects **new** | view |
| T17.5 | A spent recharge rolls in 1 and a use is spent in 1, both on the row. A counted condition's name still opens its rule | run-effects **new** | cost, state |
| T17.6 | Damage to a row marked `Conc` adds the save DC at the end of the row's second line, with no action, no notice and no new line | run-effects **new** | state |
| T18.1 | A fight's loot results reach `Loot and rewards` in 1, with the fight's link and one Undo. The row then links the block. The run file does not change | session-plan | cost, state |
| T18.2 | A line reaches an entry's Log in ≤ 4 from a done scene. The file gains one line | world-facts | cost, state |
| T18.3 | `Carry over` adds the recap link, the unplayed plan lines and the open threads in 1, below what the DM wrote. A recap that the DM wrote is kept. A recap that comes later can still be carried. A deleted carried line does not come back | session-carry-over **new** | cost, state |
| T18.4 | A note the DM did not act on does not change. A new session with no carry-over is the template | session-carry-over **new** | state |
| T19.1 | Ctrl+T opens Trackers in Lookup, and Ctrl+T again shows the entry that was open | trackers **new** | cost, state |
| T19.2 | A quick action takes 1 and names its change. `Undo` steps back through the session's acts, also after a restart | trackers **new** | cost, state |
| T19.3 | A value that passes a mark raises a Due row above the result line, the rail's count, and the bell at the end of the run's notice line. No quick action moves | trackers **new** | state, view |
| T19.4 | A full roster and Trackers fit at 960x640, with the result line and the Due rows in view | trackers **new** | view |
| T19.5 | A quick action marked for each round shows as one link at a new round, and one click records it | trackers **new** | cost, state |
| G6 | The unchanged evening of the audit gives the counts it gave before the change | dm-audit | cost |
| G7 | The plain campaign's captures differ from the reference only by the rows of the two plain-evening tables | plain-evening **new** | look, state |

### 5.4 Proposed terms for `CONTEXT.md`

- **Encounter difficulty**: the word that compares an encounter's XP with the party's XP budget: Low,
  Moderate, High or Above High. Splat shows it and blocks nothing. _Avoid_: Rating, challenge.
- **XP budget**: the XP a party can face at one difficulty, from the rules the campaign uses: for each
  character, the value for its level.
- **Effect**: words that the DM pins to one participant of a run. It can have a time (a count of rounds, or
  on its turn), a count of uses, or a recharge. It stays in that run. _Avoid_: Custom condition, timer.
- **Reminder**: the words of an effect that has a time, on the run's notice line during its participant's
  turn. It asks for nothing.
- **Scene**: a task line of a session's plan that is not a fight or a group: a talk, a person, a place or a
  secret, with a done mark. _Avoid_: Plan item, task, to-do.
- **Carry-over**: the act that adds the last session's recap link, unplayed plan lines and open threads to a
  session's note.
- **Rewards**: what the party received in a fight: its loot results and, in a campaign that uses XP, its XP,
  under the session note's `Loot and rewards`. _Avoid_: Treasure.
- **Travel check**: one roll on a marked table for one day or watch of a Travel event, kept with that event
  by its offset from the journey's start.
- **Trackers**: the view of the Lookup panel that holds the trackers and the Due list.
- **Session plan**, change to: "The fights, groups and scenes a session's note lists under its plan, with
  what has happened so far."
- **Log**, add: "The DM can also add a line in their own words."
- The tracker terms of [World and house rules](world-and-house-rules.md) stand: house rule note, in play,
  tracker, quick action, mark, Due list, ledger.

## 6. Build plan

Each pull request merges alone and leaves `main` working. `src/sessions/page.rs` (8,107 lines) and
`src/encounter_run/view.rs` (9,859 lines) are shared, so the plan first moves the parts that several features
touch into files of their own, and first builds the three shared parts.

**Rules for every pull request.**

- Its visual-test scenarios cover each changed surface **with Lookup closed and with Lookup open, at
  960x640**, and once at 1200x800. A new scenario goes at the end of `SCENARIOS`. The author looks at each new
  or wider menu and suggestion list in the 960 px capture.
- It extends an existing scenario or test when that one has the same setup, before it adds a new one.
- It breaks the code once for each new check and says so.
- For each result that stays on screen and each Undo, it tests the owner and the end: an act A, an unrelated
  act B, then Undo of A. B's change stays, and A's words go with A's data.
- For each new write it tests the row of section 4.1: an edit on disk a moment before, unsaved words with a
  changed file, a failed save, and a restart.
- It runs the two plain checks: the unchanged script gives the counts it gave before (G6), and the plain
  campaign's captures differ only by this pull request's rows of section 2.5 (G7). To prove G7 can fail, it
  turns the feature on for the plain path once and sees the check go red.
- It adds its own lines to `docs/design-system.md`, `docs/ux-intent.md`, `CONTEXT.md` and `README.md`, from
  section 5, and its task to the evening in `src/visual_tests/dm_audit.rs` as a **new** task.

| PR | Scope | Files and modules | Depends on | Scenarios | In the dm-audit script |
|---|---|---|---|---|---|
| **P0** | F8. Built in pull request 232. One repair: a continued fight that ends again starts the ending with its old outcome, selected | `src/encounter_run.rs`, `src/encounter_run/undo.rs`, `src/encounter_run/view.rs` | None | `run-continue`, `run-restart`, as built | A task: end by mistake, restart, `Undo`. A fact: the turn and round are the same |
| **P1a** | The audit. Repair the script: drop the task that clicks `Add to notes` on a played fight, and keep setup acts apart from counted acts. Freeze it. Measure `main`, and write `steps.tsv`, `summary.tsv` and the README's table. Add the plain campaign and its reference captures (G7), with a fixture that has old task lines, old free-words effects and a marked table, and split the script's totals into measured and new tasks (G6) | `src/visual_tests/dm_audit.rs`, `src/visual_tests/scenarios.rs`, `src/visual_tests/fixture.rs`, `artifacts/dm-audit/` | None | New `plain-evening` | The new yardstick. To be measured here |
| **P1b** | No change in behaviour. Move the plan pane to `src/sessions/plan.rs` and the run's dock and notice line to `src/encounter_run/dock.rs` and `notice.rs` | `src/sessions/page.rs`, `src/sessions/plan.rs`, `src/encounter_run/view.rs`, two new files | P0 merged, so that the two do not meet in `view.rs` | None new. `visual-compare` against `main` shows no difference | None |
| **P1c** | The shared parts. `add_field` out of the loot editor, with its two callers checked. One table descriptor in the content index, read by the Reader and by loot. Lookup's history holds places, and an entry that opens while an NPC is held shows under its row | `src/loot_editor.rs`, `src/design/components.rs`, `src/roll_table.rs`, `src/lookup.rs`, `src/history.rs` | None | Extend `run-loot`, `roll-table-author`, `npc-in-lookup` | None |
| **P2** | F1 | New `src/difficulty.rs`. `src/encounter_settings.rs` (the budget table). `src/encounters.rs` (heading), `src/encounters/list.rs`, `src/sessions/plan.rs` (one summary part), `src/encounter_run/ad_hoc.rs` (Arrivals) | P1b | New `encounter-difficulty`. Extend `encounter-build`, `session-plan`, `encounter-groups` | A fact for each planned fight: its word |
| **P3** | F3: scenes, `Scene` on the Add strip, `Start fight` on a reader | `src/sessions.rs` (plan lines), `src/sessions/plan.rs`, `src/design/components.rs` (the strip's last action), `src/timeline/page.rs` (`quick_add`), `src/world.rs` and `src/library/view.rs` (the readers' action), `src/encounters.rs` (start, and the untouched close) | P1b, P1c | New `session-plan-scenes`. Extend `session-notes` (More menu), `npc-in-lookup` | Tasks: type two scenes; open the NPC from its row; tick it; the fight nobody planned from a reader |
| **P4** | F2 | `src/encounter_run.rs` (the `conditions` entries, the count in `end_turn`, commands, the sheet's path), `src/encounter_run/undo.rs` (row comparison), `dock.rs`, `notice.rs` (parts 1, 2 and 5 of section 1.4), `view.rs` (the row's second line, the Conditions tab), `src/rolls.rs` (read `recharge` and `legendary_actions`) | P0, P1b, P1c | New `run-effects`. Extend `encounter-run`, `run-long-names`, `run-sync-conflict`, `run-edge-states` | The Frightened task with a count. A fact: the notice on Oren's turn |
| **P5** | F9 | `src/timeline.rs` (`table`, `checks`, the body), `src/timeline/page.rs` (the detail column and the event's menu), `src/sessions/plan.rs` (the row's menu and block), `src/roll_table.rs` (`per` in the descriptor) | P1b, P1c | New `travel-checks`. Extend `session-notes`, `roll-table-author` | Tasks: roll the road check; plan and start the result |
| **P6** | F6 | `src/world.rs` (the header action and field), `src/world/lines.rs` (reuse `add_log`), `src/world/page.rs`, one action in `src/sessions/plan.rs` | P1c, P3 | New `log-from-play`. Extend `world-facts` | A task: a Log line from the done scene. A fact: the file gained one line |
| **P7** | F7 | `src/sessions.rs` (the rewards block, the newest block), `src/sessions/plan.rs` (played row), `src/encounter_run/loot.rs` (the lines to write, read from a run), `src/difficulty.rs` (XP at 0 HP) | P2, P3 | Extend `run-loot`, `session-plan` | Tasks: `Add rewards` for two fights, `Add XP` once. A fact: the block. The old `Add to notes` task stays out |
| **P8** | F5 | `src/sessions.rs` (carry, `carried_from`), `src/sessions/page.rs` (`last_time_block`, `recap_preview`), `src/markdown_view.rs` (an embed of a heading) | P3, for scenes in the carried lines | New `session-carry-over`. Extend `session-plan` | A task: `Carry over`. A fact: the lines in the note |
| **P9** | F4, part one: house rule notes, in play, the ledger with its Undo, quick actions, the Trackers view, Ctrl+T | New `src/trackers.rs`, `src/trackers/ledger.rs`, `src/trackers/view.rs`. `src/lookup.rs` (the view, on P1c's place), `src/lib.rs` (the key), `src/campaign_content.rs` (in play), `crates/core` (`KNOWN_FIELDS`) | P1c | New `trackers`. Extend `run-lookup`, `nav-state` | None in the measured evening. J4 has its own script |
| **P10** | F4, part two: marks, the Due list, the rail's count, parts 3, 4 and 6 of the notice line, `Add to fight` and `Plan “…”` on a Due row, `Add to Timeline` | `src/trackers.rs`, `src/trackers/view.rs`, `src/encounter_run/notice.rs`, `src/lib.rs` (the rail) | P9, P4 | Extend `trackers`, `run-effects` | J4's script |
| **P11** | F4, part three: day marks | `src/trackers.rs`, `src/timeline.rs` | P10 | Extend `trackers` | J4's script |
| **P12** | The audit, again. Extend the evening to use every feature, measure it, and write `steps.tsv`, `summary.tsv` and the README's table. Add the scripts for J2, J3, J4 and J6. Run G6 and G7 on the whole | `src/visual_tests/dm_audit.rs`, `artifacts/dm-audit/` | All | The audit's own captures, and `plain-evening` | The whole evening. To be measured here |

**What can be built at the same time.**

- **First, three at once:** P1a (the audit and its fixtures), P1c (the shared parts), and P1b once P0 has
  merged. They share no file.
- **Wave A, three at once:** P2 (encounters, Arrivals), P3 (plan, readers), P5 (Timeline). They meet in two
  files, each in its own function. In `src/sessions/plan.rs`: the fight summary (P2), the scene rows (P3), the
  Travel row's menu and block (P5). In `src/timeline/page.rs`: `quick_add` (P3), the detail column (P5).
- **Wave B, three at once:** P4 (the run), P6 (the World reader), P9 (trackers and Lookup). P6 no longer
  changes `src/lookup.rs`, because the place type and the held-NPC row are in P1c. P4 and P9 share no file.
- **Wave C, two at once:** P7 and P8. Both add a function to `src/sessions.rs`; P7 is in the plan row and P8
  on the **Last time** line.
- **Then:** P10, P11, P12, in order. The full gate runs on merged source after each wave.

## 7. Limits, and what is still open

**Known limits.** Each is a limit of today's app that this design keeps and states.

- **An overlapping edit is not merged.** When Splat holds unsaved typed words for a note and the file changed,
  the comparison of today offers one version or the other. A feature's act never adds to this case: it does
  nothing until the DM has chosen (section 4.1).
- **Lookup's place is not kept across a restart,** and a held World form is lost with it. J3 counts the
  search.
- **The rewards block does not follow the fight** after it is written (F7).
- **A closed, ended fight goes on in 2 or 3 interactions,** not 1 (F8).
- **The typed route to a scene costs a tab change** where the note is behind its tab (cases B, C and D).

**The round 1 questions are answered in the text.** The save DC stays, as words after `Conc` (F2). `Effect`
has one home, the dock, and old free words do not remind (F2). Trackers is a Lookup view with Ctrl+T there
and back (F4). The recap is an embed, with the session's ID beside it (F5). XP is not set until the DM picks
it once (F7). `Continue fight` shows only when Undo cannot do the same, as built (F8). An improvised fight's
encounter is `Fight with <name>`, saved as today (F3).

**Open questions.** Work can go on with each default.

1. **Where the campaign's XP choice lives.** This design reads it from the newest rewards block, so that no
   act writes two files and nothing is hidden. The other choice is a campaign setting, which the first `Add
   rewards` would have to write as a second file or as a step of its own. *Default:* the newest block.
2. **`Start fight` on a creature's reader while a run is open.** An NPC's reader says **Add to fight** there
   today. A creature's reader has no such action today, so this design blocks `Start fight` with its reason.
   The other choice gives a creature's reader **Add to fight** too. *Default:* blocked; Arrivals adds a
   creature to an open fight, as today.
