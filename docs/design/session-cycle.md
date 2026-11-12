# The session cycle

Status: proposed, after design round 4 and its last corrections. Nothing here is built. F8 is being built in
pull request 232. A line that says "today" is about the app as it is.
This document fits nine features (F1 to F9) into the app as one workflow: prep at the desk, play at the table,
the wrap-up, and the next prep. It follows [the brief](../ux-intent.md), [the design
system](../design-system.md)
and the words in [`CONTEXT.md`](../../CONTEXT.md). The yardstick is the measured evening in
[`artifacts/dm-audit/`](../../artifacts/dm-audit/README.md): 13 page transitions and 231 interactions.

Evidence: `file:line` refers to `main` at `a4ca1e9c`. F8 is stated by its rule, not by a commit.
"Today" means that source. "New" means that this design adds it, and section 6 names the pull request.

## Decisions in brief

No feature adds a page, a panel, a tab, a strip or a dialog.

**Counts, by hand against `steps.tsv`** (section 2.1): the 41 measured tasks go from 13 page transitions and
231 interactions to 12 and 226, or 228 when the DM opens **More** twice on the Add strip. Seven records that
were on paper cost 14 more, and 2 more, once, when the campaign has several travel tables. The audit script
had drifted from `steps.tsv`. P1a repaired it and measured `main` again: 42 tasks, 17 page transitions and
240 interactions (`artifacts/dm-audit/README.md`). That is G6's yardstick for every feature.

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
current session or nothing. Both call it, and so do the ledger's Undo (F4), both Log stamps (F6) and `Add to
Timeline` (F4).

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
- **Ctrl+T shows Trackers, and Ctrl+T again goes back** (new, P9). With Lookup closed, it opens Lookup on
  Trackers, and a second press closes Lookup. With Lookup open, it shows Trackers in the panel, and a second
  press shows what Lookup showed before: its entry, its found note or its results. Ctrl+K opens Lookup on
  what it showed last, also on Trackers. Lookup's `Back` and its
  history are as today (`src/lookup.rs:1662-1666`, `:1755`).
- **Keys beside Trackers are the keys beside any Lookup view.** A tracker's typed amount holds focus until
  Enter or Escape. `U` is the run's key.
- A feature opens Lookup only when the DM clicks or presses a key, and it uses Lookup's own open path
  (`open_position`, `src/lookup.rs:1619`). No feature closes Lookup, except where a row above says that the
  same act closes it today.
- **One repair.** While Lookup holds an unsaved generated NPC, its card hides any entry that opens
  (`src/lookup.rs:2167-2169`). Lookup takes the World page's rule: one row above the reader says
  `Unsaved NPC: <name> · Show`, and the opened entry shows below it.
- **The reader's header** is one row: the title, and for an entry of the campaign `Edit`, one `L` after the
  measured title (`src/world.rs:1733-1768`). It has no menu. Lookup and the World page draw this one reader
  (`src/lookup.rs:2195-2203`); the Library page has its own header and does not change. `Start fight` (F3)
  and `Add to Log` (F6) follow `Edit`, as quiet actions, 4 px apart; `Start fight` is next to `Edit`, so it
  stays where it is for a DM who never adds to a Log, and `Add to Log` follows it (review of P6). The title
  keeps the width that it has
  today, and the row gains no line and no menu. The header shows the first of these that fits:
  1. `Start fight` and `Add to Log`, each with its word;
  2. `Start fight` with its word, and `Add to Log` as its icon (an `icon_button` with its name on hover, as
     a plan row's `More actions` is, `src/sessions/plan.rs:728`);
  3. `Start fight` with its word;
  4. `Add to Log` as its icon;
  5. neither.

  `Start fight` is never an icon. An entry that has only `Add to Log` shows its word, then its icon, then
  nothing.

**Sums for the header** (P1a measured the titles and Lookup's room: `artifacts/dm-audit/titles.tsv`;
T18.2 tests the rule). A title in `Display` (26 px) is 160.1, 196.5 and 229.5 px for 13, 16 and 19 letters:
about 12.2 px for each letter. `Edit` is 61 px wide and starts 8 px after the title. `Start fight` takes
about 104, `Add to Log` about 108, and its icon 28, each with a gap of 4 before it (estimates: these do not
exist yet). Lookup's reader is 320 px inside at case D and 406 at case B. The room after `Edit` is the width
inside, less the title, 8 and 61. A reader that scrolls has 8 px less (measured): then the 16-letter title
at case B shows `Start fight` alone, and every other cell holds. In Lookup a campaign creature has no `Edit`, and its art (64 px)
starts 8 px after the title, so it has about 3 px less room than an NPC with `Edit`. An entry that has both
art and `Edit` (`src/world.rs:1750-1753`) has about 72 px less; the plain campaign has none.

| Reader | Inside | `Captain Varra`, 13 letters (160 px) | `Oren Thistlewick`, 16 (197) | `Captain Varra Black`, 19 (230) |
|---|---:|---|---|---|
| Lookup at case D (measured) | 320 | 91 px: `Add to Log` as its icon | 54: the icon | 21: neither |
| Lookup at case B (measured) | 406 | 177: `Start fight`, and the icon | 140: `Start fight`, and the icon, with 0.4 px to spare | 107: the icon |
| The World page at 960x640 (estimate) | 512 | 272: both words | 237: both words | 202: `Start fight`, and the icon |

**Measured in P3** (the plain evening and `session-plan-scenes`): `Start fight` is 100 px wide. In Lookup at case
B it shows beside a 13-letter title and also beside `Captain Varra Black` (19 letters), so the estimate above is
low for wide titles; at case D it does not show beside a 13-letter title, and shows beside `Rook Ambry` (10). On
the World page at 960x640 it shows beside `Captain Varra Black`. A portrait sits at the right end of the title's
cell, with `Edit` after it, so beside a portrait and `Edit` no room is spare and `Start fight` does not show.
A creature with its type's art and no `Edit` (a package creature, or a campaign creature in Lookup) shows
`Start fight` between its title and its art when that room holds it, as the app does (corrected by the lead's
ruling after app round 1; seen with `Cultist` beside the encounter editor at 1200x800,
`encounter-build__06-stat-block-beside-editor`); beside `Cinder Wolf` in the plain campaign's Lookup the room
does not hold it. While the encounter editor is the page, Lookup's reader shows no `Start fight` at all: the
editor's own primary `Start fight` is the one there (app round 1, V19).

**Measured in P6** (`log-from-play`, `session-plan-scenes`, `npc-in-lookup` and the plain evening): `Add to Log`
is 103.5 px wide with its word and 28 as its icon. In Lookup at case B a 13-letter title (`Warden Ostrel`) shows
`Start fight` and the icon, as the table says; `Captain Varra Black` shows `Start fight` alone, and at case D
neither. Beside a 10-letter title (`Rook Ambry`) at case D, `Start fight` shows alone. On the World page at 960x640
and 1200x800, `Captain Varra Black` shows both words.

**The routes that stay** when the header does not show an action. `Start fight` is a line of the menu of a
scene row that links that NPC or creature: 2, at any width, on the Session page (F3). For any other NPC or
creature the World page's reader is wider: `World`, the entry, `Start fight` is 3 or 4, **T T**. `Add to Log`
is on a done scene's row (F3); or the DM uses `Edit`, types the line with its stamp under `## Log`, and
saves: 1 more than F6 counts.

### 1.3 Widths, and what never hides

The widths come from `RAIL_WIDTH` (136), `side_panel_width` (36% of the window, between 340 and 560) and the
Session page's `PLAN_BESIDE_NOTE_FROM` (720) and `TWO_COLUMNS_FROM` (640). Each wireframe names a case.

| Case | Window | Lookup | Page width | Session page | Run page |
|---|---|---|---:|---|---|
| A | 1200x800 | Closed | 1040 | Session list, plan (330) and note side by side | Roster 596, run panel 432 |
| B | 1200x800 | Open (432) | 608 | No session list. **Plan** and **Note** are tabs | Roster 596. Run panel hidden. Tiles above the dock |
| C | 960x640 | Closed | 800 | Session list. **Plan** and **Note** are tabs | Roster 442, run panel 346 |
| D | 960x640 | Open (346) | 454 | No session list. Tabs. The title has its own line | Roster 442. Run panel hidden. Tiles above the dock |

The party bar is a band above the page and above Lookup, so Lookup does not narrow it (`src/lib.rs:1492`). A
roster's width is the width inside its pane, which is 12 px wider (`GUTTER`, `src/encounter_run/view.rs:37`).

| Never hidden, in any case | Where it is | What gives first |
|---|---|---|
| That the current turn has reminders (F2) | The current row; the notice line while it has room | Section 1.4 |
| That something is due (F4) | The rail's `Due N`, on every page | Nothing |
| A scene's done box (F3) | The start of its row | The words wrap, as a plan row's title wraps today |
| `Undo` in a run | Run header, line two, right | Today's rule. No feature part takes its room (section 1.4) |

### 1.4 The run's notice line and a row's effect words

**The notice line** is the left part of the run header's second line (`live_header`,
`src/encounter_run/notice.rs:33`); `Undo` is at its right. Today it holds a state notice or `Next: <name>`,
and then the sheet-change summary. F2 adds one part. No tracker part is on this line (section 7).

| # | Part | Shows when | Words |
|---|---|---|---|
| 1 | As today: a state notice (a sync conflict first, recording silence, another sync notice, `No participants are present`; `notice.rs:168`, `:187`), or else `Next: <name>` | As today | As today: `Sync conflict: <name>`, or `Conflict: <name>` where that does not fit (`:118`, `:393`) |
| 2 | As today: the sheet-change summary with its `Log` link (`notice.rs:458-489`) | As today | As today |
| 3 | The current turn's reminders (F2) | The current participant has an effect with a time | `⚑ Wis save DC 15 (Hold Person) · Bless 9`, or the count `⚑ 3`, whose hover and accessible name hold each whole |

**Part 3 takes only the room that is spare.** Today the notice is measured first and `Undo` takes the rest;
a state notice stays whole, and `Undo` shortens its target beside it (`docs/design-system.md`, the live
run's header; `notice.rs:232-246`). Part 3 is measured after them:

1. It shows each reminder whole in the room that a state notice, part 2 and today's `Undo` label leave.
   `Next: <name>` gives way to it; the roster still marks NEXT.
2. Else it is its count.
3. Else it does not show. The current row has the reminders.

**Measured in P4.** Beside a state notice, `Undo` keeps a target only while its names stay whole, so a long
step shows the bare `Undo` and leaves room: at case D, with `⚠ Sync conflict: Oren Thistlewick  Review` and
three reminders, the count `⚑ 3` shows between `Review` and `Undo` (`run-sync-conflict`). With the panel at
960x640 and `Sync off for the party`, part 3 has no room, and the row has the reminders. The tests of F2 are
T20.1 to T20.6 in `docs/ux-intent.md`, because F8 has T17.

So no feature makes `Undo` shorter, and no state notice loses a word. One repair comes from rule K: when a
conflict's name is wider than the line beside the bare `Undo`, the name shortens by the header's rule for its
names (pull request 286 owns it); the icon, `Conflict` and `Review` stay (section 2.5, row 20). The dock's
selected name follows the dock's own rule, which keeps a telling last word (row 33).

- **Sums at case D** (estimates; T17.4 tests the rule with a short and a long name). The roster is 442 px.
  Beside Lookup `Undo: next turn` has no key letter and takes about 145 px, with a gap of 16. With no state
  notice the line has about 281: `⚑ Wis save DC 15 (Hold Person)` (about 191) fits whole, also with `· Bless 9`
  (254); three such reminders are `⚑ 3`. With `Sync conflict: Oren Thistlewick  Review` (about 268), 13 px
  are left, and part 3 does not show.

```text
Case D: a conflict, and three reminders that start with the same words
┌────────────────────────────────────────────────────────────┐
│ Round 3 · Cinder Wolf 2                      [ Next turn N ]│
│ ⚠ Sync conflict: Oren Thistlewick  Review ↶ Undo: next turn │
├────────────────────────────────────────────────────────────┤
│▶ Cinder Wolf 2                                     HP 20/26 │
│  Wis save DC 15 (Hold Person) · ⚑ +2                        │
│  Oren Thistlewick                                  HP 45/45 │
│  Frightened 9 · Conflict                                    │
└────────────────────────────────────────────────────────────┘
```

**A row's effect words (rule P).** An effect never adds a line or height: a row has the lines and the height
that the same row has with each time taken away, where each effect is its plain words (the ruling on pull
request 276, X5 after app round 2). Its second line ends before the HP column (`view.rs:6430-6446`): it has
about 260 px at case D, and about 285 at case A, where the row has a state column.

| Rule | What it means |
|---|---|
| Timed effects take the place of the conditions in today's order, before plain conditions. Every other word of today keeps its place: `No initiative`, `Bloodied`, `0 HP`, `Absent`, `Conc`, the side's word, `as <name>`, `Changed` with its bonuses, `Loot` and the sync word (`view.rs:6477-6583`) | Plain conditions fold into `N more`, as today. With `Bloodied` before them: `Bloodied · Wis save DC 15… · ⚑ +2` |
| A count has a slot as wide as the widest of its three forms at the row's scale and font, with the warning icon of `time is up`: about 75 to 98 px. P4 measures it when the effect or the width changes | A change from `9` to `last round` to `time is up` moves no word |
| An effect shows whole while the line has room. The first effect shortens its words from their end (`design::words_that_fit`) when it alone is too wide. The effects that do not fit are one count, `⚑ +2`. When no effect's words fit, the row shows the count of all its timed effects, with no plus: `⚑ 3`. When that count does not fit on the row's lines, the row shows the effects as their plain words, as the row with no time does. On the row that has the turn, an effect shown on each turn comes first and keeps its first words before the last plain condition folds (W1-D1). The count of the rest follows the last slot after a space, with no dot (X10) | At case D, with no other word, the line holds one long reminder and a count, or one counted effect and a count. `End Wis save DC 15 (Hold…  2` keeps its place through every count |
| `Conc save DC 12` takes only the room that the timed effects leave at their slot widths | Else it is in the hover and the accessible name |
| The whole words | The hover and accessible name of the line; **Conditions**, which lists each effect; and the notice line on that participant's turn, while they fit |
| A row with no timed effect | As today |

## 2. The evening, end to end

Each journey uses the step fields of `artifacts/workflow-score/journeys.md`. **Count** uses the kinds of the
audit: `Nav` (go to a page), `Sub` (a tab in a page), `Panel` (open or close a side panel), `Find` (search or
select), `Do` (the work). A page transition is marked **T**. "Must remember" is **nothing** unless a row
says so. Every count is a hand count.

### 2.1 J1: the measured evening

Window 1200x800 (case A, and case B while Lookup is open). `steps.tsv` has 41 tasks. Thirty-seven keep their
actions and counts. F1 only adds its word to lines that the DM reads when they build the
encounter and plan the fights. These four change:

| `steps.tsv` task | Surface | Action after | Count after | Before | File |
|---|---|---|---|---:|---|
| A player asks who Captain Varra is | Session plan, the scene's row; Lookup opens (case B) | Click `Captain Varra` in the row; close Lookup | Panel 2 | 3 | None |
| A wolf frightens Oren for 10 rounds (F2) | Run, dock | Select Oren; `E`; type `Frightened 10 rounds`; Enter | Find 1, Do 3 = 4 | 3, with no count | The run file: the condition with its count |
| A fight nobody planned, with a Bandit from the book, then talk (F3) | Session, Lookup, Run | Ctrl+K; type `bandit`; `Start fight`; `Close run` and `Close now`, as measured; close Lookup | Panel 2, Find 1, Nav 3 = 6, **T T** | 9, **T T T** | An encounter `Fight with Bandit` stays in **Ready to run**, as the measured task's encounter stays today |
| To link the fight in the note | | Nothing. `Add rewards` writes the fight's link | 0 | 2 | |

- **Why the second row costs one more:** the DM types the number of rounds, and no design can take a number
  with no interaction. A DM who wants no count uses **Conditions** as today, for 3.
- **The stray encounter** of the third row can go with `Remove…`: 4, with 2 page transitions, from the
  Session page (F3). Today it cannot go in Splat at all. A later `Start fight` with the same creature starts
  this encounter, so no second one is made.

**Seven records that were on paper, or had no route.**

| The DM wants | Action | Count | File |
|---|---|---|---|
| **F5.** What the last session left | `Carry over` | Do 1 | The new note: one embed line, the plan lines, the open threads, `carried_from`, `carried_parts` |
| **F3.** To plan a talk with Captain Varra and a secret | Click under `## Plan for tonight`; type `- [ ] [[Captain Varra]]`; type `- [ ] The miller is the informer` | Do 3 (by the pointer, with `Scene` in **More**: 7) | The note: two task lines. The plan shows two scenes |
| **F3.** To mark the talk done | Click the box | Do 1 | The note: `- [x] [[Captain Varra]]` |
| **F9.** To check for an encounter on the road | The row's menu; `Roll for encounters`; `Roll` | Find 1, Do 2 = 3. With several marked tables and no earlier choice, 2 more, once | `Timeline.md`: the table's key and one check line |
| **F7.** The first fight's rewards, with XP | `Add rewards`. The notes have no rewards yet, so it writes the XP line and says how to stop it | Do 1 | The note: a block under `## Loot and rewards` |
| **F7.** The second fight's rewards | `Add rewards`. It follows the first block | Do 1 | A second block. The result says `XP tonight` |
| **F6.** To put what Varra said in her note | `Add to Log` on the done scene's row, which opens Lookup with the Log field; type; Enter; close Lookup | Panel 2, Do 2 = 4 | `NPCs/Captain Varra.md`: one line under `## Log` |

Both fights' titles leave room for `Add rewards` on the row at 330 px. A long title costs 1 more: open the row.
At 960x640 with Lookup open the counts are the same, with one exception: the note is behind its tab, so the
typed scenes cost 1 more, and 1 to come back to the plan.

| J1, by hand against `steps.tsv` | Transitions | Interactions | Nav | Sub | Panel | Find | Do |
|---|---:|---:|---:|---:|---:|---:|---:|
| `steps.tsv`, 41 tasks | 13 | 231 | 14 | 7 | 12 | 49 | 149 |
| The same tasks, after | 12 | 226 | 13 | 6 | 14 | 47 | 146 |
| The same, when the DM opens **More** before `Travel` and before `Short rest` | 12 | 228 | 13 | 6 | 14 | 49 | 146 |
| Seven new records | 0 | 14 | 0 | 0 | 2 | 1 | 11 |
| **After, in all** | **12** | **240, or 242** | 13 | 6 | 16 | 48 or 50 | 157 |

- **The More clicks and the audit script.** `steps.tsv` counts no **More** click, and the script clicks
  **More** only when `Travel` or `Short rest` is not in view. Its task for a played fight clicks an `Add to
  notes` that the app no longer has (`src/sessions/plan.rs:736`). So this document gives no total for
  `main`: P1a repairs the script, freezes it, and measures `main`. P1a measured 17 transitions and 240
  interactions in 42 tasks; `artifacts/dm-audit/README.md` explains each difference from 13 and 231. The
  played fight's link now goes by the Timeline's `Copy link` and a paste in the note: 6, **T T**.

### 2.2 J2: the improvised evening

Only the steps where a feature acts, at case A. Setup and turns cost what they cost today on both sides.

| # | The DM wants | Surface | Action | Count | Today | File |
|---|---|---|---|---|---:|---|
| 2 | **F3.** To play the planned NPC | The scene's row | Click `Captain Varra`. Lookup opens on her page | Panel 1 | 2: Ctrl+K, type | None |
| 3 | **F3.** The party attacks her | Lookup, her reader; then Setup | `Start fight`; close Lookup, to see her stat block | Nav 1, Panel 1 = 2, **T** | 8, **T T** | An encounter note `Fight with Captain Varra`, and the run file, as the editor's `Start fight` saves them today |
| 4 | **F6.** She gave up, and the party holds her | Ended view; Lookup | Ctrl+K, which shows her page again; `Add to Log`; type; Enter; close Lookup | Panel 2, Do 3 = 5 | 6: Ctrl+K, `Edit`, click the text, type, `Save changes`, close | `NPCs/Captain Varra.md`: one Log line |
| 5 | **F7.** To close, keep the rewards, and add loot nobody planned | Run, then the plan and the note | `Close run`; open the row (its title is long); `Add rewards`; click under the new block; type `Varra’s signet ring` | Nav 1, Find 1, Do 3 = 5, **T** | 4: Notes, type, close Notes, `Close run`. The words go under another heading | The note: the fight's block, and one line under it |
| 6 | **F9.** Two days overland | The Travel row | `More Add actions`; `Travel`; the row's menu; `Roll for encounters`, which puts focus in **How long**; type `2 days`; `Roll all 2` | Find 2, Do 4 = 6 | 12, **T T** | `Timeline.md`: the length, the table's key and two check lines |
| 7 | **F9.** The second day's result names Wolf Pack | The Travel row, the plan, then Run | `Plan “Wolf Pack”`; `Start fight` on the new row, which is the first to come and the primary | Do 1, Nav 1 = 2, **T** | 3, **T** | The note: `- [[Wolf Pack]]` above the skipped fight. The run file |

| J2, these steps | Transitions | Interactions | Nav | Panel | Find | Do |
|---|---:|---:|---:|---:|---:|---:|
| Today | 6 | 35 | 6 | 8 | 7 | 14 |
| After | 3 | 21 | 3 | 4 | 3 | 11 |

- **Today's 35 leaves a smaller record:** the loot is prose under another heading, and neither road roll is
  saved.
- **A wrong click** on `Start fight` costs the close, as today (`Close run`, and `Close now` while the run is
  not saved yet). The encounter stays, and a later `Start fight` with her starts it again. `Remove…` takes it
  away for 4, **T T** (F3).
- **Step 3 with no stat block** costs 3: `Start fight`, her HP, Enter. A wrong click there costs Ctrl+K more.
- **At 960x640 the walk is 24:** step 3 uses the scene row's menu (1 more), and the note is behind its tab,
  so `Note` comes before step 5's typed line and `Plan` before step 6 (2 more).
- **The 21 assumes** the note in Write, and an NPC with no portrait. With a portrait, step 3 uses the scene
  row's menu at 1200x800 too (section 1.2).

### 2.3 J3: the broken evening, at 960x640 with Lookup open (case D)

The start: fight 2, round 2. Lookup shows a session note found by the search `miller`. Sync is on. A tracker
has one notice due from before the fight, so the rail says `Due 1`. The current and selected row is Cinder
Wolf 2.

| # | What happens | What the DM sees | Recovery | Count | File |
|---|---|---|---|---|---|
| 1 | The DM ends the fight by mistake | The ended view, 442 px wide. The header says `Undo`; its hover and accessible name say `Undo: end encounter` | Click `Undo`. `U` is off while Lookup is open. Play returns to the same turn | Do 1 | The run file is at round 2 again |
| 2 | The DM pins `Poisoned 3 rounds` to the wolf (`Effect`, type, Enter). The app stops in round 3 | After a restart the app opens on the run, on the current turn. The row says `Poisoned 2`. `Undo` names the last saved step. The rail says `Due 1`, read from the ledger | Nothing for the run | 0 | The run file holds the effect, its count and the Undo steps |
| 3 | Lookup is closed after the restart (a limit, section 7) | The run, with no Lookup | Ctrl+K; type `miller` | Panel 1, Find 1 = 2 | None |
| 4 | A sync conflict opens on Kestrel Dunmoor, on the wolf's turn | The notice line: `⚠ Conflict: Kestrel Dunmoor  Review`, and `Undo` with its target, as today. Part 3 has no room there (section 1.4); the wolf's row says `Poisoned 2` | `Review`; choose. **Review** closes Lookup and selects Kestrel, as today. Then: select the wolf; Ctrl+K, which shows the results of `miller`; click the note again | Do 2, Find 2, Panel 1 = 5 | The run file |
| 5 | The DM edits the session note in Obsidian: a tick, a new plan line | On the Run page nothing opens. Splat held no unsaved words, so it takes the file's text within a second | Nothing | 0 | The note is as Obsidian wrote it |
| 6 | The same, while Splat holds unsaved typed words for that note | The rail says `Notes changed`. A plan act says `The note changed on disk. Choose in the note first.` and does nothing | Ctrl+J, which closes Lookup; one choice; Ctrl+K; click the note again. It keeps one version of the lines that both sides changed (a limit of today) | Panel 2, Find 1, Do 1 = 4 | The note holds the chosen version |

| J3, recovery | Transitions | Interactions | Panel | Find | Do |
|---|---:|---:|---:|---:|---:|
| Steps 1 to 5, the clean case | 0 | 8 | 2 | 3 | 3 |
| Today with pull request 232, the same steps | 0 | 8 | 2 | 3 | 3 |

Today has no count on `Poisoned` and no reminder. Step 6 adds 4. After Ctrl+K, Lookup shows the results
of its search again, and the found note needs one more click (`src/lookup.rs:1533-1538`, `:1575-1580`). The
notice stays due through the fight: the
rail says so, and the DM finishes it after the fight (section 7). Nothing in this journey is modal, and
nothing takes focus from `Amount`.

### 2.4 J4: the race, between fights

The Tournament of [the example](world-examples/heralds-tournament.md), with its note as it is and one
addition that P9 makes: the bells' rival table gets `for:`. **Trackers are used between fights only**; in a
fight the run shows no tracker part, and the owner accepted this cost (section 7). The start: Session page, Lookup closed, the
clock at 30, Favor 2, no seals. **Paper** is the same race on the paper sheet: each mark, tick, note or
erasure is one act (**p**), and each die that a table needs is one die (**d**).

| # | The DM wants | Action in the design | Count | Paper |
|---|---|---|---|---|
| 1 | To open the numbers | Ctrl+T. Trackers stays in view beside the Session page | Panel 1 | 0. The sheet is on the table |
| 2 | The party enters a room. The clock reaches 40, and a bell is due | `Enter room`. The bell's Due row shows above the trackers | Do 1 | 1 p |
| 3 | The bell: the check for wandering, a roll for each rival, the wandering table | A real d6; on the Due row `Roll Rival Track` three times and `Roll Wandering` | Do 4, and 1 d | 4 p, and 5 d |
| 4 | To apply the bell's result: the Old Guard takes its first seal | `Take seal` on the Old Guard's line, while the result is still listed. A notice is due: a rival uses its 1-2 seals group | Do 1 | 1 p |
| 5 | To finish the bell | `Done` | Do 1 | 1 p |
| 6 | To keep that in mind for the next fight | Nothing. The notice stays due, and the rail says `Due 1` during the fight | 0 | 1 p, a note |
| 7 | The party enters the arena | `Enter room` | Do 1 | 1 p |
| | The fight: three rounds. The DM adds the rival group with **Add to fight**, as today. No tracker act | | | The same |
| 8 | Three minutes passed in the fight | Ctrl+T, because **Add to fight** closed Lookup; `Fight minute` three times. The ended view says the round | Panel 1, Do 3 = 4 | 3 p |
| 9 | The rivals came | `Done` on the seal's notice | Do 1 | 1 p, the note struck |
| 10 | The crowd loved a move, then threw a gift | `Crowd loves it`; `Throw a gift`, which also rolls the gift table | Do 2 | 2 p, and 1 d |
| 11 | A wrong press on `Throw a gift`, seen after the next right press | `Throw a gift`; `Crowd loves it`; then the opposite action, `Crowd loves it` once more | Do 3 | 3 p: two marks and one erasure |
| 12 | A soul is collected. The clock passes 60: the second bell | `Collect soul`, which changes the clock and the jar | Do 1 | 2 p: the clock and the jar |
| 13 | The second bell | A real d6; three rival rolls; the wandering roll; `Done`, after the DM applied the results | Do 5, and 1 d | 5 p, and 5 d |

| J4, between fights | Transitions | Acts | Panel | Do | Dice |
|---|---:|---:|---:|---:|---:|
| Paper | 0 | 25 | 0 | 25 p | 11 |
| After | 0 | 25 | 2 | 23 | 2 |

- **The count is not above paper:** 25 against 25, and 2 dice against 11. Two acts open the view. The design
  saves two marks: the note of step 6, because the notice is the note, and one of the two marks of step 12.
  Paper's 25 holds nine notes (four results at each bell, and step 6); a DM who writes no note makes 16
  marks. With each die as an act it is 27 against 36, or against 27.
- **In a fight the DM marks the paper sheet.** Ctrl+T still opens the view (three acts for each press), and
  the run offers nothing more. After the fight the DM enters the fight's marks, as steps 8 and 10 do; a mark
  that those raise shows then.
- **A wrong press** that the DM sees late is repaired by the opposite quick action, as an erasure on paper
  (step 11); the history keeps both presses.
- **At case D** the counts are the same while at most one short notice is due. While a bell's results are
  listed, a tracker below the clock needs a scroll: 1 more in step 4.

### 2.5 J5: the plain evening, and the contract

The DM of J1 uses no new control. The audit's script runs unchanged and must give the counts that it gives
before each feature (G6). One frozen plain campaign must give captures that differ from the reference only
by the rows below (G7, section 5.3).

**The contract** (rows 1 to 10, 12 and 13). Each row is passive words or a quiet action (rule 2). Each names
the band that it joins
today and its fit rule. No row adds a line, changes a row's height or takes width that a title or a name uses.

| # | Screen | What differs | Band today | Fit rule |
|---|---|---|---|---|
| 1 | A planned fight's row, on the plan and in **Ready to run** | The summary ends with one word, such as `· Low` (F1), when the XP, the levels and the budget table are known | The summary: `src/sessions/plan.rs:795`; `src/encounters/list.rs:553-558` | Only when the summary keeps its lines and words |
| 2 | An opened planned fight | The line of `Read encounter` ends with `· Low · XP 550 of 750` (F1); with no word because an input is not known, with `· XP not known: <name>` (W1-D2). The words start at the link's right edge, so each `·` has the same blank on each side (V9) | `plan.rs:832-835` | The line holds one link today. The word goes first, then the numbers, as F1's table says; the words show only when the whole fits on the line |
| 3 | The encounter editor | `· XP 550 of 2,500 · Low` (F1) at the end of the `Roster` heading, or of the tab row below 760 px | `src/encounters.rs:4381`; `:3932-3955` | One line. The numbers go first, then the word |
| 4 | Arrivals | A group's XP is followed by `→ Moderate` (F1) | `src/encounter_run/view.rs:3468-3520` | Only where the totals and the status words are in the heading today (`:3479-3492`) and stay there |
| 5 | A row marked `Conc` that takes damage | The row's second line ends with `Conc save DC 10` until the next step (F2) | `view.rs:6476-6486` | Only when the line keeps its lines and folds nothing that shows today. Else in the hover and accessible name only |
| 6 | Run dock, line two | A quiet `Effect`, `L` after **Absent** (F2). P4 measured that the line is laid out from the left, so a control left of **Conditions** moved **Conditions**, **Conc** and **Absent** in every view; the spare room is after **Absent** | `dock.rs:80-172` | F2: the word, then the icon, then not shown, before the name loses a word or its size |
| 7 | The Add strip's **More** menu | Its last line is `Scene` (F3) | `src/design/components.rs:2339`; `src/timeline/page.rs:89` | A menu line. The strip's visible actions are as today |
| 8 | A campaign World entry's reader | `Add to Log` after `Edit`, and after `Start fight` where it shows (F6) | The header: `src/world.rs:1733-1768` | Section 1.2: its word, then its icon, then not shown |
| 9 | A campaign creature's or NPC's reader, with no run open | A quiet `Start fight` after `Edit` (F3) | The same header | Section 1.2: with its word, or not shown |
| 10 | An ended fight's plan row | `Add rewards` (F7) | The action band: `plan.rs:692-694`; the opened row's links: `:1313-1321` | On the row only when the title keeps its lines beside the action's own width. Else the last link of the opened row, after `Timeline`. A fight whose reference an old note already has as a list line says `In notes` |
| 8 | A campaign World entry's reader | `Add to Log` after `Edit` (F6) | The header: `src/world.rs:1733-1768` | Section 1.2: its word, then its icon, then not shown |
| 9 | A campaign creature's or NPC's reader, with no run open | A quiet `Start fight` after `Add to Log` (F3) | The same header | Section 1.2: with its word, or not shown |
| 10 | An ended fight's plan row | `Add rewards` (F7) | The action band: `plan.rs:692-694`; the opened row's links: `:1313-1321` | On the row only when the title keeps its width and its line breaks beside the action's own width. Else the last link of the opened row, after `Timeline`, while that line has room. Else the last line of the row's menu (measured in P7: the links with `Add rewards` take about 326 px, and the plan at 330 px gives them 296). A fight whose reference an old note already has as a list item says `In notes`, also in the menu |
| 12 | A Travel row's menu | One more line, `Roll for encounters`, when the campaign has a marked table (F9) | `plan.rs:727-735`; `src/timeline/page.rs:875-877` | A menu line |
| 13 | A **Ready to run** row's menu, for an encounter with no saved run | One more line, `Remove…` (F3) | `src/encounters/list.rs:560-565` | A menu line |

**Ruled exceptions and repairs.** These are not of the two kinds. Each comes from a ruling, and each is a row
that G7 allows.

| # | Screen | What differs | From |
|---|---|---|---|
| 11 | The **Last time** line | `Carry over` at its end, while the session has no event and no run and the last session holds a part to carry (F5). The preview is cut to its room today (`page.rs:4106-4115`), so the action takes 81 px (measured in P8) and an `L` gap from it. The preview shortens first, to nothing; then the link's words shorten; nothing wraps | Round 3: the one ruled exception to rule J |
| 14 | The plan, when the DM's note has a task line under `## Plan for tonight` | The line is a scene: a row with a box, counted in **Tonight**, and not repeated in Read (F3). When the plan's rows do not fit its pane, the scenes fold to `N scenes` on the **Tonight** line, so no other row moves or narrows and the next fight's action stays whole and in view (F3, the fold). The measured evening has none | The ruling on scenes: it is the DM's own plan |
| 15 | The **Conditions** tab's `Other effect…` field | Words that end with a time, a count of uses or a recharge are read as F2 says: `Bless 10 rounds` counts down, and `Frightened 10 rounds` becomes that condition with a count. The field, its label, its `Add` button and its place are as today | Round 3: the field stays, so `Effect` has a way when the dock has no room |
| 16 | The **Last time** line | Its preview reads the last session's `What happened`. Today it reads that note's own recap first (`page.rs:915`) | Rule H: the preview and the carry-over read one source |
| 17 | Lookup, while it holds an unsaved generated NPC | An entry that opens shows under one row, `Unsaved NPC: <name> · Show`. Today the card hides it (`src/lookup.rs:2167-2169`) | The accepted repair of Lookup |
| 18 | Read, for an embed of a session note's heading | The heading's text shows under a muted line that links the session (F5). Today Read shows the embed as a link (`src/world.rs:380-390`) | Round 2, E30: draw by the target |
| 19 | A **Change** of status, place or group | It writes its Log line when a session is being played by W14's test. Today the test is "dated today" (`page.rs:1929-1937`). So a change during a fight or a recording on a note with another date, and a change in prep for a session dated later, now write a line | Rule M: one definition of the session being played |
| 20 | The run's notice line, when a conflict's name is wider than the line beside the bare `Undo` | The name shortens by the header's rule for its names; the dock's name follows the dock's rule (row 33). Today nothing gives | Rule K |
| 21 | A package creature's reader, with no run open | A quiet `Start fight` in a header that holds no action today (F3) | Rule Q |
| 22 | The **Last time** line | When the last note has no prose under `What happened` and no processed recap, the preview is the note's first written line outside its title and date, `Recap of last time` and `Plan for tonight`. Today the line shows the link alone (`recap_preview`, `src/sessions/page.rs`) | The lead's ruling W5-D5, from the first scored round on the app: the measured evening's two reads of last week cost 2 each because the line showed none of last week's words |
| 22 | History on the Encounters page, after `Whole run` or `Open` | The list scrolls once, at that act, so the opened run's row is at its top and its outcome, the line under Read-only and the whole Final state are in view below it (V31). The line under Read-only is `S` below it and `M` above `FINAL STATE` (V11), so what is under Read-only moves up by at most 8 px more, and the Read-only line's words are as tall as a line. Otherwise only the list's scroll offset differs: what it holds is as today, and nothing moves sideways, is cut or is resized | By the lead's ruling after app round 1 |
| 23 | A scene's row on the plan | Its words start at the fights' title column (12 px further left) and are drawn at the plan row title's size, `Body`, at every window size; its box's 24 px target ends at that column; its second line sits at a fight row's pitch (V2, V8). So a scene row's height changes, and every item below a changed scene row moves straight up by the sum of the height changes of the scene rows above it; nothing moves sideways, changes size, is cut or goes. A done scene's `Add to Log` (row 8's feature on the plan) is on its row where its words now keep their rows beside it. A done scene's words are muted (V1), which G7 does not measure | By the lead's ruling after app round 1 |
| 24 | The **Tonight** line, with the scenes folded | `N scenes` has the section caret of a folded section before it, as `HISTORY` has (V14): the link is wider by the caret, at its place and height | By the lead's ruling after app round 1 |
| 32 | Run dock, line two | `Effect` (row 6) is decided by the room after **Absent** alone, so it shows also beside a selected name that its slot cut (X9). Nothing of today's dock moves; an act shows or hides only `Effect`'s and rows 33 and 34's words | By the lead's ruling after app round 2 |
| 33 | Run dock, the selected name | A name that its slot cuts keeps its last word after its first whole words, `Ashfang, Eldest… Ridge` for `Ashfang, Eldest Wolf…`, at the same place and height in the same slot (X53). The run header's names, the notice line's and Undo's are as today | By the lead's ruling after app round 2 |
| 34 | The **Conditions** tab | Its help says `No HP, AC or roll changes by itself. A time you type counts down on that creature’s turns. Other effects stay local and do not sync.` at its place, and may take one more line below its first (X7). Where the dock has no room for `Effect`, the field's words are `Other effect, such as Bless 10 rounds` (X8) | By the lead's ruling after app round 2 |

Nothing else differs. A free-words effect from an encounter or an older run gives no reminder. A Travel row
shows no checks and no new field until the DM asks. The rail says nothing about trackers unless a notice is
due. The empty plan's words (`plan.rs:901`), the run's **Keys** list, the loot editor's table menu and
Lookup's `Back` are as today. No file changes that the DM did not act on. F8 is the app's baseline once pull
request 232 merges.

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
route for steps 5, 7 and 9; the steps that it has cost 20 and leave less record. HP carries inside one
session, as today (`src/lib.rs:328`, `src/timeline.rs:300`): the new session's first fight starts from the
sheets, and a paused fight that the DM resumes keeps its own HP. At 960x640, step 3 goes by `Edit` beside a
19-letter title (1 more, section 1.2), and
step 8 needs `Note` and `Plan` (2 more). The counts assume the note in Write. Nothing is typed twice.

## 3. The features

Each part has the same order, and a table at its end gives what a reviewer checks on paper. Every placement
follows the fit rule (rule 2), every write follows section 4.1, and **a state that a
table does not list is as today**: sync, a run that is open, a file that changed on disk or cannot be saved.

The examples of F1 use one party: 5 characters at level 3. Their budgets are Low 750, Moderate 1,125 and High
2,000.

### F1. Encounter difficulty

**Question.** "Is this fight too much, or too little, for this party?"

**Where the numbers come from.** The rule entry
`dnd2024:rules/combat_encounters_step_2_determine_your_xp_budget` has a typed table at
`splat_rule/parameters/per_character_xp`, with `level`, `low`, `moderate` and `high` for levels 1 to 20. Splat
reads it from the rules that the campaign uses, as it reads the inspiration rule (`xp_budgets` and
`rules_in_use` in `src/encounter_settings.rs`): the only table, or of several the only one in the campaign's own
notes, so a house rule's copy takes the package's place. No table is written in Rust.

**One computation** (new module `src/difficulty.rs`, one function, every screen calls it).

- **The party:** the characters on the party's side in the roster; in a run, those who are not absent. Only a
  roster with no character at all uses the campaign's party, which is every party copy of the campaign,
  also one whose note cannot be read; a run whose characters are all absent has no party, and shows no word.
  An ally on the party's side counts for neither side.
- **A level:** only from the saved sheet now (`splat_character.level` in the party copy's note, by its ID). A
  sheet that is gone, cannot be read or has no level is a level not known, and the word does not show. A
  run's copy of a sheet from when it started says who is a character, never their level.
- **The budget:** for each difficulty, the sum of the table's value for each character's level.
- **The XP:** the sum for every creature that is not on the party's side (in a run, every present one), from
  the function that groups use (`creature_xp`, now in `src/difficulty.rs`). It can have no answer for a
  creature.
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
- In Arrivals the word is what the fight becomes when the group joins. The group's drag handle keeps its
  accessible name, and its accessible description says `Adds 400 XP; the fight becomes Moderate`: the word is
  painted text, and a scenario finds the handle by its name.
- **A summary whose last line has no room** shows no word. The DM opens the row, by a click or Enter on its
  title, and the word ends the line of `Read encounter`. That is 1 `Find`, with the keyboard.
- It does not show on a running or played fight's row, on Setup's roster, in History or on Timeline.
  Arrivals shows it in Setup too.

```text
Case A, the plan at 330 px
▸ Bridge Ambush                  [▷ Start fight]
  5 party · 2 × Ember Cultist · Low
▾ Mill Road                      [▷ Start fight]
  5 party · 2 × Ember Cultist ·
  1 × Cult Fanatic · High
    Read encounter · High · XP 1,150 of 2,000
```

**Exact words.** `Low`, `Moderate`, `High`, `Above High`. `XP 1,050 of 1,125`. The hover lines above.

| State | What shows |
|---|---|
| The XP, every level and the budget table are known | The words above |
| A creature has no XP and no CR; a level is not known; there are no sheets; the rules have no table; there is no creature against the party | Nothing. The line is as today |
| A creature's XP or a character's level is not known, and the party and the table are known (by the lead's ruling after app round 1, W1-D2) | The summary is as today. The opened row's line says why, in `muted`: `Read encounter · XP not known: Nameless Thing`, or `Level not known: <name>`; with more names, the first and `and 2 more`. The names are in `Scale::weigh`'s typed answer (`Difficulty::missing`) |
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

**The row is the place to act. The notice line only repeats the row's words** (section 1.4).

**Where the DM types it: one field, in two places that exist.**

- **The Conditions tab** has the field today (`Other effect…`, `src/encounter_run/view.rs:2421-2445`). It
  stays, with its label and its place. It now reads a time when the DM types one.
- **The dock** gets the quick way, `Effect` (key `E`): a quiet action on the dock's second line, `L` after
  **Absent**. That line is laid out from the left (the name, `L`, Conditions, Conc, Absent), so the spare room
  is after Absent, and there `Effect` moves no control (measured in P4; an earlier draft put it left of
  Conditions, which moved the three controls after it). **The fit rule for `Effect`, stated once:** the room after Absent alone decides (by the
  lead's ruling after app round 2, X9): it shows with its word while that room holds it, also beside a name that
  its slot cut; with its icon while the room holds the icon; and not at all otherwise, where the Conditions
  field says `Other effect, such as Bless 10 rounds` (X8). A roll's total keeps its place first. `Death saves`
  is first on the action tiles' line above the dock, so it moves no control of the dock and takes no room from
  the name (X65). With several rows selected, the line is one sentence today (`target_words`, `dock.rs:197-213`) and has no
  `Effect`. The field in the Conditions tab is then the way, as today: `E` opens the tab with focus in the
  field, for 4. By the pointer with Lookup open it is 5 (select, **Conditions**, the field, the words, Enter),
  and Ctrl+K shows Lookup again.
- `E` shows on the button while the key works, as each dock key does. The run's **Keys** list does not change.

**Wireframe: case A, on Oren's turn in round 3.**

```text
Round 3   Oren Thistlewick’s turn                    [+ Add to fight]  [> Next turn  N]
⚑ Frightened 9 · Wis save DC 15 (Hold Person)                        [↶ Undo: next turn  U]
──────────────────────────────────────────────────────────────────────────────
 16 ■  Oren Thistlewick                    AC 18           45/45          ← current row
 TURN  Frightened 9 · ⚑ +1
…
[Amount] [Damage  Enter]  Heal  🛡 Temp HP                      [1d20] [Roll]
Oren Thistlewick   Conditions C   ✧ Conc   Absent   ⚑ Effect E
```

**The dock while the DM adds an effect.** Line one does not change. Line two becomes one field.


- **Enter adds the words, closes the field and puts focus in `Amount`.** Escape clears and closes the field
  and puts focus in `Amount`, as the loot field's Escape clears today (`src/loot_editor.rs:533`).
- **The field belongs to the row it opened for.** Its placeholder names that row.
- **While the field has focus, a letter is text.** A click on `Next turn` still advances the turn. The field
  then closes, and its words wait for that row.

**Case D (the roster is 442 px inside, and the run's panel is hidden).** The notice line and the row are the
wireframe of section 1.4. The dock:

```text
Dagger +6   Fire Bolt +7                       ← the tile line of today
[Amount] [Damage] Heal 🛡Temp HP        [1d20] [Roll]
Cinder Wolf 2   Conditions  ✧ Conc  Absent  ⚑
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
- **A recharge roll** uses the run's dice. The Log says `Rolled 5 · ready again` or `Rolled 2 · still spent`.
  The die shows in the dock's dice total place, named `Rolled 2 for Cinder Drake: Fire Breath`, and the run's
  footer says `Rolled 2 for Cinder Drake: Fire Breath still spent.` until the DM's next act: any step, roll or
  Undo (X4, by the lead's ruling after app round 2).
- **Concentration.** When a row marked `Conc` takes damage, the row's second line ends with `Conc save DC 12`
  until the next step: 10, or half the damage if that is more, and at most 30. With no room on that line, the
  words are in the row's hover and accessible name only. Splat does not roll the save or clear `Conc`. It
  reads the
  damage from the newest Undo step, so the run file gains no field.

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

**What Splat reads at the end of the words.** `10 rounds`; `1 minute`, as 10 rounds; `on its turn`; `3 uses`;
`recharge 6`, `recharge 5–6` or `recharge 5-6`. Anything else stays words: `Mage Armor 8 hours`.

**Exact words.** `Effect`. Dock field: `Effect on <name>, such as Bless 10 rounds`. On the row and the notice
line: `Bless 9`, `Bless · last round`,
`Bless · time is up`. The Conditions tab's help says `No HP, AC or roll changes by itself. A time you type
counts down on that creature’s turns. Other effects stay local and do not sync.` (X7); an official condition's
count is on its tick, `✓ Frightened 10`, and Other effects lists other words only (X11).

| State | What shows |
|---|---|
| No effect with a time | The notice line and every row are as today |
| A sheet removes a condition that has a count | The condition and its count go together. The sync notice and the Log line name what went: `Frightened, 7 rounds left`. A sheet's change is not an Undo step, as today |
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
Undo. So a count cannot outlive its condition. P4 keeps one entry type and one read rule, and it changes every
reader
of `conditions`: sync, Setup, the summaries, the party bar (`party_bar::Glance.conditions`) and the saved Undo
steps of pull request 232 (`Step::weight`, the row check). An old label stays a label. P4 proves it with a
run file saved before it and with one step that it cannot read. The row's rule of section 1.4 is in the
shared row layout, not in the current row's painter.

**Undo.** Each act is one named step of the run's Undo, which the run file keeps (F8): `Undo: effect Bless on
Oren`, `Undo: Fire Breath spent`, `Undo: recharge roll for Cinder Drake`, `Undo: remove Bless from Oren`. The
Log has a line for each, and a line when a count reaches 0.

**Keyboard.** `E` opens the dock field for the selected row; when `Effect` has no room, it opens **Conditions**
with focus in its field. Enter adds. Escape closes. The targets on a row are Tab stops, as condition words
are, and Enter on one is a click.

**What stays with the DM.** When an effect starts and ends. Every save. Whether a use was spent and when it
comes back. Whether concentration holds. Splat counts, shows and rolls when asked.

**Not built, and why.** A time on words typed in prep (no old text starts to count). A reminder for uses or a
recharge. Suggestions in the dock's field, and one effect for several rows (cut).

| Check | F2 |
|---|---|
| Primary action in view | `Next turn`, unchanged and alone |
| Components and tokens | `Effect`: `Action::quiet` with `.key("E")`, dock density, icon `icons::PIN`. Field: `add_field`. Row targets: `link` at the run scale's condition size in `participant.condition`; digits tabular; `icons::DICE` after a spent recharge. `time is up`: `result.warning` icon, words in `text`. Notice part: `Tone::Info`, icon `icons::PIN` |
| In view at 960x640 | Lookup closed and open: round, turn, `Next turn`, the current row with its effect line, the dock. No scroll |
| States | Counting: `Bless 9`. Last round: `last round`. Ended: warning icon and `time is up`. Spent: `spent` and the dice. Ready: `ready` |
| Keys and targets | `E`, Enter, Escape, Tab. Each target is at least 24 px each way, with an accessible name such as `Spend one legendary action of Adult Red Dragon` |

### F3. Plan items that are not fights

**Question.** "What did I plan for tonight besides the fights, and what is done?"

**The name.** A plan item that is not a fight is a **scene**: a talk, a person to meet, a place, a secret to
reveal. `Item` already means loot and equipment.

**Where it lives.** In the plan that exists. A task line under `## Plan for tonight` is a scene: a row with a
done box, in the note's own order. Obsidian shows the same line with the same box. This is true of a task
line that the DM typed before this feature: it is the DM's plan (section 2.5, row 14).

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
- **`Scene` in the Add strip's More menu.** Today the strip moves every action that is not always visible
  into **More** when their sum does not fit (`src/design/components.rs:2357-2372`). P3 computes that fit with
  today's actions only, and adds `Scene` to the **More** list alone. So `Scene` is always a line of **More**,
  no action that shows today moves, and a strip that needs no **More** today has no `Scene`.
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

- **The box** is at the start of the row, in the one leading slot that every plan row has, where a fight has
  its caret. Its 24 px target ends where the words start, so it reaches 2 px into the margin before the rows,
  and the scene's words start at the fights' title column (app round 1, V2). A click or Space ticks it. The
  note changes by one character.
- **The words** are drawn at the plan row title's size (`Body`) at every window size, with the Reader's links
  and emphasis, one line of `XL` each, so the second line sits at a fight row's pitch, and they wrap as a plan
  row's title wraps. No row changes height because of a state (corrected by the lead's ruling after app round
  1, V8: the Reader's inline text, a serif at a size that changed with the window, made the scenes the largest
  titles in the list). A done scene's words and links are `muted`; a link keeps its underline. A link opens its entry in
  Lookup, beside the page. `Back` returns to an entry that Lookup held, as today; a found note or event is
  searched again (2), and Ctrl+T returns to Trackers.
- **The second line** shows only when the words link exactly one World entry: its kind, status and place
  (`NPC · alive · Ashfall Keep`).
- **The action place** is empty until the scene is done. Then, when the words link exactly one World entry, it
  holds the quiet `Add to Log` of F6, by the fit rule: while the words keep their rows beside it. Else
  `Add to Log` is a line of the scene's menu (built in P6: a done scene with long words keeps its rows that
  way; since R1a draws the words at the title's size, the plain campaign's `Meet [[Hobb the Miller]] at the
  mill` keeps its rows beside `Add to Log`, which is then on its row).
- **The menu** holds `Start fight` first, for a scene that links exactly one NPC or creature while no run is
  open (section 1.2): 2; then `Add to Log` when it is not on the row; and `Remove from tonight` last. `Start
  fight` and `Remove from tonight` have the icons that the same acts have elsewhere (`icons::RUN`,
  `icons::REMOVE`). `Remove from tonight` says `Removed “<words>” from tonight.` with `Undo remove`, in the
  success tone, at the removed row's place while that place is drawn. The Undo is the note's (section 4.1): when
  that place is folded, or no longer among the rows (the fight before it was played), the result and its Undo
  show in the plan's message footer (app round 1, W6-D1, V3, V12, V13).

**A fight that nobody planned: `Start fight` in a reader's header.** It is a quiet action with the run icon,
by the rule of section 1.2. It shows for a creature and for an NPC while no run is open. While a run is open
it does not show: an NPC's reader has **Add to fight** then, as today (`src/world.rs:3160`), and **Arrivals**
adds a creature. It is blocked as the editor's `Start fight` is, with the same reasons.

- **What it does.** What the editor's `Start fight` does today for a new encounter, in one click: it saves an
  encounter named `Fight with <name>`, with the party and that creature, and it starts the run, which saves at
  once. When an encounter of that name exists, was never run, and holds what the click would save, the click
  starts that one. So a wrong click leaves no `Fight with Captain Varra 2`. Else the new one takes the next
  number. The name cannot be read as a link to the NPC, so a scene line that links the NPC stays a scene.
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
- **New: `Remove…` for an encounter.** Today no action in Splat removes an encounter: a **Ready to run** row's
  menu holds only `Edit encounter` (`src/encounters/list.rs:560-565`). P3 adds the line `Remove…` to that
  menu, only for an encounter with no saved run, so it has no blocked state. It removes the encounter's file
  and says `Removed encounter Fight with Captain Varra.` with `Undo remove encounter`: the words and the
  pattern of a saved group's row (`src/encounters/groups.rs:771`, `:934-935`). A plan line that linked it gets
  no row, and in the note its link has the hover `Not found in this campaign`, as every link to nothing has
  today.
  Cost from the Session page: `Encounters`, the menu, `Remove…`, `Session` = 4, **T T**.

**The fold** (ruled after review, built in P3; its measure ruled again after app round 1, W1-D1). The next
fight's row and its primary action are always in view in the plan; that promise comes first. Scenes give way
first. Each row counts as it is when closed: when the closed rows, with the scenes, do not fit the pane, every
scene folds. An opened row folds the scenes only when its details would push the next fight's primary action
out of view and the scenes sit above that fight; in a plan with no fight still to come (an older session), when
the rows with its details do not fit, as before. The Scene field, its suggestions and the plan's results never
fold the scenes, and the fold does not change while the Scene field has focus: the fold measures the room the
rows have with an empty message footer. With scenes folded, the
**Tonight** line says `N scenes` after its count, a link after the section caret of a folded section (as
`HISTORY` has on the Encounters page, V14), in room that is spare there today. The
fold takes no line of its own, so with scenes folded every other row has the place and the width that it has
in a note with no scene. `N scenes` opens them in place and closes them again; the DM's choice holds
for that session. Nothing else folds: a plan whose rows do not fit without scenes scrolls as today. When the
rows fit again, the scenes show again. A stated limit: when the DM opens a row above the next fight and the
scenes sit above that fight, the scenes still fold; the caret before `N scenes` is the DM's sign. A second
stated limit: the footer takes its height from the rows' room. One line of it leaves the next fight in view in
the plan at 960x640 (measured with one to nine rests above it), but a footer that grows toward its third of the
pane can cover the next fight's row; the DM then sees the plan's scroll bar and scrolls the rows to reach
`Start fight`. The scenes do not fold for it.

**Cases B, C and D.** The plan has its own tab and the full page width; the row is the same. The note is
behind its tab, so the typed route costs the tab changes.

**Exact words.** `Scene`. Field: `Plan a scene: words, a person or a place`. The empty plan's words are as
today. The box's accessible names: `Done: Captain Varra` and `Open: Captain Varra`. `Remove from tonight`.
`Undo remove`. Result of the field: `Scene added to tonight.` with `Undo scene`. `Start fight`, with the
accessible name `Start a fight with Captain Varra`. `Remove…`. `Removed encounter <name>.` `Undo remove
encounter`. The count in `TONIGHT 3` is the rows
still to come with the events of the session, as today; a done scene does not count.

| State | What shows |
|---|---|
| No scene | No row. The plan is as today |
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

**Undo.** `Undo scene` after the field adds one, directly under the field while it is open, so the field does
not move for it; once the field closes, in the plan's message footer (app round 1, W8-D7). A refused
Undo says why in the same way: under the field while it is open, else in the footer. Each scene result has one
place, decided as it is shown: its own (the row's place, the open field) while that place is drawn, else the
plan's message footer. A tick is
its own Undo: click again. Removing gives `Undo remove`. `Remove…` on an encounter gives `Undo remove
encounter`.
Section 4.1 says
when each ends.

**Keyboard.** Tab reaches the box, then the links in the words, then the menu. Space ticks. In the add field,
Enter adds, the arrow keys choose a suggestion, and Escape closes.

**What stays with the DM.** What a scene is, when it is done, and whether it happened at all.

**Not built, and why.** Editing a scene's words in its row, and dragging rows (the note holds them). Removal of
the improvised encounter by `Close run` (cut: after a restart nothing remembers which click made it).

| Check | F3 |
|---|---|
| Primary action in view | The first upcoming `Start fight` of the plan, as today. A scene never takes it. `Start fight` in a reader is quiet |
| Components and tokens | `Scene`: a line of the `More Add actions` menu, icon `icons::CHECK_SQUARE`. Field: `add_field`. Box: the theme's checkbox, 24 px. Words: `Type::Body` with the Reader's links and emphasis (`markdown_view::show_row_words`). Second line: `Type::Small` in `muted`. `Start fight` in a reader: `Action::quiet` with `icons::RUN`. `Remove…`: `menu_action` with `icons::REMOVE`. Messages: `undo_message` |
| In view at 960x640 | Lookup closed and open: the Add strip and the rows. Nothing scrolls sideways |
| States | Open: an empty box. Done: a checked box, and the words and links in `muted`, a link with its underline |

### F4. Trackers and the Due list

**Question.** "What is due, and what are the numbers?"

**The slice.** This design builds trackers for use **between fights**: the ledger, the Trackers view of
Lookup, the Due list and the rail's `Due N` (P9). In a fight the DM uses Trackers through Ctrl+T, and the owner
accepted this cost (section 7), so the run has no part of F4. Marks that a round raises are dropped. Day
marks are built in P11: the Timeline's day count raises a Due row, with the same words, rolls, proposed changes,
`Done`, `Dismiss`, ledger entries and Undo as a mark at a value.

The data model of [World and house rules](world-and-house-rules.md) stands, and this pull request changes
that document in place: Trackers is a view of Lookup, the ledger is the Undo, a mark's table can name what
it rolls for (`for:`), each Due row ends with `Done`, and each instance has its own line.

The rail's status slot, above Lookup, shows a bell and `Due 2` only while a notice is due, on every page. A
click on it, or Ctrl+T, opens the **Trackers** view in the Lookup panel.

**Wireframe: Trackers in Lookup at case D (346 px, about 322 inside), beside the Session page, with three
notices and four rolls.** The view is one scroll area, as Lookup's reader is (`src/lookup.rs:2195-2203`).
Its heading holds the Due count, and the result line keeps its place at the bottom edge.

```text
🔍 [Search…                                     ]  ×
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
Party: First seal. The party reaches level 2.
A rival uses its 1-2 seals group.         Done ⋯
PINNED
Elapsed race time                40 / 120
─────────────────────────────────────────────────────
✓ Enter room: Elapsed race time 40. Due: Bell.
  ↶ Undo: Enter room                Add to Timeline
```

- **The order in the scroll area:** the Due rows, then the pinned trackers in the note's order, then `MORE
  TRACKERS`. So what is due is in view when the view opens. A notice that a press raises shows above the
  trackers; the press is done by then, and the notice is the next step.
- **What fits at case D.** A roll button that is wider than half the panel has a line of its own, and `Done`
  ends the notice's last line. By estimate the three notices above take about 410 px of the area's 424,
  beside the Session page with its party bar. T19.4 tests it. When they do not fit, the area scrolls, as a
  reader does.
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
- **A quick action** is one press. An action that asks for a number (`add: ask`) puts a number field in its
  own place; the field holds focus until Enter or Escape.
- **A wrong press.** `Undo: <label>` takes it back while it is the newest act. Later, the opposite quick
  action corrects the value in one press, as an erasure does on paper; the history keeps both.
- **The result line** names the last act and has `Undo` and `Add to Timeline`. After `Add to Timeline` it says
  `Added to the Timeline.` with `Undo add`.
- **A tracker's history,** with **Remove**, and the list of problems show in the house rule note's reader. Not built in
  P9: the reader is `src/world.rs`, which P6 changes at the same time. P9 shows each problem in the view, at the end
  of its scroll area, and an entry with another stamp stays until it is edited out in Obsidian.
- **In a fight** the DM marks the paper sheet. Ctrl+T still opens the view (three acts for each press), and
  the run offers nothing more. After the fight the DM enters the fight's marks; a mark that those raise
  shows then.
- **Cases A and C** are the page before Ctrl+T: nothing shows unless a notice is due. **Case B** is the same
  view at 432 px.

**Exact words.** Rail: `Due 1`; hover `1 notice is due. Open Trackers (Ctrl+T).` View: `Trackers`, `DUE 1`,
`PINNED`, `MORE TRACKERS 1`. `Roll <table’s name>`. `Done`. `Dismiss`. Result: `<label>: Favor 2.`; with a
table `… Rolled 4 on 1d6: A healing potion.`; with a new notice `… Due: Bell.`; after `Done`, `Bell done.`;
after a dismissal `Bell dismissed.` `Undo: <label>`. `Add to Timeline`. A blocked action: `Favor cannot go
below 0.` With no note in play, Ctrl+T opens the empty state `No house rules are in play.` with `Turn one on
in Campaign → Rules and runs.`

| State | What shows |
|---|---|
| No house rule note in play; or in play with nothing due | Nothing in the rail |
| A notice is due | Rail: bell and `Due 1`, in the text colour |
| `Trackers.md` cannot be read | Rail: error icon and `Trackers not read`. The view names the file, says the history is safe, and offers `Try again` and `Show file`. Quick actions are blocked with that reason |
| A save of `Trackers.md` fails | Nothing is recorded and nothing waits (the lead's ruling after review round 2 of P9: a waiting act needed a snapshot of more and more state). The result line, at the view's bottom edge, says `Trackers.md was not saved: <reason>. Nothing was recorded.` as an error until the next act or until the ledger is read again. The DM presses the action again: one more act. A dice act rolls again; nothing was recorded, so no roll is lost from the record. A number typed for `add: ask` stays in its field |
| `Campaign.md` or a house rule note in play cannot be read | No empty list: the view names the file, shows no value and no old one, and every act is refused with the same words. The rail says `Trackers not read`. A note that was moved or renamed outside Splat says that Splat reads the notes again when its window gets focus |
| `Trackers.md` changed on disk | Splat reads it again within a second, and every act reads it at the press, so an edit in Obsidian counts at once and is never written over (as built in P9: no merge and no notice are needed). An Undo whose entries changed on disk writes nothing and says `The ledger changed on disk. Nothing was undone.` |

**What the files store.** Definitions are in the house rule note's Splat data, as before, with one new
optional word: `for:` on a mark's table. The ledger is one ordered list. Each act is one entry. A notice's
state is not stored: a notice is open until an entry finishes or dismisses it, and it is worked out from the
changes. An entry holds what it replaced where that cannot be worked out: `was` on a `set`. `why` holds the
action's label, so the result line and `Undo: <label>` name the act after a restart. The changes of one
press share a `use` and undo together.

```yaml
changes:
- {id: c-5e6f7a8b, tracker: race_clock, add: 10, why: Enter room, session: 070eca36-…}
- {id: c-6f7a8b9c, notice: {change: c-5e6f7a8b, at: 40}, roll: {table: rival_track, for: Old Guard, value: 4, row: Takes a seal.}, session: 070eca36-…}
- {id: c-7a8b9c0d, notice: {change: c-5e6f7a8b, at: 40}, done: true, session: 070eca36-…}
- {id: c-8b9c0d1e, tracker: favor, add: -1, why: Throw a gift, roll: "1d6: [4]", row: A healing potion, session: 070eca36-…}
```

**Undo.** `Undo: <label>` on the result line removes the newest entry that has the stamp of the session being
played (section 1.1), or no stamp when no session is played. The label is the action's own (`Undo: Throw a
gift`), or `Undo: Bell done`, `Undo: Bell dismissed`, `Undo: Rival Track roll`. The line then names the entry
before it, so the DM steps back, newest first. The ledger is a file, so the steps are there after a restart.
Undo of an entry that raised a notice removes the notice and its entries. An entry with another stamp leaves
by **Remove** in the tracker's history. `Add to Timeline` is a second act: it makes a Major event with the
action's label, in the session being played, with the Timeline's own Undo (section 4.1).

**Keyboard.** Ctrl+T, there and back. Tab walks the Due rows, then the trackers' actions. Enter presses an
action. Escape closes Lookup.

**What stays with the DM.** Awarding Favor, moving a rival, what a bell does on the map, and every rule that a
tracker's number stands for. A proposed change applies only when the DM presses `Done`.

**Day marks (P11).** A day mark (`at`, or `every` with `from`, in days) is open while the day count of the
Timeline has reached its day and no ledger entry ends it; raising it writes nothing. Its row starts with its day:
`Day 32: Move the rival powers.` Days of one mark that are due together are one row, as
[World and house rules](world-and-house-rules.md) says: `End of month ×3. Next: Day 32. …`, with `Next` in the place
of `Done` (it ends the oldest day) and `Dismiss all` in the row's menu. Day rows come first, oldest day first, then
the notices of marks at a value. The Due count counts rows, so a row of three days is `Due 1`. The result line says
`End of month, Day 32 done.` and `Undo: End of month, Day 32 done`, or `Undo: End of month ×3 dismissed`. The day
count has one answer, which the view, the rail's count and every act read (`trackers::today`). It is not known while
`Timeline.md` cannot be read, while a Timeline edit waits to be saved and the file changed, or cannot be read, under
it, and while the Timeline waits for the DM's choice. Then no day row shows, the top of the scroll area says the
Timeline's words (`Timeline.md changed on disk. Your draft is safe until you choose.`, or why it cannot be read)
and `No day mark shows until the Timeline is read.`, no act on a day row records, and the draft is kept. When the
file reads again or the choice is made, the rows come back. An act on a day row reads the Timeline again at the
click; when the row's day is no longer due, or the days of `Dismiss all` changed, it writes nothing and says so on
the result line. The rail's
`Due N` counts day rows too; the run shows nothing more.

**Not built, and why.** Any tracker part in a run (the owner accepted this cost; section 7). The parts in
[World and rules: later](world-and-rules-later.md). An action on a notice for a group (**Add to fight** is the
route, as today). A `Make house rule note` action (no band holds it today; the DM sets the note's properties).

| Check | F4 |
|---|---|
| Primary action in view | None in the view. The page's own primary stays |
| Components and tokens | Rail: the status slot with `status_link`; icon `icons::BELL`. View title: Lookup's title line. `DUE`, `PINNED`: `group_heading`. Quick actions and `Roll`: `Action::secondary` at Read density; `Roll` has `icons::DICE`. `Done`: `Action::quiet`. Proposed change: the theme's checkbox. Values: `Type::Numeral`, right-aligned with `number`. Result: `undo_message` with a second quiet action, above a hairline |
| In view at 960x640 | The rail's `Due N` on every page. In the view: the Due rows first, and the result line |
| States | Due: bell and `Due 1`. Not read: error icon and `Trackers not read`. Refused or not saved: the reason on the result line, which stays in view when the row it was pressed on goes away |
| Keys and targets | Ctrl+T, Tab, Enter. Every action is 28 px high (Read) |

### F5. Carry-over between sessions

**Question.** "What did the last session leave for this one?"

**Where it lives.** On the **Last time** line of the Session page, which already links the last session and
shows the first line of its recap (`last_time_block`, `src/sessions/page.rs:4066`). One quiet action,
`Carry over`, ends that line while the session has no event and no run, and a part is still to carry.
Carry-over is a prep act: during play and on an older session the line is as today. The preview gives the
action its room and is whole on hover: it shortens first, to nothing, then the link's words shorten, and
nothing wraps (section 2.5, row 11).

- **Not automatic at Create session,** because a note would then change with no act of the DM (rule 2).

**Three parts.** Each is done once.

| Part | From the last session | To this note | Copied text or a link | Done when |
|---|---|---|---|---|
| Recap | The prose under `## What happened` | `## Recap of last time` | **A link:** one embed line, `![[Session 1 The Road#What happened]]`. One file owns what happened, and a later correction shows here | The embed is added. Or this note already has the DM's own prose under the heading: it is kept, and the part counts as done |
| Recap, second source | A processed recording's **Recap**, only when `What happened` has no prose | `## Recap of last time` | **Copied text,** under the line `> This draft was written automatically. Check it against your memory.` | The same. Later, when the DM has written `What happened`, and the draft in this note is still the processed recap of that session with its warning line, `Carry over` shows again and puts the embed in the draft's place. A draft that the DM changed stays |
| Plan | Each row still to come under `## Plan for tonight`: a fight that was not played, a fight with a paused run, a group, a scene that is not done | `## Plan for tonight` | **Copied lines,** links kept, boxes empty | The lines are added, also when the DM deletes them later |
| Threads | Each line under `## Open threads` that is not ticked | `## Open threads` | **Copied lines** | The same |

- **A paused fight** carries as its encounter's link. Its row in the new plan has `Resume`, as a planned fight
  with a saved run has today (`src/sessions/plan.rs:780`, `:810-814`).
- **The recap can come later.** When the last session has no prose and no processed recording yet, the plan
  and the threads carry, and the recap stays to carry.
- **When the DM already wrote something.** Carry over never replaces the DM's words and never reorders. It
  adds below what a heading holds, and it skips a line that this note already has, compared without the task
  mark. A heading that this note does not have is added at the end of the note.
- **Which session.** The session before this one, by number. Once `carried_from` is set, `Carry over` reads
  only that session.
- **When the line reads the last session.** When that note's file changes and when a recording of that session
  is processed, never each frame.
- **The preview and the carry-over read one source:** the last session's `What happened`, then its processed
  recap (section 2.5, row 16).

```text
Case A, before and after
Last time:  Session 1: The Road — Hobb the miller says the toll collectors …    Carry over

Last time:  Session 1: The Road — Hobb the miller says the toll collectors …
…
✓ Carried over from Session 1: the recap, 2 plan lines and 2 open threads.   ↶ Undo carry over
```

The result sits in the Session page's message footer, as every result of that page does since that rule
landed (P8 followed it at the merge with `main`), so the line, the tabs and the plan never move.

**An embed in Read.** Read draws an embed of a session note's heading as that heading's text, under a muted
line, `From Session 1: The Road`, which links that session. The rule is by the embed's target, not by who
wrote the line, so an embed that the DM typed is drawn the same way (section 2.5, row 18). Today Read shows it
as a link. In Write it is the one line of source. The target resolves as every campaign link does, with its
folder; an ambiguous target, and an embed inside a callout or a list item, stay links (a P8 limit).

**Cases B, C and D.** `Carry over` never takes a line of its own, and the result message wraps.

**Exact words.** `Carry over`. Hover says what it will add: `Add the last session’s recap, 2 plan lines and 2
open threads to this note.` Result: `Carried over from Session 1: the recap, 2 plan lines and 2 open
threads.` It names only what it added. When this note has a recap: `… Your recap was kept.` When the recap is
not there yet: `… The recap is still to come.` When the embed takes the draft's place: `The recap now follows
your notes.` `Undo carry over`.

| State | What shows |
|---|---|
| No earlier session; no part can be carried now; every part is done | No action |
| The last note cannot be read | The action is blocked: `Splat cannot read Session 1’s note.` |
| This note changed on disk and Splat holds unsaved words | The act does nothing and says `The note changed on disk. Choose in the note first.` |
| The embed's note was renamed outside Obsidian | As the next row. Splat never rewrites an embed: the DM links again by hand. (P8 cut `Update the link` and `Undo link`: a sole recap embed that the DM kept could not be told from the one `Carry over` wrote, so the repair could rewrite the DM's own link; review of PR 277) |
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

**Undo.** `Undo carry over` puts back exactly what the act replaced. The lines that it added go. A draft that
became an embed comes back. `carried_from` and `carried_parts` have the values, or the absence, that they had
before the act. Section 4.1 says when it ends. Its result and its Undo belong to the session note by its ID: another session or
another page leaves them, so after an act elsewhere the DM comes back and still has `Undo carry over` (P8, by the
lead's rule that an Undo is kept by the ID of the thing it belongs to). After that the DM removes a carried
line as any line, and the part stays done.

**Keyboard.** Tab reaches `Carry over` after the **Last time** link. Enter presses it.

**What stays with the DM.** Which threads are still open, and what the recap says aloud.

**Not built, and why.** Carrying from sessions before the last. Ticking a thread in the old note when it is
ticked here (two files for one act).

| Check | F5 |
|---|---|
| Primary action in view | The plan's `Start fight`, as today |
| Components and tokens | `Carry over`: `Action::quiet`. Result: `undo_message`, success tone. The embed in Read: the Reader's prose under a `Type::Small` muted line with `link`. A source that is gone: `small_message`, warning tone |
| In view at 960x640 | The **Last time** line and the action, above the tabs, with Lookup closed or open |

### F6. Wrap-up to World

**Question.** "How does what happened to this person or place get into its own note?"

**Today.** A change of Status, place or group writes the value, and also a Log line when the current session is
dated today, in one step with one Undo (`src/world.rs:2280-2305`; the stamp is `log_as`,
`src/lib.rs:1187-1194`).
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
Captain Varra   ✎ Edit  ＋
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
  line. The header follows section 1.2.
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
| In view at 960x640 | The header by section 1.2, the field and the result, above the entry's text |

### F7. Rewards record

**Question.** "What did the party get tonight, and is it in the notes?"

**The note owns the rewards.** `Add rewards` writes one block in the session note: a list line that starts
with the fight's Timeline reference, and under it an XP line and the loot lines. The run stores nothing about
it. **A fight has its rewards in the notes when the note has a list item that starts with its reference,
anywhere in the note.** The heading `## Loot and rewards` is only where a new block goes; a DM who renames it
or moves the block loses nothing. Splat reads the note's real Markdown blocks once (`outline` in
`src/sessions.rs`), and the insertion, `In notes`, the XP choice and Undo all ask that one reading. **Text in a
code fence is never a heading and never a block.** **A list item inside a callout or a quote is a block**, as
the DM sees it: the row says `In notes`, the XP search reads it, and Undo leaves it, because it is no longer
the item that the act wrote. **The act reads its own result again before it writes:** it writes only when the
new words hold the fight's one block, exactly as the act made it, under the heading, so `In notes` is true
after the write. Otherwise it writes nothing and says why under the row: `A code block in the note is not
closed, so nothing was written. Close it, then add rewards.`, or, when no reason is known, `The note would not
show these rewards as a list, so nothing was written.`

**Where it lives.** On the Session page, on the plan row of a fight that has ended. That row's action band
holds only the menu today (`src/sessions/plan.rs:692-694`), and its title has the rest of the line.
`Add rewards` is a quiet action in that band, left of the menu, only when the title keeps the width and the
line breaks that it has today beside the action's own width; the same number of lines is not enough. P7 works this out when the title or the width changes, not each
frame. When the title uses the width, it is the last of the opened row's links: `Log · Whole run · Timeline ·
Add rewards` (`:4918-4926`), so no link of today moves, while that line has room for it. Measured in P7, that
line takes about 326 px and the plan at 330 px gives it 296, so there it is the last line of the row's menu
(rule 2: a menu that exists in that place), and the opened row gains no line. After the act, the same place says `In notes`: the
status that an event's row has today (`:4392`), muted words in the opened row, or the same status as the
last line of the row's menu.

```text
Case A, the plan at 330 px
▸ Shrine of Cinders             Add rewards  ⋯
  Ended in round 1
▾ Ashfall Crossing at the Old Toll Bridge     ⋯
  Held the ford · Ended · Round 3
    Log · Whole run · Timeline · Add rewards
✓ Rewards added under Loot and rewards. XP 400, as in
  your last rewards. XP tonight: 1,650 for 2 fights.
  ↶ Undo rewards
```

**What it writes.**

- The fight's link, as events write it: `[[Timeline#^t-08a07eb0ec|Ashfall Crossing]]`. A played fight has no
  `Add to notes` today (`:4388`), so this is the act that links it.
- **The XP line,** first under it, when the notes use XP (below): the XP of the creatures that ended at 0 HP,
  also those marked absent at 0 HP, and each character's share. F1's function gives the numbers. The line
  names what it counted and what still stood, so the DM can change it. With nobody at 0 HP it says `XP 0 for 0
  at 0 HP`. When a creature at 0 HP has no XP that Splat knows (a made-up creature, an NPC with no stat
  block), the line says so: `XP 450 for 3 at 0 HP. XP not known for Captain Varra: enter an award here.` The
  count is of the creatures whose XP is summed. It is still an XP line, and the link and the loot are written. An amount that is not known is left out of
  every sum and is never counted as zero. This is the rule (section 7).
- **Loot,** as lines under it: the encounter's loot and the loot of each creature at 0 HP. A roll that was
  made has its result. A roll that was not made is written with `(not rolled)`, as `Copy loot` writes it
  today, and the result says how many.

**XP has no switch.** There is no setting, no control and no stored mark. For a new block Splat looks for the
newest rewards block: in this note first (the last block in its text), then in the note of each earlier
session, newest first. A note
with no block is passed. A note that cannot be read stops the search; P7 reads the notes in order and keeps
an unreadable one in its place. A file that claims a session but whose properties cannot be read has no known
place, so it stops the search right after this note. Splat finds these files with the same scan of the folder
and its subfolders that finds the readable notes (built in P7: `earlier_notes`, `rewards_with_xp`).

| The search finds | `Add rewards` writes | The result says |
|---|---|---|
| A block with an XP line | The link, the XP line and the loot | `XP 400, as in your last rewards.` |
| A block with no XP line | The link and the loot | `No XP, as in your last rewards.` |
| No block, or a note that cannot be read | The link, the XP line and the loot | `XP added. Delete that line to leave XP out from now on.` |

So a campaign that gives no XP deletes one line, once. A DM who sets an award by hand keeps it: Splat never
rewrites a block.

**After the act** the note beside the plan scrolls to the new block. Where the note is behind its tab, the
**Note** tab shows the block when the DM goes there. The counts assume the note in Write; in Read, `Write` is
1 more.

**A limit.** Splat does not watch the block. When the fight changes later (a roll is taken back, or the last
fight goes on and ends again), the DM changes the block by hand, or deletes its first line: the row then says
`Add
rewards` again.

**Loot that nobody planned.** The DM types it under the block: one click and one line.

**The session's XP.** Splat stores no total. The result says the sum of the first number of each block's XP
line in this note, as the note stands, and how many fights it sums.

**Cases B, C and D.** The plan has the page's width, so more titles leave room. The result message wraps.

**Exact words.** `Add rewards`. `In notes`. Result: `Rewards added under Loot and rewards.`, then one line of
the table above; with XP, `XP tonight: 1,650 for 2 fights.`; with rolls that were not made, `2 loot rolls were
not made.`; with an award that is not known, `1 award is not known.` `Undo rewards`.

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

- [[Timeline#^t-08a07eb0ec|Ashfall Crossing]]
  - XP 400 for 2 at 0 HP · 80 each for 5 · 1 still stood (XP 200)
  - 120 gp in a locked coffer
  - A smoky quartz (50 gp), from 2× gems
  - Varra’s signet ring
- [[Timeline#^t-5c1d2e3f4a|Shrine of Cinders]]
  - XP 1,250 for 3 at 0 HP · 250 each for 5
```

The last line of the first block is one that the DM typed. An XP line is a line under a fight's line that
starts with `XP`; the sum reads its first number, so `(XP 200)` is not counted.

**Undo.** `Undo rewards` acts only while the block is still the complete, unchanged list item that the act
wrote: an item that the DM typed under it, also after a blank line, is part of it, as CommonMark reads it. The
bytes from the start of the block's first line to the end of its last line must be exactly the bytes that the
act wrote, so a quote mark, an indent or any other change refuses it.
Then it removes that item, and the heading only when the act added the heading and nothing else is under it.
When the block changed, Undo writes nothing: the result and its Undo end, and a press that meets the change
on disk says `That line changed on disk.` under the row, the words of the other note acts. By the lead's
ruling for P7 it is kept by the note's ID and the fight, not by the note's whole text: another act, an edit
elsewhere in the note, another session or another page leaves it. When act A added the heading and act B
wrote under it, Undo of A keeps the heading for B, and Undo of B keeps it too: only A's Undo knew that the
heading was the act's, and its bytes are the same as a template's heading. **Keyboard.** Tab reaches `Add rewards` in the row or among the opened row's links, and the result's
action after its words.

**What stays with the DM.** Who earned XP, whether a creature that fled counts, how loot is shared, and when
the party levels up. A milestone is a line that the DM writes.

**Not built, and why.** `Update rewards`, and any record in the run of what was written (two owners for one
fact). A switch for XP (cut: the note holds the choice). A stored total. A party inventory.

| Check | F7 |
|---|---|
| Primary action in view | The next `Start fight` |
| Components and tokens | `Add rewards`: `Action::quiet` in the plan row's action band, or `word_link` in the opened row. `In notes`: `design::status` with `icons::OK` in `document.saved`. Result: `undo_message` |
| In view at 960x640 | The played rows; the action on the row, or one click away in the opened row |
| States | Not added: `Add rewards`. Added: check and `In notes`. Blocked: the reason in `Small` |

### F8. Undo after a restart, and Continue fight

**Question.** "The app closed, or I ended the fight by mistake. Can I go on?"

F8 is being built in [pull request 232](https://github.com/david7a68/splat/pull/232). Its head still changes,
so this design states the rule that the build follows and adopts the build as it merges (P0). It adds no
surface to it.

| What the DM gets | The rule |
|---|---|
| Undo after **Close run**, **Resume** and a restart | The run file keeps the Undo steps (at most 100). `Undo` names and takes back the same step |
| A fight that ended by mistake, in the same opening | `Undo: end encounter`, as on `main` |
| **Continue fight** | It is offered only for the fight that was played last: the Timeline can be read, and no fight and no rest has a later place in the order of play than this fight's event. It is one named step, `Undo: continue fight`. Play returns to the turn the fight ended on, with the loot results kept |
| Any other ended run | No `Continue fight`. Its place says why: `A later fight or rest was played. Start a new run to play this one again.` A long rest counts whatever it restored. While `Timeline.md` cannot be read, the sentence is `Splat cannot read Timeline.md. Fix that on the Timeline page first.` For a fight whose event was removed from Timeline, it is `This fight is not on the Timeline. Start a new run to play it again.` When the act is pressed while `Timeline.md` changed on disk under a Timeline edit that is not saved yet, it refuses with `Timeline.md changed on disk. Settle that on the Timeline page first.` |
| An old run file | It loads with no steps: `Nothing to undo since reopening`, as before |
| Steps that this version cannot read | The run loads and drops only the steps. A warning says so once, with `Dismiss` |

**Limits of the build.** The pull request's body lists them at its head. Two matter here: a step whose save
did not finish is lost, and a sheet that changed after the ending wins at the first read after `Continue
fight`, as for any resumed run. Continuing an older fight is cut (section 7).

**One exception to rule 8, from that build.** The unread-steps warning pushes the roster down by its lines.
It shows only after a file that this version cannot read, and `Next turn` works while it shows.

**What this design changes in it.** One thing, in P4 (F2): a condition with a time is an object in a row's
`conditions`. A saved step holds rows as they are, so its `version` stays 1, and an older step reads as labels.

| Check | F8 |
|---|---|
| Primary action in view | `Next turn` in play. On the ended view, `Close run`, as today |
| Components and tokens | As built |
| In view at 960x640 | The pull request's own scenario captures each surface with Lookup closed and open |

### F9. Travel and random encounters

**Question.** "The party travels for two days. Does anything find them?"

**Where it lives.** In the Travel event's own row, in the session's plan and in the detail column on
Timeline. A Travel row is as today until the DM chooses `Roll for encounters` in the row's menu, which exists
today (`src/sessions/plan.rs:727-735`; Timeline has its own event menu). That one act puts the block
**Encounter checks** on that row: the row's length field, a table, and one line for each day or watch.


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
| Several, and an earlier journey with checks | The table of the journey with checks that is latest on the Timeline | 0 |
| Several, and no earlier journey with checks | No table. The menu says `Choose a table`, in the order of the tables' names, and `Roll` is blocked: `Choose a table first.` The event holds `- Checks table:` with no key, so the block is there after a restart | 2, once |
| An earlier choice whose table is gone | The same as no earlier choice | 2 |

The table menu's last line is `No checks`: a choice of no table. It takes the `Checks table` line out of the
event, and so the block off the row, while no result stands; with a result it is off: `Clear the results
first.` So a wrong `Roll for encounters` can go at any time.

**Wireframe: the Travel row after `Roll for encounters`, case A (the plan is 330 px), with long results.**

```text
▾  Road to the ford by the old quarry     Add to notes ⋯
   Encounter checks
   On the road ⌄ · 1d20 each day
   ✓ 2 travel checks rolled.   ↶ Undo roll
   How long   [2 days                ]
              Day 3, 08:00 to Day 5, 08:00
     Day 3   12 · Nothing stirs.                    Clear
     Day 4   16 · Wolf Pack pick up the trail
             near the quarry gate.                  Clear
             Plan “Wolf Pack”
   Title      [Road to the ford by the old quarry ]
   Notes      [                                   ]
   Timeline
```

- **The block comes first in the open row,** before **Title** and **Notes**, so the rolls stay near the row
  (repair R1b). A Travel row without the block is as today. `Encounter checks` is a label above the table
  menu, as every field's label is; `· 1d20 each day` follows the menu on its line, and dice words that move
  to a line of their own start with the words, never with `·`.
- **An act's result has one kept line** under the table's line: `2 travel checks rolled.` with `Undo roll`,
  and the results of `Roll for encounters` and `Clear` there too. The line is reserved while the block shows, so a result coming or
  going moves nothing below it, and at 960x640 it is in view where the DM pressed `Roll`. Words that do not
  fit beside Undo end in `…`; the hover keeps them whole.
- **The plan line's message** (`“Wolf Pack” is in tonight’s plan.` with `Undo plan line`, the note's Undo)
  shows in place of the `Plan “Wolf Pack”` line of the check that the DM clicked; `In the plan` takes that place
  when the Undo ends. The offer keeps from the start the height that its message needs at that width: one line
  when the words and Undo fit, else the words with Undo under them (at 960x640 the message of `Bridge Ambush`
  needs about 420 px and the row has less). So nothing moves when the message comes or goes, and it is never cut.
  When that check is gone (cleared or rolled again) while the Undo lives, the message takes the kept line. If a
  roll's result holds the kept line, the message ends the block, as before: the one known case where it moves
  what is below (repair R1b, the owner's ruling).

- **How long** is the Timeline's field. It is part of the block, so a plain Travel row does not gain a line.
- **One check at each whole interval from the start, and none at the journey's end.** Two days give two
  checks, at 0 and 24 hours. A length of zero gives none, because its only check would be at its end; its
  results stay, under `Outside this journey now`. An empty length, or one that the clock cannot read, gives
  one line, `This journey`. `Roll all 2` shows under the lines while two or more are not rolled.
- **The lines on screen follow the length as the DM types it.** New in P5, on Timeline and on the
  Session page, only for a journey whose block shows: the file takes the length on Enter or when the field
  loses focus. Every other event's **How long** changes the event on each key, as before P5 (repair R1b). An outside change to the event meets the Timeline's choices of
  today at that save. `Roll for encounters` puts focus in **How long** when the
  journey has no length.
- **A check belongs to its offset** from the journey's start: 0 hours, 24 hours, and so on. Its name is read
  from the clock: `Day 3` for a day, `Day 3, 08:00` for a watch or for hours. **A result is never deleted by
  a change of length or of table.** A result whose offset is now outside the journey, or between the new
  table's lines, stays under the muted line `Outside this journey now`, until the DM clears it.
- **A result's line ends with the quiet `Clear`,** which gives the line its buttons back. It works at any
  time, also after a restart, and whatever the words now say.
- **A result that links an encounter or a group offers `Plan “<name>”`** for each, at most three. One click
  writes the link under `## Plan for tonight` as the first line still to come. So the fight is the next plan
  row and has the primary `Start fight`. Then the line says `In the plan`, while the plan has a row still to
  come for that
  link: Splat reads this by the plan's own rule, which takes one played event for each link
  (`src/sessions/plan.rs:529-549`), and stores no mark. F5's "not played" uses the same rule. After that
  fight is played, `Plan “Wolf Pack”` shows
  again and adds a second line.
- More than 31 lines are not listed. The row says `This journey has 240 checks. Split it into shorter
  journeys to roll them here.`
- **Cases B, C and D.** The plan has the page's width, which is wider than 330 px, so nothing more gives.
- **On Timeline** the event's menu has the same `Roll for encounters`, and the detail column shows the same
  block under its own **How long**, with no `Plan “…”` (section 7).
- **The length being typed has one owner,** whichever page shows the field. Leaving the page gives it to the
  journey, as the field's loss of focus does, so a length typed on one page never outlives a newer one typed
  on the other.

**Lookup.** The table's name is the table menu's button, and opens nothing else (section 7).


**Exact words.** Menu: `Roll for encounters`. Label: `Encounter checks`. `<table’s name> · 1d20 each day`, or
`each watch`, or `every 4 hours`. `Choose a table`. `No checks`. `Roll`. `Enter a roll`. `Roll all 2`. `This
journey`. `Clear`. `Plan “Wolf Pack”`. `In the plan`. `Outside this journey now`. Results: `Encounter checks
added to this journey.` `Undo checks`; `2 travel checks rolled.` `Undo roll`; `Check cleared.` `Undo clear`;
`“Wolf Pack” is in tonight’s plan.` `Undo plan line`.

| State | What shows |
|---|---|
| The campaign has no marked table | No menu line. The row is as today |
| The DM has not chosen `Roll for encounters` on this row | The row is as today |
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
  - Check at 0h: "12 · Nothing stirs."
  - Check at 24h: "16 · [[Wolf Pack]] pick up the trail."
  %% splat session=070eca36-331c-46cc-b5a6-32a6e35265da %%
```

- **What Splat reads back:** the table's key; from each check line, the offset in its label, the number
  before `·` and the words after it. It writes each value in quotes, so that an older Splat, which keeps a
  line that it does not know as a YAML value (`src/timeline/formats.rs:86-89`), keeps it as one string, also
  when the words hold `: `.
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

**Not built, and why.** Checks on every Travel row, and a table found by its key's name. Weather, pace and a
table chosen by terrain.

| Check | F9 |
|---|---|
| Primary action in view | The plan's `Start fight`. No roll is primary |
| Components and tokens | Menu lines: quiet items. **How long**: the Timeline's field. Table: `menu_button`. `Roll`, `Roll all`: `Action::secondary` with `icons::DICE`. `Enter a roll`: `number_field`. `Clear`, `Plan “…”`: `Action::quiet`. `In the plan`: `status`. Messages: `undo_message` |
| In view at 960x640 | The open row scrolls with the plan, as open rows do today. The Add strip stays above it |
| States | Not rolled: the buttons. Rolled: the number and words. Planned: check and `In the plan`. Fault: warning icon and `Cannot roll` |

## 4. Cross-feature rules

### 4.1 Every new write

One act writes one file, as a patch of the file on disk now. There are five write paths. One function is
new, and one save is new; the table says which.

| Path | Today | For a new act | After a restart | A failed save says |
|---|---|---|---|---|
| **Session note** | A plan act changes the draft (`insert_in_plan`, `src/sessions.rs:1071`); the save comes 2 seconds later (`SAVE_AFTER`, `:26`). `Draft::save` writes only when the file is as Splat last read it (`:1282`). `follow_disk` takes the file's text when Splat holds no unsaved words, on a poll each second (`:1423`) | **New in P1c: one function, `Draft::apply`.** It follows the disk, patches one fragment, saves now through `Draft::save`, and keeps the Undo. It reports a patch or a save that failed, checks that the note is still the one it read, and tells a missing file from an outside edit, before a feature says that its act is done. Every note act of this design calls it. Today's two plan acts (`add_to_plan`, `src/sessions/page.rs:3942`; the removal, `src/sessions/plan.rs:926`) keep today's behaviour under it, also with unsaved words and a changed file | The note holds what the act wrote. The DM removes it as any words | `Notes not saved`, as today. The fragment waits in the draft |
| **World note** | A change reads the note, changes it and writes it with `write_note`, which writes only when the file still holds the text that was read (`src/world.rs:1033`) | The same call. An edit form with changes for the same entry blocks the act | The same | An error that names the file |
| **`Timeline.md`** | `Book::save` writes the file atomically, checks for an outside change, and merges by event (`src/timeline.rs:924`, `:774`) | The act changes the lines of one event. Separate events merge with a quiet notice; the same event needs a choice, as today | The lines are in the file | `Timeline not saved`, as today |
| **`Trackers.md`** | No code today | **Built in P9: the ledger's save** (`src/trackers/ledger.rs`). An act reads the file as it is on disk at the press, builds its entries from it, and writes with the shared atomic write and its outside-change check (`file_batch::apply`, which already served several files, so `Book` keeps its own). So nothing needs a merge: an outside edit stays. An act also reads the house rule notes in play at the press, and a definition that changed writes nothing | The entries, and so their Undo, are in the file | `Trackers.md was not saved: <reason>. Nothing was recorded.` on the result line. Nothing waits; the DM presses again (the lead's ruling after review round 2) |
| **Run file** | The run saves after each step and has its own choices when the file changed | As today | The step and its Undo are in the file (F8) | The run's footer, as today |

**What `Draft::apply` does with an outside edit.** It calls `follow_disk` first, so an edit made in Obsidian a
moment before is in the text that it changes. With unsaved typed words and a changed file, a new act is not
applied, and its place says `The note changed on disk. Choose in the note first.` An act on a line that
exists names the line it was drawn from: its place and its whole text as read. When the line at that place
holds other words now, it writes nothing, says `That line changed on disk.` where the DM acted, and the row
draws again from the file. It never looks for another line that reads the same (built in P3).

**When an Undo ends.**

- **A note act:** its Undo shows while the note's text is the text that the act left, as the plan's `Undo
  remove` of a fight does today (`removed_fight`, `src/sessions/plan.rs:1013-1035`). It has no time limit. Any
  other change of the text, in Splat or on disk, ends it, and so does a change of session.
- **A World note act:** until the entry's text changes or the reader shows another entry.
- **A Timeline act of this design** (`Undo checks`, `Undo roll`, `Undo clear`): until that event changes again
  or the DM leaves the page. It has no time limit. Today's `Undo add` of an event keeps its 30 seconds
  (`UNDO_ADD_FOR`, `src/sessions/plan.rs:22`, `src/timeline/page.rs:13`).
- **`Undo remove encounter`:** as a saved group's `Undo remove group` today. It writes the file back only
  when no file has that path.
- **A run step and a ledger entry** stay, also after a restart.
- **An Undo shows where the DM acted,** also when the act changed a file that is not in view.

Each row is one new write, with the fragment as the file holds it. Longer fragments are in section 3.

| Act | The one file | The fragment | Undo |
|---|---|---|---|
| `Scene` (F3) | The session note | `- [ ] [[Captain Varra]]` under `## Plan for tonight` | `Undo scene` |
| Tick or untick a scene (F3) | The session note | `- [x] [[Captain Varra]]`: one character | The box, again |
| `Remove from tonight` on a scene (F3) | The session note | That line, out | `Undo remove` |
| `Start fight` in a reader (F3) | The encounter note; then the run starts and saves, the two saves of the editor's `Start fight` today | `Encounters/Fight with Captain Varra.md`; a run file | `Close run` removes an untouched Setup's run file, as today. The encounter stays |
| `Remove…` on an encounter (F3) | The encounter note | The file, removed | `Undo remove encounter` |
| An effect: add, spend, give back, mark spent, recharge roll, remove (F2) | The run file | `{"name": "Frightened", "rounds": 10, "left": 9}` in the participant's `conditions` | The run's `Undo: <step>` |
| A tracker press, roll, `Done`, `Dismiss` (F4) | `Trackers.md` | One entry at the end of `changes` (F4). A press that changes two trackers writes two entries that share a `use`, in one save | `Undo: <label>`, step by step |
| `Add to Timeline` after a tracker act (F4) | `Timeline.md` | `- Major event: **Throw a gift** ^t-7d2c9a41be`, with its `Session` line | Today's `Undo add`, on the Trackers result line |
| `Carry over` (F5) | This session's note | The embed line or the copied recap, the copied lines, and `carried_from` with `carried_parts`, in one save (F5) | `Undo carry over`, which puts back what the act replaced |
| `Add to Log` (F6) | The World entry's note | `- [[Session 12]], Day 3: Told the party who burned the ropes.` at the end of `## Log` | `Undo Log line` |
| `Add rewards` (F7) | The session note | One list line that starts with the fight's reference, with its XP line and its loot lines (F7) | `Undo rewards` |
| `Roll for encounters`; a table choice; `No checks` (F9) | `Timeline.md` | `  - Checks table: road` in the event, or `  - Checks table:` while no table is chosen | `Undo checks`; later `No checks` |
| `Roll`, `Roll all`, `Enter a roll`, `Clear` (F9) | `Timeline.md` | `  - Check at 24h: "16 · [[Wolf Pack]] pick up the trail."` | `Undo roll`, `Undo clear`; later `Clear` |
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
| A tracker's value; a Due notice, its rolls and its end (F4) | `Trackers.md`, as entries | The Trackers view | The rail's count; the house rule note's reader |
| The recap of the last session (F5) | The last session's note, `## What happened` | That note | This note's `## Recap of last time`, by an embed; the **Last time** line |
| Which parts a note carried (F5) | The note's `carried_from` and `carried_parts` | `Carry over`, `Undo carry over` | Whether the **Last time** line has the action |
| A Log line (F6) | The World entry's note, `## Log` | `Add to Log`, or a **Change** | The session's **World changes** |
| A fight's rewards in the notes (F7) | The session note: the line with the fight's reference | `Add rewards`; then the note | The row's `In notes` |
| Whether the next block has XP (F7) | The newest rewards block in the session notes | The note: the DM writes or deletes an XP line | What the next `Add rewards` writes |
| A fight's result and its Undo steps (F8) | The run file. `Timeline.md` holds the result line | `End encounter…`, `Undo`, `Continue fight` | The plan row, Timeline, History |
| A journey's length, table and checks (F9) | `Timeline.md`, in the event's lines | The Travel row, in the plan or on Timeline | The clock; Obsidian |

### 4.3 Pairs that could show one fact twice, and which one wins

| Pair | The rule |
|---|---|
| F2 reminder and F4 Due notice | The notice is the campaign's fact, and the rail's count stays until `Done`. An effect that the DM types on a row is the DM's own second note. Splat never makes one from the other |
| F2 effect and an official condition | Words that are a condition's name become that condition, with the time. One entry holds the condition and its count, so a sheet that removes one removes both |
| F2 the dock's field and the Conditions tab's field | One reader of the words, and both write the same entry. The Conditions tab's field keeps its `Add` button, as today (`view.rs:2430`) |
| F3 scene and a Timeline event | The scene is the plan. The event is what happened. A done scene with one entry offers `Add to Log`, not an event |
| F3 a scene that links an NPC and the fight with that NPC | The improvised encounter is named `Fight with <name>`, so the NPC's own name is never an encounter's name |
| F6 `Add to Log` and a Status change | A fact that a field holds is a **Change**, which writes its own Log line. `Add to Log` is for what no field holds |
| F4 `Undo: <label>` and the run's `Undo` | Each names its own step and takes back only its own file's change. `U` is the run's |
| F7 the rewards block and the run | The note owns the rewards after `Add rewards`. Splat does not compare them again. A stated limit |
| F9 travel check and a roll in Lookup | The Travel row keeps its results in `Timeline.md`. A roll from Lookup's **Roll** button is a run step when a run is open, and otherwise only shows, as today |

### 4.4 One pattern for one act

Each act has one control, one set of words and one key, with Lookup open or closed. "Today" names where the
pattern is already in the app.

| Act | Control and words | Key | Today | Used by |
|---|---|---|---|---|
| Add a thing of a kind | A quiet icon-and-word action of the Add strip, or a line of its **More** menu: `Fight`, `Scene`, `Travel` | Tab, Enter | Fight, Group, rests, events | F3 `Scene` |
| Add typed words to a list | The add field: one field with no button. Enter adds. A suggestion adds by an arrow key and Enter, or a click. Escape clears and closes. The field stays open after Enter only where several lines follow each other | Enter, Escape | `Add loot` in the encounter editor | F2 the effect field, F3 scenes, F6 `Add to Log` |
| Put a record in the session note | A quiet action on the plan row, `Add …`. Then the status `In notes` in its place | Tab, Enter | `Add to notes` and the status `In notes` on event rows (`src/sessions/plan.rs:740-742`) | F7 `Add rewards` |
| Start a fight | `Start fight` with the run icon: a bordered button on a row and in the editor, a quiet action in a reader's header, a line of a scene row's menu | Tab, Enter | Plan, Encounters, editor | F3 in a reader |
| Finish a notice | A quiet `Done` at the end of the notice's actions. It applies the ticked proposed changes | Tab, Enter | New | F4 |
| Roll | A control with the dice icon. A result takes its place when the thing rolls once, and is listed under it otherwise. `Roll all N`; `Enter a roll` for a player's own roll | Enter | Tables in the Reader, loot, the dock | F2 a spent recharge, F4 notices, F9 checks |
| Name a table outside its own note | Its heading, from one descriptor for a marked table: source note, heading, key, dice, interval | | New. The Reader's own button says `Roll 1d20` and the loot editor's menu is as today | F4, F9 |
| Undo anywhere else | A message that says what happened, with a quiet `Undo <noun>` on its line, where the DM acted. Section 4.1 says when it ends | Tab, Enter | Plan, Timeline, World, Library | `Undo scene`, `Undo carry over`, `Undo Log line`, `Undo rewards`, `Undo checks`, `Undo roll`, `Undo clear`, `Undo plan line` |
| Remove | A line of the row's menu, with an Undo message | Tab, Enter | Fights and groups in the plan; a saved group's `Remove…` | F3 scenes and an encounter's `Remove…`, F4 `Dismiss` |

## 5. Consistency

### 5.1 New controls and their components

No feature adds a colour or a type size. Three icons join `design::icons`: `PIN`, `BELL` and `CHECK_SQUARE`.
The shared parts land before the
features, in P1c, so that every user has one copy:

- `add_field`: the loot editor's add field with suggestions, with today's behaviour. Its two callers of today
  are checked against it.
- One descriptor for a marked table: source note, heading, key, dice and interval. Today `rolls_marker` gives
  only the key and the dice (`src/roll_table.rs:66-78`).
- `Draft::apply` (section 4.1) and `played_session` (section 1.1).

### 5.2 Proposed text for `docs/design-system.md`

This pull request does not edit that file. Each pull request of the build plan adds its own lines there, and
the rule in this document is the text.

- Each surface's rule is the text: the live run (section 1.4, F2), the encounter editor (F1), the session's
  plan and notes (F3, F5, F7, F9, section 4.1), World, Lookup and the rail (section 1.2, F4, F6).
- **Icons.** `PIN`: an effect with a time. `BELL`: Due. `CHECK_SQUARE`: Scene.

### 5.3 Proposed text for `docs/ux-intent.md`

**Changed and new workflows.** Pull request 232 has its own changes to W4 and to assumption A8.

- **W5,** add: "The editor says how the roster compares with the party's XP budget." (T5.7)
- **W11,** add: "A journey that the DM asks to check rolls its encounter checks in its own row, and a result
  becomes a planned fight in one step." (T11.9, T11.10)
- **W14,** add: "The plan also holds scenes, with a done mark. A creature's or an NPC's page starts a fight
  that nobody planned." (T12.10 to T12.12)
- **Section 3, Reachable from anywhere,** add: "**Trackers:** Ctrl+T shows the trackers and the Due list in
  the Lookup panel, and goes back." (T19.1)
- **W15, new, Keep track of what lasts:** "On each turn the DM sees what they asked to be reminded of, and
  how long it lasts, with no answer needed." (T17.1 to T17.6)
- **W16, new, Close the evening and open the next:** "Rewards, what changed in the world, the recap and the
  open threads are where the next step needs them, with nothing typed twice." (T18.1 to T18.4)
- **W17, new, Run a house rule's numbers:** "Between fights, one press records a change, a due notice stays
  in view, and the history is a file." (T19.1 to T19.5)

**New rows for section 8.**

| ID | Observable test |
|---|---|
| T5.7 | The editor's roster line ends with the XP and a word from the rules data, wide and narrow. With no table, sheets or XP it is as today |
| T11.9 | A two-day journey rolls two checks in 1; each is a quoted line of the event in `Timeline.md`, and an older reader keeps a line whose words hold `: `. A shorter length deletes none. A result edited in Obsidian shows its words, and `Clear` removes it |
| T11.10 | A result that names an encounter is planned in 1, and its `Start fight` is next; `In the plan` shows only while that row is to come. With several tables and no earlier choice, `Roll` waits, and the event holds `Checks table:` through a restart. `No checks` takes the block off |
| T12.10 | A task line is a scene with a box. A tick changes one character. A plain list line gets no row. The empty plan is as today |
| T12.11 | A scene's link opens its entry in Lookup in 1, and `Back` returns to an earlier entry |
| T12.12 | A reader starts a fight in 1, and a scene row's menu in 2. An NPC with no stat block opens the made-up form with its name and focus in HP. After `Close run`, also after a restart, a second `Start fight` starts the same encounter. `Remove…` and `Undo remove encounter` remove and restore an encounter with no saved run |
| T17.1 | An effect with a count joins a row in ≤ 4, with the run's panel hidden; focus returns to Amount. With the longest name the dock has no `Effect`, and the name keeps its size and words; `E` adds the effect through the Conditions tab's field in 4, the pointer in 5 |
| T17.2 | On its turn the effect's words show on the notice line and the row. A label shows on no notice. `Next turn` takes 1 and does not move |
| T17.3 | A count of 10 says 10, then 9, says `time is up` at 0, and removes nothing. Undo of `Next turn` gives it back, also after a restart |
| T17.4 | Case D: the notice line and the rows match section 1.4 with a conflict on a 16-letter name and three reminders that start with the same words, and with no conflict. With 12 participants that each have a timed effect, every row has the height that the same row has with no effect. `End Wis save DC 15 (Hold Person) 2 rounds` keeps its place through `2`, `last round` and `time is up` |
| T17.5 | A spent recharge rolls in 1 and a use is spent in 1, on the row. A counted condition's name opens its rule |
| T17.6 | Damage to a `Conc` row adds the save DC at the end of its second line, with no action, notice or new line. With a timed effect on the row, the DC takes only the room that is left |
| T18.1 | A fight's loot reaches the note in 1, with its reference and one Undo, and the note shows the block. The row says `In notes`, also after the heading is renamed. The XP line follows F7's table, and no block that exists is changed. A made-up NPC at 0 HP gives an XP line that names the award that is not known; the next block still has an XP line, and the sum leaves that award out. A title that fills the row has the action as the last link of the opened row, at the same row height |
| T18.2 | A line reaches an entry's Log in ≤ 4 from a done scene, stamped as a **Change** at that moment. The header's actions match the table of section 1.2 for titles of 13, 16 and 19 letters at cases B and D and on the World page, with and without a portrait, and every title keeps its width |
| T18.3 | `Carry over` shows only while the session has no event and no run. It adds the recap link, the plan lines (a paused fight has `Resume`) and the threads in 1, below the DM's words. A written recap is kept. After a copied draft became the embed, `Undo carry over` gives back the draft and both properties as they were |
| T18.4 | A note the DM did not act on does not change. A new session with no carry-over is the template, and its first fight starts from the sheets |
| T19.1 | Ctrl+T opens Trackers in Lookup, and Ctrl+T again shows what Lookup showed: an entry, a found note, or nothing. `Back` is as today |
| T19.2 | A quick action takes 1, also on an instance's line. `Undo` steps back through the acts of the session being played, also after a restart. The opposite action corrects a value in 1 |
| T19.3 | A value that passes a mark raises a Due row above the trackers and the rail's count, on every page. The Run page shows no tracker part except the rail's count |
| T19.4 | Case D, beside the Session page: three notices with four rolls and the result line are in view with no scroll |
| T19.5 | The Timeline's day count raises a day mark's Due row and the rail's count, and raising it writes nothing. Days that pass in one step are one row with `Next`. A clock that goes back takes the row away and keeps what was done (P11) |
| G6 | The unchanged evening of the audit gives the counts it gave before |
| G7 | The plain campaign differs from its reference only by rows of section 2.5 |

**G7, the plain-campaign check.** One frozen campaign, with old task lines, old free-words effects, a marked
table and long names, is played by a script that uses no new control, at 1200x800 and 960x640, with Lookup
closed and open. For each screen the check compares four things with the reference: each control's and
text's **rectangle**, each row's **height**, the **words**, and the campaign's **files**. A difference passes
only when it is a row of section 2.5 that the pull request owns. The check holds the exact difference that
each row permits, the named control or words with its rectangle, and it masks no region. So a link that
moves beside `Add rewards`, a second line on the **Last time** line, an old effect that starts to count, a
changed `Back` and a file written with no act all fail. The reference lands in P1a. Each feature's
pull request proves that the check can fail, with four mutations: the difficulty word forced onto a line of
its own (a height); `Add rewards` forced onto a row whose title then wraps (a rectangle); a line `E` added to
the run's **Keys** list (words); `carried_from` written at `Create session` (files).

### 5.4 Proposed terms for `CONTEXT.md`

- **Encounter difficulty**: the word that compares an encounter's XP with the party's XP budget: Low,
  Moderate, High or Above High. Splat shows it and blocks nothing. _Avoid_: Rating, challenge.
- **Effect**: words that the DM pins to one participant of a run. It can have a time (a count of rounds, or
  on its turn), a count of uses, or a recharge. It stays in that run. _Avoid_: Custom condition, timer.
- **Reminder**: the words of an effect that has a time, on the run's notice line during its participant's
  turn.
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
  960x640**, and once at 1200x800, with the long content of its wireframes.
- It extends an existing scenario or test when that one has the same setup, and it breaks the code once for
  each new check and says so.
- For each result that stays on screen and each Undo, it tests an act A, an unrelated act B, then Undo of A.
  For each new write it tests the path of section 4.1: an edit on disk a moment before, unsaved words with a
  changed file, a failed save, and a restart.
- It runs G6 and G7, with its mutation of G7. It adds its own lines to `docs/design-system.md`,
  `docs/ux-intent.md`, `CONTEXT.md` and `README.md` from section 5, and its task to
  `src/visual_tests/dm_audit.rs` as a **new** task.

| PR | Scope | Files and modules | Depends on | Scenarios |
|---|---|---|---|---|
| **P0** | F8: merge pull request 232 when it passes its review, by the rule of F8 | As built | None | As built |
| **P1a** | The audit. Repair the script (drop the task that clicks `Add to notes` on a played fight; keep setup acts apart from counted acts). Freeze it and measure `main`. Add the plain campaign, its script and the G7 reference. Measure a `Display` title of 13, 16 and 19 letters for section 1.2, with a portrait and a scroll bar | `src/visual_tests/dm_audit.rs`, `scenarios.rs`, `fixture.rs`, `artifacts/dm-audit/` | P0 | New `plain-evening` |
| **P1b** | No change in behaviour. Move the plan pane to `src/sessions/plan.rs`, and the run's dock and notice line to `src/encounter_run/dock.rs` and `notice.rs` | Those files | P0 | None new. `visual-compare` shows no difference |
| **P1c** | The shared parts of section 5.1. `Draft::apply`, with today's two plan acts under it and their behaviour unchanged. `played_session`, called by `current_link` and `party_bar_shows` (section 2.5, row 19). An entry that opens while an NPC is held shows under its row (row 17) | `src/sessions.rs`, `src/sessions/page.rs`, `src/sessions/plan.rs`, `src/loot_editor.rs`, `src/design/components.rs`, `src/roll_table.rs`, `src/lookup.rs`, `src/lib.rs` (the Log stamp) | P1a, P1b | Extend `session-plan`, `run-loot`, `roll-table-author`, `npc-in-lookup`, `world-facts` |
| **P2** | F1 | New `src/difficulty.rs`. `src/encounter_settings.rs`, `src/encounters.rs`, `src/encounters/list.rs`, `src/sessions/plan.rs`, `src/encounter_run/view.rs` (Arrivals) | P1a, P1b, P1c | New `encounter-difficulty`. Extend `encounter-build`, `session-plan`, `encounter-groups` |
| **P3** | F3: scenes, `Scene` in the **More** list only, `Start fight` in a reader's header and in a scene row's menu, the made-up form's entry from it, `Remove…` for an encounter | `src/sessions.rs`, `src/sessions/plan.rs`, `src/design/components.rs`, `src/timeline/page.rs`, `src/world.rs`, `src/encounters.rs`, `src/encounters/list.rs`, `src/encounter_run/ad_hoc.rs` | P1a, P1b, P1c | New `session-plan-scenes`. Extend `session-notes`, `npc-in-lookup`, `encounter-list` |
| **P4** | F2. One entry type and one read rule for a condition, in every reader. The row's rule of section 1.4 in the shared row layout. A run file saved before P4, and one step that cannot be read | `src/encounter_run.rs`, `src/encounter_run/undo.rs`, `dock.rs`, `notice.rs` (part 3 of section 1.4, and row 20 of section 2.5), `view.rs`, `src/rolls.rs` | P1a, P1b, P1c | New `run-effects`. Extend `encounter-run`, `run-long-names`, `run-sync-conflict` |
| **P5** | F9. Check lines in quotes, tested through the older reader | `src/timeline.rs`, `src/timeline/formats.rs` (the event's new lines), `src/timeline/page.rs`, `src/sessions/plan.rs`, `src/roll_table.rs` (`per`) | P1a, P1b, P1c | New `travel-checks`. Extend `session-notes`, `roll-table-author` |
| **P6** | F6 | `src/world.rs` (the header band and the field), `src/world/lines.rs` (reuse `add_log`, `:459`), `src/sessions/plan.rs` | P1c, P3 | New `log-from-play`. Extend `world-facts` |
| **P7** | F7. An ordered read of the session notes that keeps an unreadable note in its place | `src/sessions.rs` (the block, the search for the newest block), `src/sessions/plan.rs`, `src/encounter_run/loot.rs`, `src/difficulty.rs` | P2, P3 | Extend `run-loot`, `session-plan` |
| **P8** | F5, with rows 11, 16 and 18 of section 2.5. The draft's provenance, and the exact inverse of `Undo carry over` | `src/sessions.rs`, `src/sessions/page.rs` (`last_time_block`, `recap_preview`), `src/markdown_view.rs` | P3 | New `session-carry-over`. Extend `session-plan` |
| **P9** | F4, the slice between fights: house rule notes, in play, the ledger with its save and its Undo, quick actions, instances, marks at a value with `for:`, the Due rows, the rail's count, `Add to Timeline`, the Trackers view and Ctrl+T. In the Tournament example: `for:` on the bells, `activation` not pinned (P10, which needed it, is dropped), and no line about **Make house rule note** | New `src/trackers.rs`, `src/trackers/ledger.rs`, `src/trackers/view.rs`. `src/lookup.rs`, `src/lib.rs`, `src/campaign_content.rs`, `crates/core`, the example note | P1a, P1c | New `trackers`, with the Session page's plan tab and `Start fight` beside the view. Extend `run-lookup`, `nav-state` |
| **P10** | Dropped (section 7): marks that a fight raises | – | – | – |
| **P11** | Day marks. Built: `at` and `every` with `from`, in days, on the Timeline's day count, `Next` and `Dismiss all` | `src/trackers.rs`, `src/trackers/ledger.rs`, `src/trackers/view.rs`, `src/lib.rs` (the act reads the day count at the click) | P9 | Extend `trackers` (T19.5) |
| **P12** | The audit, again. Extend the evening to use every feature, measure it, and add the scripts for J2, J3, J4 and J6. Run G6 and G7 on the whole | `src/visual_tests/dm_audit.rs`, `artifacts/dm-audit/` | All that are built | The audit's captures, and `plain-evening` |

**What can be built at the same time.**

- **First:** P0; then P1a and P1b at once, which share no file; then P1c.
- **Wave A, three at once:** P2, P3, P5. They meet in `src/sessions/plan.rs`, `src/timeline/page.rs` and
  `src/encounters/list.rs`, each in its own function.
- **Wave B, three at once:** P4, P6, P9. P4 and P9 share no file.
- **Wave C, two at once:** P7 and P8. Both call `Draft::apply`, so neither adds a save of its own.
- **Then:** P12, and P11 after P9. P10 is dropped (section 7). The full gate runs on merged source after each
  wave.

## 7. Limits

Each is a limit that this design keeps and states, with its count.

- **An overlapping edit is not merged** (today): with unsaved typed words and a changed file, the comparison
  of today offers one version, and a feature's act waits for that choice.
- **Lookup's place is not kept across a restart** (today), and a held form is lost with it. A found note is
  searched again after Lookup closed or showed an entry: 1 or 2 more (section 2.3, F3). `Back` is as today;
  P1c records these states in a test.
- **The rewards block does not follow the fight** after it is written (F7).
- **A reader's header at 960x640 with Lookup open** shows `Start fight` beside a title of up to about 11
  letters (143 px, measured in P1a), and the `Add to Log` icon up to about 17 (219 px); art and `Edit`
  together take about 72 px more. Section 1.2 counts the other routes.
- **A row's effects share its second line with today's words.** At 1200x800 it holds one counted effect and
  a count, or two counted effects with short names. At 960x640 with Lookup open it holds one long reminder,
  or one counted effect, and a count. The hover, **Conditions** and the notice line hold the whole words
  (section 1.4).
- **`Effect` by the pointer costs 5** when the dock has no room for it and Lookup is open (F2).
- **Removing a stray encounter costs 4,** with 2 page transitions (F3).
- **Continuing an older fight is cut** (F8): after a later fight or rest, the order of play would have to
  change, and change back, across two files. The DM starts a new run.
- **Where the note is behind its tab** (cases B, C and D), a typed line costs the tab changes: 2 in J2 and in
  J6.
- **Timeline offers no `Plan “…”`** (F9, P5). The offer needs the session's plan and its open note, which
  Timeline does not hold. The route that is left: `Open this session` in the event's detail, open the Travel
  row, `Plan “…”`: Nav 1, Find 1, Do 1 = 3, **T**.
- **A journey's table name does not open its note** (F9, P5). It is the button of the table menu, and one
  control does one thing. The route that is left: Ctrl+K, type the table's name, choose its heading: Panel 1,
  Find 2 = 3, with no page transition.

**Two rulings of the owner** (2026-11-11).

1. **Trackers in a fight.** The run gets no strip, no popover and no tracker part. In a fight the DM opens
   the Trackers view with Ctrl+T, presses, and goes back: three acts for each press, because Trackers is a
   view of Lookup and the run's own panel hides beside it. The rail's `Due N` still shows. The owner accepted
   this cost. P10 (marks that a fight raises) is dropped, and P11 (day marks) depends on P9 only.
2. **F7, an award that is not known.** The XP line names the creature whose XP Splat does not know, and
   leaves that amount out of the sum.

No question is open for the lead. While a run is open, a reader has no `Start fight` action.
