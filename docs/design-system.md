# Splat design system

The brief is `docs/ux-intent.md`. This document records the visual decisions and why
they were made. The code is `src/design/`. Tokens are the only source of colour, size
and type. A lint test (`src/design/lint.rs`) keeps other code from adding raw values.

## Art direction: a working ledger, at night

A DM keeps a ledger for hours in a dim room. The screen must feel like a well-set page
and a confident tool. It is not a dashboard, and it is not parchment.

- **Warm dark, one theme.** The app forces `ThemePreference::Dark`. All neutrals lean
  warm (hue about 35°). The page is `#1B1815`, not black. Text is paper `#E9E1D3`, not
  white. There are no large bright areas. A later light theme adds one `Colors` value.
- **One accent: blue-black ink, inverted (`#A9BDE6`).** Most state colours are warm or
  earthy (HP damage, warnings). A conflict is a saturated rose, the one colour here that
  asks to be found at a glance. Temporary HP and concentration are cool, and they
  are told apart from the accent by shape and word. The party's side mark is a warm neutral,
  so that it is not read as the accent. The accent marks the current turn, the primary
  action, focus, and links. It is not decoration.
- **Hierarchy from type, spacing and hairlines.** Surfaces carry meaning only: the rail
  is recessed (navigation), a well is an input, a raised layer floats above the page. No
  cards around everything, and no pill badges. A status is an icon, a word, and a colour.
- **Two voices.** A sans for working UI. A serif for what the books say: entry titles,
  rules prose, and stat blocks. The change of voice tells the DM "this is reference".

## Type

Fonts are in `assets/fonts/` with their licences. All are OFL. egui 0.33 rasterizes with
`ab_glyph` and has no variable axes or OpenType features, so these are static files.

| Face | Use | Why |
|---|---|---|
| IBM Plex Sans (Regular, Medium, SemiBold, Bold) | UI | Tabular digits by default and a plain zero, so numbers read fast and hold their place with no special drawing. `1`, `l` and `I` differ. Holds up at 12 px and from a seated distance |
| Source Serif 4 (Text, Italic, Semibold) | Prose | A sturdy text serif with lining tabular digits, so `65 (10d8 + 20)` lines up |
| Source Serif 4 Subhead (Semibold) | Titles | The optical cut for 18 to 28 px |
| Source Serif 4 SmText (Regular, Semibold) | Stat blocks | The optical cut for 13 to 14 px, for the compact table at 1x |
| egui's monospace | Library and template editors, code in a note | Source text only. `Type::Mono` |

Atkinson Hyperlegible was the first choice. Its digits are proportional and its zero is
slashed, and painting each digit in a fixed cell left wide gaps. So the font
changed, and no fixed cells are left. `design::number` right-aligns ordinary text, and
HP and initiative hold their place because the font's digits have one width.

**Fallback.** Every UI family falls back to Source Serif 4 for symbols such as `× – — ½ ¼
¾ ⅓ ⅔`, then Phosphor, then egui's defaults.

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
| `Label` | UI SemiBold 11 / 14, tracked +0.6, capitals | Field label, column heading |
| `Numeral` | UI SemiBold 14 | AC, initiative, outside the run's scale |
| `NumeralLarge` | UI Bold 18 | Current HP, outside the run's scale |
| `Round` | UI Bold 18 | The round number in the run header |
| `Prose` | Serif 16 / 24 | Rules text at full width |
| `ProseCompact` | Serif 15 / 22 | Rules text in a side panel |
| `ProseStrong` / `ProseItalic` | As prose | Named actions, emphasis |
| `StatLabel` / `Stat` | SmText 13 / 18 | Stat block rows |
| `Mono` | Monospace 13 | Markdown source |

**Measure.** `READER_MEASURE` is 488 px at `Prose`, about 62 characters on average and
at most 75 (T6.1). Prose never fills a wide window.

**The run has its own scale.** `RunScale` holds a row height and type sizes, and
`RunScale::fit(height, width, rows)` takes the largest that fits with no scrolling.

| Scale | Row | Name | HP / max | AC, initiative | Conditions | Turn word | Roster width |
|---|---|---|---|---|---|---|---|
| `RUN_COMPACT` | 36 | 16 | 20 / 13 | 15 | 14 | 12 | 560 |
| `RUN_REGULAR` | 52 | 18 | 24 / 15 | 16 | 14 | 13 | 640 |
| `RUN_LARGE` | 68 | 22 | 30 / 18 | 19 | 16 | 14 | 760 |

## Colour tokens

`design::colors()` returns the active `Colors`. Names say what a colour means. Each family
below maps to section 6 of the brief. **Colour is never the only signal:** each state also
has a word, an icon, or a shape, and `src/design/tokens.rs` tests the contrast pairs (text
4.5:1; large text and UI 3:1).

| Family | Tokens | Words and shapes that go with them |
|---|---|---|
| Surface | `page rail raised well hover selected current_row` | Recessed, raised, sunken |
| Line | `hairline rule field_border` | Decorative rules, and the 3:1 field edge |
| Text | `text muted faint disabled` | Primary, secondary, tertiary |
| Accent | `accent accent_fill accent_hover on_accent focus` | Focus also has a 2 px ring |
| Recording | `recording` | `Recording` with its time and a record icon, in the rail on every page |
| Turn | `turn.current turn.next` | The word `Turn` and a bar; the word `Next` |
| HP | `hp.healthy damaged bloodied zero temp unknown track` | Numbers tinted by state, a bar under them, `Bloodied`, a neutral mark with `0 HP` (a skull would be a verdict, and death is the DM's ruling), `+5` with a shield and a strip above the bar, `?` |
| Participant | `participant.absent concentrating condition changed` | `Absent`, `Conc`, each condition by name, a pencil and `Changed` |
| Side | `side.party enemies other` | A square, a diamond or a circle on a live row, where the shape alone is enough at a look, with the name on hover. The name in words in setup and on Encounters. No red and green |
| Sync | `sync.off first_read matching pending conflict read_only error` | Icon and word in every case: `Reading…`, `Synced`, `Sync pending`, `Conflict`, `Sync read-only`, `Sync error`. A row at risk has `Not sent` in the strongest tone, and `Sync stopped` is amber, with nothing to lose |
| Connection | `connection.connected offline busy` | Icon and word |
| Document | `document.clean unsaved saved invalid external read_only` | Icon and word |
| Result | `result.error warning success info` | `message` and `compact_message` add an icon and a bar |

## Space, shape and density

- **Grid:** 4 px. Tokens `XS 4, S 8, M 12, L 16, XL 24, XXL 32`.
- **Radii:** `RADIUS_CONTROL 3`, `RADIUS_LAYER 6`. Nothing is round for its own sake.
- **Density** is a level a control asks for, by what it is for, and not a setting of the
  screen. A screen mostly keeps to one, and may use a lighter one for what is rare:

| Level | Control | Row | Mostly used by |
|---|---|---|---|
| `Glance` | 36 | see `RunScale` | The run's `Next turn` and damage entry |
| `Scan` | 30 | 32 | Run setup, Encounters, Party, Lookup results |
| `Read` | 28 | 28 | Library, Settings, Campaign, and the run's footer |

Every control the app draws is at least 24 px (WCAG 2.5.8, and the `target-size` check).
The exception is egui's own scroll bar, which is 8 px. The check reports it as chrome.
It stays 8 px because egui paints the handle at the width it senses, and the wheel and
the page keys also scroll. Tab steps over it (`Splat::skip_unnamed_focus`). The check
counts a node as a bar only if it has no role and no name, lies outside a scroll
surface, and matches the full length of that surface's edge.

A column's drag handle is the same kind of chrome: 10 px wide and as tall as the column.
Tab steps over it too. It has no keyboard control. A column keeps a usable width without a drag.

## Widgets

`theme::install` sets one global `egui::Style`, so any plain egui widget still fits: hover,
active, focus ring, disabled, text selection, text edit, solid scroll bars, windows,
tooltips, and separators. Text cursors do not blink, because motion draws the eye (brief,
"Long sessions").

`design::components` holds the shared parts, each a function or a small builder:

| Part | Function | Notes |
|---|---|---|
| Page header | `page_header` | Title, one line of context, actions on the right |
| Nav item | `nav_item` | Icon and word. Accent bar when selected |
| Buttons | `Action::primary / secondary / quiet / destructive` | `.blocked(reason)` disables and shows why (G1). `.key("N")` shows the key beside the word |
| Split button | `split_action` | An `Action` that does the default, and a caret that opens a menu of the other ways |
| Field | `field` | Label, control, help, error; names the control for accessibility |
| Text fields | `text_field`, `text_input`, `number_field`, `search_field`, `named` | Always have an accessible name and a 26 px target. Numbers align left, so the caret never lies on a digit |
| Dialog | `dialog` | An `egui::Modal`, for the brief's modal tasks only: discard on close, removing a campaign or a package, copy review |
| Section | `section`, `keyed_section`, `quiet_section`, `section_icon`, `combo_icon` | Collapsing, with Phosphor arrows. `keyed_section` is for a title that changes while open, as egui makes a section's id from its title. `quiet_section` is for technical details at a page's end |
| Table row | `table_row` | Fixed cell widths and a hairline, for Markdown tables and sheets. No stripes, because a row's height follows its text |
| Rail toggle | `nav_toggle` | Lookup and panels. Its own `Open` word, so that it never looks like the current page |
| Segmented | `segmented`, `segmented_named` | Words with an underline, not a pill. `segmented_named` is for a tab that paints a mark |
| List row | `row`, `hover_row`, `two_line_row` | Fixed height, for `show_rows`. Paints hover at the current frame. `hover_row` says something, with no click or Tab stop |
| Status | `status`, `status_link`, `Status` | Icon, word, colour. `status_link` also leads to where the DM can act |
| Stat tile | `stat_tile` | A label over a large value, with no box. HP takes its state's colour |
| Icon button | `icon_button`, `icon_toggle` | One icon, with a name and a tooltip. `icon_toggle` stays pressed while its state is on |
| HP | `hp_bar`, `hp_readout`, `hp_state` | Fixed slots. Temp HP is separate |
| Empty state | `empty_state` | What this is, and the one next step |
| Message | `message`, `compact_message`, `Tone` | Inline. Never modal. `compact_message` is the run's small form, for the room under the roster |
| Side panel | `side_panel_width`, `panel_header` | One width rule for Lookup, Notes and the run. Above 820 px, the run's panel has two columns |
| Column | `Column::left / right / panel` | Every side column: the rail, a page's list, a panel. A drag handle on its inner edge. Natural width until dragged, then the DM's width, here and after a restart. A drag always leaves the page its room, and the run's roster keeps `RUN_LIST_KEEPS`. What sits beside a column reaches its edge, so no blank band opens between them. Text keeps its measure inside that room |
| Link | `link` | A real `Link` role, at least 24 px high |
| Number | `number`, `number_width` | Right-aligned digits |
| Count | `count` | `1 entry`, `2 entries` |

Icons support words. They replace a word only where the meaning is universal, and then the
control still has an accessible name and a tooltip.

## Keeping a draft

`Discard` is one press and one step of undo, not a dialog: `Draft discarded` with `Undo
discard` stays until the next edit. It stands apart from `Save`, and it shows only when
there is something to discard. A file that changed on disk stops saving and asks: the run
offers `Replace the file with this run` and `Save as a new run`, and a Library note offers `Overwrite with my
draft`, `Show disk version` and `Reload and lose my draft`. These messages cannot be dismissed.

## The Reader

`design::reader` holds the typographic parts. `library::view::show` draws one entry and is
the only entry reader: Library, Lookup, the run's side panel, and campaign entries all
call it (T6.4). A stat block is a typeset reference block: a heavy rule, then identity, then
armor class, hit points and initiative as three labelled values, then one tight attribute
table with physical and mental abilities side by side, numbers right-aligned, and one `MOD` and
one `SAVE` head over each side, then rows of details. It has no card. Section headings use the serif over a hairline.

Imported rules text is reflowed for reading, and the stored note is never changed.
`design::reader::run_in_title` finds a named paragraph (`Acid Breath (Recharge 5–6).`), and
only that turns bold. A title-case line above prose is a heading. Three or more rows of
numbers, or of a name and a number, become a table. `presentation::reading_tests` runs these
rules over every built-in note, so a fault cannot come back unseen. A window wide enough
moves the rarely wanted sections (source, engine support, technical details) to a margin, which
has one place whatever the entry holds, and the scroll bar has the pane's edge. Printed labels on an
entry form one table, closed by a rule. A dice expression does not break across lines.

## The live run

The roster is the screen. Every row has fixed columns: initiative, side mark, name, AC,
temporary HP, HP with its bar, then a state column for `Bloodied`, `0 HP` and the sync
word. Conditions take a second line that wraps before the HP column and grows the row, so
no condition is cut. A separator stands between words, and never opens a wrapped line. When a
window has no room for the state column, as at 960x640 beside the panel, the HP word opens the
second line and the sync word closes it, so that a name is never cut. On a full row, that can cost
a line.

- **Current row:** the `current_row` fill, a 2 px stroke, a 6 px bar, and the word `TURN`
  under the initiative, which has its own column. In a compressed list the word sits beside
  the number on its line, so that a row's height never depends on whose turn it is, and the
  room left over is shared across the rows. **Next row:** the word `NEXT`. The name
  line keeps one height with or without a second line, so a new condition moves no number.
- **Row height:** the scale sets the type, and spare room goes to the rows, up to 1.4 times
  the scale's height. A row with one more line takes it from what is spare first, so the others
  hold their height. When rows cannot fit, name-only rows shrink to 25 px before anything
  scrolls.
- **Header:** the round and the current turn in bold sans, larger than any row's name, and
  `Next turn`. Its second line belongs to `Undo`, which names the action, or says why there is
  none (`No Undo since reopening`). A long `Undo` gives up its name first, on the button, when a
  sync alert needs the room. The rest of the line goes to a sync alert with `Review`, then to a
  notice with no other place, then to the recording's state, then to the next name.
- **Notice:** what sync changed takes the run's small message under the list: a bar, an icon, small
  words and a dismiss on the right. It is smaller than `message`, because it shares the row's
  room. A full list keeps it to one line (`Sheet update: Brannoch Vale and 2 more.`), with the
  whole on hover and in the log. With no room for that line, it goes to the header's line
  (`Earlier Undo steps ended. See the Log.`). If a sync alert holds that line, or the row marked
  next is out of view and its name keeps it, the `Log` tab has a mark in the warning tone until the
  DM opens it. The `Log` has the whole message. While Lookup or Notes takes the panel's place, that
  tab is away: behind an alert, `See the Log` shows beside it if it fits, and otherwise the mark
  waits until Lookup or Notes closes. An accepted limit, as the message is delayed and not lost.
- **Dock:** a fixed place above the footer. `Amount`, `Damage`, `Heal`, `Temp HP`, then the
  selected name, `Conditions`, `Conc` and `Absent`. Enter applies damage. `Mark present` on
  someone with no initiative opens `Record`, with its field in focus.
- **Footer:** `Close run`, `End encounter…`, `Sync party` or `Stop sync`, `Keys`, and the save
  state. While sync has work to finish, `Close run` says what it waits for: `Close after sync`
  or `Sync first`, and with a conflict, which does not clear by waiting, `Resolve conflict to
  close` or `Resolve first`.
- **Side panel:** the title, then tabs that hold one place. Death saves and a conflict's two
  choices go under the tabs. While sync is stopped, those two choices are off, with the reason.
- **Overflow:** when rows still cannot fit, a line under the list says how many are above
  and below, with any `Conflict`, `0 HP` and `Bloodied` among them. A row cut by the edge
  counts as out of view, and is not painted. Each turn scrolls to show the current row and the next.
- **Strip:** on other pages, a line with the turn, any conflict with `Review`, `Back to run` and
  `Next turn`.
- **Setup:** `Everyone is ready.` is green only with nothing left to do. It says who is absent and
  how many are tied. `Undo` is in the footer here, far from `Begin combat`, and in the header once
  combat runs, where it is wanted every turn. A total set during combat is set once, and only Undo
  takes it back: a limit accepted because the case is rare, and a way to set it again would add a
  saved field and a second way to move someone in the order. `Mark present` puts the keyboard in
  that field, so letter keys wait for Escape, and digits and Enter set a total there. The same limit
  covers that case.
- **Changed:** a creature with overrides has `Changed` on its second line, then anything the
  run cannot apply, such as `Attack +1` or `Speed 40 ft`, because the DM needs it at the table. A setup
  row has the word `Changed` only. Its card lists each
  change at the top of the stat block's tab, so no tab moves, as `Maximum HP: 33 → 43 (+10)`. What
  the run cannot apply comes first, in larger type, and says `add by hand`. A line then says that the
  stat block below is as printed. The list scrolls with the stat block, and each participant keeps
  its own place in it. A side of its own also has its word on a live row. In setup, a row gives
  the side when it has room, and Record gives it always.
- **Groups:** a tab beside the stat block lists the campaign's encounter groups, with members,
  count and total XP. A group's name is a handle to drag onto the list, and the list takes an accent
  outline while one is held. Each group also has a field for a total, blank to roll, and an `Add`
  button that the keyboard reaches. No step asks for confirmation, because Undo names the addition.

## Sessions and the timeline

Notes are Markdown in the campaign's folder. `Write` styles the source as it is typed, in the Reader's faces: a heading
is large, has the Reader's rule, and `*these words*` are italic. Markup stays in view in the plain face and `faint`, so the
file is exactly what was typed and Obsidian reads the same text. The box fills its column, and `Read` uses the same width,
so a note keeps its line breaks between the two views. A session's notes, events and recordings follow the column, because these are the DM's own working pages. The books' rules prose keeps `READER_MEASURE`. `Read` is the one Reader. The gallery's `Markdown` sheet shows both from one note. A draft saves two seconds after the last edit, because at the
table nobody stops to press Save. A file changed elsewhere stops saving and asks, as in "Keeping a draft".

The timeline is a ledger, not a chart. It is one column in story order, with a hairline spine and an icon and
a word for each kind. A day opens with a rule, as a heading does, and numbers itself. It has no card and no
colour for kinds, because a kind is not a state. An event's detail sits beside the list, with the same width
rule as Lookup.

A live microphone has the only colour here. `Record` takes a second, inline step that says to tell the table.
While it records, the rail keeps the word and the time in view on every page. The time counts audio in the file, so it
stops when saving stops. The rail and the session's page say the same state: `Recording`, `Recording silence`, or `Not saving
audio`. A stall takes a banner at the bottom of every page, with `Record again`, because at the table the DM looks at the roster and
not at the rail. It is at the bottom so that it never moves `Next turn`. A recording or a job that fails takes the same banner, and
the rail, until the DM dismisses it or tries again. Silence takes no banner, because a table goes quiet often. After a minute it shows in the rail,
on the session's page, and on the run's own notice line.

**Notes** (Ctrl+J) opens the note of the session being played beside any page, as Lookup does, so a note never means leaving
the run. The caret starts on a fresh line under `What happened`. The panel never follows the page to another session: opening
an older note closes it. On the Sessions page, which has the same note itself, the panel stays away, and Ctrl+J goes to the editor. New events and runs go to the newest session unless the DM chooses another, and the Timeline says which before it
happens. Reading an older note changes neither. Beside the panel, the run's buttons work, and each step gives the keyboard back to the note, except a field that the DM is typing in. Lookup takes the panel's place, and the notes come back when it closes, because the DM may go on typing with their eyes on the table. Only a click on a row or on Amount takes it to Amount, for one number. A letter typed there goes to the note, with the digits before it. While Notes are open and the keyboard is on nothing, as after a stray click, what is typed or pasted still goes to the note. In the panel, Tab indents and does not leave the note, so that it cannot reach `Next turn`. Escape closes the panel. What the DM asks for in the run's own side panel, such as `Conditions`, replaces the notes when both have no room. Words from an LLM
are marked as a draft, the second pass's changes can be read, and a marker in a transcript has `Add`, not an automatic place on the timeline.

## The catalogue

Library is one catalogue of built-in, package and campaign entries. Its scope opens on what
this campaign can use, and a switch shows everything, one package, or the campaign entries. Changing the scope never changes the active campaign. Editing, save state and
`Copy to…` stay in the same page. Package actions are inline. The word `module` is gone
from the interface. `Campaign entry` is the one term for what a campaign owns. A list row and the
reader use one second line, and the reader adds a level the title has lost (`· Level 3`). On **Campaign → Packages**, required entries are in columns, listed
only under the package that holds them, with the campaign entries as their own group.

## Icons

`egui-phosphor` (MIT, Phosphor Regular). `.gitignore` forbids committed images, so all
icons are glyphs or painter shapes. `design::icons` names only the glyphs in use.

## Tests

- `design::tokens::tests`: WCAG contrast over the token pairs.
- `design::lint`: no raw colours or text sizes outside `src/design/`, with a shrink-only
  allowlist in `src/design/lint_allowlist.txt`.
- `gallery` visual scenario: the tokens and shared parts people choose between, as PNGs.
  It is a reference, not a full inventory: some tokens and whole-screen parts, such as the
  run row and the dialogs, are seen in their own scenarios.
- `visual_tests::ergonomics`: target size, accessible name, in view, overlap (including
  over labels), clipped text (any ellipsis, or text past its pane's edge, read from the
  painted shapes), keyboard reach, and the workflow goals. Each check except the workflow
  goals has a seeded failure in its unit tests. Workflow goals were shown to fail by breaking
  the app for one run.
- A run of every scenario must report every test in section 8 of the brief, and the baseline
  refuses a failed goal. `visual-compare <labelA> <labelB>` lists captures that differ above
  the rendering noise. Its own tests prove
  that a thin unnamed control in scroll content, and an unnamed control in a popup, do not
  count as egui's chrome.
