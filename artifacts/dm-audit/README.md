# DM audit: one evening of play

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
with **Continue fight** (`README.md`, Running an encounter as DM).

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
