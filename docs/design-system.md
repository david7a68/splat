# Splat design system

The brief is `docs/ux-intent.md`. This document records the visual decisions and why they were made. The code
is `src/design/`. Tokens are the only source of colour, size and type. A lint test (`src/design/lint.rs`)
keeps other code from adding raw values.

## Art direction: a working ledger, at night

A DM keeps a ledger for hours in a dim room. The screen must feel like a well-set page and a confident tool.
It is not a dashboard, and it is not parchment.

- **Warm dark, one theme.** The app forces `ThemePreference::Dark`. All neutrals lean warm (hue about 35°).
  The page is `#1B1815`, not black. Text is paper `#E9E1D3`, not white. There are no large bright areas.
- **One accent: blue-black ink, inverted (`#A9BDE6`).** Most state colours are warm or earthy (HP damage,
  warnings, conflicts). Temporary HP and concentration are cool, and they are told apart from the accent by
  shape and word. The party's side mark is a warm neutral, so that it is not read as the accent. The accent
  marks the current turn, the primary action, focus, and links. It is not decoration.
- **Hierarchy from type, spacing and hairlines.** Surfaces carry meaning only: the rail is recessed
  (navigation), a well is an input, a raised layer floats above the page. No cards around everything, and no
  pill badges. A status is an icon, a word, and a colour.
- **Two voices.** A sans for working UI. A serif for what the books say: entry titles, rules prose, and stat
  blocks. The change of voice tells the DM "this is reference".

## Type

Fonts are in `assets/fonts/` with their licences. All are OFL. egui 0.33 rasterizes with `ab_glyph` and has no
variable axes or OpenType features, so these are static files.

| Face | Use | Why |
|---|---|---|
| IBM Plex Sans (Regular, Medium, SemiBold, Bold) | UI | Tabular digits by default and a plain zero, so numbers read fast and hold their place with no special drawing. `1`, `l` and `I` differ. Holds up at 12 px and from a seated distance |
| Source Serif 4 (Text, Italic, Semibold) | Prose | A sturdy text serif with lining tabular digits, so `65 (10d8 + 20)` lines up |
| Source Serif 4 Subhead (Semibold) | Titles | The optical cut for 18 to 28 px |
| Source Serif 4 SmText (Regular, Semibold) | Stat blocks | The optical cut for 13 to 14 px, for the compact table at 1x |
| egui's monospace | Library and template editors, code in a note | Source text only. `Type::Mono` |

`design::number` right-aligns ordinary text. HP and initiative hold their place because the font's digits have
one width. Do not paint digits in fixed cells.

**Fallback.** Every UI family falls back to Source Serif 4 for symbols such as `× – — ½ ¼ ¾ ⅓ ⅔`, then
Phosphor, then egui's defaults.

**Scale.** Each role is one `design::Type` value. Do not add sizes outside it.

| Role | Face, size / line | Use |
|---|---|---|
| `Display` | Subhead Semibold 26 / 32 | Entry title, sheet title |
| `Title` | Subhead Semibold 20 / 26 | Page title, current turn |
| `Section` | Subhead Semibold 17 / 23 | Reader section heading |
| `Heading` | UI SemiBold 14 / 20 | UI group heading |
| `Body` | UI Regular 14 / 20 | Default UI text |
| `Strong` | UI Medium 14 / 20 | Button, selected item |
| `Name` | UI SemiBold 15 / 20 | A participant's name on a run row |
| `Small` | UI Regular 12 / 16 | Second line, help, status |
| `Label` | UI SemiBold 11 / 14, tracked +0.6, capitals | Column and compact group heading |
| `Numeral` | UI SemiBold 14 | AC, initiative, outside the run's scale |
| `NumeralLarge` | UI Bold 18 | Current HP, outside the run's scale |
| `Round` | UI Bold 18 | The round number in the run header |
| `Prose` | Serif 16 / 24 | Rules text at full width |
| `ProseCompact` | Serif 15 / 22 | Rules text in a side panel |
| `ProseStrong` / `ProseItalic` | As prose | Named actions, emphasis |
| `StatLabel` / `Stat` | SmText 13 / 18 | Stat block rows |
| `Mono` | Monospace 13 | Markdown source |

**Measure.** Lists of rows with actions use `LIST_MEASURE` (880 px), so actions stay near the names.
`READER_MEASURE` is 488 px at `Prose`, about 62 characters on average and at most 75 (T6.1). Prose never fills
a wide window. Every reader starts at the pane’s left edge, including Library; its text keeps
`READER_MEASURE`. A Markdown table in the Library or World reader is not prose: it takes its natural width,
past the measure, up to the pane's visible right edge, as a table in a narrow pane already does
(`markdown_view::with_wide_tables`). Lookup and
other side readers keep their tables at their own width.

**The run has its own scale.** `RunScale` holds a row height and type sizes, and `RunScale::fit(height, width,
rows)` takes the largest that fits with no scrolling.

| Scale | Row | Name | HP / max | AC, initiative | Conditions | Turn word | Roster width |
|---|---|---|---|---|---|---|---|
| `RUN_COMPACT` | 36 | 16 | 20 / 13 | 15 | 14 | 12 | 560 |
| `RUN_REGULAR` | 52 | 18 | 24 / 15 | 16 | 14 | 13 | 640 |
| `RUN_LARGE` | 68 | 22 | 30 / 18 | 19 | 16 | 14 | 760 |

## Colour tokens

`design::colors()` returns the active `Colors`. Names say what a colour means. Each family below maps to
section 6 of the brief. **Colour is never the only signal:** each state also has a word, an icon, or a shape,
and `src/design/tokens.rs` tests the contrast pairs (text 4.5:1; large text and UI 3:1).

| Family | Tokens | Words and shapes that go with them |
|---|---|---|
| Surface | `page rail raised well hover selected current_row` | Recessed, raised, sunken |
| Line | `hairline rule field_border` | Decorative rules, and the 3:1 field edge |
| Text | `text muted faint disabled` | Primary, secondary, tertiary |
| Accent | `accent accent_fill accent_hover on_accent focus` | Focus also has a 2 px ring |
| Recording | `recording` | `Recording` with its time and a record icon, in the rail on every page |
| Turn | `turn.current turn.next` | The word `Turn` and a bar; the word `Next` |
| HP | `hp.healthy damaged bloodied zero temp unknown track` | Damaged HP uses the text colour and a part-filled bar; other numbers are tinted by state, with a bar under them, `Bloodied`, a neutral mark with `0 HP` (a skull would be a verdict, and death is the DM's ruling), `+5` with a shield and a strip above the bar, `?` |
| Participant | `participant.absent concentrating condition changed` | `Absent`, `Conc`, each condition by name, a neutral muted pencil and `Changed` |
| Side | `side.party enemies other` | The side name, with a square, a diamond or a circle. No red and green |
| Sync | `sync.off first_read matching pending conflict read_only error` | Icon and word in every case |
| Connection | `connection.connected offline busy` | Icon and word |
| Document | `document.clean unsaved saved invalid external read_only` | Icon and word |
| Result | `result.error warning success info` | `message` adds an icon and a bar |

The colour-distance test keeps changed participants and damaged HP distinct from warning and conflict tokens.

## Space, shape and density

- **Grid:** 4 px. Tokens `XS 4, S 8, M 12, L 16, XL 24, XXL 32`.
- **Radii:** `RADIUS_CONTROL 3`, `RADIUS_LAYER 6`. Nothing is round for its own sake.
- **Density** is a level a control asks for, by what it is for, and not a setting of the screen. A screen
  mostly keeps to one, and may use a lighter one for what is rare:

| Level | Control | Row | Mostly used by |
|---|---|---|---|
| `Glance` | 36 | see `RunScale` | The run's `Next turn` and damage entry |
| `Scan` | 30 | 32 | Run setup, Encounters, Party, Lookup results |
| `Read` | 28 | 28 | Library, Settings, Campaign, and the run's footer |

Every control the app draws is at least 24 px (WCAG 2.5.8, and the `target-size` check). The exception is
egui's own scroll bar, which is 8 px. The check reports it as chrome. It stays 8 px because egui paints the
handle at the width it senses, and the wheel and the page keys also scroll. Tab steps over it
(`Splat::skip_unnamed_focus`). The check counts a node as a bar only if it has no role and no name and matches
the full length of a scroll surface's edge. A reader table's sideways bar has its own lane below the last row.
While that lane is below view, egui floats the bar up to the reader's visible bottom edge, and the lane goes with
it: rows pass under the lane's hairline, as they pass under the fixed header at the top, so the bar never covers
cell text. Cells under the lane are out of view and are not controls. The bar stays hidden while its table is
below view.

Reader tables keep each column at least as wide as its longest word. The prose measure never makes a table
scroll: a table that fits the pane shows whole. A table that cannot fit the pane scrolls sideways
and says how to do that. The first column stays at the reader’s left edge, with a hairline beside it that runs
through the header; a table that fits has no column rules. A rollable table keeps a slot for the roll mark
before every row's range, and its first column is sized with the slot, so the rolled row's range stays at the
others' x. The
header stays at the reader’s top edge until the table leaves view. Ordinary wheel scrolling still moves down
the note. The ergonomics check permits side clipping only where a real horizontal scroll bar can reveal the
text; it still rejects clipping in ordinary readers and text cut to an ellipsis.

A column's drag handle stays egui chrome: 10 px wide and as tall as the column. Tab gives it focus, shown
by an accent line. Left and Right move its edge by `L`. Home restores its natural width, which follows the
window again. The chosen width is saved after a drag or key press. Each handle has an accessible name and
key help. At the table, the rail uses its natural width; a wider desk width stays saved.

Lookup, Notes, Settings and the account line keep fixed places at the rail's bottom edge. The account line
has a slot of two `Small` lines below Settings; a one-line status leaves the second line empty, so an account
status that wraps never moves Lookup, Notes or Settings. Rail statuses (recording, processing, unsaved words,
sync) fill one slot that grows upward above Lookup, so a status that comes, goes or wraps never moves these
targets. Tab order still follows reading order.

No rail target moves when a state comes or goes. The Run slot is always the first item: the open run, the
paused run, or, with no run, a quiet `Run` with `No fight open` on line two that opens Encounters. All three
are one height, so Encounters, Session and the rest keep their places when a run opens or closes.

## Widgets

`theme::install` sets one global `egui::Style` for hover, active and disabled states, focus rings, selection,
text edits, solid scroll bars, windows, tooltips and separators. Text cursors do not blink. Motion draws the
eye during long sessions.

`design::components` holds the shared parts, each a function or a small builder:

| Part | Function | Notes |
|---|---|---|
| Page header | `page_header` | Title and one `Small` muted state or count on the same baseline; actions at the right; hairline, then an `L` gap. Never repeat the campaign name. The title and its count go first: the actions get the width they leave, and a status link among them wraps there, so the count is never cut |
| Nav item | `nav_item` | Icon and word; accent bar when selected |
| Buttons | `Action::primary / secondary / quiet / destructive` | `.blocked(reason)` disables and adds a hover reason and accessible description. For one blocked control, show `blocked_reason` beside it in `Small` text, then an `L` gap before the next action. `.key("N")` shows the key beside the word |
| Split button | `split_action` | Default `Action`, then a caret menu for other actions |
| Field | `field`, `small_field` | Sentence-case muted `Small` label, control, help and error; accessible name. The help is `help` |
| Help | `help` | Help under a field or a section: `Small` muted words that wrap at `READER_MEASURE`, whatever the pane's width |
| Text fields | `text_field`, `text_input`, `number_field`, `search_field`, `named` | Accessible name and a 26 px target; left-aligned numbers keep the caret off digits; placeholders use faint `Body` text |
| Editable numbers | `stepper` | A value the DM can change: − and + beside the number in a bordered well. A value without a stepper is read only, so the two never look alike. The D&D Beyond sheet panel uses it for every value Splat can send; in each of its blocks the labels share one column (the widest label plus `S`) and every well has the widest value's width, so each − and each + share an x. At a limit, − or + is disabled; when the caller gives the limit's reason, that button says it on hover and in its accessible description. An enclosing lock's reason comes first |
| Dialog | `dialog`, `dialog_actions` | `egui::Modal`, for the brief's modal tasks only; one action line, red at the left, keep and commit at the right, commit last |
| Section | `section`, `quiet_section`, `section_icon`, `combo_icon` | Collapsing, with Phosphor arrows; `quiet_section` for technical details at a page's end |
| Table row | `table_row` | Fixed cell widths and hairline for Markdown tables and sheets; no stripes; height follows text |
| Rail toggle | `nav_toggle` | Lookup and panels; its own `Open` word distinguishes it from the current page |
| Segmented | `segmented` | Words with an underline, no pill |
| List row | `row`, `two_line_row` | Fixed height for `show_rows`; paints hover in the current frame |
| Status | `status`, `status_link`, `Status`, `rail_status_detail` | Icon, word and colour; `status_link` opens where the DM can act. A two-line rail status paints its icon and its word where a one-line status paints them; its second line starts at the word |
| Stat tile | `stat_tile` | Sentence-case muted `Small` label over a large value, no box |
| HP | `hp_bar`, `hp_readout`, `hp_state` | Fixed slots; separate temporary HP |
| Empty state | `empty_state` | Icon, serif title with no full stop (`Nothing planned yet`), purpose and one next step at the top of an empty pane or page, with normal padding |
| Group heading | `group_heading` | `Label` capitals and muted, nonzero count; no rule; serif headings belong to readers and stat blocks |
| Submenu | `menu_submenu` | Match quiet Read actions: `S` horizontal padding, Strong type and Read height. Opens right of its menu when it fits inside the opening pane with `S` to spare, else left, else as far right as fits; never over a splitter or scroll bar |
| Row menu | `row_menu` | Quiet `⋯` at the right, named `More actions for <name>`; holds Remove and Delete. Opens under the `⋯`, and ends at its right edge when it would cross the pane's splitter or scroll bar |
| Add strip | `hanging_add_strip` | `Add` in its own Label column; one line of quiet icon-and-word buttons. Show all seven actions when they fit. Otherwise keep Fight, Group (plan only) and Long rest visible; put Day, Short rest, Travel and Major event in one compact `⋯⌄` menu named `More Add actions`; hover text names its actions. Keep the strip on one line |
| Message | `message`, `small_message`, `message_with_action`, `Tone` | Inline icon and bar; `message`, `message_with_action` and `undo_message` start the words `XS` after the icon whatever the caller's spacing; `message`'s words end at their last painted word, not at the space a wrapped row broke at; cap the notice at `LIST_MEASURE`; actions follow text after an `L` gap or wrap below, aligned with the text; measure the whole action with padding; move it below and align it with the text if it cannot fit; actions without icons use a secondary border. `message_with_actions` puts several choices below the words: the first starts at the text edge, wrapped lines start there too, and a red or dismissing choice keeps exactly `L` from the one before it (subtract the row's item spacing) |
| Message space | `MessageSpace`, `fixed_part` | The one space below a save row, `MESSAGE_SPACE_LINES` (two) body lines high whatever it holds. It holds Save's reason and the latest result in one part: a result whose Undo is live comes first, then the reason; otherwise the reason comes first. The result says whether its Undo is live (`has_undo`); the space chooses the order. Longer words wrap and scroll inside the part (`fixed_part`): one focusable group named with every message in that order, a lane kept for its scroll bar, arrows and Page Up or Page Down scroll it while it has focus |
| Side panel | `side_panel_width`, `panel_header` | One width rule for Lookup, Notes and the run; separate participant and fight tabs |
| Column | `Column::left / right / panel` | Rail, page lists and panels; inner-edge drag handle; natural width until dragged or moved with keys, then saved width across restarts; preserve page room and `RUN_LIST_KEEPS`; adjacent content reaches the column edge, with text measured inside it |
| Link | `link` | Real `Link` role, at least 24 px high. A word narrower than 24 px reserves 24 px of layout width from the link's own minimum size, never from spaces in the text: the word starts at the text edge, the underline covers only the word, and the next item starts after the whole target, so two short links side by side never share a press area. An icon before the words is not part of the underline: draw it as its own glyph, as `status_link` does |
| Number | `number`, `number_width` | Right-aligned digits |
| Count | `count` | `1 entry`, `2,108 entries`; zero gives no text; empty unit words give only the formatted number |

All form labels use sentence-case muted `Small`, including `field`, `text_field`, `small_field` and
`small_text_field`. Use `Label` capitals for group and column headings. A heading over one input is a field label, including `Group notes` and `Read-aloud and notes`. These two notes fields come from one builder (`notes_field`): an `S` inner margin, and the frame ends at the column's right edge. An empty section inside a list page
has one muted line below its heading, with no zero count or full empty state. Tabs and page headers omit zero
counts. Roster, Members and Recordings use an underlined tab with a muted, nonzero count. Overflow tabs use a quiet
caret menu without a button border; it names and underlines the selected page when that page is inside. Fight and Packages
menus put the caret after the word. Keep remains the default focus when closing could lose work.

Header actions have at most one bordered secondary button, for `New …`. Other header actions are quiet and
come first. Only one accent-filled primary action can be visible. An open run's strip owns primary `Next turn`
on other pages; `Back to run` is secondary. Use a `status_link`, such as `Running · Round 2`, to open its
state. An on toggle uses `selected`, its word and an icon or check, without accent fill. Use red only for an
action that loses something. Keep Delete and Remove in row menus.

A save row keeps its state and quiet `Discard` at the left and `Save` at the right. A draft that Save's own check
refuses says `Invalid`, changed or not: validity comes before whether the text changed. Other clean rows say `No
changes`, unless Save is enabled to repair something the file holds: then the state names the write with the warning
icon (`Save replaces the unread choice on line 23 with these settings`), and `Ctrl+S` saves exactly when the button
can. `No changes to save.` belongs only to blocked Save's hover text and accessible description. A save row
has one weight: its Save is primary while the draft has changes that it can save and no open run owns `Next turn`,
and secondary otherwise. An unsaved draft owns the primary over its own page's other actions: while an encounter
draft has changes, `Save encounter` is primary and `Resume` or `Start fight` is secondary until the draft is saved or
discarded. While a run is open, its strip's `Next turn` stays the only primary, whatever the draft holds (round 22).
A draft that Save refused does not own the primary until its fault is fixed. A repair that a note was opened for
stays through a failed Save, until a Save writes. `Save changes` in World and `Save repaired note` in World and
Campaign follow this. When
several Campaign repairs have changes that Save would write, only the active one (edited or focused last; else the last changed one) has
the primary Save; the others keep their drafts with a secondary Save. Unsaved,
Saved and errors keep that left slot. When one cause blocks several controls, state it once at the cause: the
save row, run strip or recording row. Other controls use its hover text and accessible description. The rail
never repeats the sentence. Its campaign switcher keeps its height and a lock; the popup starts with the
reason and disables campaign choices. The popup stays inside the navigation column, clear of its splitter; in a
narrow column its choices wrap. Package, encounter, group, repair, World edit and run-settings drafts keep their campaign-switch
reason after Unsaved and Discard in the save row. Reserve its height so the first edit moves no content. Run
settings keep their full save actions on a second line at the right edge, so neither action is cut short.

A quiet action at the start of a line uses `line_start`: its words start at the text edge of the line above,
and its padding hangs `S` into the gutter. Outside a message, such an action has no icon (Lookup's `Show N results`, `New NPC “<name>”`, `New location “<name>”`, `Generate NPC`
and `Copy the marker`), so its whole target clears every splitter without a wider gutter. Inside a message
(`undo_message`, `message_with_actions` and other message choices), an action with an icon that starts a line
hangs its icon into the message's own icon column, so its words start at the message's words. A
choice without an icon on its own line below a result message (`message_choice_row`: `Use the copy in
encounters` below a package's registration message, and `Build an encounter` below a copied campaign's) has
the secondary border, as the same choice beside a plain result has. It sits below the result and its Undo,
and its border starts at the message's words. On a result, only Undo is quiet. In a
wrapping row, the first action uses it, and a later one uses it only when it wraps to a line of its own. Lookup's
`Show all N`, `Show N results`, `New NPC “<name>”`, `New location “<name>”` and `Generate NPC`, Arrivals' `Show all`, and
World's type choices, `Make <type>`, picker actions (`Remove <group>`, `Clear value`, `Cancel`) and message choices
(`Copy text`, `Cancel`) start at the text edge above them. UI text that names a thing uses curly quotation marks
(`New NPC “Miller”`, `Processing “Session 1: Ambush” failed.`), and an apostrophe in UI text is curly too
(`campaign’s`); file content, such as YAML, shell lines and a prompt for a model, keeps straight marks. The design
lint (`src/design/lint.rs`) fails on a straight apostrophe inside a word in a UI string.
A file line that follows a warning (the path, `Edit in Obsidian` and `Show file`) is that warning's action. Draw it
in `message_with_actions`, so the path and both links start at the warning's words, and wrap a long path whole
instead of cutting it. When several warnings name one file, the file line follows the last one. The warning names
the row and the line, never the file (`Ember Cultist · line 20: …`, `Line 9: “Day 5” is day 1 in order. …`); the
file line holds the path once. The encounter editor, the run setup and Save's blocked reason build an unread roster
row's words with one function, in the first form. Encounters, Rules and runs, Campaign, Timeline and run setup follow this. In the run
setup, a file line that fits beside the last compact warning sits on its line after an `L` gap; otherwise it wraps
below, at the warning's words. A reader's
file line below its title is not a message action and keeps its place.
A pane that can hold such an action at its text edge keeps a gutter at least `L` wide, so the target never
reaches a column splitter. The World page's centre has an `L` left gutter for this.
Other message actions use a secondary border. Undo messages use `undo_message`: a quiet action and undo
icon on the message line, or below when needed. An Undo never stands alone: its result is a short message
that it follows (`Links now say “<name>”.` with `Undo link update`, after the offer `Change links from “<old>” to
“<new>”.` with `Update N links`, `The first heading now says <name>.` with
`Undo heading change`, `Encounters now use the copy.` with `Undo source change`, `The stat
block now says Hit Points 24.` with `Undo stat change`, and `<name> is registered as a new campaign. Its D&D
Beyond link is off.` with `Undo register campaign`). Another choice after the result, such as `Build an
encounter`, does not end the result or its Undo; a later result does. `Undo register campaign` takes back
only that registration: settings changed since, such as a registered package, stay. On reload, a result whose Undo would rewrite files checks
those files. If one no longer holds what the action wrote, the result and its Undo go away. A result that also
offers another action stays, and its Undo is blocked with the reason. The change still happened, so the
result's words keep the text colour: only Undo greys, on the message line, and the reason follows in `Small`
muted words at the message words (`library::kept_undo_message`). An Undo that waits for an open draft keeps its
words in the text colour too; the draft's reason is the save row's. Discard recovery is an Undo message too: info icon, bar,
`Body` text that names the draft, and quiet `Undo discard`, whose accessible name also names the draft. Inline creation uses a Label title, fields, a named Create
action, its `Small` blocked reason, an `L` gap and quiet Cancel. Creation and copy fields share one
right edge.

Several safe choices after a message share one line when they fit. Put a destructive choice at the end
after an `L` gap, or on its own line below after an `M` gap.

Library, Lookup and encounter search rank equal names in this order: campaign entries, book and package
entries, then character sheet copies. Creature results keep names close to stats with an `S` gap between
results. The CR column fits `CR 1/8` plus an `M` gap. Paths shorten before file actions wrap; hover keeps the
full path.

## Keeping a draft

Draft comparisons align unchanged lines before drawing. Blank rows mark actual additions
and deletions. Highlights and line counts use these aligned rows. Adjacent changed rows form one group.
`Next difference` moves to the start of the next group, wraps to the first, and stays hidden for one group.
Encounter and group conflict comparisons use disk layout and include outside properties and prose that
Overwrite keeps.

`Discard` takes one press, with no dialog. Show it only for a dirty draft, apart from Save. Keep `Draft
discarded` and `Undo discard` until the next edit. A later disk change also clears Undo discard. Library shows the
same Undo message in its reserved reason slot, aligned with the editor's left edge. The slot has a fixed
height of two text lines (see Files and recovery), so neither the reason nor the message moves the editor. A repair draft keeps its parse error
above the slot, and the slot below the error holds the discard result the same way. Keeping a draft in the close dialog restores
its editor focus and cursor.

A changed file stops saving. World uses `disk_conflict_choices` with the noun `note`: “This note changed on disk. Your draft is safe until you choose.” Library uses `entry` in the same choices. Encounters says the same sentence with `encounter`, `group` or `creature` (`This encounter changed on disk. Your draft is safe until you choose.`), and never names the file. When the file is gone, its red choice is `Remove encounter and lose draft` (`group`, `creature`), and the result says `<name> removed. Its draft was discarded.` with `Undo remove`, which gives the draft back. Show secondary `Overwrite with my draft` first, then
red `Reload and lose my draft`. Neither has accent fill. These messages cannot be dismissed. Library and World
draw the choices through `message_with_actions`: the row starts at the message text, not at the warning bar, and
an `L` gap comes before the red choice. Library's note of the entries that need a deleted file sits in the same
column, above the choices. World Overwrite saves valid text; invalid text returns to the repair
editor without a write. World Reload keeps the replaced draft for one `Undo reload` on its message line until
the next edit. If the note changes again on disk before that edit, the held draft comes back as unsaved work and
the choices show again: Overwrite writes only the DM's own changes over the newest file, with Undo, and Reload
holds the draft again for `Undo reload`. A deleted note does not bring the held draft back. A run offers `Save as a new run`, then an `L` gap (16 px in all, egui's item spacing included) and red `Replace the file with this run`, in `message_with_actions`: both start at the message text. An encounter, group or encounter creature draft (`src/encounters/disk.rs`) draws `Overwrite with my draft`, an `L` gap and red `Reload and lose my draft` (or `Let it go` when the file is gone) the same way, under the last of its messages: a note that another file uses the same identity comes before the choices.
Below the choices, as World's lines do, one `Small` line for each part that differs says it in the DM's words, with a
muted label: `Name: your draft says “…”; the file says “…”.`, `Notes: your draft adds “Ash on the water.”; the file adds
“Written outside Splat.”`, and for a table by its heading, `Roster: the file names the same 8 rows in another form.` A
row that holds an id is the same row whatever form its link takes. The comparison stays below.

Session conflicts call the item a note in the warning. Keep Record available because audio uses its own file.
Show draft and disk text side by side in Write's styled source face. Hide the plan and use the whole session
body, with choices above the comparison. Library uses its editor's monospace face and line height. Only a
side-by-side draft and disk comparison takes the full pane width; when the file is deleted, the lone draft
keeps the 720 px source editor width. Text wraps
at word boundaries. Selected fill and a line mark show differences. Open at the first difference with two
whole context lines above it. Show `N lines differ` and the nearest heading beside that count, such as `Under
the heading “Recovery”`; let the sentence wrap. Missing rows stay blank with a muted hatch and the accessible name `No
line`.

## The Reader

`library::view::show` is the only entry reader. Library, Lookup, the run panel and campaign entries call it
(T6.4). A caller that draws a value block above the entry tells the reader when that block drew no row
(`empty_value_block`). The reader then leaves out its own gap before the prose, unless line warnings sit
above it. World uses this, so a typed note with no set values starts its prose where a plain note's does.
Lookup has no file line: there an empty value block draws nothing, and the prose keeps the reader's gap below
the kind line, as a spell's does, and so does a plain note's prose. Lookup's reader keeps the app's row spacing whether a search or a link
opened the entry. Library draws no value block. `design::reader` holds the typographic parts. Library shows quiet Back whenever it has reading
history. Its toolbar stays on one line: Read and Edit Markdown at the left, then visible copy and creation
actions, then Back. Drop Back's key hint first; fold Copy and Make NPC note into quiet More when needed, with
Back before More. Every toolbar action, Make NPC note and the actions in More included, is a Read control
(28 px), so the toolbar row has one height and a campaign creature's title sits where a campaign spell's does. Lookup keeps its Back hint; its Back is a Read control (28 px), as its title line's icon buttons are, so the title line has one height with or without Back and the search field below never moves. Both use Alt+Left, and Alt+Right for Forward, which has no button. Lookup takes the key when focus is there. Beside the Library, the mouse's Back and Forward buttons act on the side under the pointer.

Library omits `Library / <kind>` above the title. Its kind line does not repeat the package name: where two
packages share a name, it adds only the folder that tells them apart (`Creature · CR 1 · ashen-marches/`). Owner status
and `Use in this campaign` stay on the wrapping file line in Read and Edit Markdown, so Back does not move the
title. The file line is one Read row (28 px) for every owner: built-in, campaign and package About. Its status, path,
links and quiet actions centre in that row, so the title stays at one height when the DM goes from one owner to
another. When the line is too narrow and the path goes to a second row, that row is also one Read row. During a
draft, Back opens the earlier entry in Lookup. Show file is on that line; vault entries also
have Edit in Obsidian. The path shortens from the front only and keeps its file name whole
(`…/Cinder Wolf copy.md`): when that does not fit beside the owner words and their action, the path takes its
second Read row. An unreadable entry keeps `Cannot be read` on its owner line, in place of `Used here`.
A file that the system will not read at all (not UTF-8 text, or no permission) has no text to repair, in a package
or a campaign. Library keeps why the read failed, from the error's kind, as World does. Its row's cause is short
(`Text format cannot be read`). Its reader keeps the title and a file line with only `Show file`: no Edit in
Obsidian, because no editor can open the file either. Below the title, an error names the file (`Splat cannot read
Notes/Broken.md as text.`), then the recovery in plain words, then quiet `Try again` at the message words.
Technical details holds the system's words. Edit Markdown is blocked with its reason, and the reader offers no
`Fix in Edit Markdown` or manifest replacement, until a read succeeds. Try again reads every file again. When an
open entry's file stops reading, a draft stays: the message names the file and says the draft is safe, with
`Try again` and red `Reload and lose my draft` (with Undo discard), never a save over the file. With a draft
open, the reader's `Try again` reads the file at once and compares it with the draft, as any outside change. Splat never
changes that file. Until the DM chooses, the header counts that file and its owner line says `Cannot be read`,
as without a draft.
Below its title, show only the kind when known and the specific error with its file line.
Offer bordered `Fix in Edit Markdown` below an unreadable entry's error, at the text edge.
It opens the editor at the error line. In Edit Markdown, put the error in the reserved reason
line directly below Save, then an `S` gap before the source editor. Keep front matter
out of prose. Empty readers say `No entry open` or `No note open`, then “Choose one from the list, or make a
new one.” Library omits the creation offer without a campaign or package destination.

Ordinary bold and bold italic keep the prose colour and emphasis. A bold phrase links to Lookup only when
it resolves to one entry. Explicit Markdown and wiki links stay interactive.
Reader links use accent text and an underline. Library links open in place; other readers open them in Lookup.
Each link in a paragraph is its own control: Tab reaches the links in reading order, a focus ring shows on that
link only, and Enter or Space follows it. Its accessible name is its words and where it goes (`the bell keeper
(opens Oskar)`); a missing link is not a Tab stop. A short link reserves its target in the layout: a link whose
words are narrower than 24 px gets 24 px of layout width. Inside a line, its words sit in the middle of that
space, so the spare width is split before and after them and its target is centred on them. A link that starts
a line (at the start of the paragraph, after a line break, or where it wraps) keeps its words at the text edge,
with all the space after them. Punctuation that follows a short link (`,` `.` `;` `:` `!` `?`, closing brackets
and quotes) sits directly after its words, so `Deception +2, Religion +2` has no space before the comma. The
words stay where the text puts them, the spare width goes after the punctuation (the punctuation and a blank
after it count toward it), and the target grows right from the words. A short link that ends its line does the same.
A stat block's Skills row is the one exception: each bonus ends an entry with its skill's name (`Insight +8`), so
the row keeps the text's own layout, and the bonus's target extends left over its own entry (see stat blocks).
Prose keeps this rule, also after a no-break space. A link right after another short link gets as much space after it as before it,
so the pair reads evenly. The text after a link starts after its space, and the line wraps if it must. Two links
side by side have a blank between them, so each keeps its own underline. Where the body line is shorter than
24 px (compact prose and compact rows), every line of a paragraph that holds a link is 24 px, the first line
too. Each line keeps its words where an unlinked line has them and adds the extra height below them. So every
line has the same baseline offset, the baselines are 24 px apart, and the first line starts as an unlinked
paragraph's first line does. A paragraph without a link keeps its own line height. A link's underline
sits 2 px below its words' baseline and covers only its words, never at the bottom of a taller line. So every
link target is at least 24 × 24 px, stays on its own line, and never overlaps another link's target. Links 24 px
or wider keep their layout, and the paragraph's words do not change. The focus ring sits `XS / 2` outside the
link's words, so it never covers a letter, even at the start of a line, and it stays inside the reader's visible
area (World and Library keep an `L` gutter beside the text). Focus stays on the link when the layout around it changes, and another reader with the same words, or
a reader with a document beside one without (Lookup beside a Sessions note), does not take it.
Every reader resolves `[[Name]]`, `[[Name#Heading]]` and `[[Name|text]]` against World notes and entries the
campaign can use. Duplicate names open a chooser. Missing names keep their words, a dotted underline and `Not
found in this campaign` on hover. Broken campaign and package links also have a warning icon before the dotted
text and a list-row warning. The page header counts each file with a broken link once in its `N files need
fixing` (Library) or `N notes need fixing` (World) link, and its menu opens that file. Reference keys still work. Markdown readers call `reader_markdown` through
`ContentIndex::resolve_link`. Hidden SRD provenance does not count as broken links; SRD page targets count as
found.

Readers hide reserved Splat data and any heading or rule whose filtered body is empty. Hidden source headings
and comments take no space. A stat block ends with its own gap: after it, the reader adds no lead gap, so a
creature with no source text to style (a converted or hand-written one) starts its first heading as far below the
closing rule as a styled creature does. Collapsed Source and identifiers and Technical details follow the body in its
scroll area, with one leading `M` gap. What Splat applies stays inside Source and identifiers. Source text and
structured item fields stay in this end group. Unreadable files omit empty end sections. Related entries, Here
and Mentioned in stay above them at reader measure. In narrow creature readers, each vital keeps its label and
value together when wrapping. Pictures are 64 px thumbnails beside the title and meta line. Use one `L` gap
after the kind line and one `M` gap after a spell's fact table. A standalone bold caption above a table is
plain text.

A stat block has a heavy rule, identity, labelled AC, HP and initiative, then a tight attribute table with
physical and mental abilities side by side and right-aligned numbers. Detail rows follow. Each detail row's
label sits on the baseline of its value's first line, also when the value has dice links and a taller line
(`Skills`): the label reads where the value's words are painted. Every line of every detail row has the link
pitch (24 px), whether or not the row holds a link, so no row looks grouped with another; the extra height
goes below the words. A skill and its bonus wrap as one unit (`Sleight of Hand +5` never splits), and the
bonus keeps its own link target. The row keeps one normal blank between each name and its bonus and one after
each comma (`Insight +8, Perception +14`), whatever the bonus's width. A bonus's target is at least 24 × 24 px: it
extends left from the bonus over its own skill's name (the name and bonus are one entry), stays inside that entry,
and never meets another target. A bonus that must start a line keeps the short-link rule above. No card surrounds
it. Serif section headings sit over hairlines. Vitals come once from the schema-2 Stat block table. New
creatures store identity, CR, speed and ability scores there; printed source text stays separate. An absent
saving throw leaves an empty cell. Bad cells warn with their file line; valid values still lead in the reader
and run panel. Rejected vitals keep their original text and warning in the vitals area. Unknown rows remain
readable. Stat warnings name the field, bad value, line and repair. List rows and the file warning count
include flagged stat cells. The editor warns when prose AC or HP disagrees with the table, in the DM's words
(`Hit Points line says 24, but the stat block says 22. The fight uses the stat block.`), and offers `Use
<value>` with Undo.

Spells and items show facts once, with no empty rows or machine lines. Spells use Casting Time,
Range, Components, Duration in book order; Concentration and Ritual appear in those values only when true.
Magic items use book case and put attunement in their identity line. Creature types use book case and keep
tags. CR supplies missing XP and PB; explicit mismatches warn by line, except lair values. Ability modifiers
and passive initiative follow their score or bonus. Printed creature headers stay in a folded
record quote outside the reading view. Run copy notices compare model and readable words, so a rewrite of the
same values alone does not report changed creatures. Copied creature fields use template book order, one flow line per stat.
Copies write schema-2 stat tables and keep prose. Creature templates use inline stat values and omit HP and AC
until the author sets them.

Imported rules reflow for reading without changing stored notes. `design::reader::run_in_title` bolds only a
named paragraph such as `Acid Breath (Recharge 5–6).` A title-case line above prose is a heading. Three or
more rows of numbers, or names and numbers, become a table. `presentation::reading_tests` checks every
built-in note. Sheet tabs use `Small` as a second level; later pages move into More so tabs stay on one line.

## The catalogue

Library is one catalogue of built-in, package and campaign entries. It opens on what this campaign can
use. Scope can show everything, one package or Campaign entries, without changing the active campaign.
Keep search and selection when scope changes. Editing, save state and `Copy to…` stay in this page;
package actions are inline. Do not use `module` in the interface. Manage packages uses `Heading` for its
pane title and `Label` for its sections. A filtered creature list shows `MATCHES` and its count, even
when search is empty.

The scope control says `Can use` in both layouts, at the top of the category column. Keep those words
whole at minimum width. Its menu is as wide as the control, so it stays in the column, `S` clear of the splitter;
a long package name wraps. An empty search or filter names its scope (`No entries match in Can use. Try fewer
words, or search Everything.`), and quiet `Search Everything` starts the next line at the text edge: it keeps
the words and filters. In Everything the sentence ends at `Try fewer words.` with no action. At narrow widths, scope and category share an equal-width line directly above search. The package's About row follows search as the first list row.
Hide All entries without a search. Empty categories and their headings stay hidden. Fixed scope names
wrap; package names can shorten with full hover text and accessible names. The About row stays one line, with
or without its mark: the package name shortens by whole words, so a fault moves no row below it. Both the scope menu and
selected button say `Campaign entries`. An open package About row uses selected fill with no selected
category; choosing a category restores it. About is followed by the manifest name. A manifest is never
counted or listed as an entry, even when unreadable. Keep its warning on About and in the file issue
count: About marks any problem the header counts in the manifest, such as a first heading that differs from the
name, not only when the manifest cannot be read. The mark has the fault's tone: the error icon for a manifest
that cannot be read, the warning icon for a deleted manifest or another problem. New packages write `Package.md`. Existing `Module.md` files keep their name. Messages and repair
actions for existing files name the real file. A deleted manifest uses the registered package name
as its reader title and `<package name> · Manifest deleted` on its owner line. Offer `Show folder`;
show no file name or `Show file`. Keep the neutral missing-file warning and shared message action.
Plain prose with no front matter and no `splat_` lines stays a README, even in `Package.md` or
`Module.md`. Writing the missing manifest uses `Package.md` if free, else `Module.md`; refuse the
write if both names hold notes. Never replace a README. Empty files, front matter and malformed
Splat fields stay in repair. Manifest repair selects the package scope and its warning About row.
Draw the manifest error with `message_with_action`, then its muted `Small` consequence at the same
text edge below the action. The consequence wraps at the pane's notice measure, not at the action's width. Show used and total entries and `Use every entry` with Undo. Its result
and Undo sit in the reader's results place, after the title, like manifest-write results (see Files and recovery). Omit the entry breadcrumb and Engine support. Empty package counts have no separator.

Nonzero file status links sit in the page header before Packages and New entry. The validation count says how
many files need fixing: a validation message, a broken link, a duplicate identity or more than one count a file
once, and both files of a duplicate identity count. It has the tone of the most severe fault it counts: amber with
the warning icon, or red with the error icon when a counted file cannot be read. A file is
its owner and its path, so two packages that each have `Rules/Watch.md` count as two files. Its menu
lists the messages, then a link for each file with broken links (`<path>: N broken links`) that opens it.
When two owners have the same path, the message and link name the owner first (`Ashen Marches ·
Rules/Watch.md`). Unreadable notes use CANNOT BE READ. Rows name
their kind, path and error line; the owner line says Cannot be read. The reader's title is the file name, and its
error says `this file` (`Splat cannot read this file as text.`): the file line names the path once. Messages away
from the reader keep the path. An entry row marks every fault the header counts in its file, one mark for each
fault: the icon in the fault's tone, then a short cause in the row's muted words. A file that cannot be read is an
error (the error icon in the error colour, then its cause or `Line 4`); every other fault is a warning (the warning
icon in the warning colour: `Same entry in two files`, `First heading differs`, `1 broken link`, `Needs fixing` for
any other). When each counted problem of a file is a link to a missing entry, the row says `⚠ 2 broken links` only. About and
Lookup rows draw their marks in the same tones. Opening one from a roster selects its
kind under Cannot be read and opens repair. Repair errors use file line numbers. Edit Markdown opens with
the caret on the line to fix and scrolls that line into view. A repair draft's error and its save-row state
come from the draft as it is now, checked as Save checks it. So does every other draft's: a ready state
(`Unsaved`) means Save would accept the draft, and a draft that parses but that Save refuses (another UUID,
kind or key) says Invalid with Save's reason, and Save and Ctrl+S are blocked with it. These checks never read the disk.
A Save that fails on the disk (a read-only file or folder, a write error) is an action-time failure: the state
slot says `Not saved` with the error icon, and Save's own reason sits where a refusal's reason does. A read-only
file or campaign folder names the next step (`This file is read only. Make it writable, then Save again.`), and the
owner line says `Read only` with the lock icon for as long as that failure lasts. It belongs to
that draft: the next edit, another entry or a Save that succeeds ends it. Each reason Save would refuse it (a YAML fault,
a UUID or key that another entry uses) is the error, with Invalid. Once Save would write it, the error goes and
the state says Unsaved. The owner line and the row keep Cannot be read until Save. A fault that is not in the text (another identity, a
deleted manifest, a file that is not text) stays until the file changes. Repair reloads campaign use at once. Validation
messages also update after encounter or group edits.
Other readable files stay available.

A deleted draft shows YOUR DRAFT, `Save my draft again` and red `Let it go`, with Undo let go. Its file line keeps
the path and offers `Show folder` for the nearest folder still there: no link offers the missing file, and no
Edit in Obsidian. Let it go names entries that need the deleted note. Undo let go gives back the draft, the
entry's campaign use and the header count as one step. A moved draft shows the new path and old folder; moved-from text ends the
file line. Its warning says `This entry moved on disk. Your draft is safe until you choose.`, and its two buttons
name the choices. File actions use the new path. Offer `Save my draft to its new path`; saving checks the disk ID to
prevent a second file after a move.

Source mode has one frame at the remaining pane height. All four edges stay visible; only text scrolls. Read
mode keeps its outer body scroll. Form headings use text colour and an `L` gap; labels stay muted. New entries
start in their kind folder, use `FORM_WIDTH`, show the destination path and reveal the selected row after
reload. New entry, Create package and Copy to a package default Use in this campaign to on. Help states which
selection will be saved. Package entries show their package and Used here or Not used here; Use in this
campaign saves the missing choice. Empty campaign entries offer one New entry action and link to Can use with
`entry you can use`.

Missing dependencies lead with the owner and retained member name: `Lost Patrol needs Scout, which was
deleted.` Without a known name, say `Lost Patrol needs a creature that cannot be found.` The repair opens Edit
Markdown with the caret on the missing link's line and names it in the reserved reason line (`Line 14: Lost Patrol
needs a creature that cannot be found. Replace the link on this line with another creature, or remove its row.`)
while the draft holds that link. When Save's reason or the discard result needs that line, the guidance comes first,
directly above it. The guidance belongs to its entry: when another entry is selected, by any path, it ends. The link is found as the reader reads the member's Creature cell, never by
matching its id in the text, so notes that mention the id are not the repair. Link repair to the
referring file; keep raw references in Technical details. A removed package is named without claiming its file
was deleted.

### Lookup and copies

Lookup shows up to five one-line results and selects the open result. A sole open result closes the list. Rows
use Scan height, Body names, then Small muted type and level. Campaign entries add Campaign; source appears
only when different from the campaign's main source. Shorten metadata only, by whole parts; hover and accessibility
keep the whole row. The kind and other parts before a fault go first, then parts after it. The file, which tells
duplicate rows apart, stays whole while the fault shortens to whole words and then to its warning mark; only then is
the file cut. No cut is inside a word, and a cut ends with `…`, never on a separator. Results never scroll sideways.
Clicking a result or scrolling into its reader closes the list. While the list is closed, the quiet `Show 1 result`
or `Show N results` opens it again. One result that is the entry on screen has nothing more to show, so it has no
such line. An action that shows hidden items starts with a verb; a bare count reads as a
label. Its accessible name stays `Show the results`. `Show all N` is quiet and lists every match in place,
in Lookup and in Arrivals: a scroll list at most eight rows tall that follows Tab and the arrow keys. New words
return to five. Clear sits beside nonempty search only. Back, Open in Library (Open in World for every World note,
typed or plain; a session note has neither) and the close icon share the title line. Creation actions follow results with the proposed name, only without an exact
match. Generate NPC appears only for empty search. When no creation action shows, their row takes no room: the rule
above the entry follows the results or `Show N results` at the normal gap.

A copy starts with the original name selected. Destination and dependency preview update with its fields. Copy
to… requires a campaign or package destination and states that reason on hover. Create copy saves in one step.
Edit the copy selects it and opens Markdown editing. An open encounter draft keeps Add to its encounter as the
primary completion action. Completion names the destination, with path on hover. A renamed copy changes its
first H1 only when it matches the source name.

A copy preview explains dependencies: “Also copies <entry>, because <source> links to it.” For several copies,
say “because a copied entry links to it.” Copy both copies a campaign dependency with its entry and reconnects
references. The warning action copies and reconnects one dependency; Undo restores the reference and removes
the unchanged copy. Keep room for Copy both and Cancel, with no outer scroll handle below them.

## Files and recovery

Splat reads schema 2 and Timeline version 2 only. A file with another version is an unreadable file: name its
path and the reason, as for any unreadable file. Reading and polling write nothing. `splat check` reports
schema-2 flags and unreadable files by path and line, and opens no window. Malformed
flat properties stay on disk; a line warning keeps the note available. Campaign packages use tasks; a toggle
changes one character. Rule choices, group members and saved rosters use tables. Bad cells name their line and
stay on disk. Encounter and group notes forms show DM prose with managed tables outside the form. New files
use the entry name. End fences use folded Splat data callouts; empty defaults need no fence.

On window focus, check campaign notes and registered package folders. Reload changed catalogues; compare the
disk text when a draft's file changed. Duplicate identities keep both files visible with a warning. A roster row and its message give that fault the
same warning tone; only a file that cannot be read is an error, and both block a fight. Saves
write only the open file. A renamed copy comes first, otherwise the newest file does. Encounters block
ambiguous creatures; Open in Library opens the first repair action's file. Encounter search names hidden
duplicate notes and offers file-specific identity repair with Undo.

Library and creature repairs say “These two files claim to be the same entry,” with the file names.
World note repairs use “note”, including in Encounter search, their actions and Undo.
Sessions use “session” for their repair and Undo actions. World rows say `Same note in two files`, including in Lookup. Other rows say `Same entry in two files`. World repairs say `Make <file> a separate note` and offer `Undo separate note`. Other bordered secondary repairs say `Make <file> a separate entry`. Their result
names the file and keeps `Undo separate entry` on the message line. Folder repairs use `separate
entries` and `Undo separate entries`, with one guarded Undo for the batch. In Library, each single-file
or folder choice states which file or folder an encounter will keep or use, in `Small` muted text an
`XS` gap below its own action. Library, World and Lookup draw the choices and these lines through
`message_with_actions`, so they start at the message text edge, not at the warning bar. The Library reader
draws the warning and its choices after the title, inside its scroll area, so they scroll with the entry;
Edit Markdown keeps them above the editor. A file's warnings share this one place: the first-heading
warning and its `Use <name>` action come directly above the duplicate warning, so no warning moves the
title. Its result, `The first heading now says <name>.` with `Undo heading change` on the message line, takes
the same place. Every Library result takes it too: package use, a manifest write, `<file> is now a
separate entry.`, a folder's separate entries, a discarded draft, the offer to change links after a rename and its
result, and a copied package's registration with `Use the copy in encounters`. One result place per reader: in Read
they sit after the title's kind line, so the columns, the tabs, the file line and the title never move. Edit
Markdown keeps one space of fixed height below the save row: a result part, then the reason slot directly above the
editor, each two text lines high whatever it holds. Longer words wrap and scroll inside their part, beside a lane
kept for its scroll bar, so no text is cut. Each part is one focusable group named with its whole message: Tab
reaches it, a screen reader reads all of it, and while it has focus the up and down arrows scroll it by a line and
Page Up and Page Down by the part; the wheel scrolls it too. The result part holds the guidance first when Save's reason or the
discard result holds the slot. So no result, reason, Discard or Undo discard moves the editor. With no entry
open, they sit above the empty reader, and above a form that takes the reader's place. The link offer after a
rename and its result belong to the renamed entry: they show only in its reader, stay hidden while another entry is
open, and end on its next link update or saved edit (a saved edit ends the earlier offer whatever it offers), on
Undo, or when a file the update wrote changes on disk. An offer is checked again on every load and before its write:
its entry still has the new name, and neither name names another entry, or it goes and writes nothing. Just before
the write, Splat reads the entry's own file from disk; if it no longer has the new name, the update writes nothing
and says `No links were changed. This entry changed on disk since it was renamed. Reload from disk, then rename it
again.` Each linking file must still hold what was read, or nothing is written.
Undo restores only the files that update wrote, so another entry's link update stays.
An `L` gap separates choices. Name the folder when filenames match. Making an entry separate also sets
its name and matching first heading to its file stem. Other actions name encounters whose source will
change; results explaining which entry an encounter will use are Small muted text below the action.

## Campaign

Campaign's header counts chosen entries only. Flags appear once in the content issue count. The count is a status:
the warning icon and colour before its words (`⚠ 1 content issue to repair`), and it keeps its collapse. Encounters'
count uses the same helper, words and size. Each flag's
repair choice and file line start at its words. Packages keeps its save row directly below the tabs, before notices.
The save row, package list and entry search stop at LIST_MEASURE from the tab’s left edge.
An open repair keeps the same right edge as the save row.
Repair fills the column with the invalid-note reason above it; an `L` gap and hairline separate its save row
from the package save row. Below the repair's own save row, an `L` gap and the package list's one hairline
separate it from the list; the repair adds no second rule. Invalid YAML uses plain words and marks the line to fix.
The reason above names the file, so the `REPAIR` heading has no path. A repair draft's save-row state comes from
the draft as Save checks it, as in Library: while Save would refuse it, the state says Invalid and Save is blocked
with the cause; Unsaved and a primary Save come only when Save would write it. Save's check without writing is the one World's repair uses (the path rule, YAML, identity,
campaign ownership). Each repair keeps its answer and asks again only when its draft changes, so the page never
reads the disk or parses a draft on an unchanged frame. A Save that fails on the disk (read only, missing, a write
error) is an action-time failure: that repair's state slot says `Not saved.` with Save's reason until its next
edit, its successful Save, a reload or another campaign. The failure shows only there: the message above stops at
`Fix it here or in World.` While it shows, `Save repaired note` stays open for a retry but is secondary; the next
edit gives the primary back. A refusal that Save's own check finds at Save is Invalid with its cause, never `Not
saved.` Another repair's edit, Save or failure leaves it. Each
repair editor's accessible name holds its file (`Note text: Notes/Broken.md`), in Campaign and World. While a repair blocks the package
save, its reason (`Fix Notes/Broken.md first.`) sits on its own line below `Save selections` at every width, so the
first edit in the repair moves nothing.

Needed by your entries sits below packages in the left column. Its right pane has one search and list; rows
open Lookup. Short package lists fit their rows. A package row's count has one form: `All chosen · 2,065 entries`,
`None chosen · 6 entries`, `3 of 6 chosen`, `5 of 6 chosen · 1 needed`. An off package with required entries shows
`None chosen · 3 needed`, an indeterminate checkbox and encounters that use its creatures. A package whose manifest
has a fault leads its count with the fault's icon in its tone (amber warning for a deleted manifest, red error for
one that cannot be read), as the About row does; the message above names the fault, and the row's accessible name
says it in words (`Manifest deleted · 5 of 6 chosen · 1 needed`).

A missing Campaign.md states whether Splat holds the last read copy and counts its packages and rules. Writing
held text requires an explicit action. With no copy, `Write a new Campaign.md` sits beside the explanation
that the DM can write a new one with the default packages (`Campaign.md is missing, and Splat has no copy. You can
write a new one with the default packages.`). A missing or changed Campaign.md ends `Undo discard` of the package
choices: the discarded draft belonged to the file as it was.

### Packages

Manage packages offers Clear only for filled fields. Folder is one editable field with a suggestion menu, then
help. Suggestions include existing folders, use the combo arrow and normal text colour, and permit a new
subfolder on save. New packages default to the parent of the last registered package. New package has Browse
beside Folder and the full destination below it. Creating a package opens its first entry form.

A copied package folder names the original under Add folder and puts its ID in Technical details. Add folder
is blocked with `This folder is a copy.` Bordered `Register as a new package` sits on the warning's action
line, above Technical details. It asks for a name, writes independent package and entry identities, and has
one Undo for all files. Registration names the copy and what the registration chose, not what the campaign
uses later: `Ashen Reaches is registered, with no entries chosen for this campaign.` (or `with every entry
chosen`). Its use checkbox starts
off when the original is used. Keep the warning above the `Register a copy` Label title and fields.
The named Register action is primary when the name is ready and no run is open, as `Create package` is.
Quiet Cancel follows it after an `L` gap. It closes the form without writing
and keeps the name and use choice for reopening. When use is off, help says the campaign keeps the
original and the copy is registered but not used. When on, it says every entry is used, including future
entries. `Use the copy in encounters` keeps roster names, counts and changes, with one Undo. After registration,
quiet `Undo package registration` sits on the message line, as every Undo after a message does, and bordered
`Use the copy in encounters`, without an icon, sits on the next line: its border starts at the message words. The choice names what it changes, in its result's words: its result `Encounters now use the
copy.` keeps `Undo source change` on its
message line; after an outside change that Undo is blocked with its reason.

Unavailable packages keep their last known display name. Older unnamed selections say `A package that is no
longer on this computer`. Count affected campaign entries when references are known, with Remove from this
campaign and Find the folder… beside the message. Only Technical details shows IDs. Removing an unavailable
package has Undo and keeps links to affected encounters; the message's icon centres on its first line, whose
words are those links. Unregister keeps the last folder in Settings. Library
offers Undo unregister; Campaign offers Register it again.

Missing-package recovery searches beside known package folders. A unique match gets bordered secondary Use
this folder; Find the folder… is then quiet. Show one warning naming the old and new folders, followed by
the affected encounter links with word spaces. Align Use this folder, Find the folder… and Remove from this
campaign on one action line with the warning text. Technical details follows below. Say Splat cannot find the
package only when there is no match. Use Body text throughout the missing-package sentence. The picker
starts beside the old folder and checks identity before restoring registration. A folder that holds another
package says the fault first and names both: `<folder>/ holds a different package. Choose the folder that holds
Ashen Marches.` Both recovery paths have Undo.

A broken or deleted package manifest leaves its entries available and names the manifest fault. Campaign offers
bordered Open in Library first for unreadable manifests, then `Replace with a basic <file>`. Name the real file,
Package.md or Module.md. Align the first action and the replacement's consequence below the actions with the warning text.
Campaign Packages and the Library details row say `The package manifest was deleted. Entries are still available.
Write it again, or restore it from a backup.` and offer `Write the manifest again` for a missing file. The old
file name is not stored. An existing file offers `Replace with a basic <file>` and warns that other fields
will be dropped. Keep the registered identity and reload. Writes show the success message `<file> was written
again.` and `Undo write <file>`, one verb for both, naming the file written, for a missing or a replaced file. On reload, if that file no longer holds the
written text, the message and its Undo go away.

### Link and folder

Name the campaign folder, session folder and link. The session folder has one heading and no repeated visible
Folder label; keep its accessible field name. Fields and save actions end at `FORM_WIDTH`. A clean folder form
shows the check icon and No changes. Locked controls have no clean-state reason. Folder help names its files
and permits a folder in an Obsidian vault. Disconnected campaign-link help opens Settings from an inline link.
Its state says Not connected · showing the name saved on this computer.

A session-folder change counts notes, events, templates and recordings left behind, and says what the safe
choice costs: `If you save without moving, Splat no longer shows these files.` Offer Move files and save
or Save without moving. Move files and save is primary when no other primary is in view; it is secondary while a
run is open or the campaign details form is open. Refuse collisions and copy every owned file before changing the manifest. A run-folder
change counts saved files left behind. Move runs and save is primary when no other primary is in view; it
shows only while no run is open. It stages all files without replacement, saves
settings, then removes unchanged originals. Save without moving leaves files in the old folder. Block
run-settings saves while a run is open. Folder changes name hidden unfinished runs and their states. A move
result gives counts and destination without another Saved label. A moved run has confirmation and Undo move
until the next note or plan edit.

## World

The header's New menu lists `Note…`, `NPC…`, `Location…` and `Generate NPC`. `…` marks each choice that asks for a
name before it creates anything; Generate NPC makes its card at once, so it has no mark.

Readers show the relative path, Edit in Obsidian and Show file for every note that the system reads as text. The path
shortens from the front and keeps its file name whole; when `…/<file name>` does not fit beside the links, the path
takes its own Read row and the links the row below, as in Library. Duplicate readers keep
the path on this file line only; search results keep paths to distinguish names. A duplicate Lookup row
puts the path right after the name, so the part that differs is cut last. Plain notes use
one-line rows. A plain note's `Give this note a type` section says what a type gives the DM, and does not repeat the heading:
`A type adds the fields for that kind of note, such as an NPC's place and stat block. Its text stays the same.` How Splat stores the type waits in Technical details at the end of the section. Help
text states the purpose in the DM's words; file formats and field names go to Technical details. Typed names that match show each filename on the second line. Copied typed notes remain
visible with duplicate warnings. A duplicate NPC still has its existing name-only roster override; its
warning does not imply changed stats. Encounter repair opens World for World notes. A note with no
readable body says `Nothing written yet.` only. The file line keeps the one `Edit in Obsidian` link.
A History list gives its dates one column, as wide as the widest date in the list plus an `S` gap, so event
names share one left edge and a wrapped summary stays right of the dates. The column takes at most a third of
the width; a longer date wraps in it, with its first line on the name's line. A row without a date keeps the
column empty. A row in Here, Members or History is one paragraph: the name's link, then what the note is, wrapping at
the name's edge with one `Small` line pitch; the link keeps its 24 px target on the first line, with that room above and
below the paragraph. Lookup's `Use your own names` never replaces an unsaved NPC on the World page. World opens on its own card,
which says `Lookup keeps <Lookup’s NPC>. Save or discard <this card’s NPC> to move <Lookup’s NPC> here.` while Lookup
holds that NPC, so each of the two NPCs has its name.
An unsaved generated NPC on the World page is a held draft. Only Save NPC and Discard end it. Every other path keeps
it: Problems, Open questions, another note, a repair, New, a reveal from another page and Lookup's link. While another
view is open, one row above that view says `Unsaved NPC: <name> · Show`, so the view stays usable. Show, Generate NPC
and Lookup's link open its card again; Reroll on the card replaces it. Discard says `<name> draft discarded.` with
`Undo discard`, which brings back the same NPC until another view opens. Closing the app or changing the campaign
asks about the NPC as it does for other drafts. The rail shows `Unsaved NPC` with the draft icon while the NPC is held;
it is a link that opens the NPC's card on World from any page.

Every World result with an Undo (a value, name, edit, identity or file change, and a discarded edit draft) and every
edit form with its `Undo reload` belong to the campaign that made them: Lookup keeps its reader across a campaign change, and the result shows, and its Undo acts,
only in that campaign. A World result is one message form in one place: `message` (or the shared Undo message) below the file line, where
`<file> is now a separate note.` sits, so the title never moves. `Note repaired.` is a success message there, and
`<name> draft discarded.` sits there below the open note's file line. The edit form's save result (`Changes saved.`
with `Undo changes`, or `Your changes are already in the note.`) sits there too, and a later result on the same note
replaces `Note repaired.`. An Undo message centres its words on its Undo target's line, so its words start 4 px lower
than a message without an action. In an edit form, which has no file line, a result
sits below the title. With no note open, it sits below the empty state's words.
A finished change has the success tone, as Library's results have: `Changes saved.`, a value change beside its row
(`In: … → …`, `Stat block: … → …`, Status, Part of), `Name changed to <name>.`, `Note repaired.` and
`<file> is now a separate note.` (`world::CHANGED`). Encounters' finished changes have it too: a replacement
(`Replaced Unknown creature with Ember Cultist.`, `2 Cinder Wolf rows replaced with Guard.`), a removed member or
group, a group moved or deleted on save, and a deleted run. A result that wrote nothing keeps the info tone:
`Your changes are already in the note.` and `<name> draft discarded.`. The unread `Part of` warning draws
`Edit in Obsidian` with `message_with_actions`, so the link starts at the warning's words, in the edit form and in a
reader with no file line.

Set values show labelled rows: Stat block, In, Part of and Status. An NPC also shows its empty Stat block row. A full NPC reader's kind line shows NPC and its owner; Status stays in its own row. A compact reader keeps status on the kind line because it has no Status row. A compact reader has no file line either: while a duplicate-note repair message shows, its kind line drops the fault and names the open file instead (`NPC · Campaign · NPCs/Rook Ambry copy.md`). The result rows keep the fault. Use a fixed label column, aligned
values and a 32 px pitch. Change uses one column at the widest visible value in the block plus an `L`
gap, capped at the value measure. Values wrap within their column, with Change on the first line. Read mode hides empty relationship and status values. An NPC keeps its empty Stat block row and Change action, so assigning its first stat block still takes three interactions from the reader. This row says `not set`. Edit on the title line opens all fields, including empty ones. A typed page with no set values shows its prose directly, except for this NPC Stat block row: with no row, the value block adds no gap, so the prose starts as far below the file line as a plain note's does. Missing links retain names and keep `Not found` on one line beside a label column. In a narrow pane (Lookup beside World), each label stands
above its value with an `S` gap above the label. The value then keeps the label column's room, and only Change and its
`L` gap come off its width; there a missing link's name keeps its line, wrapping by whole words only when it must, and
`Not found` has its own line directly below it. The value column measures each value as it is drawn, including a
missing group's icon and ` · Not found`, and each wrapped line of Part of apart: Change follows the widest drawn line,
and a group is measured by the name its link shows, not the words in the file. Empty or whitespace-only place, stat block and group values, and a Part of key with no value, are unset and give no warning. Broken links have a
warning icon, list warning and count in the header. A Part of target that is not found is a broken link
(`Group not found`); the header counts its note once, and the count's menu opens it. A place link that
matches more than one note also needs fixing: its list row says `Place has 2 matches`, and the header
count and its menu include the note until `Always use this one` picks one. A note whose `In` chain leads back to it is in a place loop: each note in the
loop says `Place loop` on its list row, the header count and its menu include it, and its reader says
`Loop One is in Loop Two, and Loop Two is in Loop One. Change one of them.` A row with more than one link
problem says `2 broken links` only when every target is missing; when one of them waits for a choice or is in
a loop, it says `2 links need fixing`. Table faults use the
same list warning and header repair link. Unread values name field, bad value, source line and repair, in
the form `Line N: <Field> "<text on that line>" was not read. <How to write it>.` A line warning's sentence
starts with `Line N:` and is made with the warning; no screen adds the line, so a run saved by an older Splat shows
its warnings in the words it saved. When the shape is not
obvious, the repair gives an example: Hit Points says `Start with a number, such as 22 (4d8 + 4).` Every unread-value warning
sits in one place: below the file line, above the value rows. It wraps at the reader measure in Read and at
`FORM_WIDTH` in Edit, so it keeps the right edge of the rows and fields. Every other World warning sits below the file
line too and wraps at the reader measure: a draft whose note is gone, and a place loop. The file line and its links
keep their place when a warning comes or goes. Folder groups use normal-case paths after
type groups. The empty Status field in Edit shows the faint placeholder `A short word, such as alive.`

Change pickers focus search. Place choices start at the value column beside In, with one file link and a
bordered `Always use this one` action on each row; the link centres on the action's line. Change and the choice actions share one column: the
widest value or file link in the block plus an `L` gap. A saved choice changes only its key and has Undo,
which checks the file before writing. Always use this one saves a path with Undo. Change messages name
both values as the rows show them: the name of the note a link opens, else the words in the file
(`In: Ashfall Keep → not set.`, not the alias written in the file). When another World note has the same name, a
result and a picker name the folder too (`In: Watch House → Watch House (Locations).`), so the DM can tell which file
the change chose. They follow Change when they fit, otherwise sit below the whole row block. Only a Change message that
names a row value sits there; every other World result sits below the file line. A place that matches more than one
note is the link `⚠ <name> · N matches`: the warning icon is its own glyph in the warning colour before the link
words, so the underline covers only the words, as in `status_link`. Its accessible name keeps the icon. Different file and
stored names offer two safe choices below the warning's words, `L` apart: `Rename the note to <file name>`, with a
guarded update and `Undo name change`, then `Rename the file to <name>.md`, which keeps the name the DM typed. The file
move never replaces a file, not even one that arrives during the move, and checks that the note still holds the text the
reader came from: the whole file the index read, byte order mark, line ends and comments included. Where the folder's
file system offers no move that refuses an existing file, a hard link and the removal of the old name move it; if the
old name cannot be removed, the new one goes again and nothing moved; if neither can, the message names both files and
says to delete the new one by hand. Where the file system offers neither move, the choice says `Splat cannot rename
files safely in this folder, so nothing was moved.`; its result `The file is now
<name>.md.` has `Undo file rename`, which checks the same before it moves the file back, and only in the campaign that made the move. The file choice is left out
while the note's edit form is open, or when the name cannot be a file name. New NPCs show their
future path below the name. A compact NPC reader puts Fights as and In on one read-only line; both open
Lookup. A missing link there reads as its other values: the muted label, then `⚠ <name> · Not found` in the warning
tone (`In` `⚠ Watch House · Not found`).

A campaign world entry has a quiet Edit action, with a pencil icon and the word Edit, on its title line in Lookup and World. Its icon sits one `L` after the measured title at every title width, so a kind line wider than a short title does not push it away; a title with art keeps Edit after the art. Edit replaces the rows and
the text with one form. Read and Edit show Stat block, In, Part of, then Status, so a value keeps its place when the DM
presses Edit; conflict lines after Overwrite use the same order. Text follows in the reader's prose type,
without the title line. The Text field ends at the same `FORM_WIDTH` edge as Status. One frame holds
the editor and its scrollbar, so scrolling does not shorten the field. Stat block, In and Part of share one Change
column, an `L` gap after the widest of their values, which wrap within it. The shared save row
keeps the state at the left, then quiet `Cancel` while the form has no changes or quiet `Discard` while it
has changes, and Save changes at that right edge. Save changes is primary unless a live run owns Next turn;
then it is secondary. Cancel closes the form. Discard closes the form and offers Undo discard.
A draft whose note is gone (deleted, or another note took its place outside Splat) warns below the open note's file line, with
`Copy text` and, an `L` gap after it, `Discard draft`. Discard draft lets the words go with `<name> draft discarded.`
and `Undo discard` in the same place, until the next Edit, the next Discard or the Undo.
The rail shows World draft while an edit is unsaved. If a repair is also unsaved, it shows World repair draft once.
One save writes every change with one Undo. If the note on disk already reads as the save would write it
(for example, Overwrite adds a group that Obsidian added too), write nothing, add no Undo, and close the
form with the Info message `Your changes are already in the note.` A group change refused because Part of
could not be read closes without that message; the reason shows instead. A disk change uses the shared conflict choices.
The save that finds the disk change reads the note again for its list row and header count, as a refused Part of
write does; the draft stays as it is until the DM chooses. Below the
choices, one Small line for each field that differs says what Overwrite does to it, including the groups it
adds, removes and keeps.
If Part of could not be read, block its Change action with the unread-value message, for example
`Line 9: Part of "{link: '[[Lost Guild]]'}" was not read. Write each group as a link, such as [[Arena Guild]].`
A list written below its key names its first value that is not text, and that value's line.
In Edit, a quiet `Edit in Obsidian` link follows the reason. Read shows the reason in the same warning
tone below the file line, whose link is the one `Edit in Obsidian`; a reader with no file line puts the link
after the reason. Read and Edit show Part of as `not read` in the warning colour, with Change blocked by the reason. Other fields stay
editable.
Every Part of write checks the current disk parse flag before it changes anything. A refused write closes
its picker, keeps every disk byte and reads the note again, so its list row and the header count the fault. Adding or removing a readable group keeps the raw text of the
other groups. Overwrite keeps unread disk groups, saves other draft fields and shows the reason. With a readable disk
list, Overwrite applies the draft's group additions and removals to that list, so a group added outside
Splat stays. Groups compare by target, not spelling: a link Obsidian respelled (another alias, a folder
path) is the same group, and a group both sides have keeps the note's spelling on one line. A link to no
single note compares by its link target; plain text compares as written.
Edit shows `not set` for null, empty and whitespace-only optional links. Saving other fields keeps their
source text. World calls each file a note in related-note headings, Problems and conflict messages.
`Also linked from` and the page links under `From other notes` name a copy of the note, or another note with the same
name, by its path (`NPCs/Rook Ambry copy.md`), as search rows do.
A duplicate note's values read as every reader's rows do: a muted label, then the value as a link that opens it. Only
Change is left out until the repair is done. Each duplicate-note repair has a muted Small consequence below it, including the first repair's retained
link target. An `L` gap separates the choices, and an `M` gap follows the last consequence.

Task marks at the start of a list item show an empty box for `[ ]` and a checked box for `[x]`, with the accessible names `Open` and `Done`. Stored Markdown stays unchanged. Other brackets stay as text.

A note row has selected fill only while its reader is open. Open questions, Problems, creation and repair
keep the remembered note without selected fill.

Broken notes stay under Cannot be read. A note that cannot be read is an error in its row, the header count (and
Lookup's) and its pane: each row's second line is the error icon in the error colour, then the cause in muted words; a
long path wraps and is never cut. Every other World row fault (a broken link, `Same note in two files`, unread values,
table faults) is the warning icon in the warning colour before the muted words, as Library's rows are. The group uses the same gap as folder headings. They use the Campaign repair source editor. The editor and its save row end together at `LIST_MEASURE` (880 px),
anchored left, as Campaign's do, so Save stays near the state and Discard; the two-version comparison after an outside
change keeps the pane's width. The save row says `Invalid` while Save would refuse the draft, changed or not (the
message above it names the cause), as Campaign's and Library's do: a path outside the folder, text that cannot be read,
or an identity the campaign does not allow. These checks read no file. A readable unchanged draft says `No changes`;
otherwise it says `Unsaved`. An Overwrite that these checks refuse writes nothing and shows only the cause: `Not
saved.` is only for a failure on the disk. A Save that then fails on the disk
(read only, missing, a write error) says `Not saved` with Save's reason above the editor, and Save is no longer
primary. That cause belongs to the draft: an edit, Discard, Reload (also when the file holds the draft's words), the
file followed from disk, or a successful Save ends it. Keep drafts in memory and
warn before app close or a campaign switch. The unreadable count in the header opens the first broken
note's repair, keeping other repair drafts. A note that the system will not read (not UTF-8 text, or no
permission) cannot open in the source editor. Its pane keeps the title and a file line with only `Show file`: no
editor link, because no editor can open the file either. When a change such as Part of finds an open note that can no
longer be read, its error and the note's file line also offer no editor link until the file reads again. Its pane leads with an error that
names the file only as `this note`, because the title and file line name it (`Splat cannot read this note as text.`),
then gives the recovery in plain words at the reader measure, with Try again and Cancel at the message text. Technical details below holds the system's words. Its row's cause is short
(`Text format cannot be read`). Try again reads the folder again: a fixed note opens in the reader, a note that is
still broken opens its repair, and a file moved or deleted outside Splat ends the pane. Splat never writes that file.
Every other World message about a file that the system will not read or write, away from that file's pane, names the
file and the recovery, never the system's words. A rolled table selects the matching row, marks it with dice and
states the result below in UI sans. Typed rolls use well fill, a centred placeholder and dice as the column
heading. Fixed headers cover passing rows with page fill and a hairline. Generator cards name their table
sources below attributes. Every card's `Will save <path>` is the path Save NPC writes: the folder that holds most of
the campaign's NPCs, or `NPCs/` when it has none. Without a names table, the World page's card says `Built-in names. To
use your own, add a table of names to a note. Paste the marker on the line above the table.` with a quiet `Copy the
marker` action drawn with `line_start` and no icon, so its words start at the card's text edge. It copies the exact marker line,
and then a success message in its place says `Copied. Change 1d6 to the number of names.` until the card shows another
NPC. The message keeps the action's height (a Read control), so `Will save …` and the save row below do not move. No card shows the marker as text. Lookup's card says `Built-in names.` with a quiet
`Use your own names` link, which moves the same NPC to the World page's card. A session note's card says
`Built-in names.` only, so the note keeps the keyboard. Table warnings use the words name, look, manner and
want; hover keeps the marker key.

## The encounter ledger and editor

Encounters uses one column, at most `LIST_MEASURE` (880 px), with IN PROGRESS, READY TO RUN, GROUPS and
HISTORY in that order. Groups and History start closed when they have rows. The list keeps the scroll bar's lane
whether or not the bar shows, so opening Groups moves no row action; the creature results and the encounter
roster keep it too, so `Add` and the row menus stay put as a list grows or shrinks. When the run folder cannot be
read (a folder that held runs is gone, or cannot be listed), In progress and History say so (`Splat cannot read the
run folder, so it cannot show unfinished runs.`), never their empty states, until a read succeeds. Each row reserves right-edge slots
for its main action, quiet Edit and row menu. The menu holds deletion and starting another run. An encounter
with an unfinished run appears in In progress; when it ends, it returns to Ready. Keep in Ready during a run
leaves a blocked Start fight and a reason to resume from In progress. In the editor, a muted line below the option
says what it does. The header counts all Ready rows,
including these repeatable fights. Ready rows name creatures; an empty roster blocks Start with Add
participants first. The open-run block appears once under READY TO RUN. Its Ready action says Go back to the
run. When the header cannot hold every word, as beside Lookup at 960x640, the counts go first. Then Reload from
disk keeps its icon, with its name, hint and blocked reason on hover. New encounter and New group stay whole.

Unfinished rows show Setup · Paused or Round 2 · Paused and Resume. Every paused run's line two starts with the
run icon, also when its first part is a warning (`▷ ⚠ Changed on disk · …`). Open rows use status links Setup or Round
2, with only saved time on line two. One paused run uses Paused run in the rail, then the fitted encounter
name; full hover and accessible text includes name and state. It resumes in one click. With several, Paused
runs and `2 fights` open Encounters. An open run keeps Run and its round or Setup. Rail details never end on a
separator; a short rail shows Ended or Paused with full state on hover.

History opens read-only final state and log in place, outside the live-run slot. Final state has one name column, as
wide as the longest name up to half the room, so every HP starts at one x. Rows put outcome before ended
state and retain outcome and round. The detail header shows only Read-only and end reason. History rows keep
Open or Close and their menu; Start and Edit stay in that menu.

### Rosters and search

Creature stats use CR, HP, AC in search, rosters and groups; party rows use HP, AC. NPC search keeps fixed CR,
HP, AC columns, then `NPC · as Guard`. NPC roster rows put `as Guard` last. The main source has no suffix;
other sources and campaign-owned entries show source or Campaign, including roster rows. A different draft
starts with empty search. Names use text colour and underline on hover; a creature name opens its stat block,
with Add as the one result button. Named NPCs say In roster after Add. Party members already present say In
roster; search is empty when all are present.

Encounter and group searches include NPCs and add their usable stat block with the existing display-name
override, without changing the saved encounter format. A missing NPC stat block stays visible with its name,
blocked Add reason and Open in World repair. Unreadable package creatures retain filename and error line in
the roster; Open in Library opens repair. A creature that two files claim says the short cause on its row,
`Same entry in two files` (`Same note in two files` for a World note); the sentence with the file names is on the
row's hover and in the message the check gives. Only deleted files offer replacement. Deleted creatures keep their
last known name or Missing creature, plus a warning such as `Enemies · Not found. Its package entry was
deleted.` One fault has one tone: the message above the roster and the rows that state its fault share it.
`HP not set` has the warning status (icon and words) in a search result, a roster row and a group member.
An unreadable file is an error (the error icon and colour in the row), and a deleted entry is a warning. The
check that finds the fault records its kind, and the message takes its tone from that kind, never from its
words. A roster fault names the row as the DM sees it, `Row 2 (Cinder Wolf): …`, never its stored key or a
stored field name. Replace… uses the shared creature row with CR, HP, AC and Use. Replace all matching rows defaults to
on and retains names and starting values, with one Undo. In a group's editor, Use says `Replaced Unknown creature
with Ember Cultist.` with `Undo replacement` on the member result line that Remove uses; the newest member result owns
that line. Undo puts back only the creature, so later edits stay; a move or a removal ends it.

Roster rows have name and detail, quiet Edit and a row menu. Sides sit on the second line and change in Edit.
Menus hold ordering, duplication, Make it yours and Remove; missing rows keep Edit and Remove there. A menu choice's
reason (`Already last.`) or help line (`Edit a copy in this campaign.`) starts at the choice's words, after its
padding and any icon, and ends that padding (`S`) before the menu's right edge: the menu asks for its width with
both, so a long note keeps the right margin and wraps before it. A row menu opens below or above its button
inside the pane that holds the button, never beside it over a splitter or scroll bar; while it is open, the pane
keeps its button in view, also after the window shrinks. Side
pickers offer Party, Enemies and Other…; only Other opens a name field and replaces the value. Changed stays
at the end of the second line as one collapsed link. Shorten muted detail first; keep `as <base creature>` and
the full change link, removing source before base creature. Remove a source whole without an ellipsis;
hover keeps the full detail. The ` · ` before the link has the same blank on each side as the separators between
the stats: no item spacing is added around it.

An unread roster row stays visible by participant name and line, with its file line (path, Edit in Obsidian and Show file). The roster also shows that row in its place among the rows that read: its name with no number (the run will not add it) and `⚠ Line 20 not read` in the warning tone, so two rows of one name stay apart. The warning sits below the editor header and its save row, so a fault does not move the name field, Save encounter or Start fight. The last warning ends with what to do, and its file line starts at its words. Start fight is blocked until the DM repairs or explicitly removes that row. Plan starts use the same check. A resumed setup keeps the warning and blocks Begin combat while its source has unread rows. Saved run participants stay intact.

The encounter editor keeps roster rows, loot and notes within `LIST_MEASURE`, including the scroll content
and row actions. Add participants sits directly beside that column; extra space stays at the far right.
Its note says where party members come from in the DM's words: `Party members come from the sheets Splat
saved last.`
Notes retain their own width cap. Resizing keeps unsent notes, focus and scroll. Beside the roster, `This campaign’s
creature` ends its Name field where Save creature ends. Rules and runs puts its save state, Discard and reasons at
the start of one row with `Save run settings` at its end; without room, the state's words wrap before Save.

The editor header keeps Save and Start on line one; the name field gives up width. While Save would refuse a
changed draft (a roster row with no creature, or the refusal of the last Save while it still holds), the save row
says `Invalid` and Save and Ctrl+S are blocked with the cause; otherwise a changed draft says `Unsaved`. Save encounter
also gives the open fight from that encounter its new read-aloud text and notes for Prep; its roster and combat
state stay, and other changes wait for the next fight. An unfinished encounter
offers Resume, moves Start another fight into its menu and keeps run state in the save row. Blocked Start
fight says Close the open run first beside it. Sync and repeat options follow the roster. Use group_heading
for groups of controls and sentence-case field labels over single inputs; tabbed Add participants has no repeated heading. Group frames choose where Add goes;
search says `Adding to <name>` as read-only text. The frame's Add slot keeps the width of `Adding here` with its
check in every frame, so every name field ends at one edge and does not move with the Add target. Each encounter group has a frame below the roster with name,
side, members and notes. The target frame has an accent outline. The Encounters page keeps groups in one
closed section. Frames use Move to… for ownership; full editors use Belongs to, whose open menu is as wide as its control, as
Library's scope menu is. A group that no fight can offer
says so below Belongs to as a warning `small_message`, not as muted field help.

### Groups and changes

Member rows and Group notes end at the members column's right edge (at most `LIST_MEASURE`), inside the scroll
content and anchored left. The column keeps the scroll bar's lane whether or not the bar shows, so no field or
action moves when a long list starts to scroll. The group editor's header is one block that ends at that column's
edge: the name on line one, then the save row with the state and Discard at its start and Save group at the
column's edge. A reason without room on the save row takes its own line below it. The group name field and Belongs to end at one edge: the column's edge, or
`FORM_WIDTH` when that comes first by at least `XL`. So the edges are the same or clearly apart. In an
encounter's group frame, the name starts at the frame's left edge and ends at `FORM_WIDTH` or before its
actions. Removing any member shows `Removed <name>.` and `Undo remove member` on a result line. Undo restores the full member at its place and keeps other draft edits.

A plan group editor stays beside the note when `LIST_MEASURE`, an `L` gap and `READER_MEASURE` fit.
Bound the editor at `LIST_MEASURE`, anchored left. Below that width, the note stays behind its tab. Plan summaries start directly
below name and action, without an extra item gap, and extend to the menu slot. Order summary parts as side,
member count, XP, Changed, members. Shorten members first, with full text on hover. Member counts and XP use
one dot-separated line. Members and XP share one heading. Keep a member summary below search when there are
members, also in compact editors. Member names and stats sit above Count, Edit and the menu; count stays at
the left and actions together at the right. Changes stay on one line, such as `Changed · 1 to add by hand`,
until opened. Empty frames use one muted line, no zero count or duplicate search summary. Empty groups block
Save group with Add a creature first, and the save row says `Invalid`, as it does after any refusal of Save group
while that refusal holds. The save row's reasons (`Add a creature first`, `Save or discard to switch campaign.`) each
have one place, chosen as if both showed: beside Save while there is room, else a line below the save row. Those
lines are reserved whether or not their reason shows, so the first edit or the first member moves no field.

A change form (group member or roster slot) has one right edge: Display name, Note, the Other effect field with its
Add, and the Loot field end at one x. `Roll on a table` follows the Loot field.

Groups has one level of headings: For any encounter, each encounter, then Encounter deleted. Rows keep Edit
and a Remove menu action with Undo. Planned groups use Remove from tonight and Undo remove. Remove isolated
plan links; retain words inside sentences. Moving an encounter group to any encounter says `<name> moves to
any encounter when you save.`, with Undo move until save. Move to… lists named encounters only. Make available
to any encounter is the direct way to remove ownership; keep it distinct from Delete group, with Undo before
saving.

Arrivals reserves one Add slot as wide as the widest Add label in the panel. Arrivals shares one group block, at
the pane's left edge: as wide as the widest group heading, or member summary with its Initiative and Add, and no
wider than `LIST_MEASURE`. Each group's Initiative and Add block ends at that block's right edge, so every
Initiative field shares one left edge and every Add one right edge; the member summary takes the rest. A group's
faults and their repair links may use the list's width (`LIST_MEASURE`), so a link stays on its fault's line. List, editor and Arrivals resolve the same joining members and XP. Partial groups say `1 of 3 can join · XP 50 · Will add: 1 × Goblin Warrior`; XP covers joining members only; `+ unknown` means a joining member lacks XP. Flagged list rows retain member summaries and Edit.
Flagged members name their line, with a file line (path, Edit in Obsidian, Show file), Edit and a Remove member row menu
aligned with other actions. Show Count once, in the action row; retain `Count not read` in a faulty stat summary. Put each repair link at the end of its own member fault, without repeating the link words in the sentence. Edit opens the normal change form with readable values, unread text, reason and
example. Done keeps name and note; Remove has Undo. Groups, Arrivals and plan state how many can join and name
each creature needing repair with its cause, in the roster row's form: `Scout (as Goblin Warrior) · line 15: the change
"HP lots" was not read.` (shown name, base creature when different, source line, cause). One function
(`groups::member_fault`) builds every member fault in this form: a missing creature (`Unknown creature · line 13: not in
this campaign’s packages.`), an invalid change, an unread row, a member with no maximum HP or a D&D Beyond sheet, in
the list, the editor, the plan and Arrivals. Save's own check names the member the same way without a line, since a
draft's lines can move. The example (`Use a number,
such as HP +5.`) belongs to the group editor's warning, not to these summaries.
In the plan, each
such member is a warning `small_message` below the row's summary, with Edit group in the row's action column. Readable members can
still join. While a fight is open, a group that some members can join keeps `Add to fight…` in its action column and
puts `Edit group` in its row menu; a group none of whose members can join keeps `Edit group` there, because Arrivals
could add nothing. Changed summaries also name the base creature. Wrap between members, never inside names; a member
wider than its line breaks first before its `(as …)`, then at its own spaces, never inside a word, and `(line 15)`
stays whole. When no member can join, the summary ends at the count (`Enemies · 0 of 1 can join`): no XP and no
`Will add: none`. `Will add:` stays on a line with its first member whenever the two fit one line. The editor's members heading says how many can join when not all can (`MEMBERS 3 · 2 can join ·
XP 100`).

Every change field shows printed value and result, including changes added by hand. Editors, run cards and
fault details share that preview. An invalid stat names only the limit crossed: `Maximum HP would become -28.
It must be 1 or more.` Give an upper limit only when exceeded. In a change form, that error is a `small_message`
directly below its own field, at the field's left edge, and the preview beside the field takes the error icon and
colour; only a fault that no field explains (a value a linked sheet decides) stays at the top of the form. Done closes
the form and returns to its row. Change forms use the run's condition grid. The grid, its Other effect field and Add
stay within `FORM_WIDTH` at the pane's left edge, in the run and in Starting conditions. Both use one helper,
`conditions_grid`. Each column has a fixed width: its longest choice in the selected (bordered) form, measured once per
font size. Columns are `S` apart, so they sit near each other in a wide pane, and turning a condition on or off never
moves another toggle. Other effects use the run's
function: the `OTHER EFFECTS` label, each effect in Body text with its row menu, and the `Other effect…` field. Attack and damage changes sit below the affected action, with `XS`
above and `L` below. Changed Speed uses stat-table label and value columns.

### Loot lists

One editor holds an encounter's loot and a creature's loot in its Changes, from `loot_editor::show`. Each line is
a row with a `row_menu` Remove and `Removed <line>.` with `Undo remove loot`. A text line edits in place. A Library
item shows its name, the book icon and muted `Library item`. A roll shows the dice icon, `Roll`, a `number_field`
count, `×` and the table as a `menu_button`, then muted dice and note, or a `Cannot roll` warning status with its
reason. The add field adds the typed words on Enter. Suggestions under it, `ROLL TABLES` then `ITEMS`, are chosen
only by arrows then Enter, or a click. `Roll on a table` beside the field lists every table. A creature's list
refuses `;`, with the reason beside the field. Its Changes link stays one word, `Loot`, and names any key with no
table. In a run, a standing creature's card has one muted `Loot · …` line, and the list opens at 0 HP. A result
replaces its Roll button: the found thing first, then muted `<key> · <values> on <dice>`. A count of rolls on screen
uses the roster's form, `2 × gems`, with a blank on each side of `×`; the saved line keeps `Roll 2× gems`. The ended view puts
`LOOT`, a muted `N to roll`, secondary `Roll all N` and quiet `Copy loot` on one line above Final state.

Run settings put Roll NPC initiative help directly below its dropdown, before the tie checkbox: `Run setup
starts with this choice. You can change it there for one roll. Groups that arrive later use it too.`

## The live run

### Roster and setup

The roster is the screen. Fixed columns hold initiative, side mark, name, AC, temporary HP, HP and bar, then
Bloodied, 0 HP and sync state. A lone sync state sits beside HP; a second state uses line two. Sheet loading
says Reading sheet…. When every linked row has sync off by the DM, header line two says Sync off for the party and the rows omit it. When linked rows differ, disabled rows retain Sync off and its icon. Sync that restarts by itself never says Sync off. Unlinked creatures have no sync word. Header and row conflict words are underlined review links. The sheet colours only its connection
icon and Connected; sheet and HP facts use muted Small.

Names that need more than two lines use `RUN_COMPACT.name` and wrap in the shared name column. The numeric
columns never move: AC and HP keep their x on every row, so a DM can scan down HP to find who is hurt. A name
that still needs more than three lines ends its third line in `…`; hover and the row's accessible name keep
the whole name. Pictures align with the first name line. The fight panel keeps an `L`
left gutter, so a quiet action at its text edge never reaches a splitter. The Encounters page has no gutter of its
own: its title and its first column (`IN PROGRESS`, each row's name) start at the shared page edge, as World,
Library and Campaign do, and a quiet action at that edge uses `line_start`.

Conditions start on the second line before HP. Measure without change bonuses, shorten bonuses, remeasure, then
drop the bonus before more if needed. Measure sync before folding conditions into a count. Keep the first
condition and whole multiword sync states. Hidden-condition counts always keep their word, such as `5 more`,
including beside sync state. Move sync state first if the row needs more room. If the required words still
cannot share one line, sync state continues on its own line; the row includes its full height.
At minimum width, mandatory state words may use the empty left part of the HP bar slot; the bar stays below HP
and at least `XL` long. Compact rows put sync last after an `XS` gap, apart from conditions. Alone on the second line, sync starts at
the line's edge with no gap, like Bloodied on the rows below. If required state words still do not fit, use the run mark type size on the second line. If that still needs the whole row width, move the HP bar below the detail with an `S` allowance; keep every state word and the first condition. No initiative uses
the same underline as other row links. When the state column cannot fit, such as 960×640 beside a panel, move
its words to line two so the name stays whole.

Current rows use current_row fill, a 2 px stroke, a 6 px bar and TURN under initiative in its own column.
Compressed rows put TURN beside the number so turn state does not change row height. NEXT marks the next row.
Name lines keep one height with or without detail. Scale sets type; spare room grows rows up to 1.4 times
scale height. Name-only rows shrink to 25 px before scrolling. Natural rows keep at least scale height. Rows
share columns and picture size; shortening bonuses changes none of these.

Setup reserves initiative, Absent and menu columns on the name line, centred on initiative. Only NPC rows
reserve Roll, before initiative, so party names use that room and all initiative fields stay aligned. Reserve
the place column (`XL`) from the start, with an `S` gap before Absent. Reserve the tie column from the start when the
campaign's roll mode gives two or more NPCs one shared roll, so `Roll NPCs` moves no field. Otherwise reserve it
only while the roster has tied rows, and change its width only after the initiative field loses focus. With no
shared roll to come (or individual rolls), setup shows no empty tie column. Tied rows
keep fixed up and down arrow slots, in that order, with unused directions empty. Rows use the height of their name line (at least a Scan control), their Small detail line, and an XS gap.
Do not add the live roster's minimum height to setup rows. Setup retains its first
condition and counts the rest when needed. Shared-roll brackets stay beside fields and stop before detail.
Shared NPC rolls are one roll group with quiet brackets and ordering arrows; only equal totals across
independent rolls warn as ties. Header and overflow count those tied rows; the band says `2 tied`. Arrows
still work inside shared rolls.

Setup has a `Label` column heading INITIATIVE above empty, labelled number fields.

An off Absent toggle in setup uses muted text and no fill. Setup rows stay in place. Once all present rows
have initiative, a muted ordinal follows each field, also while the last field has focus. It updates with
totals or tie rulings, and the ready line
names who goes first. Setup names its Undo action. Unread-row warnings sit below the setup header, in the notice place. Their file actions follow on the same line when they fit. Begin combat says `Fix the unread row first.` below its blocked button; the
row menu opens Correct. A setup row's menu says what each choice does: `Correct…` and `Change NPC and roll mode`
(`Hide NPC and roll mode` while they show). Unread-row notices use compact message type and the unused ready-line space
so the roster keeps its height. Overflow uses the live list's band and counts hidden missing initiatives and ties.

A made-up creature shows current and maximum HP and typed initiative as a total, without printed values, in the
stat block's three columns, so the trio keeps its place from one selected row to the next. A
side of its own keeps its word on live rows. Setup and live play share side, HP, temporary HP and AC parts.
Correct holds correction fields. Opening it does not change sync state. A row says `Sync off` only
after the DM stops sync for that character; an untouched linked row has no sync word.

### Header, dock and panels

The header has two lines. Round and current turn use bold sans larger than row names, beside Next turn.
When the turn heading does not fit, drop `’s turn` before shortening the name. Keep the final copy number,
such as `Cinder… 1`. Hover and accessibility keep the full name. Line
two keeps a result icon and notice at the left and Undo at the right. Measure notice first; Undo gets the
remainder less `L`. Undo always keeps the word `Undo`: shorten the target first, then drop the `Next:` notice
(the roster still marks NEXT). A state notice stays whole. Beside one, Undo keeps a shortened target while
one fits, such as `Undo: 3 damage to Brannoch…`: a sheet's step drops `D&D Beyond` before its name
(`Undo: HP for Brannoch…`), then Undo drops its `U` key letter. A sheet-change notice that leaves no room
for that target goes below the roster. When no shortened target fits, Undo shows its step without the target:
`Undo: 8 damage to 3` for a step on several rows (its count stays, `participants` and the half count go),
`Undo: exhaustion 0 → 3` or `Undo: initiative` for one row. Undo shows `Undo` alone only when not even that fits. A name
is never cut to only `…` and its copy number; retain copy numbers, action words, amounts and condition
directions. Hover and accessibility keep the full action. Empty history says Nothing to undo, to the left of
blocked Undo on setup and live screens. A run file from before Undo steps were saved says Nothing to undo since
reopening, and a file whose steps could not be read says so. A narrow slot uses
Nothing to undo with full hover and accessible text. Sheet updates name whose sheet changed; only sheet reads
say sheet changed. Sync-panel choices stay in Log and Undo.

Notice priority is conflict, recording silence, other sync notices, next name. Silence and short sync state
stay together when space is tight. Connect to sync loses its link before either state loses its word. Keep the
full `Sync off for the party` beside recording silence. If the Undo target cannot fit, show `Undo` and keep
its full target in hover and accessibility. An empty Undo can omit its adjacent reason when it cannot fit;
the control keeps that reason in hover and accessibility. Keep the
next participant visible when Undo is empty. Sheet notices fit after conflict with Log and Dismiss when
possible; shorten to character name and first changed value, with full text in hover and Log. Otherwise show
once below the roster, keeping Undo visible. Sheet-change notices use `small_message`: warning icon and bar,
text-colour words. A result below the roster (`Brannoch Vale joined at 20 · acts next round.`) uses the same
`small_message` in the success tone, with the past tense Arrivals uses (`joined at`, `moved to`). Centre compact notices with their Log and Dismiss controls, in the header and below
the roster. Wrapped notices keep top alignment. Keep an `XS` gap after the icon and an `S` gap before
the bar. Include those gaps in the header measure. Stat-block errors use `small_message` in the error tone: the fault in
text colour, fitted to one line, with the full error on hover, then the link `Campaign → Packages` as its action
after an `L` gap.

Long local effects show four words and more; Undo keeps four when they fit. Narrow Undo keeps Undo add or Undo
remove, two effect words and target. Concentration Undo says start concentration or stop concentration before
the target. Exhaustion Undo keeps both levels, such as `exhaustion 2 → 3 on Oren`. Exhaustion level uses the shared `stepper` at Scan height. One drag on the level is one Undo step, from the level where the drag started, committed when the drag ends; each − or + press is one step. At a limit the button says `Exhaustion stops at 6.` or `Exhaustion is already 0.` Hover and accessibility keep full text.

The dock has a fixed place above the footer: Amount, Damage, Heal, Temp HP, selected name, Conditions, Conc,
Absent. Off toggles use text colour; disabled controls stay muted. Enter applies damage. The dock's and footer's
background uses roster width; panels never cover them. The dock's controls are one block at the left, no wider than
`LIST_MEASURE`: the dice end at the block's edge, and Conditions, Conc and Absent follow the selected name after an `L`
gap, not at the roster's far edge. Fit whole words, retain a distinguishing last word or copy number or
letter, end shortened names with an ellipsis and drop trailing joining words. Hover and accessibility keep
full names.

Overflow clips above a separate page-filled band and hairline. Count rows above and below, including rows cut
by the edge, and their Conflict, 0 HP and Bloodied states. Each turn scrolls current and next rows into view.
Other pages keep the strip's turn, conflict, secondary Back to run and primary Next turn. The strip's conflict
follows the header's rule: one link, `Sync conflict: <name>` when it fits, else `Conflict: <name>`, which opens
the conflict in the run.

The panel has participant tabs Stat block or Sheet, Conditions, Correct, then `L`, then fight tabs Log and
Prep. Arrivals has no tab: a secondary Add to fight button opens it, selected while Arrivals is open. In live
play it is `S` left of Next turn, at the same height, as the strip pairs Back to run with Next turn. Below the
dock's all-words width it keeps the plus icon and the word Add. The full accessible name stays Add to fight. In setup it has a fixed place at the roster's left edge, on the Undo line, at Undo's height; Undo and its
reason take the rest of that line from the right, so no Undo label moves it. Ctrl+G also opens it. Fight-tab titles name only that tab in setup and live play. Titles stay on one line and retain copy
numbers when shortened. When tight, fight tabs share a Fight caret menu with tab underline. Never wrap the tab
row. At zero HP, party-side participants show death-save counts and actions on one line below the tabs.
Other participants show them only if a success or failure is already recorded, so no existing count is hidden.
Clicking a count edits it. Sheet
shows live death saves once, with correct singular and plural words. A character's sheet panel starts with its sync status and facts on one line, one blank before each `·`
(`Not connected · Fetched 3 h ago · HP from this run`). Correct has no duplicate fields; HP owns
its help. Correct initiative says This row moves. The current turn stays. Labels share one column, as wide as the widest label (`Initiative bonus`) and an `S` gap; blocked
initiative and DM-note commits have a short reason on its own line below the field, at the field's left. That line
stays when the reason goes, so nothing below moves as the DM types. The DM note field takes the form's width
(`FORM_WIDTH`) up to Add note. Correct HP has an `L` gap after death saves.
Empty Prep offers Edit encounter if its encounter exists.

Changed creatures show Changed and the changes the printed stat block does not show, such as Attack +1 or Speed
40 ft, on line two, aligned under the name at every scale. Setup shows only Changed. If name-only shrinking is insufficient, keep the
first table bonus and more; long conditions keep those that fit and a Conditions count link. The card puts
changes at the top of its Stat block tab, except AC, maximum HP and initiative bonus, which already appear in
the value trio. Add by hand comes first, in larger type on one summary line, and holds only what the run
cannot apply, such as speed. Splat's rolls include attack and damage changes: the card and the editors list them
as `in Splat’s rolls`, the tiles show the numbers rolled, and nothing tells the DM to add them again. An arriving member says `Changed
· arrived with <group>` once. Use run AC, current/maximum HP and initiative bonus, with printed values below
changed maxima or bonuses. Changes scroll with the stat block; each participant keeps its scroll position.
Paused snapshots state when they differ from the current file, subject first: `This stat block uses the copy from
the start of the fight. Cinder Wolf.md has changed.` (`This sheet …` for a character; the same subject when the file
was deleted, cannot be read or is not in the campaign), then `Read the new version`, whose words start at the
sentence's. The sentence is the changed-on-disk state: the warning icon in `document.external`, then words in the
text colour, so it is not read as part of the stat block. Under that notice a character's
card says only `The roster has the current values.`; without it, `Saved when this run began. The roster has the
current values.`

### Rolls

A stat block's actions are tiles: each rules paragraph with an attack, a save DC or damage, in printed order, in a
wrapping row with `S` gaps. As many 208 px tiles as fit share a line; a tile alone on its line (the only tile, or the
last of an odd row) takes the line's width, up to the reader measure, in a run, the Library and Party. Tiles on one line
share one height. A tile has a hairline frame with control radius:
a Phosphor icon (sword, shield-check, fire) in muted Body, the name in Strong, its Ctrl number in faint Small at the
right in a run, then its numbers in muted Small, then, when the text says, where it lands and what a save does
(`Reach 5 ft`, `Range 80/320 ft`, `10-ft emanation · half on a success`). Numbers and that line wrap; they are never
cut. The rules paragraph shows on hover and on keyboard focus, and it is the tile's accessible description. Hover
draws the frame in the focus colour. In a run a tile shows the numbers Splat rolls, with the run's attack and damage
changes in them; its hover adds the printed numbers. Spellcasting stays prose. A character's Actions page shows its
attacks as tiles, with the reach or range its sheet gives (`Reach 5 ft`, `Range 120 ft`).

In a run a tile arms its action: its frame turns accent on the selected fill and its icon becomes a crosshair, and
the panel stays on the actor. Under the tiles, a crosshair and the next step in Small (`Click a target to roll against
it. Enter applies the damage. Escape stops.`), the crosshair centred on the first line when the words wrap, led by the way the d20 rolls when it is not plain (`With advantage.
Click a target…`). Then quiet actions on their own line, the first of each line with `line_start`, so its words start
at the tiles' edge: `Stop`, and for a save `Roll damage` and `Half for those at or above DC`, or for a player's attack
`Roll for them`. Each other row shows its roll words in accent Small in one place, at every size: the line under its
name, after its conditions (and after its state word where the row has no state column), never in the state column.
They are the chance to hit it while an attack is armed, with the way it rolls (`51% to hit · advantage`), then the
result against it, a plain comparison with no hit or miss verdict (`17 vs AC 13 · 10 damage`). They take the longest
form that fits whole, dropping words before the result first; the result's last two parts (the comparison and the
damage) never drop, and are cut by whole words only where the line has no room. The row's accessible name holds them
whole. Roll words never move a row's columns. When the conditions leave no room for one word, the words take a line
of their own under them, at the name's edge, and that row grows by the line. A save's creature targets show their roll in the AC
cell. After `Roll damage`, each target says its own result, from the amount Enter gives it (`Saved · 1 damage
(half)`, `Failed · 3 damage`), and keeps it after Enter. Enter finishes the save: its tile disarms, the hint and `Roll
damage` go, the rows show their AC again, and the panel and the dock go back to the caster. Undo of that step arms the
save again as it stood, with its saves, its damage in Amount and its results. Outside a run a tile previews, and its
totals show in Strong under the tiles, with a muted `·` between parts (`11 to hit · 5 damage: 4 piercing, 1 fire`), as
in a run. A narrow line moves the `·` to the next line with the part after it.

While Lookup or Notes hide the panel, its tiles are one line of secondary Read buttons above the dock, with the run's
changes in their numbers (`Dagger +6`, `Fire Bolt +7`). The line keeps an `XS` gap above the dock, and its first button
starts at the dock's content edge, where Amount starts. A button shows whole or not at all. The actions with no room
go in a `menu_button` at the line's end (`1 more`): its menu lists each by name with its `Ctrl+3` number, and a click
arms it while Lookup stays open.

A right-click on a tile, on a number that rolls a d20, or on a target row while an action is armed opens a quiet menu
(Read density): `Roll`, `Roll with advantage`, `Roll with disadvantage`, and inspiration's line when the campaign's rule
and the character allow it. Every such menu opens under its tile, number or row, so what it is about stays in view,
and stays inside its pane: a tile's menu is the tile's width, so it never crosses the panel's splitter.
Each of its sections has a heading in muted Small with what it rolls: `Bite` or `Dexterity save` for a tile or a
number, `Bite against Cinder Wolf 2` for a row, then `Inspiration: reroll one die` with a line for each die (`4 on the
d20`); its choices start at the heading's words. A damage question opens there too: the condition in Small, wrapped at
320 px and left aligned (never justified), said as a statement without the rule's `if` and ending in a colon (`The
cultist moved 10+ feet straight toward the target immediately before the hit:`), then `Yes · …`, `No · …` and
`Cancel`, starting at the condition's words. Headings in these menus are left aligned too.

Every such menu opens from the keyboard as well: Shift+F10 on the focused tile, number or row, or a screen reader's
`show context menu`, opens it with the focus on its first choice, and the focus goes back to its owner when it
closes. The Menu key does the same where the toolkit passes it on (egui 0.33 does not). Hover on a roll says
`Right-click or Shift+F10 for more`.

Numbers in tables that roll (attribute Mod and Save cells, skills, a sheet's abilities, saves and skills, a death
save's `Roll`) are accent text with a quiet underline, at least 24 px each way. Attribute rows are 24 px for this.

### Arrivals

Only search shows until a name is typed; then the made-up creature form has two fixed field rows. The search field
is as wide as the group block below it, so the panel has one right edge. In the form, `Count` and `Initiative` take
one word column, so their fields start at one x, and `Side` starts where `HP` does. The form's reason line (`Type its
HP. A creature made up here has no stat block.`) keeps its height when the reason goes, so Add and the groups below
stay still. Search
shows five matches. When more match, a quiet `Show all 6` (the number of matches) lists every match in place, in a scroll list at most
eight rows tall, so the words, the chosen match and the roster stay in view. New words return to five. Its side
picker says Side and uses roster words. List encounter groups, tonight's plan groups, then any-encounter
groups. Empty sections use group headings and one muted line. New group opens Encounters. Each group's first
line has its draggable name, side, count and XP; then members, Initiative and Add. A blank Initiative field rolls initiative.
Dragging outlines the roster in accent. Added groups retain Add and name their joining round. Setup additions
say Added before combat. A status that cannot fit the member slot whole takes a full-width line above the members
and controls, and wraps there at its spaces, never inside a word. Names include count when adding more than one creature. No confirmation is needed;
Undo names the addition with the same count, `Undo: add 2 × Wolf`. The result names who joined, in one form for a
group and a creature added by hand: `Lost Patrol joined at 9.`, `Ghost of the Mill joined at 9. Tied with Ember
Cultist at 9, placed after.`, `Wolf 1 and Wolf 2 joined at 14.` Every result is a success message, tied or not; the
`Move … before …` button below a tied one shows that a choice is open. That button, the moved result and its Undo
name the arrival as its message does (`Move Lost Patrol before Ember Cultist`). When the roster has two groups of
one name, the arrival adds its first member: `Rival Team (Rival Captain 2) moved before Rival Team`. The Log keeps
`Added by hand` and `entered by the DM`.

Exclude missing, unread and invalid members. Say `1 of 3 can join`, `Will add: 1 × Goblin Warrior` and
`Add 1` using only joining members. Keep excluded members under Cannot join with reason and Edit group repair.
Only zero valid members blocks Initiative and Add. Invalid Initiative retains text, says `Enter a whole number, or clear
the field to roll.` as a warning `small_message` (amber icon and bar, text-colour words), and blocks Add and drop. Campaign → Packages opens repair.
In the group editor, an unread Changes cell (`Changes “HP lots” not read. …`) is a warning `small_message`, as
every unread value is, and its example line starts at the message words. `No members can join.` in the Encounters
list is a warning `message`, in the same form as the member faults above it.
In the group editor, a missing member's `Campaign → Packages` is a notice action under its warning
(`message_with_actions`): a bordered secondary button whose frame starts at the warning text. Arrivals and the
Encounters list draw a member fault with its repair (`Campaign → Packages` or `Edit group`) through
`groups::member_fault_row` (`message_with_action`): a bordered secondary button beside the fault after an `L` gap, or
below it at the words when there is no room. A message action without an icon is never a link.

Join status parts on one line with dot separators beside the group name when they fit, else before the first member. Use a separate line only when neither has room. `Changed` keeps its muted pencil. Member names stay intact. A line never ends with a separator: the `·` moves to the next line with the part after it. Keep `at`
with its total in arrival messages. Live group notices name only the group and its initiative totals. Member names and roll details stay in Log. Arrival faults use the shared message form (icon and bar in the fault's tone, text-colour words) and the tone recorded for that fault. The same fault keeps its tone in the editor, the Encounters list, Arrivals and the session plan: an invalid value, such as `Maximum HP would become −28. It must be 1 or more.`, is an error; a member that is missing, unread or has no maximum HP is a warning. Every view draws each member fault in its own tone. The group adds its own reason (`No members can join.`) as a warning message only when no member fault shows; with faults, `0 of N can join` and the faults already say it. Live initiative notices name who joins and the total. Before the current turn,
add acts next round. Tied arrivals name both participants and offer Move before with Undo, before the first
member of a shared-roll block. Keep tie details in Arrivals and Log, outside the Undo label.

### Saving and closing

Recovery copies keep the encounter name and preserve the other file. Start their second line with `Copy 2`, before `Closed last`, round and state. Only file names keep the UTC save-time suffix.
The last closed run owns primary Resume, unless the encounter's editor holds unsaved changes (then `Save
encounter` is primary). `Closed last` follows a copy number; Changed on disk marks a preserved disk version, after the copy number when present. Keep last-change wording from the close dialog. Saved rows use relative time today and yesterday,
then a short UTC date. Edit stays for recovery copies whose encounter exists; reserve an empty slot otherwise.
Measure actions before assigning paused-row text width. Saved time follows turn without a full stop. State
words come from one function: Setup, Setup · Paused, Round 2, Round 2 · Paused, Ended · Round 2. An open run
with other unfinished copies names its copy beside Saved and in rail hover.

A disconnected run has Connect to sync on its header only; blocked footer Sync party uses that cause on hover.
End encounter… shortens to End… before it moves into a menu. Keep End… visible while it fits, including on recovery copies. Hover and accessibility keep End encounter…. Keys keeps its word beside Lookup, Notes and a wide panel. Saved and Undo
align with the roster's right edge. The footer uses error status when saving stops. Ended runs put short
conflict and pending-change counts below blocked Close after sync at the panel's right edge, with an `L` gap
between character sync blocks. Below the sync blocks, the closed `Prep` and `Combat log` sections share one left
edge. Signed-out conflicts offer Connect to refresh before Show file. Ended sign-in
uses muted recovery text; failed Connect retains error state. Sync stopped links to Party and states recovery.

An ended view has one control that takes the ending away. While the ending is the newest Undo step, that is
`Undo: end encounter`, and no other control shows. Otherwise it is secondary `Continue fight` with the run icon,
`S` left of primary Close run: the two ways out of an ended fight, with the one that changes nothing last. Undo stays
at the roster's right edge. The title, the outcome and the save word keep their words: Undo gives up its target
first (`Undo`, with the full action in hover and its accessible name). When the actions still cannot share the
title's line, as beside Lookup at 960x640, Close run stays on that line and Undo and Continue fight take a line of
their own below it, at the right edge, with Undo's full action. A closed, ended run read in History has the same
secondary `Continue fight` on its own line under Read-only, blocked with `Close the open run first.` while another
run is open. Continue fight is offered only for the fight that was played last (`README.md`, Running an encounter
as DM). For any other ended fight the control is absent, and one muted `Small` sentence stands in its place and says
why: `A later fight or rest was played. Start a new run to play this one again.` It is a sentence and not a blocked
button, because no step in this view makes the control work. In the ended header the sentence wraps to at most
`15 × L` wide, so Close run and the title keep their places; on the two-line header it follows Undo on the second
line. In History it is the line under Read-only. Neither is modal, and Undo names the step: `Undo: continue fight`. After Continue fight, the
footer's End step starts its outcome field with the outcome of the ending that went. Whenever that field opens
with text, the text is selected, so that Enter keeps it and typing replaces it.

A run whose saved Undo steps cannot be read says so once, as a warning `message` in the error line's place with a
Dismiss button. It wraps beside Lookup and pushes the roster down by its lines. The Log keeps the line.

Footer and strip Close run close at once only when saved with nothing unsent; otherwise ask for a choice. List
lost changes on separate lines, up to three, then the remaining count. Deleted runs with unsent changes use
warning. App close says Keep running, Keep recording or Keep processing for each task; recording and job
titles name the unfinished session. Sync conflicts name the character and offer Review conflict. The run's
conflict dialog offers to `close and choose next time` only when Close now works. While a sync change is not
confirmed, it says `Choose which HP to keep before you close.`, and the cause shows once in muted `Small` text
above the actions, which all wait for it.
The app-close and run-close dialogs draw their recovery actions through one shared function: quiet, on one line,
Connect to sync before Review conflict, and the first one starts at the dialog text. Stop and
close is secondary when saved work remains.

Incoming sheet changes remove only that character's Undo steps. Remaining snapshots keep current sheet values.
Atomic steps involving others end older affected steps when needed to avoid stale values; Log names each
removed step. DM conflict choices are named Undo steps. Sync-choice Undo names the character or count of
characters.

## Sessions and the timeline

The Read/Write switch sits at the right end of the note frame’s top edge at every size.

### Notes and templates

Notes are Markdown in the campaign folder. Write styles source in Reader faces: large ruled headings, italic
emphasis, and visible markup in the plain face and faint colour. Store exactly what was typed so Obsidian
reads the same text. Read uses the one Reader, anchored left, at no more than READER_MEASURE
(at most 75 characters per line).
Write keeps the same left-aligned READER_MEASURE as Read (at most 75 characters per line), inside a
frame no wider than READER_MEASURE plus its padding, anchored left. Spare width stays outside the frame.
Stored line breaks stay unchanged. Events and recordings follow column width;
book prose keeps READER_MEASURE. The gallery's Markdown sheet shows both
views of one note. Autosave runs two seconds after editing; show muted Saving… with the busy icon during
delay. Disk changes stop saving and open the comparison in Keeping a draft. Missing notes show Notes not saved
with error status in header and rail. Rail conflicts say Notes changed, with Notes changed on disk on hover
and for accessibility.

Session title and muted date wrap together, with Record on the title line. Below 640 px, save state and
Details sit beside Read on the content tab row. Read and Write sit there while notes are visible. An empty
Read view keeps only its inline Write link; hide tab-row Write until the note has content. Notes saved
centres on the title action row. A saved note is the calm, usual state: `✓ Notes saved` is muted
(`document.clean`) on the session page and in the Notes panel, also just after a write. Last time is one line linking the previous session and recap, with full recap
on hover. Without recap, use prose under What happened and omit link-only lines; without either, show the
session link alone. An older session warns once and links the session being played, which the list marks
Playing. The action says Return to the session being played. When the two session numbers match, the warning
names both note files. Reading old notes changes neither the destination for new events nor runs: use the newest session
unless the DM chooses another, and name that destination on Timeline before adding.

Read shows Obsidian links inline exactly as written. Hide link-only Plan for tonight when its fights and
groups already appear beside the note; in tab layout show one Small muted count. Keep prose and stored
Markdown unchanged. Otherwise empty notes say Nothing written yet. with a Write link. Readers hide untouched
Personal notes help; editors retain it. Note editors keep all four frame edges visible and scroll only text.
Notes opens at What happened. New sessions use a Label heading, show the note path as fields change, and give
Number a width of five `L` units.

Templates is a quiet header action. Its page names Templates and Campaign as owner, starts with the shared
quiet Sessions back action and puts New template in the header. Hide New template and clear list selection
while its form is open. Keep the list's help outside the form. New forms have `L` above the heading, `M`
below, and `Starts with the title line only.` below Name. Built-ins leave one blank line below each heading.
Template editors use the note's styled source face, no input well, full column width and height. Deletion and
Undo stay below the template list.

### The session's plan

The plan takes 40% of page width beside notes, between 330 and 480 px. Below 720 px, plan and note have
separate tabs; stacked Timeline says All events. Each row has an icon, link and second line, with a main
action on the name line, fixed right-aligned action slot and separate menu slot. Row fields open in place,
align with the name and use Timeline form parts. A caret and accessible expanded state mark open rows; a
hairline follows details after `S`. New events stay closed with kind as title. Open rows show Title and Notes;
duration and place stay on Timeline. The Add strip stays directly below Tonight (or This session), above
messages and scrolling rows. Fight, Group and Long rest stay visible. If all seven actions do not fit on one line,
a compact `⋯⌄` button, named `More Add actions`, holds Day, Short rest, Travel and Major event. Its hover text names those actions. Keep the strip on one line at all three supported window sizes. Long rest stays one click away (T11.2). Timeline uses the same width rule.
It stays in place when messages or rows change. Empty plans use empty_state
below the strip. Add-strip messages stay below Add. In notes ends at the button
text's inner edge.

The first upcoming fight gets primary Start fight when no run is open; later fights use secondary. Use Start
fight here and in Encounters. Saved unfinished fights offer Resume, including timeline rows whose run
is not a planned note link.
Find a timeline row’s saved run by its run ID among all unfinished runs. For a planned encounter, use its
saved source ID. For a legacy run without a source ID, use the Encounters matching rule: remove the
` · saved …` suffix and require exactly one encounter with that name. The plan also requires exactly one
planned fight with that name. Delete the saved run stays in the menu. Start a new run is available for
planned encounters with a saved run, including legacy runs without a source ID. Open fights use a status link
while the strip owns primary. Planned, played and running
summaries use space below actions up to the menu. Keep summary parts and Ended · Round 1 whole.
A line never ends with a separator: the `·` moves to the next line with the part after it. Group summaries wrap between members, never inside a
name, to at most three lines, then shorten with full hover text. A flagged group's summary wraps the same way and
shows every fact, however many lines it takes. Plan and Timeline share fight summaries: open fights show round as a run status link
without Paused; ended, named and custom results keep stored words. Played fights show result and last round
once on line two, then final party HP in a grid of two or three first-name cells in the party bar's order
(players the party no longer has come last), quiet Log, Whole run and
Timeline links. Hover keeps full names and conditions; Remove stays in the name-aligned row menu.

Remove from tonight removes one fight or group link, retaining surrounding words, with Undo remove below that
row after `S`. Removed groups leave message and Undo in their old place, keeping other rows still, until the
next note or plan edit. Removing a timeline event names it by its state or result, `Removed “Bridge Ambush”
(Round 2 · Paused).`, and, when another row of that name stays in the session, says so: `The other “Bridge
Ambush” and its result stay.` The book owns that message and its Undo remove, on the plan and on Timeline,
and they belong to the removed event's session. They end at an edit of that session's note, plan or Timeline
events, and when combat begins in that session (its event reaches Timeline; opening Setup does not). Another
session's edit, opening a note, a run's new round and a reload that leaves that session alone keep them. A
newer removal replaces them. One step gives one result in the plan: after a removal, Delete the saved run
replaces the removal's message there, and Timeline keeps its Undo remove until the book's removal changes.
Undo puts the event back after the event that was before it. Plan additions offer Undo add. Every Add result
comes from one function: `<event> added to Session 2.` names the session it went to; a restoring long rest
then says `Everyone starts the next fight at full.` Timeline lists every session, so its result names only
the event. A Timeline event's link update uses the Library's words: `Links now say “<name>”.` with
`Undo link update`. A recording count stays muted inside its tab and
accessible name. Duplicate saved-run names first add session name; only same-name, same-session rows or rows
without a known session need a date. Same-date copies also show time. Such rows omit Saved … so time appears
once. When the second line is too long, its `Last:` part gives up whole words first (`Last: 1 damage to
Kestrel…`), and goes when no word of it fits; the save time stays whole. Hover keeps the full line. Resume has
the run icon.

Session and Timeline add long rests with the same full-HP ruling, message and Undo add. Every closed
long-rest row shows an icon and either Everyone at full or Not at full. Open rows hide that summary so the
ruling stays in place. Actions say Mark everyone at full
or Mark not at full; states say Everyone at full or Not at full, with the ruling's icon and colour on the
plan row, the Timeline row and the detail alike. Keep one Undo ruling until the next edit. Its message says the
ruling now in force: `Everyone starts the next fight at full.` or `The next fight starts from the HP the party
has now.`
Resuming a saved Setup after a restoring rest starts party HP full, says `Party HP is full after the long
rest.` and offers Use the saved HP. Use the saved HP is its own named step, not an Undo: the header then says
`Undo: use the saved HP`, and the same place says `Party HP set to the saved values.` with `Undo use saved HP`, which
gives back the full HP. A message's Undo names its step, as every other message Undo does. Combat retains saved HP; the rest message names those fights. A run started after a
restoring rest with no fight since logs `Party HP is full after the long rest.`, and its saved row says `Last:
full HP after long rest`; `carried over` is only for HP from a fight. Every `Last:` value that Splat writes
starts in lower case.

### Party bar

While a session is played, a band of the page above the Session page holds the party. It uses the page
surface and the panel's hairline, never `raised`, and one `Read` row of height, so nothing below it moves.
`Party` stands in its own Label column, as `Add` does on the plan's strip. Each item is a 24 px portrait, the
player's first name in `Small` text colour, an `hp_readout` at `Numeral` over `Small` with its `hp_bar` along
the item's bottom edge, temporary HP with its shield, the run's `Bloodied` or `0 HP` icon, word and colour, then stats in `Small`: a muted label, a space and
the value, in the Party list's order `AC`, `Per`, `Ins`, `DC`. `DC` is left out for a player with no spell
save DC. Items sit `M` apart, with no boxes. All items show the same number of stats; when they do not fit,
the optional stats (`Per`, `Ins`, `DC`) drop from the end, down to
`AC`. Then the portraits go together, and then `AC`: AC answers "does a 17 hit?", and a portrait repeats the
name beside it. HP and temporary HP always stay. The exception, when first names and HP still do not fit: the
names shorten to an initial (`B.`), with the full name in hover and the accessible name. If that does not fit,
initials stay and the HP state keeps its icon and colour without its word (the word stays in hover and the
accessible name). No item is ever cut off: when even that does not fit, the items that fit show whole, and a
muted `N more` names the rest, with HP, in hover and its accessible name. `N more` takes keyboard focus, and
focus shows the same list as hover. Hover shows the full name in `Strong`, then one fact to a muted `Small` line: HP, AC and DC, passive scores, conditions, and the
source. Hover fill and the 2 px focus ring use `RADIUS_CONTROL`. A click opens the sheet on Party.

The source status (where the numbers come from) is measured by itself, after the items are fitted. It
stands before Hide the party, `L` after the last item, whenever the room the items leave holds it, at any
level. When its full words (`After Bridge Ambush`, `Sheets fetched 2 h ago`) do not fit, it shows a short
form (`This fight`, `Last fight`, `Long rest`, `Sheets`, `No sheets`); hover and the accessible name keep the
full words. It never takes room from an item, so it never moves one.

Outside a fight the level fits the words in view. While a run is open, the level may drop when the words
in view need it (a `Bloodied` word, temporary HP), but it never rises again until the run ends, and no
item narrows, so nothing moves back when a player is healed. The held widths are measured again at the
held level each frame, and dropped when they would hide a player who shows without the hold; the
held level stays, so the hold never hides a player. A change of bar width (the window or the rail) or of the party
measures again. The bar keeps this in its own state, keyed to the run, never in
the party data.

### Timeline

Timeline is a story-order ledger: one column, hairline spine, icon and word per kind, no kind colours or
cards. Days number themselves with shared group headings and no rule. Omit each row's day under that heading;
keep it in searches and filters. List width stops at LIST_MEASURE. Selected detail sits directly beside it
with Lookup's width rule and spare space at the right. No selection means no detail column.

Detail headers stay above the scrolling form: kind, title, save state, Copy link, Remove menu. Measure the
title with Copy link and Saving blocked to avoid moving fields; otherwise put actions on a second line above
the scroll area. Title and label stay fixed below the header. An S gap and a hairline below Title mark the
top edge of the scrolling fields. Headers end at the fields' right edge. Opening Timeline, changing filters, selecting or adding resets fields to the top. Day
headers use only the named calendar icon; days show Starts without Ends, and start time is Kind's help line.
Field labels have `M` before them. Copy link uses a check and Copied for two seconds. Link text stays in a
quiet section; Place after is the only ordering control. Adds to shares the filter line. Context shows visible
out of total while filtering; the filter says Major events.

Added, removed and link-update notices stay in the list column. Removing an event counts notes that still link
to it and uses warning tone when any do. Each item's save state tracks its own edits: muted Saving… during
delay, Saved when unchanged, Unsaved with reason when blocked. Rests show default duration as a faint
placeholder, with Starts and Ends beside duration help. Rename never changes notes automatically. Update links
in notes names old and new titles below Title and updates old-title aliases as one batch with Undo; custom
aliases remain. Undo stays only in the list column until a note/event edit or leaving Timeline, never in
conflict. Completed link updates use success.

Version 2 stores event lists and day IDs in comments below headings. Every write renumbers days to the
displayed clock. File items use Fight and Major event; accept Encounter and Event in any case, with or without
a colon. Preserve outside prose, unknown properties and event fields. Separate app and outside event edits
save together with an info message in the list column, as other results there; same-event edits need a choice. Compare at the first difference, with
Overwrite before destructive Reload and lose my timeline. Flags name their line (`Line 9: …`); the file line
below the last flag starts at its words and holds `Timeline.md` with the shared file actions; a flagged day keeps its Day icon and adds a muted `Small` word after the heading with a warning icon (`⚠ Written as Day 5`). Rail conflicts use warning Timeline changed on disk; write failures
use error Timeline not saved. Run updates change only the Result line and never convert Timeline.

Copied sessions say `<first file> and <second file> claim to be the same session.` They offer
Make <file> a separate session with Undo separate session. The header names the clock (`Now: Day 1, 00:10`) only once a day sets it. Moved or deleted note drafts
offer the new path, a new file and red `Discard`. When Splat found the note under a new name, the warning
says so: `This note was moved or renamed. It is now <file>. Your draft is still here.`; `gone from the folder`
is only for a note with no new place; Discard says `Draft discarded.` with `Undo discard`. After
`Reload and lose my draft`, a note says `Reloaded from disk.` with `Undo reload`, as World does. Missing session and run folders offer Choose folder.

### Notes panel

Notes (Ctrl+J) opens the played session beside any page, using the same title, date and save words. Start at
What happened, with the caret on a fresh line below it. Opening an older note closes the panel; it never
follows the page. On Sessions, Ctrl+J goes to that page's editor. Lookup replaces Notes, which returns when
Lookup closes. A run-side task such as Conditions replaces Notes when both cannot fit. Escape closes Notes;
Tab indents inside it and cannot reach Next turn.

Run buttons keep working and return focus to notes after each step, except a field being typed in. Clicking a
row or Amount takes focus to Amount for one number. Letters there go to notes with preceding digits. Typing
and pasting with nothing focused still go to notes. Mark automatically written words as a draft, make second-pass changes
readable, and offer Add for transcript markers rather than placing them automatically.

## Party

Overview uses one group-heading style, with `L` above sheet groups. Saving throws and skills start at tile
edges, in two or three columns; skills sort A to Z and fill columns top to bottom. Skills use muted Small words `Proficient`, `Expertise` and `Half` after the skill name. Paired cells share the
taller height. Use a true minus for negative modifiers. Rows without detail use one Read row. At eight entries
or fewer, omit filters and separate count lines; count in the group heading. Death-save counts use words.
Compact rows omit a missing spell save DC; wide tables keep the empty cell. Spell help says: “Spell text shows the printed numbers. Splat does not work out upcasting or this character’s own bonuses.”

The Combat sync heading and its body are one block: the heading is the first row of the sheet's scroll area,
below the sheet tabs, and its body opens under it. While closed it says the sync state: `Combat sync · Off`,
`Combat sync · Waiting for the first read` with the first-read icon and colour until Splat has read the sheet,
then `Combat sync · On`.

The Party header states the connection and the sheet's age: `Connected · Fetched 3 h ago`, never `Cached`.

A stopped-sync notice sits below the Party header, across the page. Its bar starts at the pane edge;
Dismiss sits at the right end of its first line. Sheet pages use Small tabs on one line, with later pages in More.

Local-edit notices measure the full message inset, including its bar, icon and gaps. The Party header owns Connect to refresh, except while a signed-out local-edit notice shows: then the notice's secondary `Connect D&D Beyond`, before Show file, is the one way to connect, and the header shows only the state (`Not connected · Fetched 3 h ago`). One action has one control and one name on a screen. Signed-out notices say replacement keeps the DM’s version as a copy. Local-edit warnings name the character and say replacement keeps a copy. Keep each notice within `LIST_MEASURE`. Actions follow its text after an `L` gap, or wrap below, aligned with the text. The affected row says Local edits; replacement names the character, offers Show file for the kept
version and Dismiss. Empty Party has one Connect D&D Beyond action. An offline Party now line offers Connect
to sync; first-sheet-read help keeps Settings inside its Small sentence. Spell levels have counts without an
extra Spells heading; cantrips omit Prepared and use one-line rows. Coins sit below Equipment & coins. Party
and encounter rosters use the run's Bloodied heart, word and colour. Closed Technical details uses plain words
for rules not yet read or supported.

## Recording and processing

Record takes an inline consent step to tell the table; hide header Record during that step. The microphone is
the only colour on the session page. While recording, rail status stays visible everywhere and time counts
audio saved, stopping when saving stops. One linked rail slot above account status opens the recording. Use
Recording, Recording silence or Not saving audio consistently on rail and session. Silence has warning colour
everywhere after a minute, including the run notice, and no banner. On the session page, only the icon and
status word use warning colour; detail stays Small and muted. The line for a recording in another session
does the same for Not saving audio: only its icon and word are red; time and session title stay Small and
muted. The stall banner says `The recording of “<session>” stopped saving audio. The first <time> are kept.
Record again starts a new file.` Rail silence omits time; hover and accessibility say `Recording silence · 2:14:05 recorded`. Put duration in row detail, outside bold status.

Recordings' live row owns sound state, duration, input help and Stop recording, beside Process. Hide its
header status; Plan and notes retains the live header line. Other tabs keep header Stop recording. A row
offering Record again owns recording on that tab and omits its repeated session name. Failed, Stopped and
Cancelled stay in the session list until retry or dismissal. Failures and stops use error icon and colour;
cancellation uses warning. Keep each stop/failure reason and last job step beside status, even after banner
dismissal, but hide stored reasons while live or processing. Known recorder errors use plain words, such as
`Stopped by itself: the microphone is no longer connected.`; row and banner hover keep the recorder's own words
(`The recorder said: no such device.`). Unknown errors keep the recorder's words in the sentence. Cancellation names the stopped part and offers
Continue processing.

A stopped or stalled row keeps Record again beside Process. Missing inputs put Choose an input first, before
Record again and Process, in row and banner. Banners and rows share the recovery sentence. Visible recording
rows own failures, stops and stalls; other pages use a bottom banner that does not move Next turn. Automatic
stops offer Open the session. Failures stay in banner and rail until dismissed or retried. Record again is
primary when no open run owns it. Unresolved recording recovery takes priority over processing: Choose an
input when the input is missing, otherwise Record again. A blocked action never owns primary. Audio removed
stays visible in results after WAV deletion. A recording row's sentences (stop, failure, cancel and result
warnings) keep READER_MEASURE; its rule, menu and events keep LIST_MEASURE; result readers
keep READER_MEASURE for prose and its rules. The session list omits the date during recording, jobs or a processed result. State and Playing
share line two when they fit; otherwise Playing starts a line without a separator. Equal titles and dates show
each note's file name on line two (it wraps at a space, and each of its lines starts at the first line's x) and in its accessible name. The rail says Processing cancelled; the list
keeps Cancelled.

### Processing and results

Use one spinner and Processing progress line. Rail and session list say Processing; hover keeps the full step.
Errors and setup help call the chosen text service the transcript and summary service, as in Settings.
Not processed uses a neutral file icon. Continue processing resumes stopped or failed work. Process and
Continue processing are primary when no open run owns Next turn and no other primary action is in view.
Only the first recording that needs processing owns that primary action; later rows use secondary actions.
An enabled recovery action in a row or bottom banner takes priority over processing. A visible row's
recording recovery also takes priority over a processing retry in another session's banner. A blocked
banner action does not take primary from an enabled row. A blocked banner `Continue processing` shows its reason
beside it in `Small` text, then an `L` gap to the next choice. With the audio removed, the banner drops the
failure's advice to process again, so the words and the blocked control agree.
While recording consent is open, Start recording owns primary.
They are secondary while a run is open. An unresolved recording recovery keeps Choose an input or Record
again primary. Put destination below the first enabled Process or Continue processing only, in Small muted
text at reader measure. Later rows keep it on hover and in the accessible description. Keep it in Process again
confirmation. With a complete transcript, name raw transcript and session context and say audio is not sent
again. With a partial transcript, name remaining audio parts and destination.

Reserve Process again… for replacing results, in the row menu above Delete… with inline confirmation. One info
message names transcript and context sent to the LLM and says audio is not sent again. Confirmation uses
secondary Not now and red Replace the results; Keep the recording is secondary. Processing help keeps
READER_MEASURE; the Process again message sits in a scope capped at it, with Not now and Replace the results
together below. Clean and warned results say Processed in the rail until a later recording or processing replaces the result.
Opening the session acknowledges the completion notice and keeps the result and its warning. Warnings retain icon and colour. The
session list shows the warning icon and Processed in warning colour, then ` · Playing` on the same line, as
its sibling states do. A clean result is a status too: the check icon and Processed in the success colour, as
on the recording row; the recording row, list hover and accessible name use Processed, with a warning. On the timeline is a success link for added events
and transcript markers. Marker text takes its content width, capped at READER_MEASURE. A marker's Add
result is not held to the rows' width: it keeps LIST_MEASURE, so Undo add follows its words. One action column
sits an L gap after it. Add and On the timeline align left in that column. Row rules span only the combined width. The After placement line does not affect that column’s width and wraps below the facts.

The viewer is one block, READER_MEASURE wide, or as wide as its view choices when they are wider. It has back
link and muted date on line one, Add to notes at the block's right edge on Summary, and left-aligned
view choices on line two. Use one segmented control for Summary, Clean transcript, What the check changed and
Raw transcript; never wrap it. When tight, replace it with one dropdown naming the current view. Generated
views share a Small draft notice: This draft was written automatically. Check it against your memory. The row
states an empty check once: in the warning line, or, while processing again hides that line, as `The check
returned nothing` beside Raw transcript. Never both. Keep row menu
beside title and `L` between its actions and events heading.

## Settings

Use CAMPAIGNS and D&D BEYOND ACCOUNT group headings in both layouts. In compact layout with an open run, put
account before locked campaigns. State the campaign lock reason below CAMPAIGNS. Header context stays empty;
campaign counts belong in their list heading. With no campaigns, hide Reload from disk unless settings need
repair. Account help and trust text keep READER_MEASURE in column and stacked layouts.

The account form replaces status and Connect… link. Use Connect, Not connected and Signed out of D&D Beyond.
A failed Connect says `Connect failed` with the error icon and colour, on the page and in the rail; `Not connected`
is only the quiet idle state.
Show one Connecting state, with Cancel aligned to the column. Connect stays at the cookie field's right edge
and is primary when a cookie is entered, no campaign form is open and no run owns primary. Keyring warnings
own Connect again. Put quiet Disconnect after trust text. Settings and rail share an account-status function.
Rail shortens Signed out of D&D Beyond to Signed out, retaining full hover text. Wrap Connected · not
remembered on two lines with full hover and accessible text. Settings states recovery once.

Campaign add forms close on success; messages follow campaign rows below their heading. Add/edit forms put
Cancel at the left and commit at the right. An unchanged form blocks Save details with No changes to save.
Details hides Edit details, uses a plain Details heading, and puts state and Discard at the left, Save details
at the right. Discard restores saved details with Undo discard; clean forms use Cancel. Import opens below its
chosen row with a new sibling folder filled in. Keep campaign name in its field; omit remote ID, repeated
heading and the open row's Add action. Removal dialogs use the shared `dialog` and its top anchor, as every dialog does. Reload from disk names the local reload
action.

### Recording services

Recording settings clip at the tab rule, with a fixed `L` margin below tabs outside the scrolling content and
`L` above sections. Recording, Speech service and Transcript and summary use group_heading. Field labels use
muted Small through small_field and small_text_field: Service, Address, Model, API key. Accessible names keep
service context. Focused inputs scroll to the Recording heading. Speech setup first explains how to turn
recording into text. API compatibility help sits beside custom services; Codex limits appear only when
selected. Combat sync help uses muted Small text. Help on this tab keeps READER_MEASURE, as Account help does,
and says what a choice does, not how it works (`It cannot change your campaign files.`). The second service
keeps one name: `Transcript and summary instructions`, `Transcript and summary API key`, and accessible names
`Transcript and summary service`, `… address` and `… model`. A value that blocks processing says so below its
own field in the notice's tone (`Enter the address.`, `Enter the model’s name.`). When the DM chooses a
service that leaves such a field empty, the notice and that reason scroll into view together, once; a key
problem keeps its reason at the key row, so the view stays. A saved key says `Saved on this computer.`;
`keyring` stays in warnings about the keyring itself.

Warnings name the saved key's service and chosen service. A key for another service keeps Save key visible
with Paste a key first and a separate quiet remove action. A saved key for the chosen service hides Save key
and its empty reason until a replacement is typed. Saved fields invite replacement keys. Remove-key buttons
name slot and service, including accessibility, such as Remove speech key (OpenAI) or Remove transcript and
summary key (OpenAI). Put them below key state at the left edge.

### Touchpad

The Touchpad tab has one TOUCHPAD group. Scrolling speed is a segmented choice of 1×, 2×, 4×, 8× and 12×,
default 4×, with Small help `Mouse wheels keep their speed.` Save it with the computer in ui-state.json, not
with the campaign.

## Icons

Use `egui-phosphor` (MIT, Phosphor Regular). `.gitignore` forbids committed images, so icons are glyphs or
painter shapes. All icons come from `design::icons`, which names only glyphs in use. Never pass raw symbols as
widget text. Icons support words; replace words only for universal meanings, with accessible names and
tooltips.

## Tests

- `design::tokens::tests` checks WCAG contrast over token pairs.
- `design::lint` rejects raw colours or type sizes outside `src/design/`, and a straight apostrophe inside a word
  in a string of `src/` or `crates/core/src/`, with the shrink-only allowlist in `src/design/lint_allowlist.txt`.
- The gallery visual scenario shows tokens and shared choices as PNGs. Run rows, dialogs and other
  whole-screen parts appear in their own scenarios. The gallery includes clean, dirty and discarded save rows
  and the hanging Add strip.
- `visual_tests::ergonomics` checks target size, accessible names, visibility, overlap, clipping, keyboard
  reach and workflow goals. Overlap includes labels except those fully inside their interactive ancestor.
  Clipping includes ellipses and text past pane edges, read from painted shapes. Missing-glyph checks fail for
  characters absent from the chosen font and fallbacks. All checks except workflow goals have seeded unit
  failures, including an unassigned glyph. Workflow goals were verified by deliberately breaking the app.
- An overlap of an app control or text with a scroll bar or a column splitter is an app fault
  (`no-overlap/app-vs-chrome`), because egui can give the press to the bar or splitter. No baseline may
  allow it. A menu item over its own popup background stays egui chrome: egui's hit test gives the item every
  press, and a press on a disabled item reaches the background, which has no action.
- Every scenario run must report all brief section 8 tests; the baseline refuses failed goals. `visual-compare
  <labelA> <labelB>` lists differences above rendering noise. Tests ensure thin unnamed controls in scroll
  content or popups do not count as egui chrome. The capture suite rejects consecutive identical pixels at the
  same size; an intended repeat needs `expect_same` and a reason on the second step.
