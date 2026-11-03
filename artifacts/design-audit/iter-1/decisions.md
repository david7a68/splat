# Iteration 1: design decisions

These are the lead designer's decisions on the iteration-1 findings. The findings are in
this folder: `cross-page.md` (X), `run-fight.md` (F), `prep.md` (P), `session.md` (S),
`record-sync.md` (R), `content.md` (C) and `setup.md` (U). Each finding's "Proposed fix"
column is accepted **as written**, except where a decision below changes it. A finding
that is not named in any package is deferred.

## Shared rules (every package)

R-1. **Words.** `Record` means audio only. The run panel's corrections tab is `Correct`. The
button that writes a DM note to the run log is `Add note`. A button that clears a value is
`Mark unknown`, never `Unknown`. A fight is a `Fight` in every add control and filter.
`Changed` is the one word for an override (F10). `Campaign` is the one owner word (C20).

R-2. **Toggles.** A toggle that is on uses the `selected` surface, a check or its icon, and
its word. It never uses the accent fill. The accent fill is for the one primary action in
view (F9).

R-3. **Destructive actions.** `Delete`, `Remove` and similar actions never sit beside a
primary action. In a list row they go into a row menu: a quiet `⋯` icon button at the row's
right edge, named `More actions for <name>`. `Discard` is quiet and sits at the left of a
save row, beside the save-state word. `Save` sits at the right (X4, U13, F19, P21, C13).
Red is only for an action that loses something (P19).

R-4. **One primary.** At most one accent-filled button in view. While a run is open, the run
strip owns `Back to run` and the only primary; other pages show a run's state as a
`status_link` such as `Running · Round 2` that goes to the run (P4, S5).

R-5. **Disabled controls** say why in visible `Small` text beside them, not only on hover
(G1; P16, P18, P23, P26, R16, C11, U15).

R-6. **Empty states** use `empty_state` (what this is, and the one next step), placed at the
top of the empty pane with the pane's normal padding. No bare sentence at the top left.

R-7. **Lists keep a measure.** A page that is one list of rows with actions (Encounters,
Settings campaigns) is at most `LIST_MEASURE` = 880 px wide, so a row's actions stay near
its name on a wide window. Help text stays within `READER_MEASURE` (X11, U15).

## Package A: shell and design system (owner of `src/design/`)

A1. **Page header contract** (X1, X9). `page_header` is: title; one context line in
`Small` muted on the title's baseline; actions on the right; a hairline under it on every
page. The context line says this page's state or counts. It never repeats the campaign
name, which the switcher above the rail shows. Header actions: at most one bordered
secondary (the page's `New …`), the rest quiet, in the order quiet then bordered. Apply on
Encounters (`2 ready · 1 in progress`), Session (`2 sessions`), Timeline (`4 events · Now:
Day 1, hour 8`), World (`3 notes`), Library (`2,108 entries`), Party (status icon and word,
`· Fetched 3 h ago`, moved up from its own line), Campaign, and Settings. The Library scope
switch moves from the title line to the top of the category column. The encounter editor
and the run keep their own task headers.

A2. **Group headings** (X2). Every heading that groups rows in a list uses `Type::Label`
(capitals) with a muted count after it, and no rule: `NOTES 2`, `IN PROGRESS 1`. Serif
headings are only for what is read (reader and stat block). Apply to Library, World,
Encounters, Settings campaigns, Party sheet lists.

A3. **New buttons.** Library gets a `+ New entry` header button (U4, C4). The form opens in the
reader column and closes once the entry is made. Kind names in the form use plain words.
World gets a `+ New ▾` header button with `NPC`, `Location`, `Generate NPC` (C19).

A4. **Empty states** (X5, C23, U17, U5, U6, U7, P25, F14). World reader: "World notes are the
Markdown files in this campaign's World folder." with `+ New ▾` as the next step. Library
reader: `empty_state`; keep the open entry when it is still in the new scope. Library with no
own entries: hide the category column; one `empty_state` across list and reader. No
campaign: `No campaign yet` / `No campaign open`, with `Add a campaign`. U5: the add form sits
in the Campaign page's empty state. Made-up creature card: F14.

A5. **Settings** (U1, U8 to U12, R3, R12, R18, R19, U14, U18, U21). Campaigns are always the
left column, the account the right. Campaign rows: `Open` (or `Active` status), `Edit
details`, and `⋯` with `Remove…` (R-3). Statuses use `status` (U10). D&D Beyond words:
`Sign-in failing`, `Signed out of D&D Beyond` (R3).

A6. **List rows** (X10, C12, U16). World rows drop the file name. A list row's suffix omits
what the scope already says. Source suffix: show it only when it differs from the
campaign's main source, with one rule for every kind. Rows with the same name show the
parent heading, not a page number.

A7. **Gallery and docs.** Update `docs/design-system.md` for A1, A2, R-2, R-3, R-4, R-5, R-6
and R-7, and add the page header, `empty_state`, `status_link`, a blocked button with its
reason, and the row menu to the gallery.

## Package B: the run and arrivals

B1. **Panel tabs** (F1, F21, C16, U19). One tab row in two groups: participant tabs
(`Stat block`, `Conditions`, `Correct`) at the left under the participant's name, and fight
tabs (`Log`, `Prep`, `Arrivals`) at the right after an `L` gap. When a fight tab is open,
the panel title reads `This fight`. When the row does not fit on one line, the fight group
becomes one `Fight ▾` menu. The tab row never wraps.

B2. **Words** per R-1 (F4, S6, C6, F18). **Toggles** per R-2 (F9).

B3. **Changed values** (F2). The stat block's AC, HP and initiative trio shows the run's value,
with `printed 12` in `Small` muted under it when it differs. Keep the list of changes at the
top of the tab. F10 words.

B4. **Close dialogs** (F3, R10, P19). One dialog layout for every close: title says what is
unfinished (`Sync has not finished` or `This run could not be saved`), one sentence on what
is lost, then `Keep running` (default, secondary), the saving way out as primary when one
exists (`Save as a new run and close`), and red `Close and lose changes` only when something
is lost. A close that loses nothing is secondary. One footer label: `Close after sync`.

B5. **Arrivals** (F5, P11, F6, P7, F15, F22, P14). A group card is two lines, about 64 px:
line 1 is the name (the drag handle), side, then `4 members · XP 250` at the right; line 2 is
the member summary (`2 × Wolf, Captain Varra`) in muted, then a `Total` field (placeholder
`Roll`, help `Blank rolls initiative` as its tooltip and accessible description) and `Add`.
The help paragraph becomes one muted line at the top of the tab. A broken group shows each
fault on its member's line, with a warning icon, the creature's name, formatted numbers, and
an `Edit group` link. Added groups say `Added in round 1` and keep `Add`. Search results use
the editor's two-line result row without `Read`.

B6. **Absent** (F7, P10). Setup and live rows: a worded `Absent` toggle (eye-slash icon and
the word), styled per R-2 when on. An absent row shows the `Absent` status (icon, word,
colour) and the turn order marks it.

B7. **Setup rows** (F8, F11, P17, P16, F17). `No initiative` is a link that opens an initiative
field in that row. Setup rows build their second line from the live row's parts (side mark
and word, HP readout with the shield for temp HP, AC on every row). `Begin combat` is blocked
with its reason beside it. Tie arrows per F17.

B8. **Run notices and sync** (R2, R4, R11, R20, P18, F12, F13). As proposed. R4: a run that
loses its sign-in shows `Sign in again` beside the notice, which opens Settings with the
cookie field focused.

B9. **Other** (F16 deferred). C17, C18, C8 as proposed. F20: the group and encounter change
editors use the run's condition picker and its `Other effect` field.

## Package C: Encounters, the editor and run state

C1. **Encounters page** (X7, X4, P1, P2, P12, P13, P15, P20). One column, at most
`LIST_MEASURE` wide. Sections in this order, each a group heading per A2: `IN PROGRESS`,
`READY TO RUN`, `GROUPS` (closed when it has rows, as now), `HISTORY` (closed). A row is the
name, a second line, and actions at the column's right edge: its main action, a quiet
`Edit`, then `⋯`. An encounter with an unfinished run shows the status `In progress · Round
2` and `Resume` as its action; `Start another fight` and `Delete the saved run` go in `⋯`
(P2, S1). An ended run opened from History opens read-only in place and does not take the
open-run slot (P1). `Edit` and `New encounter` work while a run is open; only `Start fight` is
blocked, with its reason (P12). History rows carry their date, and `Run again` becomes
`Start fight` (P20). `Content issues` is a warning status that opens the list (P15). Saved
groups have `Remove` (P13).

C2. **Paused run** (P3). When an unfinished run exists but is not open, the rail shows a `Run`
item with `Round 2 · Paused`, which resumes it in one click.

C3. **Encounter editor** (X8, P5, P6, P24, U20, P23, P9, P21, P22). A roster row is: name and
second line, side menu, quiet `Edit`, and `⋯` (`Move up`, `Move down`, `Duplicate`, `Make it
yours`, `Remove`). Creature results: the row's name opens the stat block beside (as `Read`
does now), and the row has one `Add` button; a party member already in the roster shows the
muted status `In roster`, not a disabled button. `Sync party with D&D Beyond` and `Keep in
Ready after a run ends` move under the roster, above `READ-ALOUD AND NOTES`. Side is the same
dropdown everywhere, with `Other…`, always capitalised (P6). The header keeps its actions on
line 1 and lets the name field shrink (P24). `Add party` reads `Party in roster` when it is
(P23). Plan group editor per P9, P21, P22.

## Package D: the Session page, Timeline and Party

D1. **Session page structure** (X6, S9, S23, S11, S15). Remove the `Session notes /
Templates / Folder` tab row. `Templates` becomes a quiet header action that opens the
templates view, with a `← Sessions` back link. The session folder setting moves to Campaign →
`Link and folder`. The session header is one title line with the date in muted after it,
then one line `Last time: <session title> — <first line of its recap>` where the title is a
link to that session (no disclosure arrow); the line is cut to one line and the full text is
the link's tooltip. Remove the `NOTES` label. The session list marks the session being
played with a `Playing` status. An older session shows one warning with a link to the
session being played (S11). The rail uses the Session page's conflict words (S15).

D2. **Plan** (S1, S2, S3, S4, S5, S12, S13, S14, S21, S22). Rows per S1 and R-4 (a running
fight's row shows `Running · Round 2`, no button). In Read view, drop `Linked in this note`
and drop the `On the timeline` items that are already plan rows (S2). A played fight row per
S3: name link, `Ended in round 3` line, the party's final HP on one line, then quiet links,
with `Remove` in `⋯`. One message and one Undo (S4). Empty plan: `empty_state` ("Nothing
planned yet.") above the Add strip; the strip stays where it is. S13, S14, S21, S22 as
proposed.

D3. **Add strip** (X3). A shared `add_strip` component in `src/design/components.rs` (add
only; Package A owns the rest of that file): the word `Add` in `Label`, then quiet buttons
with icon and word, no border, wrapping as needed. One order and one set of words on both
pages: `Fight`, `Group` (plan only), `Day`, `Long rest`, `Short rest`, `Travel`, `Major
event`. The Timeline filter `Encounters` becomes `Fights`.

D4. **Timeline** (S7, S8, S16 to S20). As proposed. S18: the detail column appears only when
an event is selected.

D5. **Party** (P8, P26 to P30). As proposed.

## Package E: recording, content and recovery

E1. **Recording** (R5 to R9, R13 to R17, R21). As proposed. R14 uses `segmented`.

E2. **Sync undo** (R1). A sheet change drops only the undo steps that touch that character.
The DM's choice in a conflict is a named undo step. If keeping the other steps is not safe
for some step, drop that step and say so in the Log.

E3. **Lookup** (C1, C2). As proposed: up to 5 one-line results with the open one marked; one
`New NPC "<query>"` line only when no exact match exists; `Generate NPC` only with an empty
query.

E4. **Copy and Library authoring** (C3, C9, C10, C11, C13, C14, C15, C21, C22). As proposed.

E5. **Reader tables** (C5). The first column and the header row stay fixed while a wide table
scrolls sideways.

E6. **World rolls** (C7). As proposed.

E7. **Campaign repair** (U2, U3). As proposed. **Conflict view** (S10): disk and draft side by
side in one face, with differing lines marked.
