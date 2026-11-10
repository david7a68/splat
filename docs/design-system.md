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
`READER_MEASURE`.

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
the full length of a scroll surface's edge. A nested horizontal bar can float up to the reader's visible
bottom edge. It stays hidden while its table is below view.

Reader tables keep each column at least as wide as its longest word. A table that cannot fit scrolls sideways
and says how to do that. The first column stays at the reader’s left edge, with a hairline beside it. The
header stays at the reader’s top edge until the table leaves view. Ordinary wheel scrolling still moves down
the note. The ergonomics check permits side clipping only where a real horizontal scroll bar can reveal the
text; it still rejects clipping in ordinary readers and text cut to an ellipsis.

A column's drag handle stays egui chrome: 10 px wide and as tall as the column. Tab gives it focus, shown
by an accent line. Left and Right move its edge by `L`. Home restores its natural width, which follows the
window again. The chosen width is saved after a drag or key press. Each handle has an accessible name and
key help. At the table, the rail uses its natural width; a wider desk width stays saved.

## Widgets

`theme::install` sets one global `egui::Style` for hover, active and disabled states, focus rings, selection,
text edits, solid scroll bars, windows, tooltips and separators. Text cursors do not blink. Motion draws the
eye during long sessions.

`design::components` holds the shared parts, each a function or a small builder:

| Part | Function | Notes |
|---|---|---|
| Page header | `page_header` | Title and one `Small` muted state or count on the same baseline; actions at the right; hairline, then an `L` gap. Never repeat the campaign name |
| Nav item | `nav_item` | Icon and word; accent bar when selected |
| Buttons | `Action::primary / secondary / quiet / destructive` | `.blocked(reason)` disables and adds a hover reason and accessible description. For one blocked control, show `blocked_reason` beside it in `Small` text, then an `L` gap before the next action. `.key("N")` shows the key beside the word |
| Split button | `split_action` | Default `Action`, then a caret menu for other actions |
| Field | `field`, `small_field` | Sentence-case muted `Small` label, control, help and error; accessible name |
| Text fields | `text_field`, `text_input`, `number_field`, `search_field`, `named` | Accessible name and a 26 px target; left-aligned numbers keep the caret off digits; placeholders use faint `Body` text |
| Dialog | `dialog`, `dialog_actions` | `egui::Modal`, for the brief's modal tasks only; one action line, red at the left, keep and commit at the right, commit last |
| Section | `section`, `quiet_section`, `section_icon`, `combo_icon` | Collapsing, with Phosphor arrows; `quiet_section` for technical details at a page's end |
| Table row | `table_row` | Fixed cell widths and hairline for Markdown tables and sheets; no stripes; height follows text |
| Rail toggle | `nav_toggle` | Lookup and panels; its own `Open` word distinguishes it from the current page |
| Segmented | `segmented` | Words with an underline, no pill |
| List row | `row`, `two_line_row` | Fixed height for `show_rows`; paints hover in the current frame |
| Status | `status`, `status_link`, `Status` | Icon, word and colour; `status_link` opens where the DM can act |
| Stat tile | `stat_tile` | Sentence-case muted `Small` label over a large value, no box |
| HP | `hp_bar`, `hp_readout`, `hp_state` | Fixed slots; separate temporary HP |
| Empty state | `empty_state` | Icon, serif title, purpose and one next step at the top of an empty pane or page, with normal padding |
| Group heading | `group_heading` | `Label` capitals and muted, nonzero count; no rule; serif headings belong to readers and stat blocks |
| Submenu | `menu_submenu` | Match quiet Read actions: `S` horizontal padding, Strong type and Read height |
| Row menu | `row_menu` | Quiet `⋯` at the right, named `More actions for <name>`; holds Remove and Delete |
| Add strip | `hanging_add_strip` | `Add` in its own Label column; one line of quiet icon-and-word buttons. Show all seven actions when they fit. Otherwise keep Fight, Group (plan only) and Long rest visible; put Day, Short rest, Travel and Major event in one `More` menu. Let this row wrap only when those visible controls cannot fit |
| Message | `message`, `message_with_action`, `Tone` | Inline icon and bar; cap the notice at `LIST_MEASURE`; actions follow text after an `L` gap or wrap below, aligned with the text; measure the whole action with padding; move it below and align it with the text if it cannot fit; actions without icons use a secondary border |
| Side panel | `side_panel_width`, `panel_header` | One width rule for Lookup, Notes and the run; separate participant and fight tabs |
| Column | `Column::left / right / panel` | Rail, page lists and panels; inner-edge drag handle; natural width until dragged or moved with keys, then saved width across restarts; preserve page room and `RUN_LIST_KEEPS`; adjacent content reaches the column edge, with text measured inside it |
| Link | `link` | Real `Link` role, at least 24 px high |
| Number | `number`, `number_width` | Right-aligned digits |
| Count | `count` | `1 entry`, `2,108 entries`; zero gives no text; empty unit words give only the formatted number |

All form labels use sentence-case muted `Small`, including `field`, `text_field`, `small_field` and
`small_text_field`. Use `Label` capitals for group and column headings. A heading over one input is a field label, including `Group notes` and `Read-aloud and notes`. An empty section inside a list page
has one muted line below its heading, with no zero count or full empty state. Tabs and page headers omit zero
counts. Roster, Members and Recordings use an underlined tab with a muted, nonzero count. Fight and Packages
menus put the caret after the word. Keep remains the default focus when closing could lose work.

Header actions have at most one bordered secondary button, for `New …`. Other header actions are quiet and
come first. Only one accent-filled primary action can be visible. An open run's strip owns primary `Next turn`
on other pages; `Back to run` is secondary. Use a `status_link`, such as `Running · Round 2`, to open its
state. An on toggle uses `selected`, its word and an icon or check, without accent fill. Use red only for an
action that loses something. Keep Delete and Remove in row menus.

A save row keeps its state and quiet `Discard` at the left and `Save` at the right. Clean rows say `No
changes`. `No changes to save.` belongs only to blocked Save's hover text and accessible description. Unsaved,
Saved and errors keep that left slot. When one cause blocks several controls, state it once at the cause: the
save row, run strip or recording row. Other controls use its hover text and accessible description. The rail
never repeats the sentence. Its campaign switcher keeps its height and a lock; the popup starts with the
reason and disables campaign choices. Package, encounter, repair and run-settings drafts keep their campaign-switch
reason after Unsaved and Discard in the save row. Reserve its height so the first edit moves no content. Run
settings keep their full save actions on a second line at the right edge, so neither action is cut short.

A quiet action at the start of a line uses `line_start`: align its text with the line above and extend
its target into the gutter. This also applies to an icon-bearing quiet message action below its message.
Other message actions use a secondary border. Undo messages use `undo_message`: a quiet action and undo
icon on the message line, or below when needed. Discard recovery uses `Small` text, names the draft, and
gives Undo the same name for accessibility. Inline creation uses a Label title, fields, a named Create
action, its `Small` blocked reason, an `L` gap and quiet Cancel. Creation and copy fields share one
right edge.

Library, Lookup and encounter search rank equal names in this order: campaign entries, book and package
entries, then character sheet copies. Creature results keep names close to stats with an `S` gap between
results. The CR column fits `CR 1/8` plus an `M` gap. Paths shorten before file actions wrap; hover keeps the
full path.

## Keeping a draft

Conversion and draft comparisons align unchanged lines before drawing. Blank rows mark actual additions
and deletions. Highlights and line counts use these aligned rows. Adjacent changed rows form one group.
`Next difference` moves to the start of the next group, wraps to the first, and stays hidden for one group.

`Discard` takes one press, with no dialog. Show it only for a dirty draft, apart from Save. Keep `Draft
discarded` and `Undo discard` until the next edit. A later disk change also clears Undo discard. Library uses
its reserved reason line, aligned with the editor's left edge. Keeping a draft in the close dialog restores
its editor focus and cursor.

A changed file stops saving. World uses `disk_conflict_choices`; Library uses the same choices: “This entry
changed on disk. Your draft is safe until you choose.” Show secondary `Overwrite with my draft` first, then
red `Reload and lose my draft`. Neither has accent fill. These messages cannot be dismissed. Library gives the
choices one row at full reader width. World Overwrite saves valid text; invalid text returns to the repair
editor without a write. World Reload keeps the replaced draft for one `Undo reload` on its message line until
the next edit or disk change. A run offers `Replace the file with this run` and `Save as a new run`.

Session conflicts call the item a note in the warning. Keep Record available because audio uses its own file.
Show draft and disk text side by side in Write's styled source face. Hide the plan and use the whole session
body, with choices above the comparison. Library uses its editor's monospace face and line height. Text wraps
at word boundaries. Selected fill and a line mark show differences. Open at the first difference with two
whole context lines above it. Show `N lines differ` and the nearest heading beside that count, such as `Under
the heading “Recovery”`; let the sentence wrap. Missing rows stay blank with a muted hatch and the accessible name `No
line`.

## The Reader

`library::view::show` is the only entry reader. Library, Lookup, the run panel and campaign entries call it
(T6.4). `design::reader` holds the typographic parts. Library shows quiet Back whenever it has reading
history. Its toolbar stays on one line: Read and Edit Markdown at the left, then visible copy and creation
actions, then Back. Drop Back's key hint first; fold Copy and Make NPC note into quiet More when needed, with
Back before More. Lookup keeps its Back hint. Both use Alt+Left; Lookup takes the key when focus is there.

Library omits `Library / <kind>` above the title. Its kind line does not repeat the package name. Owner status
and `Use in this campaign` stay on the wrapping file line in Read and Edit Markdown, so Back does not move the
title. During a draft, Back opens the earlier entry in Lookup. Show file is on that line; vault entries also
have Edit in Obsidian. An unreadable entry shows `Cannot be read` once below its title and keeps front matter
out of prose. Empty readers say `No entry open` or `No note open`, then “Choose one from the list, or make a
new one.” Library omits the creation offer without a campaign or package destination.

Ordinary bold and bold italic keep the prose colour and emphasis. A bold phrase links to Lookup only when
it resolves to one entry. Explicit Markdown and wiki links stay interactive.
Reader links use accent text and an underline. Library links open in place; other readers open them in Lookup.
Every reader resolves `[[Name]]`, `[[Name#Heading]]` and `[[Name|text]]` against World notes and entries the
campaign can use. Duplicate names open a chooser. Missing names keep their words, a dotted underline and `Not
found in this campaign` on hover. Broken campaign and package links also have a warning icon before the dotted
text and a list-row warning. Reference keys still work. Markdown readers call `reader_markdown` through
`ContentIndex::resolve_link`. Hidden SRD provenance does not count as broken links; SRD page targets count as
found.

Readers hide reserved Splat data and any heading or rule whose filtered body is empty. Hidden source headings
and comments take no space. Collapsed Source and identifiers and Technical details follow the body in its
scroll area, with one leading `M` gap. What Splat applies stays inside Source and identifiers. Source text and
structured item fields stay in this end group. Unreadable files omit empty end sections. Related entries, Here
and Mentioned in stay above them at reader measure. In narrow creature readers, each vital keeps its label and
value together when wrapping. Pictures are 64 px thumbnails beside the title and meta line. Use one `L` gap
after the kind line and one `M` gap after a spell's fact table. A standalone bold caption above a table is
plain text.

A stat block has a heavy rule, identity, labelled AC, HP and initiative, then a tight attribute table with
physical and mental abilities side by side and right-aligned numbers. Detail rows follow. No card surrounds
it. Serif section headings sit over hairlines. Vitals come once from the schema-2 Stat block table. New
creatures store identity, CR, speed and ability scores there; printed source text stays separate. An absent
saving throw leaves an empty cell. Bad cells warn with their file line; valid values still lead in the reader
and run panel. Rejected vitals keep their original text and warning in the vitals area. Unknown rows remain
readable. Stat warnings name the field, bad value, line and repair. List rows and the file warning count
include flagged stat cells. The editor warns when prose AC or HP disagrees with the table and offers `Use
<value>` with Undo.

Converted spells and items show facts once, with no empty rows or machine lines. Spells use Casting Time,
Range, Components, Duration in book order; Concentration and Ritual appear in those values only when true.
Magic items use book case and put attunement in their identity line. Creature types use book case and keep
tags. CR supplies missing XP and PB; explicit mismatches warn by line, except lair values. Ability modifiers
and passive initiative follow their score or bonus. Conversion keeps printed creature headers in a folded
record quote outside the reading view. Run copy notices compare model and readable words, so conversion alone
does not report changed creatures. Copied creature fields use template book order, one flow line per stat.
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
whole at minimum width. At narrow widths, scope and category share an equal-width line above search.
Hide All entries without a search. Empty categories and their headings stay hidden. Fixed scope names
wrap; package names can shorten with full hover text and accessible names. Both the scope menu and
selected button say `Campaign entries`. An open package About row uses selected fill with no selected
category; choosing a category restores it. About is followed by the manifest name. A manifest is never
counted or listed as an entry, even when unreadable. Keep its warning on About and in the file issue
count. Show used and total entries and `Use every entry` with Undo; omit the entry breadcrumb and Engine
support. Empty package counts have no separator.

Nonzero file status links sit in the page header before Packages and New entry. The amber validation count
says how many entries need fixing; its menu opens the messages. Unreadable notes use CANNOT BE READ. Rows name
their kind, path and error line; the owner line says Cannot be read. Opening one from a roster selects its
kind under Cannot be read and opens repair. Repair errors use file line numbers and put the caret on the line
to fix. Repair reloads campaign use at once. Validation messages also update after encounter or group edits.
Other readable files stay available.

A deleted draft shows YOUR DRAFT, `Save my draft again` and red `Let it go`, with Undo let go. Let it go names
entries that need the deleted note. A moved draft shows the new path and old folder; moved-from text ends the
file line. File actions use the new path. Offer `Save my draft to its new path`; saving checks the disk ID to
prevent a second file after a move.

Source mode has one frame at the remaining pane height. All four edges stay visible; only text scrolls. Read
mode keeps its outer body scroll. Form headings use text colour and an `L` gap; labels stay muted. New entries
start in their kind folder, use `FORM_WIDTH`, show the destination path and reveal the selected row after
reload. New entry, Create package and Copy to a package default Use in this campaign to on. Help states which
selection will be saved. Package entries show their package and Used here or Not used here; Use in this
campaign saves the missing choice. Empty campaign entries offer one New entry action and link to Can use with
`entry you can use`.

Missing dependencies lead with the owner and retained member name: `Lost Patrol needs Scout, which was
deleted.` Without a known name, say `Lost Patrol needs a creature that cannot be found.` Link repair to the
referring file; keep raw references in Technical details. A removed package is named without claiming its file
was deleted.

### Lookup and copies

Lookup shows up to five one-line results and selects the open result. A sole open result closes the list. Rows
use Scan height, Body names, then Small muted type and level. Campaign entries add Campaign; source appears
only when different from the campaign's main source. Shorten metadata only; hover and accessibility keep the
whole row. Results never scroll sideways. Clicking a result or scrolling into its reader closes the list. Show
all offers the remaining results. Clear sits beside nonempty search only. Back, Open in Library and the close
icon share the title line. Creation actions follow results with the proposed name, only without an exact
match. Generate NPC appears only for empty search.

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

Splat reads schema 2 and Timeline version 2 only. Do not load old files. Say `<path> uses the old layout.
Splat no longer reads it.` Reading and polling write nothing; ordinary saves have no layout notice. Malformed
flat properties stay on disk; a line warning keeps the note available. Campaign packages use tasks; a toggle
changes one character. Rule choices, group members and saved rosters use tables. Bad cells name their line and
stay on disk. Encounter and group notes forms show DM prose with managed tables outside the form. New files
use the entry name. End fences use folded Splat data callouts; empty defaults need no fence.

On window focus, check campaign notes and registered package folders. Reload changed catalogues; compare the
disk text when a draft's file changed. Duplicate identities keep both files visible with a warning. Saves
write only the open file. A renamed copy comes first, otherwise the newest file does. Encounters block
ambiguous creatures; Open in Library opens the first repair action's file. Encounter search names hidden
duplicate notes and offers file-specific identity repair with Undo.

Library, World and Encounters say “These two files claim to be the same entry,” with the file names.
Sessions use “session” for their repair and Undo actions. Rows say `Same entry in two files`. Each bordered secondary repair says `Make <file> a separate entry`. Its result
names the file and keeps `Undo separate entry` on the message line. Folder repairs use `separate
entries` and `Undo separate entries`, with one guarded Undo for the batch. In Library, each choice
states which file an encounter will keep or use, in `Small` muted text an `XS` gap below its own action.
An `L` gap separates choices. Name the folder when filenames match. Making an entry separate also sets
its name and matching first heading to its file stem. Other actions name encounters whose source will
change; results explaining which entry an encounter will use are Small muted text below the action.

### Convert files

Library keeps its old-layout count in the header, including generated packages. Old-layout files offer
`Convert files…`, never Repair. Campaign combines them into one count line. An unused old file does not block
Save selections; name a required old file beside conversion. An open run blocks conversion and disables
old-layout links with the reason on hover.

Convert files groups files and its picker by owner. Initially only the campaign is checked. Generated
packages start unchecked but can be selected; warn that their converter overwrites these files on its
next run. Counts include checked owners only. The modal shows before and after text, counts by kind in
words and skipped files. Omit the kinds line when it has no text. Short previews fit content; long
previews give the source panes about 60% of window height, ending on whole rows. Headings sit above
that space. Leave room for the action line at small window sizes. Source lines do not wrap. Mark changes from aligned rows,
scroll both panes together and open at the first difference. Use the same change-group navigation as
draft comparisons. Encounter and group conflict comparisons use disk layout and include outside
properties and prose that Overwrite keeps.

Writes check their baseline. Keep skipped files and next steps visible; count changed and skipped files, with
warning tone for any skip. Each skipped file offers Convert files… beside it. Keep `Undo convert` when
reopening preview, until another conversion is applied. Headless `splat convert` uses the same checks; `splat
check` reports schema-2 flags and unreadable files by path and line.

## Campaign

Campaign's header counts chosen entries only. Flags appear once in the content issue count, with file and
repair actions on their warning line. Packages keeps its save row directly below the tabs, before notices.
The save row, package list and entry search stop at LIST_MEASURE from the tab’s left edge.
An open repair keeps the same right edge as the save row.
Repair fills the column with the invalid-note reason above it; an `L` gap and hairline separate its save row
from the package save row. Invalid YAML uses plain words and marks the line to fix.

Needed by your entries sits below packages in the left column. Its right pane has one search and list; rows
open Lookup. Short package lists fit their rows. An off package with required entries shows None chosen, the
needed count, an indeterminate checkbox and encounters that use its creatures.

A missing Campaign.md states whether Splat holds the last read copy and counts its packages and rules. Writing
held text requires an explicit action. With no copy, `Write a new Campaign.md` sits beside the explanation
that it uses default packages.

### Packages

Manage packages offers Clear only for filled fields. Folder is one editable field with a suggestion menu, then
help. Suggestions include existing folders, use the combo arrow and normal text colour, and permit a new
subfolder on save. New packages default to the parent of the last registered package. New package has Browse
beside Folder and the full destination below it. Creating a package opens its first entry form.

A copied package folder names the original under Add folder and puts its ID in Technical details. Add folder
is blocked with `This folder is a copy.` Bordered `Register as a new package` sits on the warning's action
line, above Technical details. It asks for a name, writes independent package and entry identities, and has
one Undo for all files. Registration names the copy and whether the campaign uses it. Its use checkbox starts
off when the original is used. Keep the warning above the `Register a copy` Label title and fields.
Quiet Cancel follows the named Register action after an `L` gap. It closes the form without writing
and keeps the name and use choice for reopening. When use is off, help says the campaign keeps the
original and the copy is registered but not used. When on, it says every entry is used, including future
entries. `Use the copy's entries` keeps roster names, counts and changes, with one Undo.

Unavailable packages keep their last known display name. Older unnamed selections say `A package that is no
longer on this computer`. Count affected campaign entries when references are known, with Remove from this
campaign and Find the folder… beside the message. Only Technical details shows IDs. Removing an unavailable
package has Undo and keeps links to affected encounters. Unregister keeps the last folder in Settings. Library
offers Undo unregister; Campaign offers Register it again.

Missing-package recovery searches beside known package folders. A unique match gets bordered secondary Use
this folder; Find the folder… is then quiet. Show one warning naming the old and new folders, followed by
the affected encounter links with word spaces. Align Use this folder, Find the folder… and Remove from this
campaign on one action line with the warning text. Technical details follows below. Say Splat cannot find the
package only when there is no match. Use Body text throughout the missing-package sentence. The picker
starts beside the old folder and checks identity before restoring registration. Both recovery paths have Undo.

A broken or deleted Package.md leaves its entries available and names the manifest fault. Campaign offers Open
in Library for unreadable manifests. Campaign Packages and the Library details row offer Write Package.md
again for a missing file. An existing file offers Replace with a basic Package.md and warns that other fields
will be dropped. Keep the registered identity and reload. Writes show a success message and Undo write
Package.md, including for a missing file.

### Link and folder

Name the campaign folder, session folder and link. The session folder has one heading and no repeated visible
Folder label; keep its accessible field name. Fields and save actions end at `FORM_WIDTH`. A clean folder form
shows the check icon and No changes. Locked controls have no clean-state reason. Folder help names its files
and permits a folder in an Obsidian vault. Disconnected campaign-link help opens Settings from an inline link.
Its state says Not connected · showing the name saved on this computer.

A session-folder change counts notes, events, templates and recordings left behind. Offer Move files and save
or Save without moving. Refuse collisions and copy every owned file before changing the manifest. A run-folder
change counts saved files left behind. Move runs and save stages all files without replacement, saves
settings, then removes unchanged originals. Save without moving leaves files in the old folder. Block
run-settings saves while a run is open. Folder changes name hidden unfinished runs and their states. A move
result gives counts and destination without another Saved label. A moved run has confirmation and Undo move
until the next note or plan edit.

## World

Readers show the relative path, Edit in Obsidian and Show file for every note. Duplicate readers keep
the path on this file line only; search results keep paths to distinguish names. Plain notes use
one-line rows. Typed names that match show each filename on the second line. Copied typed notes remain
visible with duplicate warnings. A duplicate NPC still has its existing name-only roster override; its
warning does not imply changed stats. Encounter repair opens World for World notes. A note with no
readable body says `Nothing written yet.` with `Edit in Obsidian`.

NPCs show labelled Stat block, In and Status rows; locations show In. Use a fixed label column, aligned
values and a 32 px pitch. Change uses one column at the widest visible value in the block plus an `L`
gap, capped at the value measure. Values wrap within their column, with Change on the first line. Empty
values say `not set`. Missing links retain names and keep `Not found` on one line. Broken links have a
warning icon, list warning and count in the header. Table faults use the same list warning and header
repair link. Unread values name field, bad value, source line and repair. Folder groups use normal-case
paths after type groups.

Change pickers focus search. Place choices start at the value column beside In, with one file link and a
bordered `Always use this one` action on each row. Actions share one column at the widest file link
plus an `L` gap. A saved choice changes only its key and has Undo,
which checks the file before writing. Always use this one saves a path with Undo. Change messages name
both values and follow Change when they fit, otherwise sit below the whole row block. Different file and
stored names offer `Rename the note to <file name>`, with a guarded update and Undo. New NPCs show their
future path below the name. A compact NPC reader puts Fights as and In on one read-only line; both open
Lookup.

Broken notes stay under Cannot be read and use the Campaign repair source editor. Keep drafts in memory and
warn before app close or a campaign switch. The unreadable count in the header opens the first broken
note's repair, keeping other repair drafts. A rolled table selects the matching row, marks it with dice and
states the result below in UI sans. Typed rolls use well fill, a centred placeholder and dice as the column
heading. Fixed headers cover passing rows with page fill and a hairline. Generator cards name their table
sources below attributes. Generated NPC custom names show the table marker inside the app, with no external
link. Table warnings use the words name, look, manner and want; hover keeps the marker key.

## The encounter ledger and editor

Encounters uses one column, at most `LIST_MEASURE` (880 px), with IN PROGRESS, READY TO RUN, GROUPS and
HISTORY in that order. Groups and History start closed when they have rows. Each row reserves right-edge slots
for its main action, quiet Edit and row menu. The menu holds deletion and starting another run. An encounter
with an unfinished run appears in In progress; when it ends, it returns to Ready. Keep in Ready during a run
leaves a blocked Start fight and a reason to resume from In progress. The header counts all Ready rows,
including these repeatable fights. Ready rows name creatures; an empty roster blocks Start with Add
participants first. The open-run block appears once under READY TO RUN. Its Ready action says Go back to the
run.

Unfinished rows show Setup · Paused or Round 2 · Paused and Resume. Open rows use status links Setup or Round
2, with only saved time on line two. One paused run uses Paused run in the rail, then the fitted encounter
name; full hover and accessible text includes name and state. It resumes in one click. With several, Paused
runs and `2 fights` open Encounters. An open run keeps Run and its round or Setup. Rail details never end on a
separator; a short rail shows Ended or Paused with full state on hover.

History opens read-only final state and log in place, outside the live-run slot. Rows put outcome before ended
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
the roster; Open in Library opens repair. Only deleted files offer replacement. Deleted creatures keep their
last known name or Missing creature, plus a warning such as `Enemies · Not found. Its package entry was
deleted.` Replace… uses the shared creature row with CR, HP, AC and Use. Replace all matching rows defaults to
on and retains names and starting values, with one Undo.

Roster rows have name and detail, quiet Edit and a row menu. Sides sit on the second line and change in Edit.
Menus hold ordering, duplication, Make it yours and Remove; missing rows keep Edit and Remove there. Side
pickers offer Party, Enemies and Other…; only Other opens a name field and replaces the value. Changed stays
at the end of the second line as one collapsed link. Shorten muted detail first; keep `as <base creature>` and
the full change link, removing source before base creature.

An unread roster row stays visible by participant name, file and line, with Edit in Obsidian and Show file. Start fight is blocked until the DM repairs or explicitly removes that row. Plan starts use the same check. A resumed setup keeps the warning and blocks Begin combat while its source has unread rows. Saved run participants stay intact.

The editor header keeps Save and Start on line one; the name field gives up width. An unfinished encounter
offers Resume, moves Start another fight into its menu and keeps run state in the save row. Blocked Start
fight says Close the open run first beside it. Sync and repeat options follow the roster. Use group_heading
for groups of controls and sentence-case field labels over single inputs; tabbed Add participants has no repeated heading. Group frames choose where Add goes;
search says `Adding to <name>` as read-only text. Each encounter group has a frame below the roster with name,
side, members and notes. The target frame has an accent outline. The Encounters page keeps groups in one
closed section. Frames use Move to… for ownership; full editors use Belongs to.

### Groups and changes

The group name field ends at `FORM_WIDTH`. Member rows and Group notes end at `LIST_MEASURE`, inside the scroll content and anchored left. Removing any member shows `Removed <name>.` and `Undo remove member` on a result line. Undo restores the full member at its place and keeps other draft edits.

A plan group editor stays beside the note when `LIST_MEASURE`, an `L` gap and `READER_MEASURE` fit.
Bound the editor at `LIST_MEASURE`, anchored left. Below that width, the note stays behind its tab. Plan summaries start directly
below name and action, without an extra item gap, and extend to the menu slot. Order summary parts as side,
member count, XP, Changed, members. Shorten members first, with full text on hover. Member counts and XP use
one dot-separated line. Members and XP share one heading. Keep a member summary below search when there are
members, also in compact editors. Member names and stats sit above Count, Edit and the menu; count stays at
the left and actions together at the right. Changes stay on one line, such as `Changed · 2 to add by hand`,
until opened. Empty frames use one muted line, no zero count or duplicate search summary. Empty groups block
Save group with Add a creature first.

Groups has one level of headings: For any encounter, each encounter, then Encounter deleted. Rows keep Edit
and a Remove menu action with Undo. Planned groups use Remove from tonight and Undo remove. Remove isolated
plan links; retain words inside sentences. Moving an encounter group to any encounter says `<name> moves to
any encounter when you save.`, with Undo move until save. Move to… lists named encounters only. Make available
to any encounter is the direct way to remove ownership; keep it distinct from Delete group, with Undo before
saving.

List, editor and Arrivals resolve the same joining members and XP. Partial groups say `1 of 3 can join · XP 50 · Will add: 1 × Goblin Warrior`; XP covers joining members only; `+ unknown` means a joining member lacks XP. Flagged list rows retain member summaries and Edit.
Flagged members name file and line, with Edit in Obsidian, Show file, Edit and a Remove member row menu
aligned with other actions. Show Count once, in the action row; retain `Count not read` in a faulty stat summary. Put each repair link at the end of its own member fault, without repeating the link words in the sentence. Edit opens the normal change form with readable values, unread text, reason and
example. Done keeps name and note; Remove has Undo. Groups, Arrivals and plan state how many can join and name
each creature needing repair: shown name, base creature when different, and source line. Readable members can
still join. Changed summaries also name the base creature. Wrap between members, never inside names.

Every change field shows printed value and result, including changes added by hand. Editors, run cards and
fault details share that preview. An invalid stat names only the limit crossed: `Maximum HP would become -28.
It must be 1 or more.` Give an upper limit only when exceeded. Done closes the form and returns to its row.
Change forms use the run's condition grid. Attack and damage changes sit below the affected action, with `XS`
above and `L` below. Changed Speed uses stat-table label and value columns.

Run settings put Roll NPC initiative help directly below its dropdown, before the tie checkbox: `Run setup
starts with this choice. You can change it there for one roll. Groups that arrive later use it too.`

## The live run

### Roster and setup

The roster is the screen. Fixed columns hold initiative, side mark, name, AC, temporary HP, HP and bar, then
Bloodied, 0 HP and sync state. A lone sync state sits beside HP; a second state uses line two. Sheet loading
says Reading sheet…. When every linked row has sync off by the DM, header line two says Sync off for the party and the rows omit it. When linked rows differ, disabled rows retain Sync off and its icon. Sync that restarts by itself never says Sync off. Unlinked creatures have no sync word. Header and row conflict words are underlined review links. The sheet colours only its connection
icon and Connected; sheet and HP facts use muted Small.

Names that need more than two lines use `RUN_COMPACT.name`. On that row, the numeric columns can move into unused state space to keep the name whole and compact. The other rows keep their shared columns. Pictures align with the first name line.

Conditions take one second line before HP. Measure without change bonuses, shorten bonuses, remeasure, then
drop the bonus before more if needed. Measure sync before folding conditions into a count. Keep the first
condition and whole multiword sync states. In narrow rows beside a sync state, `+1` can link to Conditions.
At minimum width, mandatory state words may use the empty left part of the HP bar slot; the bar stays below HP
and at least `XL` long. Compact rows put sync last after an `XS` gap, apart from conditions. If required state words still do not fit, use the run mark type size on the second line. If that still needs the whole row width, move the HP bar below the detail with an `S` allowance; keep every state word and the first condition. No initiative uses
the same underline as other row links. When the state column cannot fit, such as 960×640 beside a panel, move
its words to line two so the name stays whole.

Current rows use current_row fill, a 2 px stroke, a 6 px bar and TURN under initiative in its own column.
Compressed rows put TURN beside the number so turn state does not change row height. NEXT marks the next row.
Name lines keep one height with or without detail. Scale sets type; spare room grows rows up to 1.4 times
scale height. Name-only rows shrink to 25 px before scrolling. Natural rows keep at least scale height. Rows
share columns and picture size; shortening bonuses changes none of these.

Setup reserves initiative, Roll, Absent and menu columns on the name line, centred on initiative. Empty Roll
slots align player totals. Only tied rows have fixed up and down arrow slots, in that order, with unused slots
empty. Other rows fit two text lines and Scan padding, at least RUN_REGULAR height. Setup retains its first
condition and counts the rest when needed. Shared-roll brackets stay beside fields and stop before detail.
Shared NPC rolls are one roll group with quiet brackets and ordering arrows; only equal totals across
independent rolls warn as ties. Header and overflow count those tied rows; the band says `2 tied`. Arrows
still work inside shared rolls.

Setup has a `Label` column heading INITIATIVE above empty, labelled number fields.

An off Absent toggle in setup uses muted text and no fill. Setup rows stay in place. Once all present rows
have initiative, a muted ordinal follows each field, updates with totals or tie rulings, and the ready line
names who goes first. Setup names its Undo action. Begin combat's blocked reason sits below the button; the
row menu opens Correct. Overflow uses the live list's band and counts hidden missing initiatives and ties.

A made-up creature shows current and maximum HP and typed initiative as a total, without printed values. A
side of its own keeps its word on live rows. Setup and live play share side, HP, temporary HP and AC parts.
Correct holds correction fields.

### Header, dock and panels

The header has two lines. Round and current turn use bold sans larger than row names, beside Next turn. Line
two keeps a result icon and notice at the left and Undo at the right. Measure notice first; Undo gets the
remainder less `L`. Drop `Undo:` before shortening the target; retain copy numbers, action words, amounts and
condition directions. Hover and accessibility keep the full action. Empty history says Nothing to undo or
Nothing to undo since reopening, to the left of blocked Undo on setup and live screens. A narrow slot uses
Nothing to undo with full hover and accessible text. Sheet updates name whose sheet changed; only sheet reads
say sheet changed. Sync-panel choices stay in Log and Undo.

Notice priority is conflict, recording silence, other sync notices, next name. Silence and short sync state
stay together when space is tight. Connect to sync loses its link before either state loses its word. Keep the
full `Sync off for the party` beside recording silence. If the Undo target cannot fit, show `Undo` and keep
its full target in hover and accessibility. An empty Undo can omit its adjacent reason when it cannot fit;
the control keeps that reason in hover and accessibility. Keep the
next participant visible when Undo is empty. Sheet notices fit after conflict with Log and Dismiss when
possible; shorten to character name and first changed value, with full text in hover and Log. Otherwise show
once below the roster, keeping Undo visible. Stat-block errors use the notice icon and one fitted line, with
full error on hover.

Long local effects show four words and more; Undo keeps four when they fit. Narrow Undo keeps Undo add or Undo
remove, two effect words and target. Concentration Undo says start concentration or stop concentration before
the target. Exhaustion Undo keeps both levels, such as `exhaustion 2 → 3 on Oren`. Exhaustion steps use bordered Scan icon buttons. Hover and accessibility keep full text.

The dock has a fixed place above the footer: Amount, Damage, Heal, Temp HP, selected name, Conditions, Conc,
Absent. Off toggles use text colour; disabled controls stay muted. Enter applies damage. Dock and footer use
roster width; panels never cover them. Fit whole words, retain a distinguishing last word or copy number or
letter, end shortened names with an ellipsis and drop trailing joining words. Hover and accessibility keep
full names.

Overflow clips above a separate page-filled band and hairline. Count rows above and below, including rows cut
by the edge, and their Conflict, 0 HP and Bloodied states. Each turn scrolls current and next rows into view.
Other pages keep the strip's turn, conflict, secondary Back to run and primary Next turn.

The panel has participant tabs Stat block or Sheet, Conditions, Correct, then `L`, then fight tabs Log, Prep,
Arrivals. Fight-tab titles name only that tab in setup and live play. Titles stay on one line and retain copy
numbers when shortened. When tight, fight tabs share a Fight caret menu with tab underline. Never wrap the tab
row. Death-save counts and actions occupy one line below tabs at zero HP; clicking a count edits it. Sheet
shows live death saves once, with correct singular and plural words. Correct has no duplicate fields; HP owns
its help. Correct initiative says This row moves. The current turn stays. Labels share one column; blocked
initiative and DM-note commits have a short adjacent reason. Correct HP has an `L` gap after death saves.
Empty Prep offers Edit encounter if its encounter exists.

Changed creatures show Changed and unapplied values such as Attack +1 or Speed 40 ft on line two, aligned
under the name at every scale. Setup shows only Changed. If name-only shrinking is insufficient, keep the
first table bonus and more; long conditions keep those that fit and a Conditions count link. The card puts
changes at the top of its Stat block tab, except AC, maximum HP and initiative bonus, which already appear in
the value trio. Add by hand comes first, in larger type on one summary line. An arriving member says `Changed
· arrived with <group>` once. Use run AC, current/maximum HP and initiative bonus, with printed values below
changed maxima or bonuses. Changes scroll with the stat block; each participant keeps its scroll position.
Paused snapshots state when they differ from the current file.

### Arrivals

Only search shows until a name is typed; then the made-up creature form has two fixed field rows. Its side
picker says Side and uses roster words. List encounter groups, tonight's plan groups, then any-encounter
groups. Empty sections use group headings and one muted line. New group opens Encounters. Each group's first
line has its draggable name, side, count and XP; then members, Initiative and Add. A blank Initiative field rolls initiative.
Dragging outlines the roster in accent. Added groups retain Add and name their joining round. Setup additions
say Added before combat. Names include count when adding more than one creature. No confirmation is needed;
Undo names the addition.

Exclude missing, unread and invalid members. Say `1 of 3 can join`, `Will add: 1 × Goblin Warrior` and
`Add 1` using only joining members. Keep excluded members under Cannot join with reason and Edit group repair.
Only zero valid members blocks Initiative and Add. Invalid Initiative retains text, says `Enter a whole number, or clear
the field to roll.`, and blocks Add and drop. Campaign → Packages opens repair.

Join status parts on one line with dot separators beside the group name when they fit, else before the first member. Use a separate line only when neither has room. `Changed` keeps its muted pencil. Member names stay intact. Never end a line with a separator; keep separators with their preceding word and `at` with
its total in arrival messages. Live initiative notices name who joins and the total. Before the current turn,
add acts next round. Tied arrivals name both participants and offer Move before with Undo, before the first
member of a shared-roll block. Keep tie details in Arrivals and Log, outside the Undo label.

### Saving and closing

Recovery copies keep the encounter name and preserve the other file. Start their second line with `Copy 2`, before `Closed last`, round and state. Only file names keep the UTC save-time suffix.
The last closed run owns primary Resume. `Closed last` follows a copy number; Changed elsewhere marks a preserved disk version. Keep last-change wording from the close dialog. Saved rows use relative time today and yesterday,
then a short UTC date. Edit stays for recovery copies whose encounter exists; reserve an empty slot otherwise.
Measure actions before assigning paused-row text width. Saved time follows turn without a full stop. State
words come from one function: Setup, Setup · Paused, Round 2, Round 2 · Paused, Ended · Round 2. An open run
with other unfinished copies names its copy beside Saved and in rail hover.

A disconnected run has Connect to sync on its header only; blocked footer Sync party uses that cause on hover.
End encounter… shortens to End…. Keys keeps its word beside Lookup, Notes and a wide panel. Saved and Undo
align with the roster's right edge. The footer uses error status when saving stops. Ended runs put short
conflict and pending-change counts below blocked Close after sync at the panel's right edge, with an `L` gap
between character sync blocks. Signed-out conflicts offer Connect to refresh before Show file. Ended sign-in
uses muted recovery text; failed Connect retains error state. Sync stopped links to Party and states recovery.

Footer and strip Close run close at once only when saved with nothing unsent; otherwise ask for a choice. List
lost changes on separate lines, up to three, then the remaining count. Deleted runs with unsent changes use
warning. App close says Keep running, Keep recording or Keep processing for each task; recording and job
titles name the unfinished session. Sync conflicts name the character and offer Review conflict. Stop and
close is secondary when saved work remains.

Incoming sheet changes remove only that character's Undo steps. Remaining snapshots keep current sheet values.
Atomic steps involving others end older affected steps when needed to avoid stale values; Log names each
removed step. DM conflict choices are named Undo steps. Sync-choice Undo names the character or count of
characters.

## Sessions and the timeline

### Notes and templates

Notes are Markdown in the campaign folder. Write styles source in Reader faces: large ruled headings, italic
emphasis, and visible markup in the plain face and faint colour. Store exactly what was typed so Obsidian
reads the same text. Read uses the one Reader, anchored left, at no more than READER_MEASURE
(at most 75 characters per line).
Write follows the column and keeps stored line breaks. Events and recordings follow column width;
book prose keeps READER_MEASURE. The gallery's Markdown sheet shows both
views of one note. Autosave runs two seconds after editing; show muted Saving… with the busy icon during
delay. Disk changes stop saving and open the comparison in Keeping a draft. Missing notes show Notes not saved
with error status in header and rail. Rail conflicts say Notes changed, with Notes changed on disk on hover
and for accessibility.

Session title and muted date wrap together, with Record on the title line. Below 640 px, save state and
Details sit beside Read on the content tab row. Read and Write sit there while notes are visible. An empty
Read view keeps only its inline Write link; hide tab-row Write until the note has content. Notes saved
centres on the title action row. Last time is one line linking the previous session and recap, with full recap
on hover. Without recap, use prose under What happened and omit link-only lines; without either, show the
session link alone. An older session warns once and links the session being played, which the list marks
Playing. Reading old notes changes neither the destination for new events nor runs: use the newest session
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
More holds Day, Short rest, Travel and Major event. If the visible controls and More cannot fit, that one row
may wrap at the first action's left edge. Long rest stays one click away (T11.2). Timeline uses the same width rule.
It stays in place when messages or rows change. Empty plans use empty_state
below the strip. Add-strip messages stay below Add. In notes ends at the button
text's inner edge.

The first upcoming fight gets primary Start fight when no run is open; later fights use secondary. Use Start
fight here and in Encounters. Saved unfinished fights offer Resume, including timeline rows whose run
is not a planned note link.
Find the saved run by its run ID among all unfinished runs. Delete the saved run stays in the menu;
Start a new run is available when a planned encounter supplies its source. Open fights use a status link
while the strip owns primary. Planned, played and running
summaries use space below actions up to the menu. Keep summary parts and Ended · Round 1 whole; join
separators to the next part and wrap without trailing dots. Group summaries wrap to two lines, then shorten
with full hover text. Plan and Timeline share fight summaries: open fights show round as a run status link
without Paused; ended, named and custom results keep stored words. Played fights show result and last round
once on line two, then final party HP in a grid of two or three first-name cells, quiet Log, Whole run and
Timeline links. Hover keeps full names and conditions; Remove stays in the name-aligned row menu.

Remove from tonight removes one fight or group link, retaining surrounding words, with Undo remove below that
row after `S`. Removed groups leave message and Undo in their old place, keeping other rows still, until the
next note or plan edit. Plan additions offer Undo add. A recording count stays muted inside its tab and
accessible name. Duplicate saved-run names first add session name; only same-name, same-session rows or rows
without a known session need a date. Same-date copies also show time. Such rows omit Saved … so time appears
once. Resume has the run icon.

Session and Timeline add long rests with the same full-HP ruling, message and Undo add. Every closed
long-rest row shows an icon and either Everyone at full or Not at full. Open rows hide that summary so the
ruling stays in place. Actions say Mark everyone at full
or Mark not at full; states say Everyone at full or Not at full. Keep one Undo ruling until the next edit.
Resuming a saved Setup after a restoring rest starts party HP full, says `Party HP is full after the long
rest.` and offers Use the saved HP as one Undo step. Combat retains saved HP; the rest message names those
fights.

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
save together with a quiet notice; same-event edits need a choice. Compare at the first difference, with
Overwrite before destructive Reload and lose my timeline. Flags name file and line and offer shared file
actions; flagged days have a warning icon. Rail conflicts use warning Timeline changed on disk; write failures
use error Timeline not saved. Run updates change only the Result line and never convert Timeline.

Copied sessions say `<first file> and <second file> claim to be the same session.` They offer
Make <file> a separate session with Undo separate session. Moved drafts
offer the new path and Let it go. Missing session and run folders offer Choose folder.

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
Compact rows omit a missing spell save DC; wide tables keep the empty cell.

Local-edit notices measure the full message inset, including its bar, icon and gaps. The Party header owns Connect to refresh. Signed-out notices say replacement keeps the DM’s version as a copy. Local-edit warnings name the character and say replacement keeps a copy. Keep each notice within `LIST_MEASURE`. Actions follow its text after an `L` gap, or wrap below, aligned with the text. The affected row says Local edits; replacement names the character, offers Show file for the kept
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
everywhere after a minute, including the run notice, and no banner. Rail silence omits time; hover and
accessibility say `Recording silence · 2:14:05 recorded`. Put duration in row detail, outside bold status.

Recordings' live row owns sound state, duration, input help and Stop recording, beside Process. Hide its
header status; Plan and notes retains the live header line. Other tabs keep header Stop recording. A row
offering Record again owns recording on that tab and omits its repeated session name. Failed, Stopped and
Cancelled stay in the session list until retry or dismissal. Failures and stops use error icon and colour;
cancellation uses warning. Keep each stop/failure reason and last job step beside status, even after banner
dismissal, but hide stored reasons while live or processing. Cancellation names the stopped part and offers
Continue processing.

A stopped or stalled row keeps Record again beside Process. Missing inputs put Choose an input first, before
Record again and Process, in row and banner. Banners and rows share the recovery sentence. Visible recording
rows own failures, stops and stalls; other pages use a bottom banner that does not move Next turn. Automatic
stops offer Open the session. Failures stay in banner and rail until dismissed or retried. Record again is
primary when no open run owns it. Audio removed stays visible in results after WAV deletion. Recordings and
their events keep LIST_MEASURE; result readers fill the pane. The session list omits date during recording or
jobs.

### Processing and results

Use one spinner and Processing progress line. Rail and session list say Processing; hover keeps the full step.
Not processed uses a neutral file icon. Continue processing resumes stopped or failed work. Put destination
below enabled Process or Continue processing, in Small muted text at reader measure, and in Process again
confirmation. With a complete transcript, name raw transcript and session context and say audio is not sent
again. With a partial transcript, name remaining audio parts and destination.

Reserve Process again… for replacing results, in the row menu above Delete… with inline confirmation. One info
message names transcript and context sent to the LLM and says audio is not sent again. Confirmation uses
secondary Not now and red Replace the results; Keep the recording is secondary. Processing help keeps
READER_MEASURE. Clean and warned results say Processed in the rail until a later recording or processing replaces the result.
Opening the session acknowledges the completion notice and keeps the result and its warning. Warnings retain icon and colour; row,
hover and accessible name use Processed, with a warning. On the timeline is a success link for added events
and transcript markers. Marker actions sit beside text.

The viewer has back link and muted date on line one, Add to notes at the right on Summary, and left-aligned
view choices on line two. Use one segmented control for Summary, Clean transcript, What the check changed and
Raw transcript; never wrap it. When tight, replace it with one dropdown naming the current view. Generated
views share a Small draft notice: This draft was written automatically. Check it against your memory. If no check ran, put No check ran beside Raw transcript. Keep row menu
beside title and `L` between its actions and events heading.

## Settings

Use CAMPAIGNS and D&D BEYOND ACCOUNT group headings in both layouts. In compact layout with an open run, put
account before locked campaigns. State the campaign lock reason below CAMPAIGNS. Header context stays empty;
campaign counts belong in their list heading. With no campaigns, hide Reload from disk unless settings need
repair. Account help and trust text keep READER_MEASURE in column and stacked layouts.

The account form replaces status and Connect… link. Use Connect, Not connected and Signed out of D&D Beyond.
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
heading and the open row's Add action. Removal dialogs are centred. Reload from disk names the local reload
action.

### Recording services

Recording settings clip at the tab rule, with a fixed `L` margin below tabs outside the scrolling content and
`L` above sections. Recording, Speech service and Transcript and summary use group_heading. Field labels use
muted Small through small_field and small_text_field: Service, Address, Model, API key. Accessible names keep
service context. Focused inputs scroll to the Recording heading. Speech setup first explains how to turn
recording into text. API compatibility help sits beside custom services; Codex limits appear only when
selected. Combat sync help uses muted Small text.

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
- `design::lint` rejects raw colours or type sizes outside `src/design/`, with the shrink-only allowlist in
  `src/design/lint_allowlist.txt`.
- The gallery visual scenario shows tokens and shared choices as PNGs. Run rows, dialogs and other
  whole-screen parts appear in their own scenarios. The gallery includes clean, dirty and discarded save rows
  and the hanging Add strip.
- `visual_tests::ergonomics` checks target size, accessible names, visibility, overlap, clipping, keyboard
  reach and workflow goals. Overlap includes labels except those fully inside their interactive ancestor.
  Clipping includes ellipses and text past pane edges, read from painted shapes. Missing-glyph checks fail for
  characters absent from the chosen font and fallbacks. All checks except workflow goals have seeded unit
  failures, including an unassigned glyph. Workflow goals were verified by deliberately breaking the app.
- Every scenario run must report all brief section 8 tests; the baseline refuses failed goals. `visual-compare
  <labelA> <labelB>` lists differences above rendering noise. Tests ensure thin unnamed controls in scroll
  content or popups do not count as egui chrome. The capture suite rejects consecutive identical pixels at the
  same size; an intended repeat needs `expect_same` and a reason on the second step.
