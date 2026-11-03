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
  earthy (HP damage, warnings, conflicts). Temporary HP and concentration are cool, and they
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

**Measure.** Lists of rows with actions use `LIST_MEASURE` (880 px), so actions stay near the names. `READER_MEASURE` is 488 px at `Prose`, about 62 characters on average and
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
| Side | `side.party enemies other` | The side name, with a square, a diamond or a circle. No red and green |
| Sync | `sync.off first_read matching pending conflict read_only error` | Icon and word in every case |
| Connection | `connection.connected offline busy` | Icon and word |
| Document | `document.clean unsaved saved invalid external read_only` | Icon and word |
| Result | `result.error warning success info` | `message` adds an icon and a bar |

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
counts a node as a bar only if it has no role and no name and matches the full
length of a scroll surface's edge. A nested horizontal bar can float up to the
reader's visible bottom edge. It stays hidden while its table is below view.

Reader tables keep each column at least as wide as its longest word. A table
that cannot fit scrolls sideways and says how to do that. The first column stays at the reader’s left edge, with a hairline beside it. The header stays at the reader’s top edge until the table leaves view. Ordinary wheel
scrolling still moves down the note. The ergonomics check permits side clipping
only where a real horizontal scroll bar can reveal the text; it still rejects
clipping in ordinary readers and text cut to an ellipsis.

A column's drag handle is the same kind of chrome: 10 px wide and as tall as the column.
Tab steps over it too. It has no keyboard control. A column keeps a usable width without a drag.

## Widgets

Library, Lookup and encounter search rank equal names the same way: campaign entries, book and package
entries, then character sheet copies. File paths shorten before their file actions wrap;
the full path stays on hover. All save rows call their discard action `Discard`. A clean
Library editor shows `No changes` in the left status slot.

Creature vitals come from `splat_stats` and appear once in the reader. The editor warns
when a prose AC or HP line disagrees and offers `Use <value>` with Undo. Missing creature
replacements use the shared search row, with CR, HP, AC and `Use`. Replace all matching
rows is on by default; it keeps their names and starting values, with one Undo.

World uses labelled `Stat block`, `In` and `Status` rows with `Change` actions. Values share a fixed column and text baseline; empty values say `not set`. Missing links keep their names. New NPCs
show their future path below the name. A missing place stays visible as `Not found`.

`theme::install` sets one global `egui::Style`, so any plain egui widget still fits: hover,
active, focus ring, disabled, text selection, text edit, solid scroll bars, windows,
tooltips, and separators. Text cursors do not blink, because motion draws the eye (brief,
"Long sessions").

`design::components` holds the shared parts, each a function or a small builder:

| Part | Function | Notes |
|---|---|---|
| Page header | `page_header` | Title and one `Small` muted state or count on the same baseline; actions on the right; a hairline below, then an `L` gap. Never repeat the campaign name |
| Nav item | `nav_item` | Icon and word. Accent bar when selected |
| Buttons | `Action::primary / secondary / quiet / destructive` | `.blocked(reason)` disables and adds a hover reason and accessible description. For one blocked control, put `blocked_reason` directly beside it in visible `Small` text, then an `L` gap before the next action. `.key("N")` shows the key beside the word |
| Split button | `split_action` | An `Action` that does the default, and a caret that opens a menu of the other ways |
| Field | `field` | Label, control, help, error; names the control for accessibility |
| Text fields | `text_field`, `text_input`, `number_field`, `search_field`, `named` | Always have an accessible name and a 26 px target. Numbers align left, so the caret never lies on a digit |
| Dialog | `dialog` | An `egui::Modal`, for the brief's modal tasks only: discard on close, removing a campaign or a package, copy review |
| Section | `section`, `quiet_section`, `section_icon`, `combo_icon` | Collapsing, with Phosphor arrows. `quiet_section` is for technical details at a page's end |
| Table row | `table_row` | Fixed cell widths and a hairline, for Markdown tables and sheets. No stripes, because a row's height follows its text |
| Rail toggle | `nav_toggle` | Lookup and panels. Its own `Open` word, so that it never looks like the current page |
| Segmented | `segmented` | Words with an underline, not a pill |
| List row | `row`, `two_line_row` | Fixed height, for `show_rows`. Paints hover at the current frame |
| Status | `status`, `status_link`, `Status` | Icon, word, colour. `status_link` also leads to where the DM can act |
| Stat tile | `stat_tile` | A sentence-case `Small` muted label over a large value, with no box. Sheet group headings keep `Label` capitals and an `L` gap above |
| HP | `hp_bar`, `hp_readout`, `hp_state` | Fixed slots. Temp HP is separate |
| Empty state | `empty_state` | What this is, and the one next step. At the top of an empty pane or page, with its normal padding |
| Group heading | `group_heading` | `Type::Label` in capitals and a muted, nonzero count after it. No rule. Serif headings belong to readers and stat blocks |
| Row menu | `row_menu` | Quiet `⋯` at the right edge, named `More actions for <name>`. Put Remove and Delete here |
| Add strip | `hanging_add_strip` | `Add` in its own Label column; quiet icon-and-word buttons wrap at the first button's left edge. Fight, Group (plan only), Day, Long rest, Short rest, Travel, Major event |
| Message | `message`, `message_with_action`, `Tone` | Inline. Measure the full action, including padding. Move it below the text, aligned with the text, when both cannot fit on one line |
| Side panel | `side_panel_width`, `panel_header` | One width rule for Lookup, Notes and the run. The run separates participant tabs from fight tabs |
| Column | `Column::left / right / panel` | Every side column: the rail, a page's list, a panel. A drag handle on its inner edge. Natural width until dragged, then the DM's width, here and after a restart. A drag always leaves the page its room, and the run's roster keeps `RUN_LIST_KEEPS`. What sits beside a column reaches its edge, so no blank band opens between them. Text keeps its measure inside that room |
| Link | `link` | A real `Link` role, at least 24 px high |
| Number | `number`, `number_width` | Right-aligned digits |
| Count | `count` | `1 entry`, `2,108 entries`; zero returns no text. Empty unit words return only the formatted number |

Header actions have at most one bordered secondary button, for `New …`. Other header actions are quiet and come before it.
The Library scope control uses Can use in both layouts and sits at the top of its category column. At a narrow width, scope and category share one line at equal widths above search. Hide the All entries heading when there is no search.

**R-6: Empty states.** The full `empty_state` (icon, serif title and text) belongs to an empty pane or page only. An empty section inside a list page shows one muted line under its heading, with no `0` count.

One accent-filled primary action can be in view. On other pages while a run is open, the run strip's `Next turn` owns the primary fill. `Back to run` is secondary.
A page uses `status_link`, for example `Running · Round 2`, to show a run's state and open it.
An on toggle uses `selected`, its icon or a check, and its word. It does not use the accent fill.
Delete and Remove belong in a row menu, away from the primary action. A save row has the save-state word and quiet `Discard` at the left, and `Save` at the right.
Use red only for an action that loses something.

When one cause blocks several controls, state the reason once at the cause: the draft's save row, the run strip, or the recording row. Other blocked controls use the same reason as hover text and accessible description. The rail never repeats the sentence. Its campaign switcher keeps its height and shows a lock; its popup starts with the reason and disables campaign choices. A package draft does not block a campaign switch.

Every icon comes from `design::icons`. Do not pass a raw symbol as widget text. The `missing-glyph` ergonomics check reads painted text and fails if the selected font and its fallbacks lack a character. Its unit test seeds an unassigned character.

Empty readers use `No entry open` or `No note open`, then “Choose one from the list, or make a new one.” Library omits the creation offer when no campaign or package is available.

Icons support words. They replace a word only where the meaning is universal, and then the
control still has an accessible name and a tooltip.

## Keeping a draft

`Discard` is one press and one step of undo, not a dialog: `Draft discarded` with `Undo
discard` stays until the next edit. It stands apart from `Save`, and it shows only when
there is something to discard. A session note changed on disk shows the draft and disk text side by side in the Write view’s styled source face. Text wraps at word boundaries. A selected surface and a line mark identify differing lines. The choices stay above the comparison, which uses the whole session body while the plan is hidden. A file that changed on disk stops saving and asks: the run
offers `Replace the file with this run` and `Save as a new run`, and a Library note offers `Overwrite with my
draft` and the separate red `Reload and lose my draft`. Library uses the same side-by-side comparison, with `N lines differ` and `Next difference` only for two or more differing lines. Both comparisons open with the first difference and two lines of context in view. A missing line says `No line`. Neither choice has an accent fill. One message says: “This note changed on disk. Your draft is safe until you choose.” Library calls the item an entry in this message. These messages cannot be dismissed.

## The Reader

Library's toolbar stays on one line. Drop the Back key hint first; then move Copy and
Make NPC note into a quiet More menu. Lookup keeps the Back key hint. Both readers use
Alt+Left, with Lookup taking the key when focus is there. The owner line supplies the
package name in Library. The kind line does not repeat it.

New creatures store identity and CR in `splat_creature`, and speed and ability scores in
`splat_stats`. The CR field takes precedence over a legacy body line on every surface.
Each template stat uses one YAML line. Legacy CR and Speed lines appear only in their
detail rows, without Markdown line breaks. An absent saving throw leaves its cell empty.
Campaign spells use the book's fact order: Casting Time, Range, Components, Duration.
Concentration and Ritual appear in those values only when true.
The reader uses one L gap after the kind line and one M gap after a spell's fact table.
World relationship rows use a 32 px pitch, with aligned values and Change actions.
Values wrap inside their column, with Change on the first line. A change's message and
Undo follow Change when they fit; otherwise they sit below the whole row block.
Broken links have a warning icon, a list-row warning and a count in the World header.

`design::reader` holds the typographic parts. `library::view::show` draws one entry and is
the only entry reader: Library, Lookup, the run's side panel, and campaign entries all
call it (T6.4). A stat block is a typeset reference block: a heavy rule, then identity, then
armor class, hit points and initiative as three labelled values, then one tight attribute
table with physical and mental abilities side by side and numbers right-aligned, then rows
of details. It has no card. Section headings use the serif over a hairline.

Imported rules text is reflowed for reading, and the stored note is never changed.
`design::reader::run_in_title` finds a named paragraph (`Acid Breath (Recharge 5–6).`), and
only that turns bold. A title-case line above prose is a heading. Three or more rows of
numbers, or of a name and a number, become a table. `presentation::reading_tests` runs these
rules over every built-in note, so a fault cannot come back unseen. Collapsed source, engine support and technical details follow the body in its scroll area.
Here and Mentioned in stay above them, at the reader measure.

## The session's plan

Beside the note, the plan takes 40% of the page width, between 330 and 480 px.
Fight summaries keep each part together and join a separator to the next part.
Group summaries shorten to one line with the full text on hover. A recording count
sits inside its tab, in muted text and in the tab's accessible name. Notes saved
centres on the title's action row. A new template has an L gap above its heading
and an M gap below it; the list's help stays out of the form.

The Session page is a list the DM works down, beside the note. A row is an icon, a link and a second line, with one action on
the right. A row opens in place, and its fields use the same form parts as the Timeline page. The first fight to come has
the primary button when no run is open, and the others the secondary. A saved unfinished run offers Resume, with Start a new run and Delete the saved run in its menu. The open run has a status link; the strip owns the primary. The word for a fight is **Start
fight**, on this page and on Encounters. All plan rows reserve the same right-aligned action slot and a separate row-menu slot. Each planned fight has Remove from tonight in its menu. It removes one plan link, keeps other words and has Undo remove below that row. Add buttons stay at the foot of the plan, and a message opens above them, so
that they do not move under the pointer. An empty plan uses `empty_state` above the strip. A played fight shows its result and last round once, on its second line. Its final party HP uses first names in a grid of two or three cells per line, then quiet Log, Whole run and Timeline links. Hover gives full names and conditions. Remove stays in its row menu, aligned with the name. A long rest and its full-HP ruling share one message and one Undo; Not at full waits inside the opened row. In a page narrower than 720 px, the plan and the note have a tab each.

## The encounter ledger and editor

Encounters uses one column, at most 880 px wide. The sections are `IN PROGRESS`,
`READY TO RUN`, `GROUPS`, and `HISTORY`, in that order. Group headings use `Label`
and a muted count, without a rule. Empty sections hide the count and show one muted
line with the next step. Groups and History start closed when they have rows. Each row keeps
its main action, quiet `Edit`, and `⋯` menu at the column’s right edge. The menu is
named `More actions for <name>`. It holds deletion and starting another run.
An unfinished run shows `Setup · Paused` or `Round 2 · Paused` and `Resume`. An open run shows the
status link `Setup` or `Round 2`; its second line keeps only the saved time. The strip owns the primary `Next turn` and secondary
`Back to run`. An encounter with an unfinished run appears only in In progress. When
the run ends, its encounter returns to Ready to run. With `Keep in Ready during a run`
on, its Ready row keeps `Start fight` blocked and says to resume from In progress.
History opens a read-only final state and log in place. It never takes the live-run slot.
The detail header shows only Read-only and the end reason; the row keeps the outcome
and round. The page header counts every Ready row, including a repeatable fight
with an unfinished run. Creature stats use CR, HP, AC in search, roster and group
rows. Party rows keep HP, AC. A different draft starts with an empty search.
One paused run uses `Paused run` in the rail and the encounter name below, cut to fit. Hover and accessibility keep the full name and state, for example `Bridge Ambush · Round 2`. It resumes in one click. With two or more, the rail uses `Paused runs` and `2 fights`; it opens Encounters. An open run keeps `Run` and its round or `Setup`.
History rows keep only `Open` or `Close` and their row menu. Start and Edit stay in that menu.
Ready rows name their creatures; an empty roster blocks Start with `Add participants first`.
The open-run block appears once under `READY TO RUN`.

Roster rows have the name and second line, side menu, quiet `Edit`, then `⋯`.
The menu holds ordering, duplication, `Make it yours`, and removal. A creature
result’s name opens its stat block; its one button is `Add`. A party member already
present shows `In roster`, and leaves an empty search once the whole party is present.
Results use text-coloured names, underlined on hover. The main source has no suffix; other sources
and campaign-owned entries show their source or `Campaign`, including roster rows.
The sync and repeat options follow the roster. The
header keeps Save and Start on its first line, and the name field gives up width.
An encounter with an unfinished run offers Resume and moves Start another fight into its menu.
Its save row includes the run state.
`Discard` is quiet beside the save-state word. A blocked Start shows its reason.
The side picker offers `Party`, `Enemies`, and `Other…` in every editor. Only choosing Other opens a name field; it replaces the value.
All editor section headings use `group_heading`. In the tabbed layout, Add participants has no repeated heading.

A plan group editor takes the full notes column; the note stays behind its tab.
Plan summaries sit directly below names, with actions centred beside both lines.
Member counts and XP use one dot-separated line. Count stays at the left; Edit and
the row menu sit together at the right. Encounter editors show one clean-state word.

Member names and stats have their own lines above Count, Edit, and the row menu.
Member changes stay on one line, such as `Changed · 2 to add by hand`, until opened.
An empty group frame has one muted line and no zero count or duplicate search summary.
The Groups list has one level of headings: For any encounter, each encounter, then Encounter deleted.
Its rows keep Edit and a menu with Remove, which has Undo. A planned group also has Remove with Undo; it removes the link and keeps the note’s words.
A summary of members stays below the search when it has members. Removing an encounter’s group makes
it available for any encounter on Save, with Undo until that save.

A saved Setup resumed after a restoring long rest starts party HP at full. Its
notice says `Party HP is full after the long rest.` and offers `Use the saved HP`
as one Undo step. Combat keeps its saved HP, and the rest message names those fights.
New templates say `Starts with the title line only.` below the name field.

## The live run

The roster is the screen. Every row has fixed columns: initiative, side mark, name, AC,
temporary HP, HP with its bar, then a state column for `Bloodied`, `0 HP` and the sync
word. Conditions take one second line before the HP column. Measure them without change bonuses;
shorten bonuses first, then use a count only for conditions that do not fit. Keep the first condition.
Long local effects show four words and `more`.
The count opens their full text in `Conditions`. When a window has no room for the state column, as at 960x640 beside
the panel, those words open the second line instead, so that a name is never cut.

- **Current row:** the `current_row` fill, a 2 px stroke, a 6 px bar, and the word `TURN`
  under the initiative, which has its own column. In a compressed list the word sits beside
  the number on its line, so that a row's height never depends on whose turn it is, and the
  room left over is shared across the rows. **Next row:** the word `NEXT`. The name
  line keeps one height with or without a second line, so a new condition moves no number.
- **Row height:** the scale sets the type, and spare room goes to the rows, up to 1.4 times
  the scale's height. When rows cannot fit, name-only rows shrink to 25 px before anything
  scrolls.
- **Header:** the round and the current turn in bold sans, larger than any row's name, and
  `Next turn`. The header always has two lines. Line two keeps the notice at the left
  and Undo at the right. Measure the notice first; Undo takes the remaining width, less an L gap.
  If needed, drop `Undo:`, then shorten only the target name, keeping a copy's number.
  Keep action words, amounts and condition directions. Hover and the accessible name keep
  the full action. An empty history shows `Nothing to undo`, or `Nothing to undo since reopening`.
  After a sheet update, the reason names whose sheet changed. Both screens put this reason to the left of blocked Undo. A narrow slot uses
  `Nothing to undo`; hover and accessibility keep the full reason.
  A sync conflict comes first, then recording silence, then other
  sync notices, then the next name. Silence and the short sync state stay together when
  space is short. Connect to sync gives up its link before either state gives up its word.
- **Dock:** a fixed place above the footer. `Amount`, `Damage`, `Heal`, `Temp HP`, then the
  selected name, `Conditions`, `Conc` and `Absent`. Enter applies damage.
- **Overflow:** when rows still cannot fit, a clipped list ends above a separate page-filled band with a hairline. The band says how many are above
  and below, with any `Conflict`, `0 HP` and `Bloodied` among them. A row cut by the edge
  counts as out of view. Each turn scrolls to show the current row and the next.
- **Strip:** on other pages, a line with the turn, any conflict, secondary `Back to run` and primary `Next turn`.
- **Changed:** a creature with overrides has `Changed` on its second line, then anything the
  run cannot apply, such as `Attack +1` or `Speed 40 ft`, because the DM needs it at the table. A setup
  row has the word `Changed` only. The second line starts under the name at every scale. If the list still does not fit after name-only rows shrink, keep the first table bonus and a `more` link to the full changes. A long condition list keeps the conditions that fit and a count link to Conditions. Its card lists changes at the top of the stat block's tab, except AC, maximum HP and initiative bonus, which the value trio already shows. What
  the run cannot apply comes first, in larger type, and says `add by hand`. The stat block uses the
  run's AC, maximum HP and initiative bonus, with the printed value below each changed value.
  The list scrolls with the stat block, and each participant keeps its own place in it.
  A side of its own also has its word on a live row. Setup and live rows use the same side,
  HP, temporary HP and AC parts. `Correct` holds the correction fields.
- **Encounter editor:** each of the encounter's groups has a frame below the roster, with its name,
  side, members and notes. The frame that takes the search's results has an accent outline, and
  `Add to` above the search names it. The Encounters page keeps these groups in one closed section.
- **Panel:** one tab row has participant tabs (`Stat block` or `Sheet`, `Conditions`, `Correct`)
  at the left, then an `L` gap and fight tabs (`Log`, `Prep`, `Arrivals`) at the right.
  Fight tabs name only the fight tab in the title in setup and live play. When space is short, they share a `Fight ▾` menu.
  The row never wraps.
- **Arrivals:** a tab beside the stat block adds one creature, found or made up. Only search
  shows until a name is typed. The form then has two fixed rows of fields. Below it, the tab
  lists the encounter's own groups, then those in tonight's plan, then the
  others for any encounter, with members, count and total XP. A group's name is a handle to drag onto the list, and the list takes an accent
  outline while one is held. Each group has two lines: name handle, side, count and XP; then
  members, a `Total` field (blank rolls initiative), and `Add`. Member summaries wrap at the
  minimum width. Each fault has its own member line and an `Edit group` link. Added groups
  keep `Add` and say which round they joined. No step asks for confirmation, because Undo
  names the addition.

Setup keeps fixed initiative, Roll, Absent and menu columns. All controls share the name line, centred with the initiative field. Empty Roll slots keep player totals aligned. Only tied rows have Tie and two fixed arrow slots below the total. Other rows fit their two text lines and Scan padding, with RUN_REGULAR as the minimum height. The header always reports ties. Undo stays at the top right in setup and live play. The blocked Begin combat reason sits directly below that button. Setup's row menu opens Correct for that participant.

Death-save counts and actions occupy one line below the panel tabs at zero HP. Clicking a count edits it; Correct has no duplicate fields. Correct initiative and DM notes show a short reason beside a blocked commit action.

Arrivals uses group headings and one muted line for an empty group section. New group opens Encounters. A broken group's initiative field and Add action are both blocked, and Campaign → Packages opens repair. A setup arrival says Added before combat. Add names include the count when more than one creature will join.

Recovery copies use a UTC save-time suffix and preserve the other file. The run closed last owns primary Resume. Saved rows show the last change, mark the preserved file Changed elsewhere, and keep an empty Edit slot when no encounter exists. Run state words come from one function: Setup, Setup · Paused, Round 2, Round 2 · Paused, Ended · Round 2.

A disconnected run keeps Connect to sync on its header only. The footer's blocked Sync party uses the same cause as hover text. End encounter… shortens to End…; Keys keeps its word beside Lookup, Notes and a wide run panel.

The app-close dialog uses Keep running for a run, Keep recording for audio, and Keep processing for a job. Recording and processing titles name the unfinished session. A sync conflict names the character and offers Review conflict. Stop and close is secondary when saved work is kept; an action is red only when it loses work.

## Sessions and the timeline

Timeline keeps the save state, Copy link and menu on the title line when all parts
fit. Otherwise, the actions take a second line above the scrolling form. Account
help and trust text keep READER_MEASURE in both column and stacked Settings layouts.
The gallery shows clean, dirty and discarded save rows and the hanging Add strip.

The session title and muted date wrap together, with Record on the title line. Below 640 px, the save word and Details sit beside Read on the content tab row. The Notes panel uses the same title, date and save words, and opens with What happened at the top. Last time is one line: a link to the previous session and its recap, with the full recap in the link's tooltip. When the recap is empty, use plain prose under What happened and skip link-only lines. With neither, show the session link alone. Read and Write sit on the content tab row when the note is visible. Templates is a quiet header action. Its list starts with a Sessions back link, and New template sits in the page header. Templates use the note's styled source editor at the full column width; template fields stay faint. The list has no selected row while the new-template form is open. The session folder setting belongs to Campaign, Link and folder. A folder change counts the notes, events, templates and recordings left behind. It offers Move files and save or Save without moving. Moving refuses file collisions and copies every owned file before changing the manifest. A new session uses a Label heading and shows its note path as the fields change. An older session shows one warning and a link to the session being played; the list marks that session as Playing. Built-in templates leave one blank line below each heading. Read view shows Obsidian links inline, with the link text exactly as written. Renaming a Timeline event never changes a note by itself. An explicit Update links in notes action changes old-title aliases in one batch, with Undo; custom aliases stay as written. The offer names the old and new titles and sits directly below Title in the event detail. Its Undo message stays in the Timeline list column only, until any note or event edit or until the DM leaves Timeline. It never appears in a conflict. A Plan for tonight section made only of fights already in the plan shows one Small muted count instead of repeated links; the file keeps the links. New plan events stay closed and use the kind word as their title. Open rows show Title and Notes; duration and place stay on Timeline. A caret and accessible expanded state mark open rows. Their details align with the name, and the hairline comes after them with an S gap.

Notes are Markdown in the campaign's folder. `Write` styles the source as it is typed, in the Reader's faces: a heading
is large, has the Reader's rule, and `*these words*` are italic. Markup stays in view in the plain face and `faint`, so the
file is exactly what was typed and Obsidian reads the same text. The box fills its column, and `Read` uses the same width,
so a note keeps its line breaks between the two views. A session's notes, events and recordings follow the column, because these are the DM's own working pages. The books' rules prose keeps `READER_MEASURE`. `Read` is the one Reader. The gallery's `Markdown` sheet shows both from one note. A draft saves two seconds after the last edit, because at the
table nobody stops to press Save. During the autosave delay, notes show Saving… in muted with the busy icon. A file changed elsewhere stops saving and asks, as in "Keeping a draft".

Plan row actions show their message and Undo below the affected row with an S gap. Removing a planned group puts the message and Undo in its old row’s place and keeps the other rows still. The second line of a plan row can use the width below its action. Summary items wrap together with their separator, so a line never ends with a dot. Add-strip messages stay beside Add. A removed plan group stays in page state until the next note or plan edit. Duplicate saved-run names include their session or a save date; copies saved on the same date also show the save time. Resume has the run icon. Session and Library conflicts use the shared comparison. It opens at the first difference with two whole lines of context above it. Session also names the nearest heading.

The timeline is a ledger, not a chart. It is one column in story order, with a hairline spine and an icon and
a word for each kind. A day uses the shared group-heading style, without a rule, and numbers itself. It has no card and no
colour for kinds, because a kind is not a state. An event's detail sits beside the list, with the same width
rule as Lookup. The detail column exists only when an event is selected. Its header stays above the scrolling form: kind, title, save state, Copy link and a row menu with Remove. Selecting a different event, opening Timeline or changing its filters resets the fields to the top. The scrolling fields clip below the fixed header. A Day shows Starts without an Ends time. A copy uses a check and Copied on the button for two seconds. Link text stays in a quiet section; Place after is the one ordering control. Adds to shares the filter line. Added and removed notices sit in the list column. Selecting or adding an event resets its form to the top. Its context line shows the visible count out of the total while a search or filter is active. The filter says Major events. Removing an event says how many notes still link to it. Its save word tracks its own edits: Saving… in muted during the delay, Saved when unchanged, and Unsaved with a reason when saving is blocked. A rest shows its default duration as a faint placeholder, and Starts and Ends beside the duration help. Field labels have an M gap before them.

Party uses one group-heading style throughout Overview. Saving throws and skills use two or three columns. Lists with eight or fewer entries have no filter or separate count line; the shared group heading holds the count. Local-edit warnings name the character, keep actions on the first line at the right edge and say that replacement keeps a copy.
The affected list row says `Local edits`. A replacement names the character, with Show file for the kept version and Dismiss.
An empty Party has one Connect D&D Beyond action. Spell levels have their own counts, with no extra Spells heading; cantrips omit Prepared and use one-line rows. Coins sit below the Equipment & coins heading. Bloodied uses the run’s heart icon, word and colour on Party and encounter roster rows.
Technical details stays closed and uses plain words for rules not read yet or not supported.

A live microphone has the only colour here. `Record` takes a second, inline step that says to tell the table.
While it records, the rail keeps the word and the time in view on every page. The time counts audio in the file, so it
stops when saving stops. Processing help keeps READER_MEASURE, while recording rows fill their column. A processed warning uses the same full state, Processed, with a warning, on the row and in the rail's hover text and accessible name; the rail's short word is Processed. Continue processing resumes work kept after a stop or a failure. A recording that stops by itself has Open the session in its banner. On the timeline is a success status link wherever it opens the added event. The rail and the session's page show the same state: `Recording`, `Recording silence`, or `Not saving
audio`. In the rail, silence is shortened to `Silence 0:00`, with the full state in its hover text and accessible name. A stall takes a banner at the bottom of every page, with `Record again`, because at the table the DM looks at the roster and
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
this campaign can use, and a switch shows everything, one package, or the campaign's own
entries. Changing the scope never changes the active campaign. Editing, save state and
`Copy to…` stay in the same page. Package actions are inline. The word `module` is gone
from the interface.

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
  over labels, except a label fully inside its own interactive ancestor), clipped text (any ellipsis, or text past its pane's edge, read from the
  painted shapes), keyboard reach, and the workflow goals. Each check except the workflow
  goals has a seeded failure in its unit tests. Workflow goals were shown to fail by breaking
  the app for one run.
- A run of every scenario must report every test in section 8 of the brief, and the baseline
  refuses a failed goal. `visual-compare <labelA> <labelB>` lists captures that differ above
  the rendering noise. Its own tests prove
  that a thin unnamed control in scroll content, and an unnamed control in a popup, do not
  count as egui's chrome.

## Recording and content recovery

The rail has one recording status link at its foot. It includes the saved audio time and opens the recording. The session list keeps Failed, Stopped or Cancelled until retry or dismissal. A missing-input banner puts Choose an input first and keeps Record again quiet. A stalled recording puts its reason and Record again on the visible recording row; other pages keep the bottom banner.

Each recording keeps its failure or stop reason beside its status, including the last processing step. Dismissing a banner does not remove that row state. While a recording is live or processing, its stored stop or cancellation reason stays hidden. A cancellation says which part stopped and offers Continue processing. Failed and Stopped use the error icon and colour; Cancelled uses warning. A stopped recording keeps its Settings link to the recording input. Silence uses the warning colour on every page. The result viewer uses one segmented control for Summary, Clean transcript, What the check changed, and Raw transcript. Every generated view uses the same Small draft notice. If the check did not run, the row says `No check ran` beside Raw transcript. The row menu stays beside the title, and L space separates its actions from the events heading. The result viewer has its back link and muted date on the first line, with Add to notes at the right on Summary. Its view choices start at the left on a second line. The segmented control never wraps; if the full set does not fit, one dropdown shows the current view name. Tabs and page headers omit zero counts. Templates names its own page and uses Campaign as the owner word.

Lookup shows up to five one-line results with the open result selected. A result shows its name, then muted type and level. Campaign-owned entries add Campaign; a source shows only when it differs from the campaign’s main source. Only the muted part is shortened with an ellipsis. The hover and accessible name keep the full row. Result lists never scroll sideways. A click on a result, or scrolling into its reader, closes the list. `Show all` offers the remaining results. Creation actions follow the results and include the proposed name. They appear only without an exact match; Generate NPC appears only for an empty search.

A copy starts with the original name selected. Its destination and dependency preview update with the fields. Create copy saves it in one step. Edit the copy selects the result and opens its Markdown editor. An open encounter draft keeps Add to its encounter as the primary completion action. A Library disk conflict uses the full reader width for its single row of choices.

An incoming sheet change removes only undo steps for that character. Remaining snapshots keep the current sheet values. An atomic step that also affects another participant ends older affected steps if retaining them would restore stale values; the Log names each removed step. A DM choice in a conflict is a named undo step.

Campaign repair opens at the reader measure, with the invalid-note reason above it. Needed by your entries is an item below the packages in the left column. Its right pane has one search and one list; rows open Lookup. Short package lists shrink to their rows. Invalid YAML marks the line to fix and uses plain words. A rolled World table selects the matching row, marks it with a dice icon, and states the result below in UI sans.

Library keeps search and the selected result when its scope changes. Empty categories and their headings stay hidden. Scope buttons use short words. Library shows quiet Back whenever it has reading history, with its Alt+Left hint when room permits. Reader links use accent text and an underline. Links in Library open in place; links in other readers open in Lookup. An empty reader says No entry open or No note open, then “Choose one from the list, or make a new one.” Empty campaign entries have one New entry action.

Inline creation forms use a Label title, fields, then a named Create action, its Small blocked reason, an L gap, and quiet Cancel. A Library draft states its cause once in the existing save row; controls that share the cause use its hover text. Typing never inserts a line above the editor. Campaign save state also belongs only in its save row. Lookup Back and Open in Library share the title line. Plain World notes use one-line rows. A typed table roll has the well fill, a centred placeholder, and the dice as its column heading. Fixed table headers cover passing rows with page fill and a hairline.

Settings uses `CAMPAIGNS` and `D&D BEYOND ACCOUNT` group headings in both layouts. In the compact layout, the open account form replaces the status line and its `Connect…` link. The account verb is `Connect`; states include `Not connected` and `Signed out of D&D Beyond`. A keyring warning owns its `Connect again` action. `Disconnect` is quiet and follows the trust text. A quiet action on its own line aligns its word with the text above it; its target can extend into the gutter. The add-campaign form closes after a successful add.

Settings shows one Connecting status, and Cancel aligns with the column. Connect is primary when a cookie is entered, no campaign form is open and no run owns the primary action. An unchanged campaign form blocks Save details with No changes to save. Settings leaves its header context empty; campaign counts stay on their list heading. The rail shows Connected · not remembered when it fits, or Connected with the warning icon and colour; its hover text and accessible name keep the full state. The import form keeps the campaign name in its field and omits the remote identifier, repeated heading and open row's Add action. An API key saved for another service keeps Save key visible with Paste a key first; removing that other key is a separate quiet action.

Recording settings clip at the tab rule, with `L` above each section. Visible field labels are `Service`, `Address`, `Model` and `API key`; accessible names keep the service context. A remove-key button names its slot and service, for example Remove speech key (OpenAI). It sits below the key state at the column’s left edge. A saved key for the chosen service with no replacement typed hides Save key and its empty-field reason.

Recording retries say Continue processing. Process again is reserved for replacing existing results; its one info message names the transcript and context sent to the chosen LLM, and says the audio is not sent again. Its confirmation has secondary Not now and red Replace the results. A live row puts Stop recording first, beside Process. A job shows one spinner and one Processing progress line. The session list omits the date while a recording or job runs. Not processed uses the neutral file icon. Transcript marker actions sit beside the marker in the text.

Settings and the rail use one account status function. The rail shortens Signed out of D&D Beyond to Signed out with the full words on hover. Settings shows the recovery step once. Campaign import opens directly under the chosen row with a new sibling folder filled in. Editing details hides the row's Edit details action and uses a plain Details heading. The removal dialog is centred. Reload from disk names the local reload action.

On the World page, readers show the relative file path with Edit in Obsidian and Show file on every note.
NPCs show Stat block and In below that line; locations show In. Each has a quiet Change picker.
The search takes keyboard focus. A saved choice changes only its key and offers Undo. An undo
checks the file before writing. Broken notes stay in a Cannot be read group and open the same
source editor as Campaign repair. Its draft stays in memory and warns before app close or a
campaign switch. A generator card names its table sources below the attributes.

## Run and encounter audit, iteration 3

A shared NPC initiative roll is one roll group. It has a quiet bracket and ordering
arrows. Only equal totals from different rolls show a tie warning. Setup names its
Undo action. The live header keeps the next participant visible when Undo is empty.
Sheet pages use one tab line, with later pages in More when space is short. At the
table, the rail uses its natural width; a wider desk width stays saved.

Recovery copies have unique names, including seconds and a suffix when needed.
The run closed last and a file changed elsewhere lead their row's second line.
History puts the outcome before the ended state and shows the outcome, end reason
and final round above the final roster.

The encounter and group searches include NPCs with a usable stat block. Add uses
the stat block and the existing display-name override. The saved encounter format
stays the same. Roster sides sit on the second line and change in Edit. A collapsed
change list uses one Changed link in both editors. Group menus distinguish Make
available to any encounter from Delete group, with Undo before saving.

Changing the run folder with saved runs shows how many files stay behind. Move runs
and save stages all files without replacing an existing file, saves the settings,
then removes unchanged originals. Save without moving keeps the files in the old
folder. Saving run settings is blocked while a run is open.

New entry, Create package and Copy to a package keep Use in this campaign on by default. The help says what selection will be saved. A package entry shows its package and Used here or Not used here; Use in this campaign saves the missing choice. Clean save rows show No changes at the left; blocked Save gives No changes to save only on hover and in its accessible description. Unsaved, Saved and errors keep their left status.

Every reader resolves `[[Name]]`, `[[Name#Heading]]` and `[[Name|text]]` against World notes and the entries the campaign can use. Duplicate names open a chooser. Missing names keep their words, with a dotted underline and Not found in this campaign on hover. Reference keys keep working. The shared resolver is `ContentIndex::resolve_link`; Markdown readers call `reader_markdown` through the current index.

A package manifest opens from About followed by its name. It is not an entry row. Creation forms show their destination path. Creating a package opens its first entry form. Folder suggestions include existing folders, and saving can create a new subfolder. Reader pictures are 64 px thumbnails beside the title and meta line. Run-in titles use bold text. Sheet tabs use Small as a second level.

An unavailable package keeps its last known display name in the campaign selection. Older selections without a name say A package that is no longer on this computer. The message counts affected campaign entries when the selected references are known, with Remove from this campaign and Find the folder… beside it. The folder picker checks the package identity before restoring its registration. Only Technical details shows its ID.

Package and campaign readers keep their owner status in both Read and Edit Markdown.
Their file line offers Show file; entries in a vault also offer Edit in Obsidian.
An encounter row whose creature was deleted keeps its last known name or says Missing creature. Its warning line says, for example, Enemies · Not found. Its package entry was deleted. Edit opens a creature replacement search; Remove stays in the row menu. The NPC Add to run form labels its side picker SIDE and uses the roster's side words.
Folder is one editable field with a suggestion menu, followed by its help line.
New packages default to the parent of the last registered package.
A completed copy says where it went and keeps its path in hover text.
Share both copies a campaign dependency with its entry and reconnects the new references.
The warning action copies and reconnects one dependency. Undo restores the reference and removes the unchanged copy.
A compact NPC reader shows Fights as and In on one read-only line; both names open in Lookup.
Lookup keeps Back and its close icon on the title line. Clear appears beside search only when search has text. Result rows use Scan height, Body text for names, and Small muted text for metadata.

Library puts owner status on the file line so Back does not move the title. During a draft, Back opens the earlier entry in Lookup. Related entries follow the reader body. A standalone bold caption above a table is plain text; other reader links have an underline.

Keep Can use whole at the minimum list width. A shortened package name keeps its full value in hover text and its accessible name. In a narrow creature reader, each vital's label and value wrap together as one field. File links share the same wrapping row as the owner status.

On window focus, check campaign notes and registered package folders. Reload changed catalogues and show the disk comparison if a draft's file changed. Package About reports used and total entries, offers Use every entry with Undo, and omits the entry breadcrumb and Engine support. Removing an unavailable package offers Undo and keeps links to affected encounters.

Creation and copy fields share one right edge. New package has a Folder field with Browse beside it and a full destination path below it. Creature templates use inline stat values, AC 10 and HP 1, and select max_hp on opening. A renamed copy changes its first H1 only when it matches the source name.

## Run and encounter audit, iteration 4

The run dock and footer use the roster column's fixed width. A panel never covers
their words. The dock fits whole words at every width and keeps the last word or
number that distinguishes a participant. Its hover text and accessible name keep
the full name. An off Absent toggle uses muted text and no fill.

Setup keeps roster rows in place. Once every present row has initiative, a muted ordinal follows each initiative field,
updates after each total or tie ruling, and the ready line names who goes first.
The compact fight menu reads Fight and uses the tab underline. Its panel title
names the open fight tab.

Every change field shows its printed value and result, including changes added by
hand. Editors, run cards and fault details share the same preview. Done closes a
change form and returns to its member row. An empty group blocks Save group with
Add a creature first. A tied arrival states who it tied with and offers Move before
as one step with Undo. Member summaries wrap between members, never inside a name.

## Run and recording audit, iteration 5

A creature made up during a run shows current and maximum HP and the typed initiative
as a total. It has no printed values. Every setup row keeps its first condition; a count
stands for further conditions when space is short. Compact live rows put sync last,
after an S gap, separate from conditions. The dock ends a cut name with an ellipsis and
drops a trailing joining word, while keeping a copy's final number or letter.

Every tied arrival offers Move before with the target's name. A shared roll is one block;
the arrival moves before its first member. The message says acts next round when that
place has passed. Tie details stay in Arrivals and the log, outside the Undo label.
An open run with other unfinished copies names its copy beside Saved and in the rail hover.
Attack and damage changes sit directly below the affected action, with an XS gap above
and an L gap below. Encounter and group change forms use the run's condition grid.

Recording and processing keep one linked rail slot above account status. A recording row
owns its failure, stopped or stalled message while it is open; other pages keep the banner.
A live row owns Stop recording on the Recordings tab; other tabs keep the header button.
A row that offers Record again also owns the recording action on that tab.
The recordings list and its events keep LIST_MEASURE; result readers use the full pane.
A stopped row keeps
Record again beside Process, so recovery stays one press. A processed row puts
Process again… in its menu above Delete…, with the same inline confirmation.
Keep the recording is secondary. The consent step hides the header Record action.

A quiet Action that starts a line uses line_start to align its text with the line above;
its target extends into the gutter. A message action below its message uses the same rule.
A draft comparison offers Next difference only when at least two lines differ.

## Session and preparation audit, iteration 5

Record remains available during a note conflict because it writes its own file.
A missing note says Notes not saved with the same error status in its header and the rail.
The rail shortens Notes changed on disk to Changed on disk and keeps the full name on hover and for accessibility.
Built-in templates use the styled source face without an input well, at full column height.
The session Number field is five L units wide.

Encounter drafts keep Save or discard to switch campaign on their save row.
A blocked Start fight shows Close the open run first beside it. Clean group saves show No changes at the left. Save group keeps its clean reason on hover.
NPC search results keep CR, HP and AC in fixed columns and put NPC · as Guard after AC.
An offline Party now line offers Connect to sync. The first-sheet-read help keeps its Settings link inside the Small sentence. Initiative tie arrows have one fixed slot
before the ordinal and use the standard setup row height.

## Session and preparation audit, iteration 6

Read omits a fight-only plan section when the plan stands beside the note. An otherwise
empty note shows Nothing written yet. with a Write link. The tab layout keeps the count.
A moved run has a confirmation and Undo move in the message slot until the next note or plan edit.
A folder move reports the file counts and destination, with no separate Saved label.
A Day's start time is the Kind field's help line.

Sheet rows without a second line use one Read row. Correct HP owns the HP help;
Correct initiative says This row moves. The current turn stays. Correction field labels
share one column. Signed-out sheet conflicts keep Show file as their only action.
An open Ready run says Go back to the run. Paused rows include their saved date and time.
NPC roster rows put as Guard last. Group frames choose where Add goes; the search shows
Adding to as read-only text. Members and XP share one heading line.

The capture suite rejects consecutive steps with identical pixels at the same size.
An intended repeat needs expect_same with a reason on its second step.

## Run and recording audit, iteration 7

Paused rows give text the space left by their measured actions. Their saved time follows
the turn without a full stop. Recovery copies keep Edit when their encounter exists.
Readers hide the untouched Personal notes help; the editor keeps it.

Sync-choice Undo names the character, or the count of characters. Ended-run sync blocks
have an L gap between characters. An ended sign-in uses muted recovery text; a failed
Connect keeps its error state. A warned result says Processed in the rail, with its full
warning on hover and in its accessible name. A stalled row omits its session name.

Processing destinations show beside Process and Continue processing, and in the
Process again confirmation. A saved key field invites a replacement key.


## Library and preparation audit, iteration 9

Library keeps its validation count in the page header. Its menu opens the messages.
Repair errors use file line numbers and put the caret on the line to fix. A repaired
note reloads campaign use at once. A later disk change clears Undo discard.
Draft reasons keep their reserved height and start at the editor's left edge.
New entry reveals its selected list row after the catalogue reloads. Hidden source
headings and comments add no reader space. Manage packages offers Clear only for
filled fields. Folder suggestions use the combo arrow and normal text colour.

Plan group summaries put XP first, then members, Changed and side. They measure
against their own action. Tabbed member and roster views omit the repeated count.
Sheet group headings use an L gap. Death-save counts use words; Correct HP has an
L gap after death saves. Missing encounter entries have a warning icon.
