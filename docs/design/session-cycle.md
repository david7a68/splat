# The session cycle

Status: proposed. Nothing here is built, except where a line says "today". This document fits nine features
(F1 to F9) into the app as one workflow: prep at the desk, play at the table, the wrap-up, and the next prep.
It follows [the brief](../ux-intent.md), [the design system](../design-system.md) and the words in
[`CONTEXT.md`](../../CONTEXT.md). The yardstick is the measured evening in
[`artifacts/dm-audit/`](../../artifacts/dm-audit/README.md): 13 page transitions and 231 interactions.

Evidence: `file:line` refers to `main` at `20fd453a`. Screens were read from a capture of that commit.

**How to read this.** Section 1 holds the rules that every feature obeys, with the rule for the side panels.
Section 2 walks six evenings step by step, with counts. Section 3 holds one part for each feature. Sections 4
to 7 are the tables that a reviewer or an engineer needs: one owner for each fact, one pattern for each act,
the components, the proposed text for other documents, the build plan and the open questions.

## Decisions in brief

Each line is a decision. Section 3 gives the reasons.

| # | Feature | Where it lives | What the DM does | New page, panel or modal |
|---|---|---|---|---|
| F1 | Encounter difficulty | A word after a fight's summary in the editor, the Encounters list, the plan and Arrivals. One function computes it from the rules data | Nothing. It is in view | None |
| F2 | Effects and turn reminders | Words on the roster row, where the DM also acts on them. The header's notice line echoes them on that participant's turn. One new dock control, `Effect` | Select, `E`, type `Bless 10 rounds`, Enter | None |
| F3 | Plan items | A task line under `## Plan for tonight` is a plan row with a done box: a **scene** | `Add` → `More` → `Scene`, type, Enter. Click the box when it is done | None |
| F4 | Trackers and the Due list | One view of the Lookup panel, **Trackers** (Ctrl+T). A Due count in the rail. A due notice's words on the run's notice line | Click a quick action. Tick a notice | None. The old tracker panel is cut |
| F5 | Carry-over | `Carry over` on the Session page's **Last time** line | One click fills the recap, the unplayed plan lines and the open threads | None |
| F6 | Wrap-up to World | `Add to Log`, beside the Status **Change** of today, in a World entry's reader. One click leads there from an ended fight's row, a done scene and a note | Click, type the words, Enter | None |
| F7 | Rewards | `Add rewards` on a played fight's plan row. `Add loot` on the ended view | One click writes loot and XP under `## Loot and rewards` | None |
| F8 | Saved Undo and Continue | The run's Undo button, as today. `Continue fight` on the ended view | `U` after a restart. One click to go on with a fight | None |
| F9 | Travel checks | The open Travel row in the plan and on Timeline | Type `2 days`, `Roll all 2`, `Plan “Wolf Pack”` | None |

Counts for the measured evening (section 2.1): **13 page transitions before and 13 after. The 41 measured
tasks cost 231 interactions before and 231 after.** The DM also keeps seven records in Splat that were on paper
or were not kept before. These cost 18 more interactions, for 249 in all. The audit's files are older than the
Add strip's **More** menu; section 2.1 says what a new measurement of `main` should give.

## 1. Rules for every feature

### 1.1 Six rules

1. **No new page, panel, tab, strip or modal.** Each feature joins a row, a line or a view that exists. The one
   new view is Trackers, and it is a view of the Lookup panel.
2. **A new word comes from the DM's own act.** A count, a notice or a heading shows only after the DM used a
   new control. There are three exceptions, and none needs an answer: the difficulty word of F1; the save DC
   on a row that the DM marked `Conc` and then damaged (F2); and the reminder of a free-words effect that an
   encounter or an older run already holds (F2). Section 2.5 lists every difference that a DM who uses no new
   control can see.
3. **One file owns each fact.** The note or run file holds what the screen shows, in a form Obsidian reads.
   Section 4.1 names the owner of each fact.
4. **Undo, not confirm.** Each change has a named Undo where the DM acted. No feature adds a dialog.
5. **Splat prompts. The DM rules** (principle 7). A count goes down, a notice shows, a roll is offered. Splat
   never removes an effect, spends a use, ends a fight, plans a fight or starts one by itself.
6. **`Next turn`, `Amount` and the current row never move, wait or lose focus** because of a feature. A prompt
   needs no action to go away.

### 1.2 The side panels: the rule today, and the rule for new features

Three things want the space beside a page. This is how the app decides today.

| Thing | Opens with | Rule today | Evidence |
|---|---|---|---|
| **Lookup** | Ctrl+K, a link, a condition word | One panel beside the page at a time. When Lookup opens, Notes give way, and they come back when Lookup closes | `src/lib.rs:1229-1238`, `:2457-2460` |
| **Notes** | Ctrl+J | When Notes open, Lookup closes. On the Session page the panel does not show, because the page has the note | `src/lib.rs:1240-1243`, `:2469` |
| **The run's own panel** (Stat block, Conditions, Correct, Log, Prep, Arrivals) | A row, `C`, **Add to fight** | It is part of the Run page. It stays beside Lookup or Notes only when the roster keeps 560 px. With less, it hides, and the current stat block's action tiles go in one line above the dock | `src/lib.rs:1888-1892`, `src/encounter_run/view.rs:665`, `:924-932` |
| A run task the DM asks for (Conditions, Correct, Arrivals, Log, Keys, Review conflict) | A click or key | Lookup closes. Notes close only when they are what hides the run's panel | `src/lib.rs:1915-1925` |
| Letter keys in a run (`N`, `U`, `A`, `C`) | | Off while Lookup or Notes is open, because a letter is then text. The buttons still work | `src/encounter_run/view.rs:444` |

**The rule for F1 to F9: no feature adds a contender for this space.**

- A feature that needs room beside a page is a view of Lookup. There are two: an entry's reader with
  **Add to Log** (F6), and **Trackers** (F4). So the one-panel rule above does not change, and the old design's
  separate tracker panel is cut.
- A feature opens Lookup only when the DM clicks or presses a key. It uses Lookup's own open path
  (`open_position`, `src/lookup.rs:1619`), so **Back** (Alt+Left) returns to what the DM had open.
- An unsaved edit form in Lookup is kept for its entry, as today (`src/world.rs:1476`). One fault must be
  repaired first: while Lookup holds an unsaved generated NPC, its card hides any entry that opens
  (`src/lookup.rs:2178`). Lookup takes the World page's rule: one row above the reader says
  `Unsaved NPC: <name> · Show`, and the opened entry shows below it. The NPC stays until Save or Discard.
- Ctrl+T is a Lookup key. With Lookup closed, it opens Lookup on Trackers. With Lookup open on another entry,
  it goes to Trackers, and Back returns. With Trackers in view, it closes Lookup. Notes give way as for Ctrl+K.
- One rule above gets an exception. While Lookup shows Trackers, the run's letter keys work, because that
  view has nothing to type into. A click in Lookup's search field makes a letter text again, as today.
- What must never hide is never in a panel. It is on the row, the header, the dock, the plan row or the rail.

### 1.3 Widths, and what never hides

These widths come from the tokens (`RAIL_WIDTH` 136, `side_panel_width` = 36% between 340 and 560,
`PLAN_BESIDE_NOTE_FROM` 720, `TWO_COLUMNS_FROM` 640). Every wireframe in section 3 names one of these four cases.

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
| `Amount`, `Damage`, the dock | Dock, line one | Nothing new is on line one |
| A reminder for the current turn (F2) | The current row's second line, always, with every act. Also the notice line when no sync or recording notice holds it | On the row: later effects fold into `2 more`. The first effect takes a line of its own when it must and is never cut |
| The Due count (F4) | The rail's status slot, above Lookup, on every page | Nothing. The slot grows upward and moves no target |
| The difficulty word (F1) | The end of the fight's summary line | Member names shorten first, with the full text on hover |
| A scene's done box (F3) | The start of its row | The words wrap to at most three lines |
| `Undo` in a run | Run header, line two, right | The target shortens, then the `U` letter drops (today's rule) |

## 2. The evening, end to end

Each journey below uses the step fields of `artifacts/workflow-score/journeys.md`. **Count** uses the kinds of
the audit: `Nav` (go to a page), `Sub` (a tab in a page), `Panel` (open or close a side panel), `Find` (search
or select), `Do` (the work). A page transition is marked **T**. "Must remember" is **nothing** for every row
unless the row says so.

### 2.1 J1: the measured evening, with each feature where a DM would use it

Window 1200x800 (case A, and case B while Lookup is open). Rows with a feature label are new or changed. All
other rows are the 41 measured tasks of `steps.tsv`, with the same actions and counts.

**Before: prep at the desk**

| # | The DM wants | Surface | Action | Count | Today | File |
|---|---|---|---|---|---:|---|
| 1 | To start tonight's note | Rail, then Session | `Session`; `New session`; type the title; `Create session` | Nav 1, Do 3 = 4, **T** | 4 | A new session note from the template |
| 2 | **F5.** What the last session left | Session, the **Last time** line | `Carry over` | Do 1 | New. By hand: about 10 | The new note: one embed line under the recap, the unplayed plan lines, the open threads |
| 3 | To recall last session | Session, **Last time** | Open **Last time** | Find 1 | 1 | None |
| 4 | To compare passive Perception | Party | `Party` | Nav 1, **T** | 1 | None |
| 5 | To build tonight's encounter. **F1** answers "is it too much?" while the roster grows | Session, then the encounter editor | As measured: back to Session, `Fight`, `New encounter`, name, search and add twice, `Save encounter` | Nav 2, Find 6, Panel 1, Do 6 = 15, **T T** | 15 | The encounter note. The roster heading says `XP 550 · Low` |
| 6 | To make the boss their own | Editor | As measured | Do 6 | 6 | The creature copy and the encounter |
| 7 | To write the read-aloud text | Editor | As measured | Do 3 | 3 | The encounter note |
| 8 | To plan both fights. **F1:** each row ends with its word | Session plan | `Back to session`; `Fight`; plan the second fight | Nav 1, Find 1, Do 2 = 4, **T** | 4 | The note: two links under `## Plan for tonight` |
| 9 | **F3.** To plan a talk with Captain Varra and a secret | Session plan, Add strip | `More Add actions`; `Scene`; type `Captain Varra`; ↓; Enter; Enter; type `The miller is the informer`; Enter | Find 1, Do 7 = 8 | New. On paper: 2 lines. Typed in the note: 3 | The note: `- [ ] [[Captain Varra]]` and `- [ ] The miller is the informer` |

**During: at the table**

| # | The DM wants | Surface | Action | Count | Today | File |
|---|---|---|---|---|---:|---|
| 10 | To start recording | Session | As measured | Do 2 | 2 | The recording |
| 11 | To read the recap aloud | Session | Open **Last time** | Find 1 | 1 | None |
| 12 | **F3.** A player asks who Captain Varra is | Session plan, the scene's row. Lookup opens (case B) | Click `Captain Varra` in the row; close Lookup | Panel 2 | 3 | None |
| 13 | To note what the party decides | Session, the note | As measured | Panel 1, Do 1 = 2 | 2 | The note |
| 14 | **F3.** To mark the talk done | Session plan, the scene's row | Click the box | Do 1 | New. On paper: 1 tick | The note: `- [x] [[Captain Varra]]` |
| 15 | To record half a day of travel | Session plan | `Travel` | Do 1 | 1 | `Timeline.md` |
| 16 | **F9.** To check for an encounter on the road | Session plan, the Travel row | Open the row; `Roll` | Find 1, Do 1 = 2 | New. Today with Lookup: 4 | `Timeline.md`: the check and its row |
| 17 | To read the read-aloud text and start the fight | Session, then Run setup | As measured | Nav 1, Do 11 = 12, **T** | 12 | The run file. The timeline event |
| 18 | Round 1 | Run | 8 turns: select, type, Enter, `Next turn` | Find 8, Do 24 = 32 | 32 | The run file |
| 19 | To read Fireball | Run, Lookup | As measured | Panel 2, Find 1 = 3 | 3 | None |
| 20 | Fireball on three enemies | Run | As measured | Find 3, Do 3 = 6 | 6 | The run file |
| 21 | **F2.** A wolf frightens Oren for 10 rounds | Run, dock | Select Oren; `E`; type `Frightened 10 rounds`; Enter | Find 1, Do 3 = 4 | 3, with no count | The run file: the condition and its count |
| 22 | To read what Frightened does | Run | As measured | Panel 2 | 2 | None |
| 23 | To mark Concentration on Iselle | Run | As measured | Find 1, Do 1 = 2 | 2 | The run file |
| 24 | Round 2. **F2:** on Oren's turn the notice line says `Oren: Frightened, 10 rounds left`, and his row says `Frightened 10` | Run | 8 turns, as measured | Find 8, Do 24 = 32 | 32 | The run file |
| 25 | To check Iselle's spell save DC | Run | As measured | Find 2, Sub 1 = 3 | 3 | None |
| 26 | To roll a save for a monster | Run, dock | As measured | Do 4 | 4 | The run's Log |
| 27 | To read a monster's reaction | Run | As measured | Find 2 | 2 | None |
| 28 | What the miller said | Run, Lookup | As measured | Panel 2, Find 1 = 3 | 3 | None |
| 29 | To note a moment | Run, Notes | As measured | Panel 2, Do 1 = 3 | 3 | The note |
| 30 | Round 3 | Run | 8 turns, as measured | Find 8, Do 24 = 32 | 32 | The run file |
| 31 | To take a dead wolf out of the order | Run | As measured | Find 1, Do 1 = 2 | 2 | The run file |
| 32 | To end the fight | Run, then Session | As measured | Do 3, Nav 1 = 4, **T** | 4 | The run file. The timeline result |
| 33 | **F7.** To put the fight's loot and XP in the note | Session plan, the played fight's row | `Add rewards` | Do 1 | New. Today: `Copy loot`, go to the note, paste = 3 | The note: a block under `## Loot and rewards` |
| 34 | To record a short rest | Session plan | As measured | Do 1 | 1 | `Timeline.md` |
| 35 | To start the next fight | Session, then Run | As measured | Nav 1, **T** | 1 | The run file |
| 36 | To roll and begin | Run setup | As measured | Do 11 | 11 | The run file |
| 37 | To end it | Run, then Session | As measured | Do 2, Nav 1 = 3, **T** | 3 | The run file |
| 38 | A fight nobody planned, then talk | Session, Encounters, Run | As measured | Nav 4, Find 2, Do 3 = 9, **T T T** | 9 | An encounter note. A run file |
| 39 | To stop recording | Session | As measured | Do 1 | 1 | The recording |

**After: the wrap-up**

| # | The DM wants | Surface | Action | Count | Today | File |
|---|---|---|---|---|---:|---|
| 40 | To review the fight | Session plan | As measured | Find 1 | 1 | None |
| 41 | To say where it happened | Session | As measured | Sub 1, Do 3 = 4 | 4 | `Timeline.md` |
| 42 | To process the recording | Session, Recordings | As measured | Sub 3, Do 1 = 4 | 4 | The transcript and summary |
| 43 | To add a suggested event | Session, Recordings | As measured | Do 1 | 1 | `Timeline.md` |
| 44 | To write the wrap-up | Session, the note | As measured | Sub 1, Do 3 = 4 | 4 | The note |
| 45 | To link the fight in the note | Session plan | As measured | Do 2 | 2 | The note |
| 46 | **F7.** To add the second fight's rewards | Session plan, the played fight's row | `Add rewards` | Do 1 | New | The note. The session's XP line now sums both fights |
| 47 | **F6.** To put what Varra said in her note | Session plan, the done scene's row. Lookup opens (case B) on her page, with the Log field open | `Add to Log`; type `Told the party who burned the ropes.`; Enter; close Lookup | Panel 2, Do 2 = 4 | New. Today: World, find the note, edit in Obsidian = 6 or more | `NPCs/Captain Varra.md`: one line under `## Log` |
| 48 | To check the party's HP | Party | As measured | Nav 1, **T** | 1 | None |

**Totals for J1**

| | Transitions | Interactions | Nav | Sub | Panel | Find | Do |
|---|---:|---:|---:|---:|---:|---:|---:|
| Measured today | 13 | 231 | 14 | 7 | 12 | 49 | 149 |
| The same 41 tasks, after | 13 | 231 | 14 | 6 | 12 | 48 | 151 |
| Seven new records (rows 2, 9, 14, 16, 33, 46, 47) | 0 | 18 | 0 | 0 | 2 | 2 | 14 |
| **After, in all** | **13** | **249** | 14 | 6 | 14 | 50 | 165 |

- **Two measured tasks change.** Row 12 falls from 3 to 2, because the planned NPC is one click from the plan.
  Row 21 rises from 3 to 4. **Reason:** the DM types the number of rounds, and no design can take a number with
  no interaction. A DM who wants no count uses **Conditions** as today, for 3. Row 21 also needs no tab change.
- **Overhead does not grow for the measured tasks:** 82 before, 80 after. The new records add 4 overhead
  interactions (`More Add actions`, open the Travel row, open and close Lookup) and 14 of work.
- **Each new record costs less than its paper form.** Row 2 replaces copying about five lines by hand. Row 16
  replaces a search for the table. Row 33 replaces copy, go and paste. Row 47 replaces an edit in another program.
- **Table time:** the 180 measured interactions stay 180. The DM adds 4 for three records (rows 14, 16, 33).
- **The yardstick is older than the More menu.** `steps.tsv` counts `Travel` (row 15) and `Short rest` (row
  34) as one click each. On `main` both are in `More Add actions` at this width, and the script clicks that
  first (`src/visual_tests/dm_audit.rs:297`, `:422`). So a new measurement of `main` should give 233. This
  design then gives 233 for the same tasks and 251 in all. No feature moves that number: `Scene` joins the
  **More** menu, and the strip's visible actions stay as they are. P1 of the build plan measures the evening
  again before any feature merges.

### 2.2 J2: the improvised evening

Only the steps where a feature acts. Setup and turns cost what they cost today. Each side counts the close of
Lookup.

| # | The DM wants | Surface | Action | Count | Today | File |
|---|---|---|---|---|---:|---|
| 1 | To skip the first planned fight | Session plan | Nothing. The row stays. The second row's `Start fight` still takes one click | 0 | 0 | None. F5 carries the unplayed line to the next session |
| 2 | **F3.** To play the planned NPC | Session plan, the scene's row | Click `Captain Varra`. Lookup opens on her page | Panel 1 | 2: Ctrl+K, type | None |
| 3 | **F3.** The party attacks her | Lookup, her reader; then Run setup | `Start a fight`; close Lookup, to see her stat block | Nav 1, Panel 1 = 2, **T** | 8, **T T**: close Lookup, `Fight`, `New encounter`, name, to the search, search, add, `Start fight` | Nothing yet. At `Begin combat`: an encounter note `Captain Varra` with the party and her stat block, its link under `## Plan for tonight`, the run file and the timeline event |
| 4 | **F7.** Loot nobody planned | Run, ended view | `Add loot`; type `Varra’s signet ring`; Enter | Do 3 | 3, in Notes, under another heading | The run file: one more loot line |
| 5 | **F6.** She gave up, and the party holds her | Run, ended view, her row; then Lookup | Click her name on the row; `Add to Log`; type `Gave up at the ford. The party holds her.`; Enter; close Lookup | Panel 2, Do 3 = 5 | 7: Ctrl+K, type, `Edit`, click the text, type, `Save changes`, close | `NPCs/Captain Varra.md`: one Log line |
| 6 | **F7.** To close and keep the rewards | Run, then Session plan | `Close run`; `Add rewards` | Nav 1, Do 1 = 2, **T** | 1, and the loot is not in the note | The note: a rewards block. The run file: what was noted |
| 7 | **F9.** Two days overland | Session plan, the Travel row | `Travel`; open the row; click **How long**; type `2 days`; `Roll all 2` | Find 1, Do 4 = 5 | 11, **T T**: `Travel`, open the row, `Timeline`, click, type, `Session`, Ctrl+K, type, `Roll` twice, close | `Timeline.md`: the length, two checks and their rows |
| 8 | **F9.** The second day's result names Wolf Pack | The Travel row, then the plan, then Run | `Plan “Wolf Pack”`; `Start fight` on the new row, which is the first to come and the primary | Do 1, Nav 1 = 2, **T** | 3, **T**: `Fight`, pick, `Start fight` | The note: `- [[Wolf Pack]]` above the skipped fight. The run file |

| J2, these steps | Transitions | Interactions | Nav | Panel | Find | Do |
|---|---:|---:|---:|---:|---:|---:|
| Today | 6 | 35 | 6 | 8 | 7 | 14 |
| After | 3 | 20 | 3 | 4 | 1 | 12 |

- **W7.** The fight that nobody planned costs one click, as a planned one does, and leaves the same record at
  `Begin combat`. A wrong click on `Start a fight` costs one more: `Close run` in Setup leaves nothing.
- Step 3 with no stat block on the NPC asks for one first, as **Add to fight** does today: 3 more.
- **Must remember:** in step 7, nothing; the row says which day each result is for.

### 2.3 J3: the broken evening, at 960x640 with Lookup open (case D)

| # | What happens | What the DM sees | Recovery | Count | File |
|---|---|---|---|---|---|
| 1 | The DM ends fight 2 by mistake in round 2 | The ended view, 454 px wide, with no panel of its own. The header's second line says `Undo: end encounter`, whole | Click it. `U` is off while Lookup is open, as today. Play returns to the same turn of round 2 | Do 1 | The run file is at round 2 again. The timeline result says `Round 2` |
| 2 | The app stops in round 3. `Poisoned 3 rounds` is on Cinder Wolf 2 | After a restart the app opens on the run, on the current turn. The row still says `Poisoned` and its count. On the wolf's turn the notice line says it. `Undo` names the last saved step. With sync on, a sheet that changed says `Undo for Kestrel stops here`, as today | Nothing. Lookup is closed after a restart, as today; Ctrl+K opens it again | 0, or Panel 1 | The run file holds the effect, its count and the Undo steps. At most the one step that was not yet saved is lost |
| 3 | A sync conflict opens on Kestrel while a reminder is due on the current row, and a Due notice is open | The notice line holds the conflict and `Review`, because a conflict comes first. The reminder is on the current row, with every act of F2. The rail says `Due 1` | `Review`; choose. Lookup closes for the review, as today. Then the notice line shows the reminder | Do 2 | The run file. The ledger does not change |
| 4 | The DM edits the session note in Obsidian while Splat has it open | On the Run page nothing opens. When Splat held no unsaved words, the note and the plan show the edit, and a box ticked in Obsidian is ticked in the plan. When it held unsaved words, the rail says `Notes changed`, and the Session page shows the comparison of today | Nothing; or one choice on the Session page | 0 or 1 | The note keeps the Obsidian edit until the DM chooses |

Totals: **no transition; 3 or 4 interactions, and 1 to open Lookup again.** Today step 1 costs the same in one
sitting. After a restart today, nothing can be undone. Nothing in this journey is modal, and nothing takes
focus from `Amount`.

### 2.4 J4: the race

The Tournament of [the example](world-examples/heralds-tournament.md). Its note is in play. The DM opens
**Trackers** once with Ctrl+T and leaves it open (case B between fights; cases B and D in a fight).

| # | The DM wants | Surface | Action | Count | Must remember | On paper today |
|---|---|---|---|---|---|---|
| 1 | To open the numbers | Any page | Ctrl+T | Panel 1 | Nothing | Find the sheet of paper |
| 2 | The party enters a room | Trackers | `Enter room` | Do 1 | Nothing | 1 tally |
| 3 | The clock passes 40. A bell is due | Trackers: a row under **DUE**. The rail says `Due 1` | Nothing | 0 | Nothing | Must remember the bell |
| 4 | To roll for each rival and for wandering | The Due row | `Roll Rival Track` three times; `Roll Wandering, 0:40 to 1:19` | Do 4 | Nothing: each result stays under the row | Find two tables: 8 or more |
| 5 | To finish the bell | The Due row | Tick its box | Do 1 | Nothing | 1 tick |
| 6 | The Old Guard takes a seal | Trackers | `Take seal`; `Old Guard` | Do 2 | Nothing | 1 tally |
| 7 | In a fight: one minute passes each round | Run, with Trackers open. `N` still works | `Fight minute` | Do 1 | To press it each round. Splat does not count rounds into a tracker. An effect `Fight minute` on the first row is a reminder, if the DM wants one | 1 tally, and the same memory |
| 8 | The crowd loves a move. Then the crowd throws a gift | Trackers | `Crowd loves it`; `Throw a gift` | Do 2 | Nothing | 2 tallies and a table |
| 9 | A rival group arrives on its mark | Run. **Add to fight** closes Lookup; the notice line then says `Due: First seal. … A rival uses its 1-2 seals group.` | `Add to fight`; `Add` on the group; Ctrl+T | Panel 2, Do 1 = 3 | Nothing: the notice line names the group | The same 2 as today, and 1 to come back |
| 10 | The Practice Ring will open on the operator's next turn | Run, dock | Select the operator; `Effect`; type `Ring opens`; Enter | Find 1, Do 3 = 4 | Nothing after this | 1 note |
| 11 | A wrong click on `Throw a gift` | Trackers, the result line at the panel's bottom | `Undo change` | Do 1 | Nothing | Erase |

Totals: **no transition; 20 interactions (Panel 3, Find 1, Do 16).** Each tracker control in a run gives focus
back to `Amount`. A reminder (F2) and a Due notice (F4) never show the same fact: section 4.2.

### 2.5 J5: the plain evening

The DM of J1 uses no new control. **The script of the audit runs unchanged and gives the counts that it gives
today: 13 transitions and 231 interactions by `steps.tsv`** (233 if `main` is measured again; see 2.1). This is
true by rule 2 of section 1.1, and the build plan adds a test that fails when the unchanged script gives
another count than it gave before the feature.

No file changes that the DM did not act on. No heading and no count appears. One notice can appear, the last
row of this table. A plain evening sees these differences, and no others:

| Screen | What differs | Why it stays |
|---|---|---|
| Plan rows, Encounters rows | A planned fight's summary ends with one word, such as `· Low` (F1) | It answers the question of the row and costs no step. It shows nothing when the rules or the levels are not known |
| The encounter editor | The roster heading adds `· XP 1,050 · Moderate`, with one muted budget line below it (F1) | The same |
| Arrivals | One muted line, `Now: XP 450 · Low …`, and `→ Moderate` after a group's XP (F1) | The same |
| Run dock, line two | One more quiet word, `Effect` (F2). The **Conditions** tab no longer has `Other effect…`, which moved here | One field for free words, in view when a panel hides the tab |
| The run's notice line | A free-words effect that the DM already uses, from **Other effect** in an encounter or in an older run, now also shows on its participant's turn (F2) | The DM pinned those words to that creature. The notice needs no answer |
| Add strip's **More** menu | One more line, `Scene` (F3). The strip's visible actions are as today | It is in a menu |
| An NPC's reader, with no run open | `Start a fight`, where **Add to fight** is while a run is open (F3) | One quiet word |
| A World entry's reader | A quiet `Add to Log` after the Log (F6). While Lookup holds an unsaved NPC, an entry that opens shows under a row `Unsaved NPC: <name> · Show`; today the card hides it | It is where the Log is. The row repairs a fault |
| A session note's `/` list | One more command, `Log` (F6) | It shows only after `/` |
| A played fight's plan row | A quiet `Add rewards` in the row's empty action place, when the fight has loot results or XP (F7) | It is the next step after a fight |
| **Campaign → Rules and runs** | One checkbox, `Add XP to rewards` (F7) | A setting, on a settings page |
| Ended view | A quiet `Add loot` (F7). `Continue fight` when Undo cannot do the same (F8). An NPC's name is a link (F6) | Each is one quiet word or an underline |
| Run header after a restart | `Undo` names a step. Today it says `Nothing to undo since reopening` (F8) | It is the feature |
| **Last time** line | A quiet `Carry over`, when the last session left something (F5) | The note changes only when the DM presses it |
| An open Travel row | **How long**, above Title (F9). With a marked table and no travel table, the row's menu has `Roll for encounters` | The length belongs to the journey. Today it needs a trip to Timeline |
| A row marked `Conc` that takes damage | The row's result words add `Concentration: Constitution save DC 10` until the next step (F2) | In the measured evening this shows when a monster's damage reaches Iselle after her mark. It costs no interaction, records the DM's own mark and needs no answer. Open question 1 |

### 2.6 J6: the week between

| # | The DM wants | Surface | Action | Count | Today | File |
|---|---|---|---|---|---:|---|
| 1 | The party searches the last fight | Run, ended view | `Roll all 2` | Do 1 | 1 | The run file |
| 2 | **F6.** Verori died here | Ended view, Verori's row; then Lookup | Click his name; **Change** on Status; type `dead`; Enter; close Lookup | Panel 2, Do 3 = 5 | 6: Ctrl+K, type, **Change**, type, Enter, close | `NPCs/Verori.md`: the status, and `- [[Session 12]], Day 3: Status alive → dead.` |
| 3 | To go back to the evening | Run, then Session | `Close run` | Nav 1, **T** | 1 | None |
| 4 | **F7.** Rewards for each of three fights | Session plan, the played rows | `Add rewards` three times | Do 3 | 9: copy, go, paste for each | The note: three blocks under `## Loot and rewards`. The result says `XP tonight` |
| 5 | A week later: a new session | Session | `New session`; title; `Create session` | Do 3 | 3 | A new note |
| 6 | **F5.** What the last session left | **Last time** line | `Carry over` | Do 1 | 12 or more: open the old note, copy the lines | The new note: the recap embed, two unplayed plan lines, three open threads, and `carried_from` |
| 7 | **F3.** Three scenes | Add strip, then the add field | `More Add actions`; `Scene`; three times: type, Enter | Find 1, Do 7 = 8 | 4 in the note | The note: three `- [ ]` lines |
| 8 | **F1.** Is each of the two fights right for this party? | Session plan | Read the word on each row. Open a row for the numbers | 0, or Find 2 | 10 or more: add the XP, find the table, compare | None |
| 9 | To start | Session plan | `Start fight` | Nav 1, **T** | 1 | The run file |

Totals: **2 transitions and 23 interactions (25 with the numbers). Today: 2 transitions and 47 or more.**
The carried HP of today still starts the fight (T12.2). Nothing is typed twice. After step 2 the reader and
any scene row that links Verori say `dead`.

## 3. The features

Each part has the same order: the question, the place, the wireframes for the four cases of section 1.3, the
exact words, the states, the file, Undo, keys, what stays with the DM, and what is not built. A table at the
end of each part gives what a reviewer checks on paper: the one primary action, the component and token of
each control, and the targets.

All examples use one party: 5 characters at level 3. Their budgets are Low 750, Moderate 1,125 and High 2,000.

### F1. Encounter difficulty

**Question.** "Is this fight too much, or too little, for this party?"

**Where the numbers come from.** The rules data holds the table. The rule entry
`dnd2024:rules/combat_encounters_step_2_determine_your_xp_budget` has a typed table at
`splat_rule/parameters/per_character_xp`, with `level`, `low`, `moderate` and `high` for levels 1 to 20
(`builtin/dnd2024-definitions/rules/combat_encounters_step_2_determine_your_xp_budget.md`). Splat reads that
table from the rules that the campaign uses, as it reads the inspiration rule
(`src/encounter_settings.rs:421`). No table is written in Rust. A campaign whose packages have no such rule
shows no difficulty anywhere.

**One computation.** A new module, `src/difficulty.rs`, has one function. Every screen calls it.

- **The party:** the characters on the party's side in the roster. In a run, those who are not absent. A roster
  with no character uses the campaign's party, and says `for the party`. A level comes from the saved sheet.
- **The budget:** for each of Low, Moderate and High, the sum of the table's value for each character's level.
  So a party of mixed levels is exact.
- **The XP:** the sum of the XP of every creature that is not on the party's side. It uses the XP function that
  groups use today (`creature_xp`, `src/encounters/groups.rs:535`): the printed XP, or the XP of the CR.
- **The word:** `Low` when the XP is at most the Low budget, `Moderate` when it is at most the Moderate budget,
  `High` when it is at most the High budget, and `Above High` for more. The book spends up to a budget, so the
  word is the smallest budget that pays for the fight.
- The table loads when the content loads. A row's word is computed when its encounter or the party changes,
  never each frame.

**Where it shows.** The answer has one full form and one short form.

| Place | Form | Example |
|---|---|---|
| Encounter editor, the roster heading | Full | `ROSTER 8 · XP 1,050 · Moderate`, then one muted line: `Budget for 5 at level 3: Low 750 · Moderate 1,125 · High 2,000 · Rule` |
| Encounters list, a Ready row's second line | Short | `5 party · 2 × Ember Cultist · Low` |
| Session plan, a planned fight's second line | Short. The open row has the full line | `5 party · 2 × Ember Cultist · Low` |
| Run, Arrivals | The fight now, and what each group makes it | `Now: XP 450 · Low`, and on a group `Enemies · 2 members · XP 400 → Moderate` |

It does not show on a running or played fight's plan row, in Setup, in History or on Timeline. Those rows say
what happened, and the question is over.

**Wireframe: the encounter editor, case A (1200x800, Lookup closed).**

```text
← [Bridge Ambush                                         ]   [Save encounter] [▷ Start fight]  ⋯
✓ No changes
────────────────────────────────────────────────────────────────────────────────────────────────
ROSTER 8 · XP 1,050 · Moderate                          │ ADD PARTICIPANTS
Budget for 5 at level 3: Low 750 · Moderate 1,125 ·     │ Party members come from the sheets …
High 2,000 · Rule                                       │ [🔍 Find a creature by name or detail…  ]
1. Brannoch Vale                         Edit   ⋯       │ Aboleth                         [Add]
   Party · HP 58/62 · AC 19                             │ CR 10   HP 150  AC 17
…                                                       │ …
```

- **Case B and D (Lookup open).** The roster column narrows. The heading keeps `ROSTER 8 · XP 1,050 · Moderate`
  on one line. The budget line wraps at a `·`, and a line never ends with a separator. **Add participants**
  goes behind its tab, as today.
- **Case C.** The same as A with a narrower search column.
- `Rule` is a link. It opens the rule entry in Lookup beside the editor, as a creature's name does. With
  Lookup open on another entry, Lookup goes to the rule and Back returns.

**Wireframe: the plan row, case A, and the same row in case D.**

```text
>  Shrine of Cinders                 [▷ Start fight]  ⋯      >  Shrine of Cinders        [▷ Start fight] ⋯
   5 party · 2 × Ember Cultist · Low                            5 party · 2 × Ember Cultist · Low
```

When the line is too long, the member names shorten first, as today, and the word stays. If the line still
does not fit, it wraps before the `·` of the word.

**Wireframe: Arrivals, cases A and C.** In cases B and D the Arrivals panel is the run's own panel, so opening
it closes Lookup, as today.

```text
This fight · Arrivals
Now: XP 450 · Low for 5 at level 3
Drag a group name to the list, or use Add.
[🔍 Find or name a creature…                    ]
ENCOUNTER GROUPS 5
⠿ Cultist Pair             Enemies · 2 members · XP 400 → Moderate
  2 × Ember Cultist                  Initiative [Roll ] [Add]
```

**Exact words.** `Low`, `Moderate`, `High`, `Above High`. `XP 1,050`. `Budget for 5 at level 3:`. For mixed
levels: `Budget for 5 at levels 3 to 4:`. With no character in the roster: `Budget for the party of 5 at level
3:`. In Arrivals: `Now: XP 450 · Low for 5 at level 3`. A group's accessible name says `adds 400 XP; the fight
becomes Moderate`.

**States.**

| State | What shows |
|---|---|
| No creature against the party | No XP and no word |
| A creature has no XP and no CR | `XP 400 + unknown`, as groups say today, and no word. The editor's line says `Cinder Wolf has no XP or CR, so the difficulty is not known.` |
| A character in the roster has no known level | No word. The editor's line says `Oren’s level is not known, so there is no budget.` |
| The campaign has no party sheets at all | The XP, and no word and no line. A DM with no sheets is told nothing |
| The campaign's rules have no budget table | The XP, and no word and no line, on every screen |
| The encounter file cannot be read, or changed on disk | The warnings of today. The word is computed from the draft on screen |
| Run open | The open fight's plan row shows its status link, not a word. Arrivals shows the live numbers |
| Sync on | No effect. Levels come from the saved sheets, and the editor's existing line says how old they are |
| Narrow window | As the wireframes say. The word is never cut |

**What the file stores.** Nothing. The answer is computed from the encounter note, the sheets and the rule.

**Undo.** None is needed. The word follows the roster's own Undo.

**Keyboard.** `Rule` is a Tab stop in the editor. Nothing else takes focus.

**What stays with the DM.** Everything. Splat shows numbers and a word. It blocks nothing, warns about
nothing, and gives the word no colour, because red or green would be a verdict. Terrain, surprise, allies on
the party's side and a creature the party cannot reach are the DM's to weigh.

**Not built, and why.** A slider to scale a fight (Splat would then author the fight). A warning for one
creature far above the party's level (the rule note `Powerful creatures` is prose, with no typed data). The
2014 thresholds and multipliers (no typed table in the SRD 5.1 data). A difficulty history for played fights.

| Check | F1 |
|---|---|
| Primary action in view | The screen's own, unchanged: `Start fight` or `Save encounter`. F1 adds no action |
| Components and tokens | Heading: `group_heading` with its muted `Small` parts. The word: `Type::Small` in `text`. Numbers: `Type::Small` in `muted`, with `number` digits. `Rule`: `link`, at least 24 px. No icon and no state colour |
| In view at 960x640 | Lookup closed: heading and budget line above the first roster row. Lookup open: the same, with the budget line on two lines |
| States | Words only, as the table says. A level that Splat should know and does not is named in the editor. A list says nothing |
| Loss and failure | Nothing to lose |
| Keys and targets | `Rule` by Tab and Enter. No new target under 24 px |

### F2. Effects and turn reminders

**Question.** "What must I remember on this creature's turn, and how long does it last?"

**The idea.** An **effect** is words that the DM pins to one participant. The words stay on that row. On that
participant's turn they also show on the header's notice line: that is the **reminder**. An effect can count
rounds, count uses, or wait for a recharge roll. All five needs are this one thing.

| Need | The DM types | The row says | On its turn the notice line says |
|---|---|---|---|
| A note for a recurring save | `Wis save DC 15 (Hold Person)` | `Wis save DC 15 (Hold Person)` | `Oren: Wis save DC 15 (Hold Person)` |
| A duration on a condition | `Frightened 10 rounds` | `Frightened 10` | `Oren: Frightened, 10 rounds left`, later `last round`, then `time is up` |
| A duration on any effect | `Bless 1 minute` | `Bless 10` | The same |
| Recharge | `Fire Breath recharge 5–6` | `Fire Breath ready`, or `Fire Breath spent ⚄` | When spent: `Cinder Drake: Fire Breath is spent` |
| Legendary actions | `Legendary actions 3 uses` | `Legendary actions 2 of 3` | When some are spent: `Adult Red Dragon: Legendary actions 2 of 3` |
| Concentration | Nothing. `Conc` exists | `Conc`, as today | Nothing. When the row takes damage, its result words say the save DC |

**The row is the place to act. The notice line only echoes it.** The notice line has no control, so it asks
for nothing and loses nothing when a conflict or a sync notice holds it.

**Where it lives, and why not elsewhere.**

- **Entry: the dock.** One new quiet control, `Effect` (key `E`), on the dock's second line, between
  **Conditions** and **Conc**. The dock stays in view when Lookup or Notes hides the run's panel, and it is where
  the DM already types for the selected row. The run's field `Other effect…` leaves the **Conditions** tab, so
  a run has one field for free words. **Conditions** keeps the 15 conditions, Exhaustion, and the list where an
  effect is removed.
- **Entry in prep.** The encounter and group editors keep **Other effect** under a slot's starting conditions
  (`src/encounters/groups.rs:1751`). Its words are effects in the run, and a count in them is read when the
  run starts. So `Legendary actions 3 uses` typed once in prep is on the dragon's row in every fight.
- **Result: the row and the notice line.** No panel view, no tab and no strip is added.
- **Not the run's panel:** the user ruled that run features stay off it. **Not a dialog:** nothing is modal.

**What is an effect.** Every word on a row that is not one of the 15 conditions or Exhaustion is an effect,
however it came there: the dock, a starting condition, or an older run file. Each gives a reminder on its turn.

**Wireframe: the live run, case A (1200x800, Lookup closed), on Oren's turn in round 3.**

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
Oren Thistlewick                    Conditions C   Effect E   ✧ Conc   Absent
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

- **Enter adds the words, closes the field and puts focus in `Amount`.** Escape closes the field, keeps the
  words for the next `E`, and puts focus in `Amount`.
- **The field belongs to the row it opened for.** Its placeholder names that row. A click on another row does
  not change it.
- **While the field has focus, a letter is text.** `N` does not advance the turn; a click on `Next turn` does.
  The field then closes, keeps its words for the next `E`, and adds nothing.
- An arrow key and Enter, or a click, puts a suggestion in the field. A second Enter adds it. This is the loot
  field's rule.

**Case D (960x640, Lookup open): the roster is 454 px and the run's panel is hidden.**

```text
Round 3   Oren…                         [+ Add]  [> Next turn]
⚑ Oren: Frightened, 9 rounds left · Wis save…          [↶ Undo]
────────────────────────────────────────────────────────
 16 ■  Oren Thistlewick            AC 18        45/45
 TURN  Frightened 9 · Wis save DC 15 (Hold Person)
…
Dagger +6   Fire Bolt +7                       ← the tile line of today
[Amount] [Damage] Heal 🛡Temp HP        [1d20] [Roll]
Oren Thistlewick      Conditions  ⚑ Effect  ✧ Conc  Absent
```

- **What gives, in order.** On the notice line the reminder is a state notice, so `Undo` gives up its target
  and then its `U`, as today. A reminder that still does not fit shortens by whole words with `…`; hover and
  the accessible name keep it whole, and **the row keeps it whole**. A reminder takes the place of
  `Next: <name>`; the roster still marks NEXT, as today when that notice gives way. On the dock's second line
  the key letters go first, then the selected name shortens by whole words (today's rule), then `Effect` keeps
  its icon alone, with its name on hover.
- **A row's height does not change when it becomes current.** On every row, effects come before plain
  conditions, and the first effect shows whole: when the line has no room it takes a line of its own, as roll
  words do. More effects fold into `2 more`, and a click on that opens **Conditions**, as today.
- **Case B** is case D with more room: nothing shortens. **Case C** is case A with the run's panel at 346 px.
- **With Lookup or Notes open,** `E` is text, as every letter is. `Effect` takes a click. With Notes open, focus
  goes back to the note after Enter, as for every run button today.

**How a count works.**

- **Rounds.** The count is the number of this participant's turns that the effect still covers, with the
  current turn. It goes down by one when that participant's turn ends, in the same step as `Next turn`, so
  Undo of `Next turn` gives it back. An effect of 10 rounds added before Oren's turn says 10 during that turn
  and 9 during the next. At 1 the words say `last round`. At 0 they say `time is up`, and the effect stays
  until the DM removes it. An absent participant's count does not go down. To time an effect by its caster's
  turns, pin it to the caster.
- **Uses.** A click on the word spends one. At `0 of 3`, a click gives all of them back. Splat does not give
  them back by itself.
- **Recharge.** A click on `Fire Breath ready` marks it spent. `Fire Breath spent ⚄` has the dice icon: a
  click rolls 1d6 on the run's dice. The row's result words say `Rolled 5 · ready again` or `Rolled 2 · still
  spent`, in the place of roll results.
- **A condition's name.** Words that are an official condition, such as `Frightened 10 rounds`, add that
  condition with a count. The row never says it twice, and sync sends it as it sends the condition today.
  Words that are already on the row change their count and add nothing.
- **Several rows.** With rows added by Ctrl+click, one effect goes to each, as one step of Undo.
- **Concentration.** When a row marked `Conc` takes damage, its result words say `Concentration: Constitution
  save DC 12`: 10, or half the damage if that is more, and at most 30. They go at the next step, as roll
  results do. Splat does not roll the save and does not clear `Conc`.

**What a click on an effect's word does.** One act for each state. Hover and the accessible name say it.

| The word | A click | Hover |
|---|---|---|
| Plain words, or a count that still runs (`Bless 9`) | Opens **Conditions**, where the effect is removed, as for an effect today | `Open Conditions to remove Bless` |
| `Bless · time is up` | Removes the effect | `Remove Bless` |
| `Legendary actions 2 of 3` | Spends one | `Spend one` |
| `Legendary actions 0 of 3` | Gives all back | `Back to 3` |
| `Fire Breath ready` | Marks it spent | `Mark Fire Breath spent` |
| `Fire Breath spent ⚄` | Rolls the recharge | `Roll 1d6. 5–6 makes it ready` |

To remove an effect with uses or a recharge, the DM selects the row and opens **Conditions** (`C`), which
lists every effect with its remove control, as today.

**What Splat reads at the end of the words.** `10 rounds` or `1 round`. `1 minute` or `2 minutes`, as 10
rounds each. `3 uses` or `1 use`. `recharge 6`, `recharge 5–6` or `recharge 5-6`. Anything else stays words
with no count: `Bless for a while`, `Mage Armor 8 hours`.

**The suggestions.** Those marked `stat block` come from the creature's typed data: a feature's `recharge`
block and its `legendary_actions` (`builtin/dnd2024-definitions/creatures/adult_red_dragon.md`). Those marked
`used before` are the last effects of this fight, so `Bless 10 rounds` on a third target needs no typing.

**Exact words.** `Effect`. Field: `Effect on <name>, such as Bless 10 rounds`. With several rows: `Effect on 3
participants, such as Bless 10 rounds`. On the row: `Bless 9`, `Bless · last round`, `Bless · time is up`,
`Fire Breath ready`, `Fire Breath spent`, `Legendary actions 2 of 3`. On the notice line: `<name>: Bless, 9
rounds left`; `<name>: Bless, last round`; `<name>: Bless, time is up`; `<name>: Fire Breath is spent`;
`<name>: Legendary actions 2 of 3`. Help under the **Conditions** grid, in place of the old sentence: `These
record your ruling. No number changes by itself. Effects stay in this run and do not sync. Add one with Effect
in the dock.`

**States.**

| State | What shows |
|---|---|
| No effect | Nothing. The notice line says `Next: <name>`, as today |
| Several effects on the current row | The notice line joins them with `·`, in the order they were added. When they do not fit: the first, then `2 more`, with all on hover and on the row |
| A conflict, recording silence or a sync notice at the same time | That notice keeps the line (today's order). The reminder is on the current row, with every act |
| A Due notice at the same time (F4) | The reminder comes first. The Due count is in the rail |
| Nobody selected | `Effect` is blocked: `Select a participant first.` |
| The run has ended | Effects show as words on the Final state rows. Nothing counts, and a click does nothing |
| The run's panel is hidden | Every act of the table above works except the first. For that one, **Conditions** opens and Lookup closes, as today |
| Run file cannot be saved, or changed on disk | The run's own messages of today. Effects are part of the run's state |
| Sync on | An official condition syncs. An effect and every count stay in the run |
| Setup | No `Effect`. Starting conditions come from the encounter |

**What the run file stores.** The words stay in `conditions`, without the count. A map beside them holds the
count. A run file from before has no map, and loads.

```json
"conditions": ["Frightened", "Wis save DC 15 (Hold Person)", "Fire Breath", "Legendary actions"],
"effects": {
  "Frightened": {"rounds": 10, "left": 9},
  "Fire Breath": {"recharge": [5, 6], "spent": true},
  "Legendary actions": {"uses": 3, "left": 2}
}
```

An effect with no count needs no entry in the map.

**Undo.** Each act is one named step of the run's Undo, which F8 saves with the file: `Undo: effect Bless on
Oren`, `Undo: Fire Breath spent`, `Undo: recharge roll for Cinder Drake`, `Undo: legendary action of Adult Red
Dragon`, `Undo: remove Bless from Oren`. The Log has a line for each, and a line when a count reaches 0.

**Keyboard.** `E` opens the field for the selected row. Enter adds. Escape closes. The words on a row are Tab
stops, as condition words are, and Enter on one is a click.

**What stays with the DM.** When an effect starts and ends. Every save. Whether a use was spent and when it
comes back. Whether concentration holds. Splat counts, shows and rolls when asked.

**Not built, and why.** A link from a caster's concentration to the effects it holds (it needs the effect
model of `CONTEXT.md`, which the engine does not have). Automatic `spent` when a tile is used (two ways to
spend would differ for actions with no tile). Lair actions on initiative 20 (pin `Lair action` to any row).

| Check | F2 |
|---|---|
| Primary action in view | `Next turn`, unchanged and alone |
| Components and tokens | `Effect`: `Action::quiet` with `.key("E")`, dock density, icon `icons::PIN`. Field: `add_field` (the loot field, shared), with an accessible name. Row words: `link` at the run scale's condition size in `participant.condition`; digits tabular; `icons::DICE` after a spent recharge. `time is up`: `result.warning` icon and words in `text`. Notice: the header's notice with `Tone::Info`, icon `icons::PIN`, or `icons::HOURGLASS` for a count. No control on it |
| In view at 960x640 | Lookup closed and open: round, turn, `Next turn`, the current row with its first effect whole, the dock. No scroll |
| States | Counting: hourglass and `Bless 9`. Last round: `last round`. Ended: warning icon and `time is up`. Spent: `spent` and the dice. Ready: `ready` |
| Loss and failure | Every step has the run's named Undo. A count Splat cannot read stays as words, and Escape keeps typed words. A blocked `Effect` says why |
| Keys and targets | `E`, Enter, Escape, Tab. Each row word is at least 24 px high, as condition words are, with an accessible name such as `Spend one legendary action of Adult Red Dragon` |

### F3. Plan items that are not fights

**Question.** "What did I plan for tonight besides the fights, and what is done?"

**The name.** A plan item that is not a fight is a **scene**: a talk, a person to meet, a place, a secret to
reveal. The app uses that one word. `Item` already means loot and equipment.

**Where it lives.** In the plan that exists. A task line under `## Plan for tonight` is a scene: a row with a
done box, in the note's own order. Obsidian shows the same line with the same box.

- **Not a Timeline event,** because a planned scene has not happened.
- **Not a new list or tab,** because the DM reads one plan from top to bottom.
- **The order of the plan:** what has happened in this session, as today; then every line still to come, in
  the note's order: fights, groups and scenes together. A done scene stays in its place, so no row jumps.

**Which line is which row.**

| A line under `## Plan for tonight` | Row |
|---|---|
| A link to an encounter, alone or with a task box or other words | A fight, as today. Its box, if it has one, is not shown: a fight is done when it is played |
| A link to an encounter group, anywhere in the note | A group, as today |
| A task line, `- [ ] words` or `- [x] words`, that links no encounter | A scene |
| A plain list line, a numbered line, a paragraph, or a line nested under another | No row. It stays text in the note, as today |

**Wireframe: the Session page, case A (1200x800, Lookup closed).**

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

**Adding a scene.** `Scene` joins the **More** menu of the Add strip, as its first line, at every width. So
the strip's visible actions do not change for anyone. `Scene` opens one field at the end of the plan, with
focus:

```text
+  [Plan a scene: words, a person or a place                ]
   Captain Varra · NPC
   Ashfall Keep · Location
```

Enter adds the words as a task line and keeps the field open for the next line, as the loot field does.
Escape closes it and keeps the words. While the DM types, the campaign's World entries that match show under
the field, at most four. An arrow key and Enter, or a click, puts a link in the field. A line that the DM
types in the note, in Splat or in Obsidian, becomes a row in the same way, so the DM can also write the plan
as text.

**A row.**

- **The box** is at the start of the row, where a fight has its caret. A click or Space ticks it. The note
  changes by one character.
- **The words** are drawn by the Reader. A link opens its entry in Lookup, beside the page.
- **The second line** shows only when the words link exactly one World entry: its kind, status and place, as
  Lookup's compact reader says them (`NPC · alive · Ashfall Keep`).
- **The action place** at the right is empty until the scene is done. Then, when the words link exactly one
  World entry, it holds the quiet `Add to Log` of F6.
- **The menu** holds `Remove from tonight`, with `Undo remove` under the row, as for a fight.

**Every plan act changes the note's draft.** A tick, a new scene, a removal, `Carry over` (F5) and `Add
rewards` (F7) change the text in the note pane, and it saves as typed words do. So unsent words in the note
are never lost to a plan act, and one save holds both.

**An NPC the party attacks.** The NPC's reader, in Lookup and on World, has **Add to fight** while a run is
open, as today. With no run open, the same place says **Start a fight**. It opens Setup with the party and
the NPC's stat block. Nothing is written yet. At **Begin combat**, Splat saves an encounter named after the
NPC and links it under `## Plan for tonight`, in the same step that puts the fight on the Timeline today.
`Close run` in Setup leaves nothing behind, as a closed Setup does today. So a fight that nobody planned
costs one click from the page of the person attacked, and a wrong click costs one more. If an encounter has
that name, the new one is `Captain Varra 2`. An NPC with no stat block is asked for one first, as **Add to
fight** asks today: that is 3 more.

**Cases B, C and D.** The plan has its own tab and the full page width (608, 568 or 454 px). The row is the
same. The words wrap to at most three lines, then shorten with the full text on hover. In case D, Lookup on an
entry and the plan are both in view; the note is behind its tab.

**Lookup.** A click on a link in a row opens that entry in Lookup. If Lookup shows another entry, it goes to
this one, and Back returns. An unsaved edit form stays with its own entry. An unsaved generated NPC stays
under `Unsaved NPC: <name> · Show` (section 1.2). Nothing opens Lookup without a click.

**Exact words.** `Scene`. Field: `Plan a scene: words, a person or a place`. Empty plan: `Use Add to plan a
fight, a group, a scene or an event.` The box's accessible names: `Done: Captain Varra` and `Open: Captain
Varra`, the words that World's task marks use. `Remove from tonight`. `Undo remove`. `Start a fight`. The
count in `TONIGHT 3` is the rows still to come: fights, groups, and scenes that are not done, with the events
of the session, as today.

**States.**

| State | What shows |
|---|---|
| No scene | No row and no heading. The plan is as today |
| A link to nothing | The words keep the dotted underline and `Not found in this campaign`, as in every reader |
| A link to two notes | The link opens Lookup's chooser, as today |
| The note cannot be read | The plan shows the problem that it shows today |
| The note changed on disk, and Splat holds no unsaved words | Splat reads it. The rows follow the file |
| The note changed on disk, and Splat holds unsaved words | On the Session page, the comparison of today takes the note's place and hides the plan until the DM makes one choice. On every other page only the rail says `Notes changed`, as today, and nothing opens |
| Run open | Scenes work. `Start a fight` is `Add to fight` |
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

**Undo.** Adding a scene gives `Scene added to tonight.` with `Undo add`, in the plan's message place below
the Add strip. It belongs to this note and ends at the next note or plan edit. A tick is its own Undo: click
again. Removing gives `Undo remove`.

**Keyboard.** Tab reaches the box, then the links in the words, then the menu. Space ticks. In the add field,
Enter adds, the arrow keys choose a suggestion, and Escape closes.

**What stays with the DM.** What a scene is, when it is done, and whether it happened at all.

**Not built, and why.** Editing a scene's words in its row (the note is beside it). Dragging rows to reorder
(the note's order is the order). Scenes with a time or a place (that is a Timeline event). Nested lines.

| Check | F3 |
|---|---|
| Primary action in view | The first upcoming `Start fight`, as today. A scene never takes it |
| Components and tokens | `Scene`: a line of the `More Add actions` menu, icon `icons::CHECK_SQUARE`. Field: `add_field`, with suggestions in `Type::Small` and a muted kind. Box: the theme's checkbox, 24 px. Words: the Reader's inline text with `link`. Second line: `Type::Small` in `muted`. Messages: `undo_message` |
| In view at 960x640 | Lookup closed and open: the Add strip and the rows. Nothing scrolls sideways |
| States | Open: an empty box. Done: a checked box, and the words in `muted` |
| Loss and failure | `Undo add`, `Undo remove`. A tick is one character. A failed write shows the note's own `Notes not saved` |
| Keys and targets | As above. The box, each link and the menu are 24 px or more |

### F4. Trackers and the Due list

**Question.** "What is due, and what are the numbers?"

The data model of [World and house rules](world-and-house-rules.md) stands: house rule notes, in play,
trackers, quick actions, marks, the Due list and the ledger `Trackers.md`. That design is older than the
Session page, the plan, the party bar and the rail. Five things in it must change. The same pull request
changes that document in place.

| The old design said | Now | Why |
|---|---|---|
| A tracker panel opens beside any page with Ctrl+T, as a third panel | **Trackers** is a view of the Lookup panel. Ctrl+T opens it | One rule decides the side space (section 1.2). A third panel would hide the stat block more often and would need a new rule |
| The run strip and the panel's button show the Due count | The rail's status slot shows `Due 2` on every page. In a run, the notice line shows a notice's words while Trackers is not in view | The rail is in view on every page and moves nothing. The notice line is where the DM looks in a fight |
| A tracker action in a run also puts a step on the run's undo stack, so that `U` takes it back | A tracker action is never a run step. Its Undo is on its own result line | F8 saves the run's steps in the run file. A step there that changes `Trackers.md` could not be kept true |
| A quick action's table roll goes to the run's Log on the run's dice | The ledger records the roll and its row. The run does not | One owner for one fact. The run's Log is the fight's record |
| A notice's roll results stay until the app closes, and **Done** and **Dismiss** have no Undo | The ledger keeps a notice's rolls. A tick and a dismissal have `Undo change`, which brings the row back | A crash must not lose three rolls, and a wrong tick must come back (W6) |

**Where it shows.**

| Surface | What shows | When |
|---|---|---|
| The rail's status slot, above Lookup | `Trackers`, or `Due 2` with a bell | While a house rule note is in play. A click opens Trackers |
| The Lookup panel | The **Trackers** view: Due rows, then pinned trackers, then the rest | Ctrl+T, or a click on the rail's status or on the notice line's words |
| The run's notice line | `Due: Bell. The Cistern floods.` It is a link to Trackers | While a notice is due and Trackers is not in view, when no conflict, sync notice or reminder holds the line. So the words that name an arriving group stay in view while **Arrivals** is open |
| The Session page | Nothing of its own. The rail's status is on this page too | |

**Wireframe: Trackers in Lookup, case B (1200x800), beside the plan or a run. The panel is 432 px.**

```text
🔍 [Search…                                          ]   ← Back   ×
Trackers
DUE 2
☐  Bell. The Cistern floods. Announce the standings.
   Roll each rival. Check d6 for wandering.              ⋯
   [⚄ Roll Rival Track]   [⚄ Roll Wandering, 0:40 to 1:19]
   Rival Track: 4 · The Keyring take a seal.
☐  End of month. Move the rival powers.                  ⋯
   ☑ Charges +600 for Verori    ☑ Research points +5 for Verori
PINNED
Elapsed race time                              42 / 120
  [Enter room] [Cistern or Sands] [Reach Kitchens seal]
  [Alcove rest] [Fight minute] [Collect soul]
Crowd Favor                                           3
  [Crowd loves it] [Throw a gift]
Seals    Party 1 · Old Guard 2 · Keyring 0 · Convicts 0
  Old Guard: Coming for the party
  [Take seal ⌄] [Lose seal ⌄]
> MORE TRACKERS 2
──────────────────────────────────────────────────────────
✓ Favor −1. Rolled 4 on 1d6: A healing potion.
  ↶ Undo change     Add to Timeline
```

- **A Due row** starts with a box, as a scene does. A tick means done: it applies the proposed changes that
  are still ticked under the row, and the row goes. The row's menu holds `Dismiss`, which applies nothing. A
  table's `Roll` lists its result under the row, and each further press adds a result. The button names the
  table by its heading, as the Reader's roll button does today, never by its key.
- **A quick action** is one press. A tracker with instances (`per`) opens a menu of them. An action that asks
  for a number (`add: ask`) puts a number field in its own place.
- **The result line** has a fixed place at the panel's bottom edge, so it is in view at every height. It names
  the last change and has `Undo change` and `Add to Timeline`. `Add to Timeline` makes a Major event with the
  action's label in the session being played, so it is a row of tonight's plan at once. The list above the
  result line scrolls, with Due rows first.
- **In a run, every control of this view gives focus back to `Amount`,** so the next digits are damage and
  Enter applies it.
- **The run's letter keys work while Lookup shows Trackers,** because this view has nothing to type into. A
  click in Lookup's search field makes a letter text again, as today. So `N` still advances the turn in a
  race.
- **Case D (960x640):** the panel is 346 px. Quick actions wrap two to a line. A Due row's words wrap. In a
  run, the roster keeps 454 px and its tile line, as in the Practice Ring test of the old design.
- **Case A and C** are the page before Ctrl+T: only the rail's status shows.

**Lookup's own rules.** Trackers is a place in Lookup's Back history. Ctrl+K and a search leave it, and Back
or Ctrl+T returns. An unsaved edit form or NPC in Lookup stays held (section 1.2). Opening **Add to fight** or
**Conditions** in a run closes Lookup, as today; Ctrl+T brings Trackers back in one press.

**Reminders and Due notices.** They are two things and never show one fact twice.

| | A reminder (F2) | A Due notice (F4) |
|---|---|---|
| Belongs to | One participant of one run | A tracker or a day mark of the campaign |
| Raised by | That participant's turn | A value that passes a mark, or a day |
| Counted in | Rounds, uses | A tracker's value, days |
| Stored in | The run file | `Trackers.md` |
| Shows on | The row, and the notice line on its turn | The rail, the Trackers view, and the notice line while the view is closed |
| Ends | When the DM removes the effect, or with the fight | When the DM ticks or dismisses it. It outlives the fight and the session |

A mark never makes a reminder, and a reminder never writes the ledger. When a mark's words name a turn
("It opens on the operator's next turn"), the DM pins an effect to that row, as in J4 step 10.

**Exact words.** Rail: `Trackers`; `Due 1`; hover `1 notice is due. Open Trackers (Ctrl+T).` View: `Trackers`,
`DUE 1`, `PINNED`, `MORE TRACKERS 2`. `Roll <table’s name>`. `Dismiss`. `Undo change`. `Add to Timeline`.
Result: `Favor −1.`; with a table `Favor −1. Rolled 4 on 1d6: A healing potion.`; after a tick `Bell done.`;
after a dismissal `Bell dismissed.` Notice line: `Due: <the mark's words>`. A blocked action: `Favor cannot go
below 0.` No note in play, after Ctrl+T: the empty state `No house rules are in play.` with `Turn one on in
Campaign → Rules and runs.`

**States.**

| State | What shows |
|---|---|
| No house rule note in play | No rail status. Ctrl+T opens the empty state above |
| In play, nothing due | Rail: `Trackers`, muted |
| A notice is due | Rail: bell and `Due 1`, in the text colour |
| `Trackers.md` cannot be read | Rail: error icon and `Trackers not read`. The view names the file, says the history is safe, and offers `Try again` and `Show file`. Quick actions are blocked with that reason |
| `Trackers.md` changed on disk | Separate changes merge with a quiet notice, as `Timeline.md` does. The same change needs a choice |
| `Timeline.md` cannot be read | Day marks stop, and the Due list says why (the old design's rule) |
| A definition cannot be read | The tracker says `Definition unavailable, history is safe` |
| Run open | The same view. No tracker action is a run step |
| Sync on | No effect |

**What the files store.** As [World and house rules](world-and-house-rules.md) says: definitions in the house
rule note's Splat data, history in `Trackers.md`. A change with its roll, and a notice with its rolls:

```yaml
changes:
- id: c-5e6f7a8b
  tracker: favor
  add: -1
  roll: "1d6: [4]"
  row: A healing potion
  session: 070eca36-331c-46cc-b5a6-32a6e35265da
notices:
- change: c-1a2b3c4d
  at: 40
  state: open
  rolls:
  - {table: rival_track, roll: 4, row: The Keyring take a seal.}
```

**Undo.** `Undo change` is on the result line, which names the change: `Favor −1.` It takes back the ledger's
newest change of this session: a quick action, a tick with the changes it applied, or a dismissal. The
ledger is a file, so the Undo is there after a restart. The result line and its Undo belong to the ledger and
end at the next tracker action. A change from an older session leaves by **Remove** in the tracker's history,
as the old design says. Undo of a change that raised a notice removes the notice. `U` is the run's Undo and
names its own step in the run's header; it never takes back a tracker change.

**Keyboard.** Ctrl+T. Tab walks the Due rows, then each tracker's actions. Space ticks a Due row. Enter
presses an action. Escape closes Lookup.

**What stays with the DM.** Awarding Favor, moving a rival, what a bell does on the map, and every rule that a
tracker's number stands for. A mark's proposed changes apply only when the DM ticks the row. A tally for each
round of a fight is the DM's press: Splat does not count rounds into a tracker.

**Not built, and why.** The parts in [World and rules: later](world-and-rules-later.md). Quick actions in the
run's dock or footer (the footer has no room in case D, and one view is easier to learn). Due rows in the
plan (a notice would then show in two places).

| Check | F4 |
|---|---|
| Primary action in view | None in the view. The page's own primary stays |
| Components and tokens | Rail: the status slot with `status_link`; icon `icons::BELL`. View title: Lookup's title line. `DUE`, `PINNED`: `group_heading`. Box: the theme's checkbox. Quick actions: `Action::secondary` at Read density. A menu of instances: `menu_button`. Values: `Type::Numeral`, right-aligned with `number`. `Roll`: `Action::secondary` with `icons::DICE`. Result: `undo_message` with a second quiet action, above a hairline |
| In view at 960x640 | The rail's status on every page. In the view: the result line at the bottom edge, and the Due rows first in the list above it |
| States | Due: bell and `Due 1`. Not read: error icon and `Trackers not read`. Blocked: the reason beside the action |
| Loss and failure | `Undo change` for an action, a tick and a dismissal. `Dismiss` is in a menu, apart from the box. An unreadable file blocks writing and says what to do |
| Keys and targets | Ctrl+T, Tab, Space, Enter. Every action is 28 px high (Read) |

### F5. Carry-over between sessions

**Question.** "What did the last session leave for this one?"

**Where it lives.** On the **Last time** line of the Session page, which already links the last session and
shows the first line of its recap (`last_time_block`, `src/sessions/page.rs:4065`). One quiet action,
`Carry over`, ends that line. It is there while this note has not carried over and the last session left
something.

- **Not automatic at Create session,** because a note would then change with no act of the DM (rule 2), and
  the plain evening of J5 would differ.
- **Not in the New session form,** because the act would then have two controls: one at creation, and one for
  a note made in Obsidian or before the recording was processed.

**What it carries, and how.**

| From the last session | To this note | Copied text or a link | Why |
|---|---|---|---|
| The prose under `## What happened`, which holds the summary when the DM added it with **Add to notes** | `## Recap of last time` | **A link:** one embed line, `![[Session 1 The Road#What happened]]` | One file owns what happened. Obsidian shows the text in place. A later correction in the old note shows here too |
| A processed recording's **Recap**, only when `What happened` has no prose | `## Recap of last time` | **Copied text,** under the line `> This draft was written automatically. Check it against your memory.` | An embed cannot point into a recording's file by a name that a person can read |
| Each row still to come under `## Plan for tonight`: a fight that was not played, a group, a scene that is not done. Plain text in the plan stays behind | `## Plan for tonight` | **Copied lines,** links kept, boxes empty | A plan line must be ticked or removed in this session without changing the last one |
| Each line under `## Open threads` that is not ticked: task lines and plain lines | `## Open threads` | **Copied lines** | The same reason. A ticked thread is closed and stays behind |

**When the DM already wrote something.** Carry over never replaces and never reorders. It adds below what a
heading holds. It skips a line that this note already has, compared without the task mark. A heading that the
template does not have is added at the end of the note.

**Once for each note.** The act writes one property in the note, `carried_from`, with the last session's
name. While the note has it, the action is gone. So a carried line that the DM deletes or rewrites stays as
the DM left it, and nothing offers it again. Only `Undo carry over` removes the property.

**Wireframe: case A, before and after.**

```text
Last time:  Session 1: The Road — Hobb the miller says the toll collectors …    Carry over

Last time:  Session 1: The Road — Hobb the miller says the toll collectors …
✓ Carried over from Session 1: the recap, 2 plan lines and 3 open threads.   ↶ Undo carry over
```

In Read, the embed shows the last session's text under a muted line, `From Session 1: The Road`, which is a
link to that session. In Write it is the one line of source. The note beside the plan shows the new lines at
once, and the plan shows the carried fights and scenes as rows.

**Cases B, C and D.** The line wraps. `Carry over` then starts a line of its own at the text edge
(`line_start`). The result message is capped at the list measure and wraps.

**Lookup.** Carry over does not use Lookup. Lookup open or closed changes only the width.

**Exact words.** `Carry over`. Hover: `Add the last session’s recap, unplayed plan and open threads to this
note.` Result: `Carried over from Session 1: the recap, 2 plan lines and 3 open threads.` It names only what
it added. `Undo carry over`.

**States.**

| State | What shows |
|---|---|
| No earlier session, or nothing to carry | No action |
| This note has `carried_from`, or already holds all of it | No action |
| The last note cannot be read | The action is blocked: `Splat cannot read Session 1’s note.` |
| This note changed on disk and Splat holds unsaved words | The comparison of today comes first. The action is blocked: `Settle the note’s changes first.` |
| The last session's recording is not processed, and `What happened` is empty | The recap is skipped. The plan lines and threads carry |
| An older session is open | The action is on that session's own **Last time** line and writes that note |
| Run open, sync on | No effect |

**What the note stores.**

```markdown
---
session_number: 2
carried_from: Session 1 The Road
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
and the `carried_from` property. The message and its Undo belong to this note and end at the next edit of the
note or the plan.

**Keyboard.** Tab reaches `Carry over` after the **Last time** link. Enter presses it.

**What stays with the DM.** Which threads are still open, and what the recap says aloud. The DM can delete or
rewrite any carried line.

**Not built, and why.** A recap that an LLM writes for reading aloud (the summary exists; the DM chooses to
add it). Carrying from sessions before the last (an open thread that the DM never ticks travels forward one
session at a time). Ticking a thread in the old note when it is ticked here (two files for one act).

| Check | F5 |
|---|---|
| Primary action in view | The plan's `Start fight`, as today |
| Components and tokens | `Carry over`: `Action::quiet`, with `line_start` when it starts a line. Result: `undo_message`, success tone. The embed in Read: the Reader's prose under a `Type::Small` muted line with `link` |
| In view at 960x640 | The **Last time** line and the action, above the tabs, with Lookup closed or open |
| States | Blocked: the reason beside the action in `Small` |
| Loss and failure | One Undo for all lines. Nothing is replaced |
| Keys and targets | Tab, Enter. A Read control, 28 px |

### F6. Wrap-up to World

**Question.** "How does what happened to this person or place get into its own note?"

**Today.** A change of Status, place or group in play writes the value and a Log line in one step, with one
Undo (T15.4, `src/world.rs:2280-2298`). The session's Read view lists those lines under **World changes**,
from each entry's Log (`src/sessions/page.rs:3856`). Two things are missing: no action writes the DM's own
words, and nothing leads from a fight or the plan to the entry.

**Two acts, in one place.** Both are in the entry's own reader, in Lookup and on World.

| The DM means | The act | What the file gains |
|---|---|---|
| A fact that a field holds: "Verori died" | **Change** on Status, as today | The status, and the line `Status alive → dead.` |
| A fact that no field holds: "She told the party who burned the ropes" | **Add to Log**, new: a quiet action at the end of the entry's Log, or after its text when it has no Log yet | One line, in the DM's words |

A death is a status. `Add to Log` does not change a status, and Splat proposes no "died", because death is
the DM's ruling. When the DM does both, the Log has two lines for two facts.

**Three ways in, each one click.**

| From | The click | What opens |
|---|---|---|
| A fight's ended view | The name on a Final state row is a link when it is exactly one World entry, as the run's panel matches names today | Lookup on that entry |
| A scene that is done and links one World entry (F3) | `Add to Log` in the row's action place | Lookup on that entry, with the Log field open. The field holds the scene's words, selected, so typing replaces them and Enter keeps them |
| A session note | A link to the entry in Read opens Lookup. While writing: the `/` command **Log**, which finds the entry as **Search** does and then takes the words | Lookup on that entry; or the words go from the note's own card |

**Wireframe: Lookup on an entry, case B (the panel is 432 px), beside the ended view or the Session page.**

```text
🔍 [Search…                                          ]   ← Back   ×
Verori   ✎ Edit
NPC · Campaign · alive
Stat block   Bandit Captain                    Change
Status       alive                             Change
Tiefling. Runs the toll at the ford.

Log
• Session 11, Day 2: In Ashfall Keep → The Ford.
  Session 12, Day 3:  [What happened?                              ]

✓ Added to Verori’s Log.   ↶ Undo add
```

- `Add to Log` opens one field, the app's add field. The line's start, `Session 12, Day 3:`, shows before it
  in muted text, so the DM sees what the file will say. Enter adds the line and closes the field. Escape
  closes it and keeps the words with that entry, as an edit form is kept, also when Lookup goes to another
  entry or closes.
- **Which session.** The one the DM came from: the run's session, the plan's session, or the note's session.
  From Lookup or World with no such start, it is the session being played today. With none, the line starts
  with the campaign day only.
- **The result** shows under the Log, where the line appeared, so it is in view on a long entry.
- **Case D:** the panel is 346 px. The field takes a line of its own under the muted start.

**Lookup.** This feature uses Lookup. When Lookup shows another entry, the click goes to the new entry, and
Back returns. An unsaved edit form stays with its entry. An unsaved NPC stays under its row (section 1.2).
While the same entry has an edit form with changes, `Add to Log` is blocked, so that Splat never makes its own
"changed on disk" warning.

**Exact words.** `Add to Log`. Field's accessible name: `Log line for Verori`. Placeholder: `What happened?`.
Result: `Added to Verori’s Log.` `Undo add`. The `/` command: `Log`, with the help `Add a line to an entry’s
Log`. A Final state row's link has the accessible name `Verori (opens the World entry)`.

**States.**

| State | What shows |
|---|---|
| The entry has no Log | The action follows the text. The first line makes `## Log` as the last heading |
| The entry's edit form has changes | Blocked: `Save or discard your edit first.` |
| The entry belongs to a package | Blocked: `This entry belongs to a package. Copy it to the campaign to keep a Log.` |
| The file cannot be read | Blocked: `Splat cannot read NPCs/Verori.md. Repair the file first.` |
| The file changed on disk | Splat reads the file again and adds the line to what is there. Nothing of the outside edit is lost |
| The write fails | An error under the Log names the file. The field stays open with its words |
| Run open, ended or not | The same. A Log line is not a run step |
| A Final state row that matches no entry, or two | Its name is plain text |
| Sync on | No effect |

**What the note stores.** One more line, and nothing else:

```markdown
## Log

- [[Session 11]], Day 2: In [[Ashfall Keep]] → [[The Ford]].
- [[Session 12]], Day 3: Told the party who burned the ropes.
```

The session's Read view then lists `Verori  Told the party who burned the ropes.` under **World changes**,
with no second copy in the session note.

**Undo.** `Undo add` removes that one line, after a check that the file still holds it. The message and its
Undo belong to that entry. They end when the DM adds another line to the same entry or edits it. A change to
another entry does not end them. A Status change keeps its own Undo, as today.

**Keyboard.** In the reader, Tab reaches `Add to Log`; Enter opens the field; Enter adds; Escape closes. In
a note: `/log`, Enter, the name, Enter, the words, Enter.

**What stays with the DM.** The words, and whether a status changed.

**Not built, and why.** A Log line on a Location for each fight there (the Timeline's place is plain text). A
Log for player characters (they are not World entries). Editing a Log line in Splat (Edit and Obsidian do it).
Words on a Status change's own line (two lines say two facts, and the act stays as it is today).

| Check | F6 |
|---|---|
| Primary action in view | None in Lookup. On the ended view, `Close run` stays the one primary |
| Components and tokens | `Add to Log` in a reader: `Action::quiet` with `line_start`, no icon. On a scene's row: `Action::quiet` in the row's action slot. A Final state name: `link`. Field: `add_field`. Start of the line: `Type::Small` in `muted`. Result: `undo_message` |
| In view at 960x640 | The ended view keeps its rows and `Close run`. Lookup shows the title, the Log's last lines, the field and the result |
| States | Blocked: the reason in `Small` after the action |
| Loss and failure | `Undo add`. A failed write and Escape both keep the typed words |
| Keys and targets | As above. Every target is 24 px or more |

### F7. Rewards record

**Question.** "What did the party get tonight, and is it in the notes?"

**Where it lives.** On the Session page, in the plan row of a fight that has ended: a quiet `Add rewards` in
the row's action place, which is empty today. The DM lands on this page after `Close run`, with the note beside
the plan, so the next step is in view and its result shows where the DM looks. It writes one block for that
fight under `## Loot and rewards`. `Copy loot` stays on the ended view, for a chat or another program.

- **Not on the ended view,** because the note is not in view there, and a fight with no loot has no loot line
  to hold the action.
- **Not part of `Add to notes`,** which links the fight under `## What happened` today. A DM who links a fight
  would get lines they did not ask for.

**What it writes.**

- The fight's link, as events write it: `[[Timeline#^t-08a07eb0ec|Ashfall Crossing]]`.
- **XP,** on that line: the XP of the creatures that ended at 0 HP, also those marked absent at 0 HP, and
  each character's share. F1's function gives the numbers. A creature that still stood is not counted, and
  the line says so. The DM changes the line when the ruling is different.
- **Loot,** as lines under it: the encounter's loot and the loot of each creature at 0 HP, with the result of
  each roll.

**The session's XP.** Splat does not write a total, because a stored sum goes stale when the DM changes one
fight's line. Splat adds the `XP` numbers of the fight lines as they stand in the note. It says the sum in the
result of each `Add rewards`, and Read shows it under the heading as one muted line: `XP tonight: 1,650 · 330
each for 5`. It gives a share only when every fight had the same number of characters present.

**When the fight changes after its rewards were added.** The run file keeps the lines that Splat wrote for
this fight. The row compares them with the fight as it is now.

| The note and the fight | The row says | A press |
|---|---|---|
| The note has no block for this fight | `Add rewards` | Writes the block |
| The fight is as it was when the block was written | `Rewards in notes`, a status | Nothing to press |
| The fight changed: a roll was made or taken back, loot was added, or the fight went on and ended again (F8) | `Update rewards` | If the block still reads as Splat wrote it, Splat writes it again. If the DM changed the block, Splat adds only the lines that are new, changes nothing else, and says so |

A line that the DM added words to, such as a name, is still that line: Splat finds a line by its start.

**Loot that nobody planned.** The ended view gets the encounter editor's loot field: a quiet `Add loot` under
the loot list, or alone when the fight has none. The words join the fight's loot in the run file, as one step
of the run's Undo. Then `Add rewards` carries them like any other line.

**Wireframe: the plan, case A.**

```text
>  Ashfall Crossing                         Add rewards   ⋯
   Held the ford · Ended · Round 3
>  Shrine of Cinders                 ✓ Rewards in notes   ⋯
   Ended in round 1
✓ Rewards added under Loot and rewards. XP tonight: 1,650.   ↶ Undo add
```

**Wireframe: the ended view, case A, left column.**

```text
LOOT  1 to roll                                    [⚄ Roll all 1]  Copy loot
Sunken Vault
• 120 gp in a locked coffer
⚄ A smoky quartz (50 gp)
⚄ gems · 1d4   [⚄ Roll]
Add loot
```

**Cases B, C and D.** The plan has the page's width. `Add rewards` keeps its place at the right of the name
line. On the ended view with Lookup open, the view has one column, and `Add loot` is under the list.

**Lookup.** Neither act uses Lookup.

**Exact words.** `Add rewards`. `Rewards in notes`. `Update rewards`. Result: `Rewards added under Loot and
rewards. XP tonight: 1,650.` `Undo add`. After an update of a block that the DM changed: `New rewards added.
Your changes to this block stay, so check its XP.` `Add loot`; its field: `Type loot, or find an item or
table`, as in the editor. The setting, under **Campaign → Rules and runs**: the checkbox `Add XP to rewards`,
on at first, with the help `Turn this off when the party levels up by milestone.`

**States.**

| State | What shows |
|---|---|
| The fight has no loot result, and XP is off or not known | No action |
| The fight is not ended | No action. The row has its status link |
| The run file cannot be read | Blocked: `Splat cannot read this fight’s run, so its rewards are not known.` |
| The note changed on disk and Splat holds unsaved words | Blocked: `Settle the note’s changes first.` |
| The note has no `## Loot and rewards` | The heading is added at the end of the note |
| The DM deleted the block | `Add rewards` again |
| An older session's fight | The block goes to that session's note |
| Run open | The same. A fight in play has no action |
| Sync on | No effect |

**What the note stores.**

```markdown
## Loot and rewards

- [[Timeline#^t-08a07eb0ec|Ashfall Crossing]] · XP 400 for 2 at 0 HP · 80 each for 5 · 1 still stood (XP 200)
  - 120 gp in a locked coffer
  - A smoky quartz (50 gp), from 2× gems
  - Vault key (Vault Guard)
- [[Timeline#^t-5c1d2e3f4a|Shrine of Cinders]] · XP 1,250 for 3 at 0 HP · 250 each for 5
```

**What the run file stores.** The loot that the DM added, in the run's state, and the lines that went to the
note, beside the state:

```json
"rewards_noted": [
  "[[Timeline#^t-08a07eb0ec|Ashfall Crossing]] · XP 400 for 2 at 0 HP · 80 each for 5 · 1 still stood (XP 200)",
  "120 gp in a locked coffer",
  "A smoky quartz (50 gp), from 2× gems",
  "Vault key (Vault Guard)"
]
```

**Who holds what.** Splat does not record it. The players keep their sheets on D&D Beyond, and a second list
would go stale. A loot line is words, and the DM can write a name on it.

**Undo.** `Undo add` removes the lines that the act added, when they still stand as written, and the run
file's record of them. The message and its Undo belong to that note and end at its next edit. `Add loot` on
the ended view is a run step: `Undo: add loot Varra’s signet ring`.

**Keyboard.** Tab reaches `Add rewards` in the row. On the ended view, Tab reaches `Add loot`; Enter adds the
typed words.

**What stays with the DM.** Who earned XP, whether a creature that fled counts, how loot is shared, and when
the party levels up. A milestone is a line that the DM writes.

**Not built, and why.** A party inventory. A level-up prompt. Loot with no fight (the DM types it in the
note; Splat does not move it under the heading).

| Check | F7 |
|---|---|
| Primary action in view | Session: the next `Start fight`. Ended view: `Close run` |
| Components and tokens | `Add rewards`, `Update rewards`: `Action::quiet` in the plan row's action slot, as `Add to notes` on an event row. `Rewards in notes`: `status` with the check icon in `result.success`. `Add loot`: `Action::quiet` with `line_start`, then `add_field`. The sum in Read: `Type::Small` in `muted`. Checkbox: the theme's, with a `Small` muted help line |
| In view at 960x640 | The played rows and their action, with Lookup closed or open |
| States | Not added: `Add rewards`. Added: check and `Rewards in notes`. Changed since: `Update rewards`. Blocked: the reason in `Small` |
| Loss and failure | `Undo add`. Splat never rewrites a block that the DM changed |
| Keys and targets | As above. Read controls, 28 px |

### F8. Undo after a restart, and Continue fight

An implementer builds this now. This part is the design for it, the checks against the rest of the app, and
four things in that direction that this design changes.

**Question.** "I made a mistake, or I ended the fight too soon. Can I still take it back?"

**Undo after a restart.** The run file holds the Undo steps: each step's state, its name, and whether a sheet
made it. `Undo` works after `Close run`, after **Resume**, and after the app restarts. It looks and works as
today: `Undo: 7 damage to Ember Cultist`, `U`, up to 100 steps.

- The texts `Nothing to undo since reopening` and `Reopened, so the ending cannot be undone.` go. An empty
  list says `Nothing to undo`. A run file from before this feature opens with no steps and says the same.
- After a restart the first read of each linked sheet can remove that character's steps, as an incoming
  change does today. The notice says `Undo for Kestrel stops here` and leaves as it does today, and the Log
  names each removed step.
- When the run file changed on disk, the choices of today apply. `Save as a new run` keeps this run's steps in
  the new file.

**Continue fight.** An ended run can go on. One named step removes the ending, and play returns to the turn
it ended on, in the same round, with the same HP, effects and counts.

**Wireframe: the ended view, case A, after a loot roll.**

```text
Run · Sunken Vault
Ended · Round 2            ✓ Saved  [↶ Undo: loot roll on gems]  Continue fight      [Close run]
Outcome: Held the vault
```

**Case D (960x640, Lookup open).** The header has 454 px, and an ended view has no `Next turn` to keep room
for. Line one keeps the title and `Close run`. `Undo`, with its whole target, and `Continue fight` take a
second line that starts at the left, under the outcome. So the DM always reads what Undo takes back before
the click, also when `U` is off.

**Checks against the rest of the app.**

| Part of the app | What Continue does |
|---|---|
| The session plan | The fight's row goes from its result (`Held the vault · Ended · Round 2`) to the open fight's status link (`Round 2`) |
| The Timeline | The same event, found by its run. Its result says `Round 2` again (`apply_run`, `src/timeline.rs:1240`). No event is added, so T4.6 holds |
| History | The run leaves History while it is open and comes back when it ends again. The read-only view in History has `Continue fight` in its header. It is blocked while another run is open: `Close the open run first.`, the words of `Start fight` |
| Retired encounters | Retirement is not stored. It follows the run, so the encounter is In progress while the fight goes on |
| Carried state | The continued run keeps its own HP. When it ends again, it is the last fight of the session, and the next Setup starts from it and says so. A fight that began after the first ending keeps the HP it began with |
| A long rest after the first ending | The continued run keeps its saved HP, as a resumed fight in combat does today |
| Loot rolled after the ending | The results stay. The party found those things. When the fight ends again, the ended view shows them in place of their buttons |
| Sync | Sync starts again with a first read, as on Resume. A difference is a conflict of the usual kind |
| The Log | One line: `Fight continued in round 2.` The earlier `Ended manually…` line stays, because it happened |
| The outcome | The next `End encounter…` starts with the earlier outcome, selected. Enter keeps it, and typing replaces it |

**Four changes to the direction.**

1. **Show `Continue fight` only when Undo cannot do the same.** Right after a wrong ending, the header says
   `Undo: end encounter`, and that one press goes back. A second control with the same effect breaks the rule
   that one action has one control on a screen. `Continue fight` shows when a later step stands above the
   ending (a loot roll), when the file has no steps, and in History.
2. **Keep loot results through Continue.** If Continue removed them, the DM would lose rolls the table saw.
3. **Save each step without the creatures' frozen definitions.** A run state holds a full copy of each stat
   block (`definition`, `src/encounter_run.rs:40`). One hundred steps of that would grow the file and slow
   every autosave. The definitions are the same in every step, so the file needs them once.
4. **A tracker action (F4) and a Log line (F6) are never run steps.** They change other files. A step in a
   run file that changes another file cannot be kept true after a restart. Each has its own Undo where the DM
   acted.

**Exact words.** `Continue fight`. Hover: `Remove the ending and go on from this turn.` The notice line after
it: `Fight continued. The ending was removed.` The Undo of it: `Undo: continue fight`, which ends the fight
again with the same outcome. Blocked: `Close the open run first.`

**States.**

| State | What shows |
|---|---|
| The newest step is the ending | `Undo: end encounter`. No `Continue fight` |
| A later step, no steps, or History | `Continue fight` |
| Another run is open | Blocked, with the reason |
| The run file changed on disk | The choices of today come first |
| Pending sync on an ended run | `Close after sync`, as today. Continue starts sync again |
| The run file cannot be read | History says so, as today. No action |

**What the run file stores.** The steps, beside the state:

```json
"history": [
  {"label": "7 damage to Ember Cultist", "from_sheet": false, "state": {"round": 2, "cursor": 3, "…": "…"}},
  {"label": "end encounter", "from_sheet": false, "state": {"round": 2, "cursor": 4, "…": "…"}}
]
```

**Undo.** Continue is a step. `Undo: continue fight` takes it back. The owner of each step is the run, and a
step ends only when 100 newer steps stand above it or a sheet's change removes it.

**Keyboard.** `U` and Ctrl+Z, as today. `Continue fight` has no letter, so a stray key cannot reopen a fight.
Tab reaches it before `Close run`.

**What stays with the DM.** Whether the fight goes on, and what the players' HP means when another fight
began in between.

**Not built, and why.** Redo. Undo across two runs. Undo for the Timeline or the notes from the run's button.

| Check | F8 |
|---|---|
| Primary action in view | `Close run`, alone |
| Components and tokens | `Undo`: the header's control of today. `Continue fight`: `Action::quiet` with `icons::RUN`; `.blocked(reason)` |
| In view at 960x640 | `Ended · Round 2`, the outcome, `Undo`, `Continue fight`, `Close run` |
| States | As the table says. A blocked action shows its reason beside it |
| Loss and failure | Every step has Undo, also after a restart |
| Keys and targets | `U`, Ctrl+Z, Tab. Read controls |

### F9. Travel and random encounters

**Question.** "The party travels for two days. Does anything find them?"

**Where it lives.** In the Travel event's own row, opened in place: in the session's plan, and in the detail
column on Timeline. The row gets two things. **How long** is the Timeline's field, so the DM sets the length on
the page for tonight; today that needs a trip to Timeline. **Encounter checks** is a list with one line for
each day or watch of the journey.

- **Not a new kind of event,** because Travel exists and already moves the clock.
- **Not in Lookup,** where a roll on a table only shows and is not kept.

**The table marker.** The marker gets one optional word, `per`. Nothing else changes.

```markdown
## On the road

<!-- splat:rolls key=travel-road dice=1d20 per=day -->

| d20 | On the road |
| --- | --- |
| 1–14 | Nothing stirs. |
| 15–17 | [[Wolf Pack]] pick up the trail. |
| 18–20 | A toll gang blocks the way: [[Bridge Ambush]]. |
```

- A **travel table** is a marked table whose key starts with `travel`, as the NPC generator finds its tables
  by key. The row's menu lists travel tables first, then every other table. The campaign's last choice is the
  first choice next time.
- The app names a table by its heading (`On the road`), as the Reader's roll button does today, never by its
  key.
- `per` says how often to roll: `day`, which is the value when there is none; `watch`, which is 8 hours; or a
  number of hours, such as `4h`. A word that Splat cannot read is a warning on that table only, and the table
  rolls per day.
- A row's words can link an encounter or an encounter group. This needs no marker, because it is the note's
  own text.

**Wireframe: the open Travel row in the plan, case A. The plan is 330 px.**

```text
⌄  Road to the ford                      Add to notes   ⋯
   How long   [2 days                ]
              Day 3, 08:00 to Day 5, 08:00
   Title      [Road to the ford      ]
   Notes      [                      ]
   Encounter checks    On the road ⌄ · 1d20 each day
     Day 3   [⚄ Roll] [Typed]
     Day 4   [⚄ Roll] [Typed]
     [⚄ Roll all 2]
   Timeline
```

After the rolls:

```text
     Day 3   12 · Nothing stirs.
     Day 4   16 · Wolf Pack pick up the trail.
             Plan “Wolf Pack”
✓ 2 travel checks rolled.   ↶ Undo roll
```

- **The lines follow the length as the DM types it.** Each key press that gives a length the clock can read
  sets the lines at once, with no Enter. So the list does not move when the DM goes to press `Roll all`.
- The number of lines is the length divided by `per`, rounded up, and at least one. A check for a day is named
  by its day (`Day 3`). A check for a watch or for hours is named by its start (`Day 3, 08:00`). A length that
  the clock cannot read gives one line, `This journey`, beside the note that the length was not read.
- **A result that links an encounter or a group offers `Plan “<name>”`** for each, at most three. One click
  writes the link under `## Plan for tonight` as the first line still to come. So the fight is the next plan
  row and has the primary `Start fight`, and a group is first in Arrivals. Then the line says `In the plan`,
  for as long as the plan holds that link: Splat reads this from the note and stores no mark. A result with no
  link offers nothing, and the DM uses `Fight` on the Add strip, as today.
- More than 31 lines are not listed. The row says `This journey has 240 checks. Split it into shorter
  journeys to roll them here.`
- **Cases B, C and D.** The plan has the page's width, which is wider than 330 px, so nothing more gives. In
  case A a result's words wrap under the day, and `Plan “…”` starts a line at the words' edge.
- **On Timeline** the detail column shows the same block under **How long**. `Plan “…”` adds to the plan of
  the session that the event belongs to.

**Lookup.** The table's name is a link. It opens the table's note at its heading in Lookup, beside the page.
A link in a result opens as every Reader link does. Both go through Lookup's own path, so Back returns and a
draft is kept. Nothing opens by itself.

**Exact words.** Field label: `Encounter checks`. `<table’s name> · 1d20 each day`, or `each watch`, or
`every 4 hours`. `Roll`. `Typed`. `Roll all 2`. `Plan “Wolf Pack”`. `In the plan`. Result: `2 travel checks
rolled.` `Undo roll`. In the row's menu, when the campaign has tables and none is a travel table: `Roll for
encounters`. A table with a fault: the warning status `Cannot roll` and its reason, as a loot line says it.

**States.**

| State | What shows |
|---|---|
| The campaign has a travel table | The open row shows **Encounter checks** |
| The campaign has marked tables, and none is a travel table | No block. The row's `⋯` menu has `Roll for encounters`, which shows the block with the menu of all tables |
| The campaign has no marked table | No block and no menu line |
| The table's rows do not cover the dice | `Cannot roll`, and why |
| The table changed after a roll | Each result keeps the words it was rolled with |
| `Timeline.md` cannot be read, or changed on disk | The plan's problem block and the Timeline's choices of today. Nothing rolls until they are settled |
| Run open | The same. A travel check is not a run step and uses its own dice |
| Sync on | No effect |
| A journey of an older session | The block shows its results. `Typed` stays open for a correction |

**What `Timeline.md` stores.** The event keeps its table and its checks. Splat writes the body from the list,
so Obsidian shows each result under the event.

```yaml
- id: t-5c1d2e3f4a
  kind: travel
  title: Road to the ford
  duration: 2 days
  session: 070eca36-331c-46cc-b5a6-32a6e35265da
  table: travel-road
  checks:
  - {at: "Day 3", roll: 12, row: Nothing stirs.}
  - {at: "Day 4", roll: 16, row: "[[Wolf Pack]] pick up the trail."}
```

An older Splat keeps these fields, because unknown event fields stay in the file.

**Undo.** `Undo roll` removes the results of the last press, one roll or all of `Roll all`. `Plan “…”` gives
the plan's `Undo add`. Each message belongs to the event's row and ends at the next edit of the plan or the
event.

**Keyboard.** Tab walks **How long**, the fields, the table menu, each `Roll` and `Typed`, then `Roll all`.
Enter presses. A number and Enter in `Typed` records a player's own roll.

**What stays with the DM.** Whether a result happens, where and when on that day, and what it means. Splat
does not plan a fight or start one. A fight on the road goes on the Timeline when it begins, as every fight
does.

**Maps.** A separate design defines maps. When a Location can give a travel time to another, **How long** can
start with that value. Nothing here needs it.

**Not built, and why.** Weather, pace, getting lost and foraging (each is a table the DM can mark and roll in
Lookup). A table chosen by terrain or Location (it waits for maps). A result that rolls its own dice, such as
`2d4 wolves` (the DM uses the dock or the fight's roster).

| Check | F9 |
|---|---|
| Primary action in view | The plan's `Start fight`. No roll is primary |
| Components and tokens | **How long**: the Timeline's field (`text_field` with its help line). Label: `small_field`. Table: `menu_button`. `Roll`, `Roll all`: `Action::secondary` with `icons::DICE`. `Typed`: `number_field`. Result: `Type::Small`, the number in `text`, the words in `muted`. `Plan “…”`: `Action::quiet` with `line_start`. `In the plan`: `status`. Messages: `undo_message` |
| In view at 960x640 | The open row scrolls with the plan, as open rows do today. The Add strip stays above it |
| States | Not rolled: the buttons. Rolled: the number and words. Planned: check and `In the plan`. Fault: warning icon and `Cannot roll` |
| Loss and failure | `Undo roll`, `Undo add`. A failed save shows the Timeline's own `Timeline not saved` |
| Keys and targets | As above. `Roll` and `Typed` are 28 px high and at least 24 px wide |

## 4. Cross-feature rules

### 4.1 One fact, one owner, one place to act

"Acts on it" is the one place where the DM changes the fact. "Also shows" is a read-only echo with the same
words, drawn from the same owner.

| Fact | Owner file | The DM acts on it | Also shows, read-only |
|---|---|---|---|
| A fight's difficulty (F1) | None. Computed from the encounter note, the saved sheets and the rule entry | Nowhere. It follows the roster | Editor heading, Encounters row, plan row, Arrivals |
| An effect and its count (F2) | The run file. In prep: the encounter's or group's starting conditions | The dock's `Effect` field to add; the word on the row for every other act; **Conditions** to remove | The notice line on its turn; the Final state rows |
| A reminder (F2) | None. It is the effect, shown on its turn | The effect's word on the row | Nowhere else |
| A scene and its done mark (F3) | The session note, under `## Plan for tonight` | The plan row's box; the note | Obsidian |
| A tracker's value (F4) | `Trackers.md` | The Trackers view | The house rule note's page |
| A Due notice and its rolls (F4) | `Trackers.md` | Its row in the Trackers view | The rail's count; the run's notice line while the view is closed |
| The recap of the last session (F5) | The last session's note, `## What happened` | That note | This note's `## Recap of last time`, by an embed; the **Last time** line |
| An open thread (F5) | The note of the session where it is open | That note | Nowhere. A carried line is this session's own line |
| That a note carried over (F5) | The note's `carried_from` property | `Carry over`, `Undo carry over` | The **Last time** line has no action |
| An NPC's status (today) | The entry's note | **Change** in the entry's reader | The reader's kind line; a scene row's second line |
| A Log line (F6) | The World entry's note, `## Log` | `Add to Log`, or a **Change**, in the entry's reader | The session's **World changes**; the pages of entries that the line links |
| What the party found in a fight (F7) | The run file | The ended view: `Roll`, `Add loot` | The creature's card; `Copy loot` |
| A fight's rewards in the notes (F7) | The session note, `## Loot and rewards`. The run file keeps what Splat wrote there | `Add rewards` and `Update rewards` on the played row; the note | Obsidian |
| The session's XP (F7) | None. The sum of the fight lines in the note | Those lines | The result of `Add rewards`; one muted line in Read |
| A fight's result (today, F8) | The run file. `Timeline.md` holds the result line | `End encounter…`, `Undo`, `Continue fight` | The plan row, Timeline, History |
| The Undo steps of a fight (F8) | The run file | `Undo` in the run's header | Nowhere |
| A journey's length and checks (F9) | `Timeline.md` | The Travel row, in the plan or on Timeline | The clock; the Timeline's body in Obsidian |
| That a check's fight is planned (F9) | The session note: the link under `## Plan for tonight` | `Plan “…”`; the plan row's `Remove from tonight` | `In the plan` on the check |

### 4.2 Pairs that could show one fact twice, and which one wins

| Pair | The risk | The rule |
|---|---|---|
| F2 reminder and F4 Due notice | A mark's words name a turn, and both speak of it | They never share a fact (the table in F4). On the notice line the reminder wins. The Due count stays in the rail |
| F2 effect and an official condition | `Frightened` twice on a row | Words that are a condition's name become that condition, with the count. One word on the row |
| F2 effect in the run and **Other effect** in an editor | Two kinds of free words | Both are effects. Each gives a reminder, and a count in its words is read |
| F2 `Conc` and an effect named Concentration | Two marks for one state | `Conc` is the one mark. Splat suggests no effect by that name |
| F2 the row and the notice line | Two places to act on one effect | The row is the place to act. The notice line has no control |
| F3 scene and a Timeline event | A scene as a plan line and as a Major event | The scene is the plan. The event is what happened. Nothing links them by itself. A done scene with one entry offers `Add to Log`, not an event |
| F3 scene and a World open question | An unticked task counted in two lists | World counts only World notes' tasks, as today (`src/world/checks.rs:72`). A scene is in a session note |
| F3 done scene and F6 Log line | The same sentence in the session note and in the entry | The Log line is the entry's fact. The plan line stays the plan. `Add to Log` only offers the scene's words; it copies nothing by itself |
| F6 `Add to Log` and a Status change | A death written as words while the status says alive | A fact that a field holds is a **Change**, which writes its own Log line. `Add to Log` is for what no field holds. Every journey here records a death with **Change** |
| F4 quick action and a Timeline event | A change in the ledger and on the Timeline | The ledger owns the change. `Add to Timeline` is a second, chosen act that makes an event with the label |
| F4 table roll and the run's Log | One roll in two records | The ledger owns a quick action's roll and a notice's rolls. A roll from a note's own **Roll** button stays a run step, as today |
| F4 `Undo change` and the run's `Undo` | Two Undo controls in view in a fight | Each names its own step and takes back only its own file's change. `U` is the run's |
| F5 recap and F5 summary | The summary copied, and embedded | The embed of `What happened` wins. The summary is copied only when `What happened` is empty |
| F5 carried line and the old line | The same plan line in two notes | Each note owns its line. The old one says it was not played. Carry over never changes the old note |
| F6 Log line and **World changes** | A change listed in the session note and in the entry | The entry owns it. The session's list is computed, as today |
| F7 `Add rewards` and `Add to notes` | A fight linked twice | `Add to notes` links the fight under `## What happened`. `Add rewards` writes under `## Loot and rewards`. Each checks its own heading for the event's link |
| F7 the rewards block and the run | The note says what the run no longer holds, after an Undo or `Continue fight` | The run file keeps what Splat wrote. When the fight differs from it, the row says `Update rewards` |
| F7 the session's XP and the fight lines | A stored sum that is wrong after an edit | No sum is stored. Splat adds the lines as they stand |
| F7 `Add rewards` and `Copy loot` | Loot pasted and added | `Copy loot` writes no file. A pasted copy is the DM's own text, and `Add rewards` adds its block beside it |
| F7 XP and F1 XP | Two sums for one fight | One function. F1 counts every creature against the party. F7 counts those at 0 HP and names the rest |
| F8 `Undo: end encounter` and `Continue fight` | Two controls, one effect | `Continue fight` shows only when Undo cannot do the same in one press |
| F9 travel check and a roll in Lookup | A roll shown twice | The Travel row keeps its results. A roll in Lookup only shows, as today |
| F9 `In the plan` and the plan | A mark that outlives the plan line | Splat stores no mark. It reads the plan |
| F9 planned fight and the fight's own event | A fight on the plan and on the Timeline | As today: the plan link is the intent, and the run adds the event at Begin |
| F1 word in the plan and in the editor | Two values | One function, called by both |

### 4.3 One pattern for one act

Each act has one control, one set of words, one place and one key, with Lookup open or closed. "Today" names
where the pattern is already in the app.

| Act | Control | Words | Place | Key | Today | Used by |
|---|---|---|---|---|---|---|
| Add a thing of a kind | A quiet icon-and-word action of the Add strip, or a line of its **More** menu | The kind: `Fight`, `Scene`, `Travel` | The strip under **Tonight**, and on Timeline | Tab, Enter | Fight, Group, rests, events | F3 `Scene` |
| Add typed words to a list | The add field: one field with no button. Enter adds. An arrow key and Enter, or a click, puts a suggestion in the field, and a second Enter adds it. Escape closes and keeps the words. The field stays open after Enter only where the DM adds several lines in a row (a plan, a loot list). It closes after Enter where one line is the act (an effect, a Log line) | A placeholder that says what to type | The end of its list. In the run, the dock's second line | Enter, Escape | `Add loot` in the encounter editor | F2 `Effect`, F3 scenes, F6 `Add to Log`, F7 `Add loot` |
| Put a record in the session note | A quiet action in the row's action place. Then a status in the same place | `Add …`, then `… in notes` | The plan row, right of the name | Tab, Enter | `Add to notes`, `In notes` on event rows | F7 `Add rewards`, `Rewards in notes` |
| Mark done | A box at the start of the row | Accessible name `Done: …` or `Open: …` | Where a fight row has its caret | Space | World's task marks, read-only | F3 scenes, F4 Due rows |
| Roll | The dice icon marks every control that rolls. In a list or a panel it is a bordered button, `Roll …`. On a run row it is the word with the dice after it. A result takes the button's place when the thing rolls once, and is listed under it when it can roll again. `Roll all N` rolls several. `Typed` takes a player's own roll | `Roll`, `Roll all 2`, `Roll <table’s name>` | Beside the thing rolled | Enter | Tables in the Reader, loot, the dock, numbers in a stat block | F2 a spent recharge, F4 notices, F9 checks |
| Undo in a run | The header's one button, which names the step | `Undo: <step>` | Header, line two, right | `U`, Ctrl+Z | Every run step | F2, F7 `Add loot`, F8 after a restart and `Undo: continue fight` |
| Undo anywhere else | A message that says what happened, with a quiet Undo on its line. It belongs to one file and ends at that file's next change | `<What happened>.` then `Undo <noun>` | Where the DM acted | Tab, Enter | Plan, Timeline, World, Library | F3 `Undo add` and `Undo remove`, F4 `Undo change`, F5 `Undo carry over`, F6 `Undo add`, F7 `Undo add`, F9 `Undo roll` |
| Take back a tick | The box, again | | The row | Space | New | F3. A Due row's tick applies changes, so it has `Undo change` |
| Record a fight's result | The ending's field and Enter | `End encounter…` | The run's footer | Enter | Today | F8 adds the way back, `Continue fight` |
| Change a fact that a field holds | **Change** beside the value | `Change` | The entry's reader | Tab, Enter | Status, place, group, stat block | F6 uses it for a death |
| Remove | The row's menu, with an Undo message | `Remove from tonight`, `Dismiss` | The row's `⋯` | Tab, Enter | Fights and groups in the plan | F3 scenes, F4 `Dismiss` |
| Open a thing beside the page | A link with the thing's name | The name | In the words | Tab, Enter; Alt+Left returns | Reader links, condition words | F1 `Rule`, F3 scene links, F4 the rail's status and Ctrl+T, F6 a Final state name, F9 the table's name |
| Say why a control is off | `.blocked(reason)`, with the reason beside a lone control | A short sentence | Beside the control | | Every blocked control | Every state table in section 3 |

## 5. Consistency

### 5.1 New controls and their components

No feature adds a colour or a type size. Three icons join `design::icons`: `PIN`, `HOURGLASS` and `BELL`.
Section 3 names the component and token of each control in its check table. Two parts move to a shared place
so that every user has one copy:

- The loot editor's add field with suggestions becomes the shared part `add_field`, for loot, scenes, effects
  and Log lines.
- The Timeline's **How long** field becomes a form part that the plan row also draws.

### 5.2 Proposed text for `docs/design-system.md`

This pull request does not edit that file. Each pull request of the build plan adds its own lines.

- **Widgets, Add strip.** "On a plan, `More Add actions` starts with Scene. The visible actions do not change."
- **Widgets, a new row.** "Add field | `add_field` | One field with no button. Enter adds the words. An arrow
  key and Enter, or a click, puts a suggestion in the field; Enter then adds it. Escape closes and keeps the
  words. It stays open after Enter in a plan and a loot list, and closes after an effect and a Log line.
  Suggestions sit under it, or above it at a pane's bottom edge, at most four."
- **The encounter ledger and editor.** "A planned fight's summary ends with its difficulty word (`Low`,
  `Moderate`, `High`, `Above High`) in `Small` text colour, after the members. Members shorten first; the word
  is never cut. The roster heading reads `ROSTER 8 · XP 1,050 · Moderate`, with one muted `Small` budget line
  below it. The word has no colour and no icon. With no budget table, no sheets or no XP, no word and no line
  show. A character whose level is not known is named in the editor only."
- **The live run, Header.** "Notice priority is conflict, recording silence, other sync notices, the current
  turn's reminder, a Due notice while Trackers is not in view, next name. A reminder is a state notice: Undo
  gives up its target before the reminder shortens. A reminder has no control: every act is on the row."
- **The live run, Roster.** "Every word on line two that is not a condition is an effect. Effects come before
  plain conditions, and the first shows whole, on a line of its own when it must, on every row; a row's
  height does not change when it becomes current. A count follows its words in tabular digits (`Bless 9`). An
  effect's word is a link, and its hover says what a click does. A spent recharge ends with the dice icon."
- **The live run, Dock.** "The dock's second line is the selected name, then Conditions, Effect, Conc, Absent.
  When the words do not fit: key letters go, then the name shortens, then Effect keeps its icon. Effect turns
  the second line into one add field for the row it opened for; line one never changes. Enter and Escape put
  focus in Amount."
- **The live run, Panels.** "The Conditions tab has no free-words field. Its help says: `These record your
  ruling. No number changes by itself. Effects stay in this run and do not sync. Add one with Effect in the
  dock.`"
- **Saving and closing.** "The run file holds the Undo steps. An empty list says `Nothing to undo`. The ended
  view offers quiet `Continue fight` only when Undo's newest step is not the ending. At a narrow header, Undo
  with its whole target and `Continue fight` take a second line at the left; `Close run` keeps its place. A
  Final state row's name is a link when it is exactly one World entry."
- **The session's plan.** "A task line under `Plan for tonight` that links no encounter is a scene: a 24 px
  box where a fight has its caret, the words as Reader text, and a second line only when the words link one
  World entry. Rows to come keep the note's order. A done scene stays in place, with muted words. A played
  fight's action place holds quiet `Add rewards`, then the status `Rewards in notes`, or `Update rewards` when
  the fight changed. An open Travel row shows How long, Title, Notes, then Encounter checks. Every plan act
  changes the note's draft."
- **Notes and templates.** "The Last time line ends with quiet `Carry over` until the note has carried over.
  Its result is an Undo message below the line. In Read, an embed of another note's heading shows that text
  under a muted `From <session>` link. Under `Loot and rewards`, Read shows the sum of the XP lines as one
  muted line."
- **World.** "A campaign entry's reader has quiet `Add to Log` after its Log, drawn with `line_start`. It
  opens one add field after a muted line start (`Session 12, Day 3:`). Its result sits under the Log, where
  the line appeared. With no run open, an NPC's reader says `Start a fight` where it says `Add to fight`
  during a run. While Lookup holds an unsaved NPC, one row says `Unsaved NPC: <name> · Show`, and the opened
  entry shows below it."
- **Lookup and copies.** "Trackers is a Lookup view, opened by Ctrl+T or the rail's status. It is a place in
  Lookup's Back history. Its result line keeps a fixed place at the panel's bottom edge, and the list above
  it scrolls. In a run its controls give focus back to Amount, and the run's letter keys work until the
  search field takes focus. No feature adds a panel beside Lookup and Notes. A table is named by its heading,
  never by its key."
- **Space, shape and density, the rail.** "The rail's status slot can hold `Trackers` or `Due N` with the bell
  icon, while a house rule note is in play."
- **Icons.** "`PIN` marks an effect with no count. `HOURGLASS` marks an effect that counts. `BELL` marks Due."

### 5.3 Proposed text for `docs/ux-intent.md`

**Changed workflows.**

- **W4,** add: "After a restart, Undo still names the last step. A fight that ended too soon goes on from its
  turn." Done well: T4.7, T4.8.
- **W5,** add: "The editor says how the roster compares with the party's XP budget, from the rules the
  campaign uses." Done well: T5.7.
- **W11,** add: "A journey rolls its encounter checks in its own row, and a result becomes a planned fight in
  one step." Done well: T11.9, T11.10.
- **W14,** add: "The plan also holds what is not a fight: a scene, with a done mark. The note holds it."
  Done well: T12.10 to T12.12.
- **Section 3, Reachable from anywhere,** add: "**Trackers:** Ctrl+T opens the trackers and the Due list in
  the Lookup panel. A Due count stays in the rail."
- **Section 3, What must persist,** change the run row: "Open run: selection, amount, scroll, undo | Yes | Run
  yes, undo yes".
- **Section 7, A8,** change: "Drafts stay in memory. Undo is saved with the run."

**New workflows.**

- **W15. Keep track of what lasts.** "Trigger: a spell, a poison, a breath weapon or a legendary creature.
  Goal: on each turn the DM sees what to remember for that creature, and how long it lasts, with no answer
  needed. Context: table. Splat counts and prompts. The DM rules." Done well: T17.1 to T17.6.
- **W16. Close the evening and open the next.** "Trigger: the last fight ends, or the next session starts.
  Goal: rewards, what changed in the world, the recap and the open threads are where the next step needs
  them, with nothing typed twice. Context: desk." Done well: T18.1 to T18.5.
- **W17. Run a house rule's numbers.** "Trigger: a race, a clock, a count of seals. Goal: one press records a
  change, a due notice stays in view, and the history is a file. Context: table and desk." Done well: T19.1
  to T19.4.

**New rows for section 8.**

| ID | Observable test | Scenario | Type |
|---|---|---|---|
| T4.7 | After a restart, Undo names the last step and takes it back in 1 | run-restart | state, cost |
| T4.8 | A fight ended by mistake goes on from its turn in 1. The plan, Timeline and History follow, and no event is added | run-restart | cost, state |
| T5.7 | The roster heading shows XP and a word from the rules data. With no table, no sheets or no XP it shows no word | encounter-difficulty **new** | state |
| T11.9 | A two-day journey rolls two checks in 1, in the plan, and each result is in `Timeline.md` | travel-checks **new** | cost, state |
| T11.10 | A result that names an encounter is planned in 1, and its `Start fight` is the next one | travel-checks **new** | cost, state |
| T12.10 | A task line becomes a scene with a box. A tick changes one character of the note. A plain list line gets no row | session-plan-scenes **new** | state |
| T12.11 | A scene's link opens its entry in Lookup in 1, and Back returns to what was open | session-plan-scenes **new** | cost, state |
| T12.12 | An NPC with a stat block starts a fight in 1 from its reader. Nothing is written before Begin combat | session-plan-scenes **new** | cost, state |
| T17.1 | An effect with a count joins a row in ≤ 4, with the run's panel hidden. Focus is in Amount after it | run-effects **new** | cost, view |
| T17.2 | On its turn the effect's words show on the notice line and on the row. `Next turn` takes 1 and does not move | run-effects **new** | state, view |
| T17.3 | A count of 10 says 10 on the first turn it covers and 9 on the next, says `time is up` at 0, and removes nothing. Undo of `Next turn` gives it back | run-effects **new** | state |
| T17.4 | A conflict holds the notice line, and the reminder stays whole on the current row at 960x640 beside Lookup. No row changes height at `Next turn` | run-effects **new** | view |
| T17.5 | A spent recharge rolls in 1 and a use is spent in 1, both on the row | run-effects **new** | cost |
| T17.6 | Damage to a row marked `Conc` shows the save DC with no action | run-effects **new** | state |
| T18.1 | A fight's loot results and XP reach `Loot and rewards` in 1, with one Undo. After a later roll the row says `Update rewards` | session-plan | cost, state |
| T18.2 | A line reaches an entry's Log in ≤ 4 from a done scene and from an ended fight's row. The file gains one line | world-facts | cost, state |
| T18.3 | `Carry over` adds the recap link, the unplayed plan lines and the open threads in 1, below what the DM wrote. A deleted carried line does not come back | session-carry-over **new** | cost, state |
| T18.4 | A note the DM did not act on does not change. A new session with no carry-over is the template | session-carry-over **new** | state |
| T18.5 | Loot that nobody planned joins an ended fight in ≤ 3 and is in its rewards | run-loot | cost, state |
| T19.1 | Ctrl+T opens Trackers in Lookup, and Back returns to the entry that was open | trackers **new** | cost, state |
| T19.2 | A quick action takes 1, names its change, leaves focus in Amount, and has an Undo that survives a restart | trackers **new** | cost, state |
| T19.3 | A value that passes a mark raises a Due row, a rail count, and the notice line's words while the view is closed. A tick has an Undo | trackers **new** | state |
| T19.4 | A full roster and Trackers fit at 960x640, the result line is in view, and `N` takes the next turn | trackers **new** | view, cost |
| G6 | The unchanged evening of the audit gives the counts it gave before the change | dm-audit | cost |

### 5.4 Proposed terms for `CONTEXT.md`

- **Encounter difficulty**: the word that compares an encounter's XP with the party's XP budget: Low,
  Moderate, High or Above High. Splat shows it and blocks nothing. _Avoid_: Rating, challenge.
- **XP budget**: the XP a party can face at one difficulty, from the rules the campaign uses: for each
  character, the value for its level.
- **Effect**: words that the DM pins to one participant of a run, with or without a count of rounds or uses.
  Every word on a participant that is not a condition is one. It stays in that run. It is the run's form of
  an active effect. _Avoid_: Custom condition, timer.
- **Reminder**: an effect's words on the run's notice line during its participant's turn. It asks for nothing.
- **Scene**: a task line of a session's plan that is not a fight or a group: a talk, a person, a place or a
  secret, with a done mark. _Avoid_: Plan item, task, to-do.
- **Carry-over**: the act that adds the last session's recap link, unplayed plan lines and open threads to a
  session's note.
- **Rewards**: what the party received in a session: the loot results and the XP of its fights, under the
  note's `Loot and rewards`. _Avoid_: Treasure.
- **Travel table**: a roll table whose key starts with `travel`. **Travel check**: one roll on it for one day
  or watch of a Travel event, kept with that event.
- **Trackers**: the view of the Lookup panel that holds the Due list and the trackers.
- **Session plan**, change to: "The fights, groups and scenes a session's note lists under its plan, with
  what has happened so far."
- **Log**, add: "The DM can also add a line in their own words."
- The tracker terms of [World and house rules](world-and-house-rules.md) stand: house rule note, in play,
  tracker, quick action, mark, Due list, ledger.

## 6. Build plan

Each pull request merges alone and leaves `main` working. `src/sessions/page.rs` (8,107 lines) and
`src/encounter_run/view.rs` (9,859 lines) are shared, so the plan first moves the parts that several features
touch into files of their own.

**Rules for every pull request.**

- Its visual-test scenarios cover each changed surface **with Lookup closed and with Lookup open, at
  960x640**, and once at 1200x800. A new scenario goes at the end of `SCENARIOS`. The author looks at each new
  or wider menu and suggestion list in the 960 px capture, because the gate does not compare a popup with a
  splitter under it.
- It extends an existing scenario or test when that one has the same setup, before it adds a new one.
- It breaks the code once for each new check and says so.
- For each result that stays on screen and each Undo, it tests the owner and the end: an act A, an unrelated
  act B, then Undo of A. B's change stays, and A's words go with A's data.
- It adds its own lines to `docs/design-system.md`, `docs/ux-intent.md`, `CONTEXT.md` and `README.md`, from
  section 5.
- It adds its task to the evening in `src/visual_tests/dm_audit.rs` as a **new** task, and it runs the plain
  evening: the unchanged script must give the counts it gave before the change (G6).

| PR | Scope | Files and modules | Depends on | Scenarios | In the dm-audit script |
|---|---|---|---|---|---|
| **P0** | F8: the Undo steps in the run file; `Continue fight`. In progress elsewhere. This design asks for the four changes in F8 | `src/encounter_run.rs` (`SavedRun`, a `Continue` command), `src/encounter_run/view.rs` (ended header, empty-Undo words), `src/encounters/history.rs` | None | Extend `run-restart`, `encounter-history`, `run-loot` | A task: end by mistake, restart, `U`. A fact: the turn and round are the same |
| **P1** | No change in behaviour. Move the plan pane to `src/sessions/plan.rs` and the run's dock and notice line to `src/encounter_run/dock.rs` and `notice.rs`. Measure the evening again on `main` and write `steps.tsv` and `summary.tsv`, because the files are older than the **More** menu. Add the plain-evening test (G6), and split the script's totals into measured and new tasks | `src/sessions/page.rs`, `src/sessions/plan.rs`, `src/encounter_run/view.rs`, two new files, `src/visual_tests/dm_audit.rs`, `artifacts/dm-audit/` | P0 merged, so that the two do not meet in `view.rs` | None new. `visual-compare` against `main` shows no difference | The new yardstick, and G6 |
| **P2** | F1 | New `src/difficulty.rs`. `src/encounter_settings.rs` (the budget table). `src/encounters.rs` (heading), `src/encounters/list.rs`, `src/sessions/plan.rs` (one summary part), `src/encounter_run/ad_hoc.rs` (Arrivals) | P1 | New `encounter-difficulty`. Extend `encounter-build`, `session-plan`, `encounter-groups` | A fact for each planned fight: its word |
| **P3** | F3: scenes, `Scene` in the **More** menu, `add_field`, the held-NPC row in Lookup, `Start a fight` in the NPC's reader | `src/sessions.rs` (plan lines), `src/sessions/plan.rs`, `src/loot_editor.rs` and `src/design/components.rs` (`add_field`), `src/timeline/page.rs` (the menu of `quick_add`), `src/lookup.rs` (the held-NPC row), `src/world/page.rs`, `src/encounters.rs` (an unsaved encounter in Setup) | P1 | New `session-plan-scenes`. Extend `session-notes` (More menu), `npc-in-lookup` | Tasks: plan two scenes; open the NPC from its row; tick it |
| **P4** | F2 | `src/encounter_run.rs` (the `effects` map, the count in `end_turn`, commands, counts in starting conditions), `src/encounter_run/dock.rs`, `notice.rs`, `view.rs` (the row's second line, the Conditions tab), `src/rolls.rs` (read `recharge` and `legendary_actions`) | P1, and P3 for `add_field` | New `run-effects`. Extend `encounter-run`, `run-long-names`, `run-sync-conflict`, `run-edge-states` | The Frightened task with a count. A fact: the notice on Oren's turn |
| **P5** | F9 | `src/timeline.rs` (`table`, `checks`, the body), `src/timeline/page.rs` (the detail column: the shared block and the How long part), `src/sessions/plan.rs` (open row), `src/roll_table.rs` (`per`) | P1 | New `travel-checks`. Extend `session-notes`, `roll-table-author` | Tasks: roll the road check; plan and start the result |
| **P6** | F6 | `src/world.rs` (the reader's action and field), `src/world/lines.rs` (reuse `add_log`), `src/world/page.rs`, `src/sessions/slash.rs`, one link in `src/encounter_run/view.rs` (Final state rows) and one action in `src/sessions/plan.rs` | P3 | New `log-from-play`. Extend `world-facts` | A task: a Log line from the done scene. A fact: the file gained one line |
| **P7** | F7 | `src/sessions.rs` (the rewards block), `src/sessions/plan.rs` (played row), `src/encounter_run/loot.rs` (`Add loot`, export), `src/encounter_run.rs` (`rewards_noted`), `src/encounter_settings.rs` (the checkbox), `src/difficulty.rs` (XP at 0 HP), `src/markdown_view.rs` (the sum in Read) | P2, P0 | Extend `run-loot`, `session-plan` | Tasks: `Add rewards` for two fights. A fact: the XP in the result |
| **P8** | F5 | `src/sessions.rs` (carry, `carried_from`), `src/sessions/page.rs` (`last_time_block`), `src/markdown_view.rs` (an embed of a heading) | P3, for scenes in the carried lines | New `session-carry-over`. Extend `session-plan` | A task: `Carry over`. A fact: the lines in the note |
| **P9** | F4, part one: house rule notes, in play, the ledger, quick actions, the Trackers view, Ctrl+T, the rail's status | New `src/trackers.rs`, `src/trackers/ledger.rs`, `src/trackers/view.rs`. `src/lookup.rs` (the view switch and Back), `src/lib.rs` (key, rail, letter keys), `src/campaign_content.rs` (in play), `crates/core` (`KNOWN_FIELDS`) | P3, for the held-NPC row; P6 merged, so that the two do not meet in `lookup.rs` | New `trackers`. Extend `run-lookup`, `nav-state` | None in the measured evening. J4 has its own script |
| **P10** | F4, part two: marks, the Due list with its rolls and Undo, the run's notice, `Add to Timeline` | `src/trackers.rs`, `src/trackers/view.rs`, `src/encounter_run/notice.rs` | P9, P4 | Extend `trackers`, `run-effects` | J4's script |
| **P11** | F4, part three: day marks | `src/trackers.rs`, `src/timeline.rs` | P10 | Extend `trackers` | J4's script |
| **P12** | The audit. Extend the evening to use every feature, measure it, and write `steps.tsv`, `summary.tsv` and the README's table again. Add the scripts for J2, J3, J4 and J6 | `src/visual_tests/dm_audit.rs`, `artifacts/dm-audit/` | All | The audit's own captures | The whole evening: 13 transitions, and the measured tasks plus 18 |

**What can be built at the same time.**

- **First:** P0, then P1. Nothing else starts in `view.rs` or `page.rs` before P1 merges.
- **Wave A, three at once:** P2 (encounters, Arrivals), P3 (plan, Lookup), P5 (Timeline). They meet in two
  files, each in its own function. In `src/sessions/plan.rs`: the fight summary (P2), the scene rows (P3), the
  open event row (P5). In `src/timeline/page.rs`: the **More** menu of `quick_add` (P3), the detail column
  (P5).
- **Wave B, two at once, then one:** P4 (the run) and P6 (World and the reader). They meet in `view.rs` at two
  places that are far apart: the dock and row, and the ended view's rows. P9 (trackers) starts when P6 has
  merged, because both change `src/lookup.rs`.
- **Wave C, two at once:** P7 and P8. Both write the session note; P7 is in the plan row and P8 on the
  **Last time** line. Both add one function to `src/markdown_view.rs`.
- **Then:** P10, P11, P12, in order.

## 7. Open questions

Work can go on with each default.

1. **The save DC on a row marked `Conc` shows with no new act.** It is the one notice that the unchanged
   evening meets: when a monster's damage reaches Iselle after her mark. *Default:* show it. It asks for
   nothing and records the DM's own mark. The other choice is to cut it from F2.
2. **`Other effect…` moves from the Conditions tab to the dock as `Effect`, and every free-words effect gives
   a reminder on its turn,** also one that an encounter's starting conditions hold today. *Default:* both, so
   free words have one rule and a field that no panel can hide.
3. **Trackers is a view of Lookup, with no panel of its own.** The run's letter keys stay on while it shows.
   *Default:* the Lookup view. A footer control for one quick action can follow if play shows the need.
4. **The recap is a link (an embed), not copied text.** In Write and in a program that shows no embeds, the
   DM sees one line and not the words. *Default:* the embed, so that one file owns what happened.
5. **XP in the rewards counts the creatures at 0 HP, and `Add XP to rewards` starts on.** A milestone campaign
   turns it off once. The session's sum is shown and not stored. *Default:* as stated.
6. **`Continue fight` shows only when Undo cannot do the same.** The implementer's direction shows it on
   every ended run. *Default:* hide it while `Undo: end encounter` is the newest step.
7. **`Start a fight` on an NPC saves an encounter note named after the NPC at `Begin combat`.** The older
   idea was one kept party-only encounter. *Default:* a note for each, so History names the fight.
