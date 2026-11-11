# Splat UX intent

This document states who uses Splat, what they try to do, and why each screen
exists. The design system and the redesign review use it as their brief. It
separates what the user needs from how the app works today. No current screen
is fixed. Full-page and cross-page changes are allowed.

**Reading order.** For the 10-minute version, read section 1, workflows W1 to W4,
section 3, and section 5. Sections 4, 6, 7, and 8 are reference tables.

Evidence: `file:line` refers to the working tree on 2026-11-01 (base `7a25305`).
Another agent is editing `src/`, so line numbers can move by a few lines.
No screenshots existed during this research. Layout claims come from the code.
Items marked *(estimate)* need a check against a screenshot.

**Interaction** has the meaning that `src/visual_tests/driver.rs` uses: one
click, one typed string, one key press, or one scroll gesture. To click a field
and then type in it is 2 interactions. All counts for "today" use this meaning.

## 1. Product intent

**Splat helps a Dungeon Master (DM) prepare D&D 2024 encounters and run them at
the table.** It keeps the rules, the party, and the combat state in one local
desktop app (`CONTEXT.md:3`, `README.md:3-5`). Files stay in the user's own
folders, often an Obsidian vault (`README.md:30-32`).

**Splat records rulings. It does not make them.** The DM resolves attacks,
conditions, and death. The run stores the result (`src/encounter_run.rs:1`,
`README.md:219-221`).

**Splat is not:**
- a virtual tabletop. Maps and geometry are out of scope (`docs/design/README.md:72`).
- a player app or a character builder.
- a publishing tool. Packages are local (`docs/design/packages.md`, Scope).
- a simulator today. Simulation is designed but is a placeholder (`README.md:5`,
  `src/main.rs:255-265`). Automatic combat resolution is also later work.

**The user.** One DM, working alone on one computer, with a D&D Beyond campaign
and players who keep their own sheets there (`docs/character-viewer.md:43`). See
assumptions A1 to A3.

**Two contexts of use.** They have opposite needs. Most design disputes come from
this difference.

| | Desk (between sessions) | Table (live session) |
|---|---|---|
| Attention | On the screen | On the people. The screen gets short looks |
| Time | Long, calm, chosen by the DM | Seconds, while someone talks |
| Interruption | Rare | Constant |
| Task | Read, author, compare, decide | Find state, record a change, look one thing up |
| Failure cost | A small delay | The table waits, and the DM loses the thread |
| Density need | Reading, scanning | Glanceable |

## 2. Workflows

Order is importance. Frequency values are inferred (A2). Test IDs map to section 8.

### W1. Run combat, turn by turn
**Trigger:** initiative is set, combat starts. **Goal:** always know whose turn it
is and keep every HP and condition correct. **Context:** table. **Frequency:**
every session, dozens of times per combat.

**Today (Campaign → Encounters, run open):**
1. `Next turn`: 1 interaction. It selects the current participant.
2. Damage on another participant: click the name, click `Amount`, type, click
   `Apply damage`: 4. Add 1 scroll if the controls are below the fold.
3. Condition: click the name, click the field, type the name, click `Add condition`: 4.
4. Monster turn: scroll down, expand `Creature details (starting snapshot)`, read.

**Friction:**
- The controls sit below all participant rows in one scroll area
  (`src/encounter_run.rs:707-736`). With about 8 rows at 960x640 they fall below
  the fold *(estimate)*. Selecting a name does not scroll to them.
- The current row is marked only by the text `CURRENT TURN` (`:712`). Nothing
  scrolls it into view, so in a long roster it can be off-screen.
- HP, temp HP, and AC share one plain text string (`:715`). Three explanation lines
  stay above the list during play (`:616-620`).
- `Amount` keeps its last value after use (`:744`, `:802-803`). This can repeat a
  wrong number. `Apply damage`, `Heal`, and `Replace temporary HP` have equal weight.
- `Save and close run` sits beside `Next turn` and closes with no confirmation
  (`:660-672`, `:679`). A mis-click loses undo history and sync (`:1119`, `:1128`).
- Conditions are typed from memory. The 15 official names exist only inside the
  sync panel (`src/combat_sync.rs:21-37`). A mistyped name stays local and does not
  sync (`src/encounter_run.rs:852-875`).
- `Undo` does not say what it undoes (`:654-659`). A dice roll takes an undo step.
- The dice control, `End encounter`, and the log are at the bottom of the scroll
  area (`:787-796`).
- There are no keyboard commands. The source has only Ctrl+S in the Library and
  Escape in the copy dialog (`src/library.rs:983`, `src/copy_dialog.rs:67`).

**Done well:** T1.1 to T1.8.

### W2. Look something up during a run, then return
**Trigger:** a player asks what a spell does, or a monster's turn starts.
**Goal:** read the text and keep the combat in view. **Context:** table.
**Frequency:** several times per combat. **This is the most important cross-page journey.**

**Today:**
1. Click `Library` (1). The run disappears, and no trace of it remains on screen.
2. Click `Search`, type (2). The top result opens by itself (`src/library.rs:806-807`).
   Click another result if needed (0 or 1).
3. Click `Campaign` (1). This always lands on Content (`src/main.rs:156-161`).
4. Click `Encounters` (1). Total: 5 or 6, with the run hidden throughout.

**Friction:**
- The run and the rules cannot be on screen together.
- Clicking `Campaign` while already in it also leaves the run (`src/main.rs:156-161`).
- Library search covers one package only, and only name, path, and section
  (`src/library.rs:33-37`, `:593-604`). It does not list campaign-owned entries.
- If the Library has an unsaved draft, every other entry is locked
  (`src/library.rs:765`, `:849`). The DM cannot look anything up without first saving.
- Spell links exist only in the Library reader. The run uses a plainer reader with
  no links and no styled stat block (`src/encounter_run.rs:779`, `src/library.rs:1050`).
- To see a player's full sheet: `Party`, member, page, then `Encounters`: 4.

**Done well:** T2.1 to T2.5.

### W3. Keep player state correct with D&D Beyond
**Trigger:** a player changes their sheet, or the DM damages a linked character.
**Goal:** both sides agree, and the DM knows at once when they do not.
**Context:** table. **Frequency:** optional. Every round if used (A10).

**Today:** select the participant, expand `D&D Beyond combat sync`, tick the
checkbox: 2 or 3 per character, and again after every resume (`src/encounter_run.rs:776`,
`:1128`). On conflict: select, expand, then `Use D&D Beyond values` or
`Keep Splat values and retry` (`src/combat_sync.rs:696-743`).

**Friction:**
- A participant row shows no sync state (`src/encounter_run.rs:711-721`). A conflict
  stays hidden in a collapsed section of one participant. The log only says
  "See sync status" (`:982`).
- Incoming changes clear undo history with no visible warning (`:985-987`).
- `Save and close run` is disabled during sync, but its reason is an `on_hover_text`.
  egui does not show that on a disabled control (`:662-665`, egui 0.33 `response.rs`).
- App close does not warn about pending sync (`src/main.rs:98`,
  `src/encounters.rs:293-296`), although the README tells the user to wait
  (`README.md:266-267`).
- Two HP editors exist for one character: the run controls and the sync panel.

**Done well:** T3.1 to T3.5.

### W4. Go from prep to the table: start or resume a run
**Trigger:** the session starts, or the app restarted mid-session. **Goal:** reach the
first turn, or the exact turn left off, fast. **Context:** boundary of desk and table.
**Frequency:** 1 to 4 times per session.

**Today, new run:** app opens on Library (`src/main.rs:79-80`). `Campaign`,
`Encounters`, open `Select encounter`, pick it, `Run encounter as DM`: 5. Then
`Roll NPC initiative` (1). For each player: tick `Initiative`, click value, type: 3.
`Begin combat` (1). With 4 players the total is 19.

**Today, resume:** `Campaign`, `Encounters`, expand `Saved runs`, `Resume`: 4.

**Friction:**
- The app does not remember the last page or an open run.
- During setup, rows re-sort on each initiative change (`src/encounter_run.rs:600-602`).
  The next row can move while the DM enters values.
- Setup and live play share one screen and one layout.
- Resume selects the first participant, not the current one (`:1120`).
- Saved run labels are name, round, and status. They have no date (`:1143-1152`).
  There is no way to delete a run. Runs are not grouped by encounter.
- Saved runs and run settings are in collapsed sections of the builder page.
- A resumed ended run could not be reopened, because undo was not saved (`:261`, `:1119`). Now the run file keeps
  the Undo steps, and **Continue fight** takes an ending away from the fight that was played last (T17.1 to
  T17.4, T17.8).

**Done well:** T4.1 to T4.6.

### W5. Build an encounter
**Trigger:** the DM plans the next session. **Goal:** a saved roster the DM trusts.
**Context:** desk. **Frequency:** a few per session of prep.

**Today (4 players, 2 goblins, 1 boss):** `Campaign`, `Encounters`, `New encounter`,
name (2), `Add all cached party members`, search (2), `Add`, `Duplicate`, search (2),
`Add`, `Save encounter`: about 14.

**Friction:**
- Search results show a name only. No CR, HP, or AC (`src/encounters.rs:675-680`).
  To judge a monster, leave for the Library and return: 5 or more.
- Results sit below the roster in one scroll area. The list is not virtualised, with
  about 340 built-in creatures (`:675`).
- `Move up` has no `Move down` (`:649`). Phases and notes need the Markdown editor
  in another workspace (`README.md:198-200`).
- A new draft counts as dirty until saved (`:92`). `Reload` is disabled while dirty
  (`:498`), and it drops the open draft when it runs (`:308`).
- **Defect:** edit `Run folder` with no draft open, then click `Save encounter`. The
  button is enabled and calls `unwrap` on a missing draft (`:522-528`). The app should
  panic. From code reading only; not run.

**Done well:** T5.1 to T5.5.

### W6. Read rules and content
**Trigger:** the DM studies a monster, a class, or a rule. **Goal:** find it, read it
in comfort, follow related entries. **Context:** desk, long reading. **Frequency:**
every prep session.

**Today:** the app opens here. Search: 2. Pick: 0 or 1. Follow a link: 1.

**Friction:**
- Links clear the search and have no Back (`src/library.rs:485-500`).
- Three readers exist: the Library reader, `markdown_view`, and the character sheet.
  Campaign entries and runs get the weakest one (`src/campaign_content.rs:665`).
- The header, `New entry`, validation, and search stack above the reader
  (`src/library.rs:581-737`). This uses height at 960x640.
- List subtitles show schema terms such as "Definition · creature"
  (`src/library/presentation.rs:85-94`). Most DMs want CR or spell level instead (A12).

**Done well:** T6.1 to T6.4.

### W7. Read a character sheet
**Trigger:** the DM needs a player's stats or spells. **Goal:** the answer, and how fresh
it is. **Context:** desk and table. **Frequency:** weekly at the desk, sometimes live.

**Today:** `Campaign`, `Party` (Content opens first), member card, page tab: 4.
`Filter` on 4 pages adds 2.

**Friction:**
- Member cards show name, level, and class only (`src/party.rs:120-127`). To compare
  AC or HP, open each sheet.
- `Source & parity` holds technical data at the same level as `Overview` (`:245-252`).
- New characters do not reach the builder until `Reload` (`README.md:196-197`).

**Done well:** T7.1 to T7.3.

### W8. Make content your own and use it
**Trigger:** the DM wants a changed monster or a homebrew spell. **Goal:** the entry is
in the encounter. **Context:** desk, cross-page. **Frequency:** sometimes.

**Today (copy a monster into the campaign and add it to an open encounter):** `Library`,
search (2), result, `Copy to…`, open destination, pick, `Review copy`, `Create copy`,
`Close`: 10. Then `Campaign`, `Encounters`, `Reload`, reopen the encounter (2), search
(2), `Add`, `Save encounter`: 9. Total: about 19. To change the copy: `Campaign`,
`Content`, `Campaign entries`, entry, `Edit Markdown`, then YAML by hand.

**Friction:**
- The copy dialog refreshes Library and Content, but not Encounters (`src/main.rs:277-280`).
- If the encounter draft is unsaved, `Reload` is blocked (`src/encounters.rs:498`).
- All authoring is raw Markdown and YAML (`src/library.rs:886`, `:1026-1031`). This
  may be acceptable (A1).
- Campaign entries have no search (`src/campaign_content.rs:559-599`).
- The UI says "module" in Campaign and "package" in Library. `CONTEXT.md:30` says
  avoid "module". Use **package**.

**Done well:** T8.1 to T8.4.

### W9. Choose what a campaign uses
**Trigger:** a new campaign or a new homebrew package. **Goal:** the right content is
available, and missing dependencies are clear. **Context:** desk. **Frequency:** rare.

**Today:** `Campaign` → Content → `Modules`: tick a package or entries, `Save selections`: 3
or more. The built-in default means many DMs never need this.

**Friction:** this rare setup task is the first screen of the Campaign workspace
(`src/main.rs:160`). Invalid notes appear as repair editors above the tabs
(`src/campaign_content.rs:162-197`). Save is disabled with no stated reason (`:242`).

**Done well:** T9.1 to T9.2.

### W10. Set up and switch campaigns
**Trigger:** first launch, or a second campaign. **Goal:** a working campaign, linked if
wanted. **Context:** desk. **Frequency:** rare.

**Today:** `⚙ Settings`, open sign-in in a browser, copy the `CobaltSession` cookie from
developer tools, paste, `Connect`. Then `Add from D&D Beyond`, pick a campaign, `Browse…`,
`Open campaign`. To switch: `Campaign`, then the selector.

**Friction:** the campaign selector is visible only in the Campaign workspace
(`src/main.rs:162-171`). `Remove from Splat` and `Unregister package` have no confirmation
or impact count (`src/campaigns.rs:402`, `src/library.rs:679-683`). A run or draft blocks
switching (`src/main.rs:164`), which is correct and must stay.

**Done well:** T10.1 to T10.3.

### W11. Keep the session's notes and its timeline
**Trigger:** the DM prepares a session, or something happens at the table. **Goal:** the notes are in the
campaign's folder, and each encounter, rest, journey, day and major event is on one timeline that any note
can refer to. **Context:** desk and table. **Frequency:** every session.

**Done well:** T11.1 to T11.4, and T11.8.

### W12. Record a session and process it later
**Trigger:** the table agrees to be recorded. **Goal:** the audio is safe, its state is plain, and later it
becomes a raw transcript, a clean transcript with event markers that a second pass has checked, and a draft
summary, from services the DM chose. **Context:** table to
start and stop, desk to process. **Frequency:** optional.

Recording records. It does not rule: a marker is a suggestion, and only the DM puts it on the timeline.

**Done well:** T11.5 to T11.7.

### W13. Prepare arrivals, and add them to a fight
**Trigger:** rival adventurers, reinforcements, or a released beast can arrive in any room, or in the one room they
were prepared for. **Goal:** the group is ready once, is offered only where it belongs, and it joins the running fight
in one step when it arrives, with its changes in plain view. **Context:** desk to
prepare, table to add. **Frequency:** every fight that has arrivals.

An encounter group is added by the DM. Splat records it. It does not decide that the group arrives. A change
to a creature is never hidden: the editor and the card show the old value, the new value, and what the DM must
add by hand.

A creature can also arrive with nothing prepared. The DM finds it in the campaign, or names it and types its
HP, and it follows the same initiative rules as a group. A creature made up this way stays in that one run.

**Done well:** T13.1 to T13.7, T14.6.

### W14. Run the evening from one page
**Trigger:** the DM sits down at the table, or a fight ends. **Goal:** the next thing to do is on the page already: the
recap, the plan, the note, the rest to record. **Context:** table. **Frequency:** every session. The session's page holds
the plan, so that a DM never chooses between Encounters, Timeline and the note in the gap between two fights. A measured
evening is in `artifacts/dm-audit/README.md`.

When a fight ends, the party searches it. The ended view lists the encounter's loot and the loot of each creature
at 0 HP, and each roll on a loot table is one button, with Undo.

While a session is played, a party bar stays above the Session page: each player's portrait and HP, then AC,
passive Perception and Insight, and spell save DC, as many as the width allows, with the rest on hover. A click
opens the sheet. The numbers come from the open fight, then tonight's fights, then the sheets, and the bar says
which. A session is played when a fight is open, a recording is live, or the current session is not dated
before today. The DM can hide or show the bar, and Splat remembers it. The Run page has no bar: its rows
already show the party.

The plan also holds scenes, with a done mark. A creature's or an NPC's page starts a fight that nobody planned.

**Done well:** T12.0 to T12.12, T14.1 to T14.5, T14.7 and T14.8.

## 3. Information architecture

### Current model
The sidebar has two levels: `Library` or `Campaign`, then 5 tabs under Campaign. `⚙ Settings`
is a value of the same `Tab` enum (`src/main.rs:77-93`).

| # | Problem | Evidence | Effect |
|---|---|---|---|
| 1 | Clicking a workspace overwrites the tab | `main.rs:150-161` | Returning to Campaign always shows Content |
| 2 | A run is not a place | `encounters.rs:417-432` | It has no nav item and no trace on other pages. Its only hint is text about changing campaigns (`main.rs:167-171`) |
| 3 | Settings is a tab in both workspaces | `main.rs:183-185, 205` | Two items look selected at once |
| 4 | Dead destinations | `main.rs:177-178, 255-265` | `Creatures` and `Simulations` are placeholders. The text "content editing… not implemented" is false |
| 5 | Prep-rare screen first | `main.rs:160` | Content opens before Encounters and Party |
| 6 | Content is split by owner | `library.rs:593`, `campaign_content.rs:537` | Three lists, three searches, three readers |
| 7 | Campaign setup is split | `campaigns.rs:341`, `encounters.rs:457` | Link and folder in Settings; run folder and initiative in Encounters |
| 8 | Data goes stale between pages | `main.rs:277-280`, `README.md:197` | A manual `Reload` that drops the draft |
| 9 | Windows behave differently | `main.rs:144-146`, `library.rs:634` | Copy blocks the app; `Manage packages` does not |
| 10 | The app starts on Library | `main.rs:79-80` | A DM at the table starts 5 interactions from a run |

### Recommended model
One flat level, always in the context of the active campaign. The design stage chooses the
visual form (sidebar or top bar).

**Top-level**
- **Run**: exists only while a run is open. Always first. It is a place, not a mode of a tab.
- **Encounters**: runs in progress, encounters ready to run, encounter groups, and a collapsed History of ended runs under their sessions. Reached first when a campaign is active.
- **Sessions**: session notes, session templates, recordings and their processing.
- **Timeline**: every encounter, rest, travel interval, day and major event, apart from any one session.
- **Party**: sheets.
- **Library**: one catalogue of built-in, package, and campaign-owned entries. Default scope
  is what this campaign can use, with a switch to all. Editing and `Copy to…` stay here.
  Browsing must not change the active campaign (`README.md:115`).
- **Campaign**: package selections, rule choices, run folder, D&D Beyond link, folder, and repair.
- **Settings**: account connection and the campaign list. App-level only.

Remove `Creatures` (the Library covers it) and `Simulations` until it exists. When built,
simulation is an action on an encounter, because it answers a question about one encounter (A9).

**Reachable from anywhere**
- **Run strip:** while a run is open, every page shows round, current participant, a sync warning,
  and `Next turn`. One click returns to the run.
- **Lookup:** one shortcut or click opens search and a reader beside the current page. It does not
  navigate and does not take away the run strip.
- **Campaign switcher:** always visible. Blocked with a stated reason during a run or draft.
- **Status:** connection state, and which drafts are unsaved.

**Modal, and only these:** discard unsaved work on close or switch; an action with no undo that
removes access to data; the copy review, as a focused task. **Never modal:** damage or condition
entry, sync conflicts (the table must not stop), lookup, errors, confirmation messages.

### What must persist

| State | Across navigation | Across restart | Today |
|---|---|---|---|
| Current page and sub-view | Yes | Yes | Lost on workspace click; lost on restart |
| Open run: selection, amount, scroll, undo | Yes | Run yes, selection = current turn, undo yes (A8) | Navigation yes. Restart keeps the run, its turn and its Undo steps |
| Sync enabled per participant | Yes | Yes, with a fresh read first | Lost on resume |
| Library: scope, section, search, entry, scroll | Yes | Scope and section | Navigation yes, expected for scroll; verify |
| Lookup: last query and entry | Yes | No | Does not exist |
| Reading history for Back | Yes | No | Does not exist |
| Library and campaign drafts | Yes | Warn on close (A8) | Navigation yes |
| Encounter draft | Yes, **and through a data refresh** | Warn on close | Lost on `Reload` |
| Party: member, page, filter | Yes | Member | Navigation yes |
| Active campaign | Yes | Yes | Yes |
| Column widths the DM dragged | Yes | Yes | Yes |

## 4. Per-screen intent

These describe need, not layout. Screens can merge or split.

| Screen | Question it answers | Primary action | Key information, in order | Secondary | Density | States to handle |
|---|---|---|---|---|---|---|
| **Run: live** | Whose turn, and who is hurt? | `Next turn`; apply change to a participant | Round, current participant; next up; each row: name, side, HP, temp, AC, conditions, concentration, absent, 0 HP, sync | Current stat block, dice, notes, log, encounter notes | Glanceable | HP unknown; 0 HP; absent; all absent; autosave failed; changed on disk; sync states (section 6); long roster; long names; many conditions |
| **Run: setup** | Is everyone ready to start? | `Begin combat` | Missing initiative or HP; initiative order; ties | Roll mode, NPC flag, initiative rule in use | Scanning | Missing HP; missing bonus (`Roll` off); no NPCs; tie order locked by rule; absent at start |
| **Run: ended** | What happened? | Close; undo ending if still possible | Outcome, end reason, final round, final HP | Log | Scanning | Undo available or not; controls read-only |
| **Encounters** | What can I run, and what is in progress? | Start or resume a run | Runs in progress with date; templates; roster summary | Run history, new encounter | Scanning | No campaign; loading; load error; no encounters; unreadable run; content issues |
| **Encounter editor** | Is this roster what I want? | Add participant, save | Roster with side and key stats, and each change with its base value; creature results with CR, HP, AC | Overrides, initiative group, reorder, notes | Scanning | New; dirty; saved; validation error; external change; unavailable creature; party-slot binding; needs repair; no party copies; no search results |
| **Group editor** | Is this set of creatures what I want to add? | Add creature, save | Members with counts, and each change with its base value; default side; notes | Overrides, reorder | Scanning | New; dirty; saved; empty; unavailable creature; invalid change |
| **Run: arrivals** | Who can arrive now, and what do they add? | Add one creature, add a group, or drag a group to the list | Each group's name, members, count, XP | Notes, a typed total | Scanning | No groups; a group that cannot be added, with its reason; XP not known 
| **Library: browse/read** | What does this say? | Search, read | Title, type, summary, body; related links | Engine support, source, technical details | Reading | Loading; load error; no results; empty section; invalid note; read-only; unavailable link; validation messages |
| **Library: author** | Is my entry correct and saved? | `Save` | Editor text; parse and validation errors; save state | `Copy to…`, `Discard changes`, `New entry`, packages | Reading | Clean; dirty; saved; invalid note; duplicate identity; external change; name clash; built-in read-only |
| **Lookup** (new) | What does this say, right now? | Type, read | Result name, type; entry text | Open in Library | Glanceable to reading | No results; loading first time; draft elsewhere does not block |
| **Campaign: packages** | What can this campaign use? | Tick, `Save selections` | Package state (all, some, none); required dependencies and reasons; unavailable packages | Entry list and search | Scanning, 2,000+ rows | No manifest; dirty; saved; partial; required locked; unavailable package; content issues; repair blocks save; missing folder |
| **Campaign entries** | What does this campaign own? | Open, edit | Name, kind, text | `Copy to…` | Reading | Empty; no selection; dirty; invalid note repair; save error |
| **Party** | What is this character's state, and how fresh? | Choose member, read | Name, HP, AC, status with age; page content | `Refresh party`, Combat sync, Source & parity | Scanning, then reading | Not linked; no session; offline cached; refreshing; live; polling; no members; no local copy; fetch error; storage warning |
| **Sessions** | What did we do, and what is next? | Write notes; `New session` | Session list; notes; save state; recording state | Read view, events, recordings, templates, folder | Reading | No session; new form; unsaved; changed on disk; save failed; no recorder; recording in another session |
| **Recordings** | Is the audio safe, and what became of it? | `Record`, `Stop recording`, `Process` | Live state and time; each recording's length and state; where Process sends it | Transcript, marked transcript, summary, markers, delete | Glanceable to record, reading after | Consent step; hearing; silent; stalled; stopped by itself; not configured; processing; cancelled; failed; processed |
| **Timeline** | What happened, in what order? | Add an event, read | Days; events in story order; kind; duration, place, result; session | Filters, search, reference, mentions, move, remove | Scanning | Empty; no match; changed on disk; unreadable file; save failed; removed with undo |
| **Sync panel** | Do Splat and D&D Beyond agree? | Resolve conflict | Conflict: Splat value, D&D Beyond value; pending; last match | Slots, limited uses, conditions | Glanceable when in conflict | Off; first read; matching; pending; conflict; read-only; wrong campaign; duplicate character; error; disconnected |
| **Settings: account** | Am I connected? | `Connect`, `Disconnect` | State; error or keyring warning; steps | Trust text | Scanning | Busy; connected; not connected; refresh failed; keyring unavailable; removal failed |
| **Campaign list** | Which campaigns exist, where? | `Open campaign` | Active campaign, folder state, link state | Details, remove | Scanning | None; invalid settings (read-only, blocked); folder unavailable; offline cached name; link conflict; blocked by draft or run |
| **Copy review** | What exactly will this create? | `Create copy` | Destination, name, path; dependencies | Markdown preview | Scanning | Loading; no destination; plan stale; private dependency block; save error; complete with `Close` and `Copy another` |
| **Unsaved on close** | What will I lose? | Return to the work | What is unsaved, and where | Discard and close | Glanceable | Library draft; content draft; encounter draft; run failed to save; pending sync |
| **First launch / no campaign** | What do I do first? | Add campaign | One next step; the Library still works | Connect D&D Beyond | Glanceable | No campaigns; none active; settings invalid |

## 5. Design principles

**1. The table wins.** When a choice helps the run but costs prep, choose the run. *Dispute:* a
shared sidebar helps desk navigation but costs run width at 960 px. Reduce the sidebar.

**2. Prominence during a run follows this order:** (1) round and current participant, (2) every
participant's HP and conditions, (3) the current participant's actions, (4) entry controls, (5)
everything else. *Dispute:* a large stat block or a neat action form that pushes rows off-screen
loses. The DM must be able to read items 1 and 2 in one look, from a normal seated distance, with
no scrolling and no hover.

**3. Never leave the run to use the run.** Lookup, stat blocks, sheets, and sync conflicts open
beside the initiative list. *Dispute:* a clean full-page reader against a smaller side panel. In a
run, the side panel wins, and `Next turn` must stay clickable.

**4. Act from the row. Undo, do not confirm.** Frequent actions start on the participant row and
take no dialog. Mistakes are fixed by a named `Undo` ("Undo: 7 damage to Goblin 2"). *Dispute:* "Are
you sure?" on damage or `Next turn` is rejected. Confirmation is only for work that undo cannot restore.

**5. Reading is its own mode.** Long text gets one column, about 60 to 75 characters per line,
comfortable line spacing, a clear heading scale, and no competing controls. Every entry uses one
reader everywhere, including the run. Stat lines and tables keep alignment. Source, identifiers,
and engine support stay collapsed. *Dispute:* a reader that fills a wide window, or a dense "power
user" reader, is rejected. Dense layouts are for lists, not for rules prose. A session's own notes
and transcripts are the exception: they follow their column, at the width the DM chooses.

**6. Show what is unsure.** Stale, offline, local-only, unsaved, conflicting, read-only, and
unsupported states are always visible where the DM looks, as words or icons as well as colour.
*Dispute:* a sync dot in a collapsed panel, or a colour-only state, fails. A disabled control always
states why.

**7. Record, do not rule.** The interface must not imply a rule was applied. Zero HP does not remove
a turn. A condition does not change a number. *Dispute:* greying out or skipping a 0 HP participant
automatically is rejected. A visible "0 HP" label with a one-click `absent` is accepted. Engine
support is never shown as approval of an entry.

## 6. Design direction constraints

**Window.** Minimum 960x640, default 1200x800 (`src/main.rs:29-30`). All run essentials fit at the
minimum with 12 participants, and work with 20 by list scroll alone. Also check high-density displays.

**Density.** Three levels, from section 4: glanceable, scanning, reading. A level belongs to a
screen, not to the app. Run rows are compact but use large type for HP and name. Library rows stay
efficient at 2,065 entries.

**Keyboard.** The run needs commands for `Next turn`, `Undo`, focus on the amount, and open Lookup.
Numeric entry commits with Enter. Lookup opens, types, and closes with the keyboard. Ctrl+S saves
every draft, not only Library drafts. Escape closes the top layer. Tab order follows reading order.
Pointer use remains complete: the DM may have one hand on dice.

**Pointer targets.** The run's frequent controls have large targets and clear separation. `Next turn`
is never next to a run-closing or destructive control.

**Long sessions.** Sessions can last hours in dim light (A4). Avoid large bright areas and motion that
draws the eye. The current code mixes theme-aware text with fixed light grey (`src/library.rs:534-538`
against `src/encounter_run.rs:614`, `src/encounters.rs:401`, `src/party.rs:37`). On a light OS theme,
those fixed screens are likely to be hard to read. Every theme shipped must be tested on every screen.

**Colour meaning.** Colour is one signal, never the only one. Each meaning below is stable across
screens and is not reused for decoration.

| Domain | States that need a distinct meaning |
|---|---|
| Turn | Current; next up; not current |
| HP | Healthy; damaged; Bloodied (half or less; A11); 0 HP; temp HP (separate from HP); unknown (`?`) |
| Participant | Present; absent; concentrating; each condition as a readable label |
| Side | Party; enemies; other named sides, without implying good or bad |
| Sync | Off; first read; matching; pending; conflict; read-only; error |
| Connection | Connected; offline with cached data; refreshing; live; polling |
| Document | Clean; unsaved; saved; invalid; external change; read-only |
| Result | Error; warning; success, each with a word as well as a colour |

**Data scale.** Design with real volumes: 2,065 Library rows (`artifacts/list-scrolling/README.md:5`),
including 339 spells, 415 items, 336 creatures, and 975 rule entries (counted from
`builtin/dnd2024-definitions`). 51 search results for "dragon". Note bodies across `builtin/` have a median of
about 270 words and a maximum near 5,600 words. Lists
must stay virtualised (debug median 2.3 ms per frame today). Encounter creature results must join them.
Expect a run log of hundreds of lines and an unbounded list of saved runs.

**Long text in tight places.** Handle long names ("Rohan of Unc"), numbered copies ("Goblin 12"), 5 or more
conditions on a row, and long condition text from custom effects. Truncation always has a way to the full text.

**Terms.** Use `CONTEXT.md` terms: package, not module; campaign, not project. Avoid schema words in everyday
text: "reference", "definition", "UUID", "frontmatter". Keep them in technical details.

## 7. Open assumptions

Nothing below has direct evidence. Please check A1, A3, A4, A5, and A9 first.

| ID | Assumption | Evidence | Recommended default |
|---|---|---|---|
| A1 | One experienced, technical DM is the only user. Raw Markdown authoring is acceptable | Personal paths and one campaign in the docs; cookie paste from developer tools | Do not design for beginners or add a form editor now. Keep first-run clear |
| A2 | All frequency values in section 2 | None | Use them for priority only |
| A3 | Laptop or desktop with mouse and keyboard at the table. No touch. No second screen | Linux desktop build only | Keyboard and pointer. No touch work |
| A4 | Players can see the screen but do not read it. It stays a DM view. The room may be dim and sessions are 3 to 5 hours | The heading says "DM view". No player view exists | No spoiler mode and no player display. Dark-first, tested in both themes if both ship |
| A5 | Rosters are usually 5 to 12, and up to 20 | Party of 3 in the docs. Code limit of 2,000 is only a guard | Design for 12, test 20 |
| A6 | Players roll real dice and tell the DM their initiative | Run text and `README.md:210` allow both | Typed totals are the fast path. `Roll` remains |
| A7 | The DM reads monster actions on most monster turns | `README.md:221` lists details as "reference" only | The current stat block is visible or 1 interaction away |
| A8 | Drafts stay in memory. A run's Undo steps are saved in its file | `README.md`, Run storage | Warn on close for drafts. A run that closes or ends by mistake comes back with its Undo |
| A9 | Unifying Library and campaign entries is acceptable. Removing `Creatures` and `Simulations` is acceptable | `README.md:115` keeps Library separate from the campaign. Tabs exist as placeholders | One catalogue with a scope switch. Simulation returns as an encounter action |
| A10 | D&D Beyond sync is used live. Live writes are still unverified | `README.md:271-273` | Show sync state in full, but the run works fully without it. Do not show it for unlinked participants |
| A11 | Bloodied is worth showing as a state. Splat does not compute it today | 61 built-in notes mention it, and the SRD defines it (`grep builtin/`) | Show the label. Add no automation |
| A12 | The DM wants CR, spell level, or item type in lists and creature results. The designer must not invent values | Data exists in entries; not shown today | Show existing fields only. If a field is missing, show nothing |

## 8. Workflow-to-test mapping

**Types:** `cost` = `Driver::goal` interaction count against the target. `view` = `is_visible` or a
geometry check at the stated size. `keys` = keyboard run with `keyboard_result`. `state` = assert text
or control after a scripted step. `look` = screenshot for human review. Sizes are 960x640 unless stated.
Scenarios marked **new** do not exist in `src/visual_tests/mod.rs:37` yet. "Fail" means expected from
reading the code, not from a test run.

| ID | Observable test | Scenario | Type | Today |
|---|---|---|---|---|
| T1.1 | `Next turn` takes 1 and is visible with 12 participants | encounter-run | cost, view | 1, pass |
| T1.2 | Damage to a named non-current participant in ≤ 3 (target, type, commit), without leaving or scrolling the list | encounter-run | cost | 4 to 5 |
| T1.3 | Whole roster of 12 and the damage entry are visible together | run-large-roster **new** | view | Fail *(estimate)* |
| T1.4 | After every `Next turn` over a full round of 20, the current row is in view | run-large-roster **new** | view | Fail |
| T1.5 | Official condition on a participant in ≤ 3, without typing the full name | encounter-run | cost | 4, typed |
| T1.6 | `Undo` names the action before it is clicked | encounter-run | state | Fail |
| T1.7 | Next turn, damage, undo, and lookup work by keyboard alone | encounter-run | keys | Fail |
| T1.8 | Round, current, HP, temp, AC, conditions, and concentration read without hover or expansion; 0 HP and absent are distinct without colour | encounter-run | look | Partly |
| T2.1 | From a run, read a spell in ≤ 3. Roster and `Next turn` stay visible and clickable | run-lookup **new** | cost, view | 3 to 4; run hidden |
| T2.2 | Return to the run in ≤ 1. Selection and scroll unchanged | run-lookup **new** | cost, state | 2 |
| T2.3 | Lookup finds built-in, package, and campaign-owned entries with no package switch | run-lookup **new** | state | Fail |
| T2.4 | Spell named in a run stat block opens in ≤ 1 | run-lookup **new** | cost | Fail |
| T2.5 | Lookup works while a Library draft is unsaved. Draft is kept | run-lookup **new** | state | Fail |
| T3.1 | Each synced row shows sync state with nothing expanded | run-sync-conflict **new** | state | Fail |
| T3.2 | A conflict shows on the row and the run strip, does not block `Next turn`, and resolves in ≤ 2 | run-sync-conflict **new** | cost, state | Hidden; 3+ |
| T3.3 | Enable sync for the whole party in ≤ 2 | run-sync-conflict **new** | cost | 2 or 3 per player |
| T3.4 | A message appears when undo history is cleared. Accepted limit: if the list leaves no room for one more line, only the Log records it | run-sync-conflict **new** | state | Fail |
| T3.5 | App close with pending sync warns. A disabled close states its reason | dialogs | state | Fail |
| T4.1 | Cold start with a saved encounter to run setup in ≤ 3 | prep-to-table **new** | cost | 5 |
| T4.2 | Cold start to the interrupted turn in ≤ 2, current participant selected | run-resume **new** | cost, state | 4; first row |
| T4.3 | A player's initiative in ≤ 2. Rows do not move during entry | prep-to-table **new** | cost, state | 3; rows move |
| T4.4 | Unfinished runs show their encounter, date and status, and come first | encounter-build | state | Fail |
| T4.6 | An unfinished run appears only in In progress unless Keep in Ready is on, which blocks Start fight there. Its encounter returns to Ready when it ends. Ended runs wait under their session in a collapsed History. Opening one adds nothing to the timeline | encounter-history | state | Fail |
| T4.5 | Setup and live views differ clearly. Live has no setup controls | encounter-run | look | Partly |
| T5.1 | Creature results show existing CR, HP, and AC | encounter-build | state | Fail |
| T5.2 | Read a full stat block with the draft open in ≤ 2, without leaving the editor | encounter-build | cost | 5+ |
| T5.3 | Roster of 4 party, 2 + 1 monsters saved in ≤ 10 | encounter-build | cost | About 14 |
| T5.4 | Creature results stay smooth at about 340 rows | encounter-build | state | Not virtualised |
| T5.5 | Edit `Run folder` with no draft, click every enabled button: no panic | encounter-build | state | Panic expected |
| T5.6 | With sync on, the roster shows what the party has now. With it off, the roster says nothing reaches D&D Beyond | encounter-build | state | Fail |
| T6.1 | Prose line length ≤ 75 characters at 1200x800 and 1920x1080; reader usable at 960x640 | library-browse | view, look | 760 px cap |
| T6.2 | Back after a link returns to the prior entry and scroll in 1 | library-browse | cost, state | Fail |
| T6.3 | List scroll at 2,065 rows stays at or below today's median | library-browse | state | 2.3 ms debug |
| T6.4 | One reader: same entry looks the same in Library, Lookup, and run | library-browse | look | Fail |
| T7.1 | Open a participant's sheet from the run in ≤ 1, roster still visible | party-from-run **new** | cost, view | 4; hidden |
| T7.2 | Freshness text is visible on every sheet state | party | state | Pass |
| T7.3 | Sync is on with no click, and leaving Party does not stop it. Only a lost connection does, and the rail says so | party | state | Fail |
| T8.1 | Copy a Library entry to the campaign, then add it to an unsaved encounter draft, with no manual reload and no draft loss | library-to-encounter **new** | state, cost | Blocked, about 19 |
| T8.2 | Read another entry while a draft is open | library-author | state | Fail |
| T8.3 | Failed save (invalid note, external change, name clash) keeps the draft and states the fix | library-author | state | Pass |
| T8.4 | Find a campaign-owned entry by search | campaign-content | state | Fail |
| T9.1 | Required dependencies and their reasons are visible with the built-in package open | campaign-content | view | Pass, verify |
| T9.2 | Save disabled by repair states the reason. The repair is reachable without leaving | campaign-content | state | Reason missing |
| T10.1 | First launch shows one next step. Library works with no campaign | empty | state, look | Partly |
| T10.2 | No campaign: each page says what to do, and no page is blank | empty | state | Partly |
| T10.3 | Connect states: busy, error, keyring warning, connected, each with words | settings | state, look | Partly |
| T11.1 | A titled session from a template in ≤ 4, or 2 with no title. The note holds the filled template | session-notes | cost, state | New |
| T11.2 | A long rest on the timeline in ≤ 2, from the session's page | session-notes | cost | New |
| T11.3 | An encounter run puts itself on the timeline, in the newest session. Reading an older note does not change that | session-notes | state | New |
| T11.4 | An event has a reference. A note can link it, reads as its words, and the event lists the note | session-notes | state | New |
| T11.5 | Recording needs a stated step, and its true state stays in view on every page, including a stall or a stop. Stop keeps the file | session-recording | state | New |
| T11.6 | Process states what is missing, and where audio and words go, before anything is sent. A failure stays in view on every page | session-recording | state | New |
| T11.7 | A marker reaches the timeline only when the DM adds it. What the second pass changed can be read | session-recording | state | New |
| T11.8 | A note during a run in ≤ 2, beside the roster, in the session being played | session-notes | cost, view | New |
| T14.1 | A plain note reads in Splat, with no change to the file | world | state | New |
| T14.2 | A link in a note opens the note it names, as in Obsidian | world | state | New |
| T14.3 | A marked table rolls, with a typed result as well, and its row shows | world | state | New |
| T14.4 | A rule found by its heading, beside a run, in ≤ 2 | world | cost, view | New |
| T14.5 | A new NPC joins the fight, and the run stays on screen | world | state | New |
| T14.7 | The generator gives a name and three attributes, and writes nothing until the DM saves | world | state | New |
| T15.1 | A page shows each line on another page that names it, under that page, and a press opens the line's page | world-facts | state | New |
| T15.2 | A group lists its members, each with what it is | world-facts | state | New |
| T15.3 | A page shows the dated entries that concern it, oldest first | world-facts | state | New |
| T15.4 | A change in play writes one Log line in the entry, the session lists it, and one undo removes the value and the line | world-facts | state | New |
| T15.5 | The World page lists the open questions and the problems, and each opens its page | world-facts | state | New |
| T16.1 | An action tile arms in 1, and every row shows the chance to hit it | run-rolls | cost | New |
| T16.2 | A click on a row rolls against it and Enter applies the damage, in 2 | run-rolls | cost | New |
| T16.3 | A second attack against the same target applies in 2 | run-rolls | cost | New |
| T16.4 | An area save against two creatures, then its damage, in ≤ 6 | run-rolls | cost | New |
| T16.5 | Ctrl+number arms an action in 1 | run-rolls | cost | New |
| T16.6 | A player's attack with their own typed damage in ≤ 4 | run-rolls | cost | New |
| T16.7 | A Library tile previews an attack in 1 | run-rolls | cost | New |
| T16.8 | A tile's right-click menu arms an attack with advantage in 2 | run-rolls | cost | New |
| T16.9 | A row's right-click menu rolls against it with disadvantage in 2 | run-rolls | cost | New |
| T16.10 | An attack whose damage depends on a condition asks, then rolls, in 3 | run-rolls | cost | New |
| T16.11 | A row's menu rerolls one die of a player's roll with inspiration in 3 | run-rolls | cost | New |
| T13.1 | The run's group panel lists each group's members and total XP | encounter-groups | state | New |
| T13.2 | A group joins the running fight in ≤ 1 with a roll, or with a typed total. Undo removes every member | encounter-groups | cost, state | New |
| T13.3 | A changed creature shows its base and changed values in the editor and on the run's card, marked as changed | encounter-groups, run-overrides | state | New |
| T13.4 | Dragging a group onto the initiative list adds every member | encounter-groups | state | New |
| T13.5 | A run offers its encounter's groups and those for any encounter, and adds its own in ≤ 1, in setup and in combat. The Encounters page keeps an encounter's own groups in a closed section | encounter-owned-groups | cost, state | New |
| T13.6 | An encounter's editor shows each of its groups in a frame of its own, and builds a new one there, with no other page. It saves with the encounter | encounter-owned-groups | state | New |
| T13.7 | One creature, made up or found, joins the running fight from the Arrivals panel. A made-up creature says what it needs, and copies get numbered names | run-add-creature | cost, state | New |
| T12.1 | A planned fight starts from the session in 1, and Close run comes back there in 1. The plan is words in the note | session-plan | cost, state | New |
| T12.2 | A new run starts each player from the last fight of the session, says so, and goes back to the sheets in 1. A long rest the DM marks starts everyone at full | session-plan | cost, state | New |
| T12.3 | One amount on several targets, with half for those who saved, in 2 once chosen, and one step of undo | session-plan | cost, state | New |
| T12.4 | Dice roll beside the amount in ≤ 3, with the stat block in view | session-plan | cost, view | New |
| T12.5 | Escape selects the current turn. A fight's final HP reads in the plan in 1 | session-plan | cost, state | New |
| T12.6 | A new session shows what the last one left, with nothing to open | session-plan | state | New |
| T12.7 | Setup and Prep show the read-aloud text. Technical details are closed | session-plan | state | New |
| T12.8 | A condition's rule opens from its word on the row, and is gone again, in 2 | session-plan | cost | New |
| T12.9 | Lookup finds words in a session note and keeps the run in view | session-plan | state, view | New |
| T12.10 | A task line is a scene with a box. A tick changes one character. A plain list line gets no row. The empty plan is as today | session-plan-scenes | cost, state | New |
| T12.11 | A scene's link opens its entry in Lookup in 1, and `Back` returns to an earlier entry | session-plan-scenes | cost | New |
| T12.12 | A reader starts a fight in 1, and a scene row's menu in 2. An NPC with no stat block opens the made-up form with its name and focus in HP. After `Close run`, also after a restart, a second `Start fight` starts the same encounter. `Remove…` and `Undo remove encounter` remove and restore an encounter with no saved run | session-plan-scenes, npc-in-lookup, encounter-build | cost, state | New |
| T12.0 | A creature made the campaign's own in ≤ 7, with no page change. Party compares passive scores with no sheet open. A Library save after an encounter save is not a conflict | session-plan | cost, state | New |
| T14.2 | A wrong initiative is corrected during combat, as one step of undo, and the current turn stays | session-plan | state | New |
| T14.3 | Exhaustion is one level with a control, and each step replaces the last | session-plan | state | New |
| T14.4 | A fight planned a second time can start a second time | session-plan | state | New |
| T14.5 | After a restart, Close run goes to the page the run began on, in the session the DM chose | session-plan | state | New |
| T14.6 | A group is assembled in the session's plan, with the note beside it. It is linked in the plan when saved, and comes first in the run's Arrivals panel. With a fight open, its row goes to that panel in 1; a group none of whose members can join offers Edit group instead, so its repair stays the next step | session-plan-groups | cost, state | New |
| T14.8 | A creature at 0 HP with loot says Loot on its row, and its card lists the loot. The ended view lists the encounter's loot and each downed creature's, and a loot roll is one step of undo | run-loot | state | New |
| T14.1 | An interrupted run says Resume, and can be deleted with one step of undo. Ended runs are in the history | session-plan | state | New |
| T17.1 | After Close run and Resume, after a restart and after another page, Undo names and takes back the same step on the same turn, in ≤ 1 | run-continue | cost, state | New |
| T17.2 | An ended fight has one control that takes the ending away: Undo while the ending is the newest step, otherwise Continue fight. Play returns to the turn the fight ended on in 1, and Undo puts the ending back with its loot | run-continue | cost, state | New |
| T17.3 | A closed, ended fight that was played last goes on in ≤ 2 from History and in ≤ 3 from the plan's played row | run-continue | cost, state | New |
| T17.4 | After Continue fight, Timeline, the plan row and the Encounters lists show a live fight. After Undo, and after a second ending, they show an ended one. Close run puts the last result in `Timeline.md` at once | run-continue | state | New |
| T17.5 | A run whose saved Undo steps cannot be read opens on its turn, says so once, and Undo starts again | run-continue | state | New |
| T17.6 | The ended view, the History row, the sentence that stands where Continue fight would be, and the unread-steps message are whole with Lookup open and closed at 960x640, and none of their actions closes or changes Lookup | run-continue | view, state | New |
| T17.7 | After Continue fight, End encounter starts with the outcome of the ending that Continue removed, selected: Enter keeps it and typing replaces it | run-continue | state | New |
| T17.8 | Continue fight is offered only for the fight that was played last. After a later fight or long rest, and while `Timeline.md` cannot be read, the ended view and History offer no Continue fight and say why in its place. When that is no longer so, the control is back | run-continue | state | New |
| G1 | Every disabled control states why, in text or a working tooltip | all | state | Fail |
| G2 | Go through every top-level page and back: each page restores section 3 state | nav-state **new** | state | Fail |
| G3 | No dead nav items | nav-state **new** | state | Fail |
| G4 | All pages have no ergonomics violations at 960x640 and one high density | all | `ergonomics.rs` | Not measured |
| G5 | Read text stays readable under every shipped theme | all | look | Fail on fixed-grey screens |

**New scenarios, in priority order:** `run-lookup`, `prep-to-table`, `run-resume`, `run-large-roster`,
`run-sync-conflict`, `nav-state`, `party-from-run`, `library-to-encounter`.

The `Seed::Campaign` fixture already has a run in progress and a party (`src/visual_tests/fixture.rs:35-37`).
`run-large-roster` needs a roster of 20. `run-sync-conflict` needs a simulated D&D Beyond response, as the
sync tests use (`README.md:271`).
