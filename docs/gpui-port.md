# Port from egui to GPUI

This file is the shared plan for the agents that port Splat's UI from egui to GPUI. When the port
merges, its architecture notes move into `docs/development.md` and `docs/design-system.md`, and
this file is deleted.

## Goal

Splat runs on GPUI. Every page, dialog and flow that the egui app has works in the GPUI app, and
looks substantially the same or better. The visible words stay the same (`CONTEXT.md`,
`docs/user-guide.md`). At the end, no crate depends on `egui`, `eframe`, `egui_extras`,
`egui_kittest` or `egui-phosphor`.

The user accepts a smaller visual test harness than the egui one: headless GPUI screenshots of
each page's main states, plus interaction tests of the main flows.

## Versions

- `gpui = { package = "gpui-pre", version = "0.3.14" }` (a crates.io snapshot of Zed's GPUI).
- `gpui_platform = { package = "gpui-pre-platform", version = "0.3.14", features = ["wayland", "x11"] }`.
  Start the app with `gpui_platform::application()`. Tests use the `test-support` feature,
  `gpui_platform::current_headless_renderer()` and `HeadlessAppContext::capture_screenshot`.
- `gpui-component = "0.11"` (built on `gpui-pre` 0.3.14) for the hard widgets: text input and
  editor, scroll bars, resizable panels, popovers and menus, virtual lists. Pages never call
  `gpui-component` directly; they call the Splat kit (below), which wraps it in Splat's tokens.
- Do not add `gpui` 0.2.x: its `xattr` dependency does not build against the locked `libc`.

## Build

- Every cargo command goes through the shared build-slot wrapper, from the worktree root:
  `/home/david/.t3/worktrees/splat/gpui-port/cargo.sh build`, `... cargo.sh test --lib gui::`,
  `... cargo.sh clippy --all-targets -- -D warnings`. It holds one of four machine-wide slots and
  runs cargo inside `devenv shell`. Do not call `cargo` directly, and do not raise the slot count.
- Keep the target in the worktree's own `target/`. Never put a target or a big log in `/tmp`
  (it is RAM). Put logs and screenshots under `artifacts/screenshots/` (ignored by git).
- `devenv.nix` provides the libraries GPUI needs (fontconfig, FreeType, xkbcommon, Wayland, xcb,
  Vulkan loader).

## Shape of the code

- The GPUI UI lives in the `splat` crate as `src/gui/`, so it can use the app's private state.
  - `src/gui/mod.rs`: `run()`, the window, the root view and the page rail.
  - `src/gui/theme.rs`: Splat's tokens (`src/design/tokens.rs`, `docs/design-system.md`) as GPUI
    values: colours, type sizes, spacing, radii, fonts.
  - `src/gui/kit/`: shared parts, one file per part. Pages build from these.
  - `src/gui/pages/<area>/`: one folder per page area. Each page agent owns its folder.
  - `src/gui/harness.rs` (`#[cfg(test)]`): headless screenshots and interaction helpers.
- Other workspace crates with egui views (`splat-trackers`, `splat-party`, `splat-content-ui`, …)
  keep their state and logic. Their GPUI view goes in `src/gui/pages/<area>/` and uses their
  public API; make an item `pub` when the view needs it.
- The root view owns the app state (the existing `Splat` state and page structs) and the GPUI-only
  state (input entities, scroll handles, focus handles). Event handlers use `cx.listener` and call
  the same state methods that the egui code calls. Keep logic in the state types; the view only
  reads state and sends actions.
- Background work (file watch, sync, recording, network) that egui polled each frame is polled by
  one timer task in the root view, which calls `cx.notify()` only when something changed.

## Waves

1. **Foundation** (one agent, this checkout): dependencies, `devenv.nix`, `src/gui/` with the
   window and rail, the theme, the core kit, a basic Markdown view, the screenshot harness, and a
   placeholder slot for each page. `cargo run` opens the GPUI app. The egui code still compiles and
   still runs its tests; nothing in it is deleted yet.
2. **Pages** (one agent per area, each in its own worktree branched from the foundation commit).
   Each agent ports its area completely, adds kit parts it needs as new files in `src/gui/kit/`,
   and adds screenshot scenarios and interaction tests for its area. It does not delete egui code.
3. **Cleanup**: switch every remaining caller, delete the egui UI code, the egui visual tests,
   the egui-only lints and audit tools, and the egui dependencies; move logic tests that lived in
   egui files; update the docs; final review.

## Page areas (wave 2)

| Area | egui source (reference) | GPUI folder |
| --- | --- | --- |
| Sessions | `src/sessions/`, `src/sessions.rs`, `src/sessions_ui/`, `src/session_records/` | `pages/sessions/` |
| Encounter run | `src/encounter_run.rs`, `src/encounter_run/`, `src/combat_sync.rs`, `src/rolls.rs`, `src/roll_table.rs` | `pages/run/` |
| Encounters | `src/encounters.rs`, `src/encounters/`, `src/difficulty.rs`, `src/creature_filter.rs`, `src/loot_editor.rs`, `src/encounters_ui/` | `pages/encounters/` |
| Library | `src/library.rs`, `src/library/`, `src/processing.rs`, `src/processing/`, `src/dndbeyond.rs`, `src/credentials.rs`, `src/copy_dialog.rs`, `src/library_page/` | `pages/library/` |
| World | `src/world.rs`, `src/world/`, `src/world_page/` | `pages/world/` |
| Timeline and trackers | `src/timeline.rs`, `src/timeline/`, `src/trackers/` | `pages/timeline/`, `pages/trackers/` |
| Campaigns, party, settings, lookup | `src/campaigns.rs`, `src/campaign_content.rs`, `src/campaign_content/`, `src/party*.rs`, `src/party_support/`, `src/party_page/`, `src/settings.rs`, `src/lookup.rs` | `pages/campaigns/`, `pages/party/`, `pages/settings/`, `pages/lookup/` |
| Reader | `src/markdown_view.rs`, `src/markdown_source.rs`, `src/content_ui/` | `kit/markdown/`, `pages/reader/` |

## Rules for every agent

- Read `AGENTS.md`, `CONTEXT.md`, `docs/design-system.md`, `docs/user-guide.md` and the egui
  source of your area before you write code. The egui code is the specification of behaviour:
  every control, keyboard path, message, Undo, confirm and empty state it has, the port has.
- Reference screenshots of the egui app at 1200×800 are in
  `artifacts/screenshots/egui-reference/` (named `<scenario>__<step>__1200x800.png`; the scenarios
  are in `src/visual_tests/scenarios*`). Render your GPUI page in the same states with the harness
  and compare them by eye. Write the differences you kept on purpose in your final message.
- UI text: same words as the egui app. Colours, sizes and type come from `gui::theme`, never
  literals in a page.
- Targets are at least 24 px. Keep the egui layout's spacing tokens.
- Do not edit another area's folder. Shared kit: add new files; change an existing kit file only
  when the change is small and additive, and say so in your final message.
- Commit on your branch in small steps with clear messages. Do not push and do not open a PR; the
  lead merges.
- Before you finish: `cargo.sh build`, `cargo.sh clippy --all-targets -- -D warnings`,
  `cargo.sh test --lib gui::` and the tests of your area pass. Report what you ported, what you
  did not, and the screenshots you made.

## Kit guide

Wave 1 built this; the wave-2 merges made each shared part one part (one footer data API, one
Link, one virtual list, one section of each kind, one portrait, one HP readout). Page agents read
this section and the code under `src/gui/`; each part's doc comment says when to use it. Before
you add a part, look for it in the table below: a page's own copy of a kit part is merged away.

### How the app runs

- `splat::run()` (`src/main.rs`) runs a command when there are arguments, takes the instance lock,
  then opens the GPUI app (`gui::run`). `SPLAT_EGUI=1 cargo run` opens the egui app instead, and
  `splat::run_egui()` stays for comparison until cleanup.
- `gui::SplatRoot` is the root view. `root.app` is the same `crate::Splat` the egui app draws (all
  its fields: `app.sessions`, `app.encounters`, `app.lookup`, …; private items of the crate root are
  visible to `src/gui/`). `root.pages.<area>` is your area's GPUI-only state. `root.bridge.ctx()` is
  a headless `egui::Context` for state methods that still take one for data (`refresh_content(ctx,
  …)`, cross-page requests left in `ctx.data`); a method that draws egui needs a GPUI view instead.
  `bridge::color(egui::Color32)` and `bridge::status(&design::Status)` turn a state method's egui
  colour or status into the kit's.
- Every frame, `SplatRoot::render` first runs `frame_logic` in egui's `update` order: the shared
  background polls (`Splat::poll_background`), `sync_party`, each area's `poll`, the tracker acts,
  `Splat::route` and `follow_page_change`, then `ui_state` is saved when it changed. Between frames
  one timer task (every 50 ms) runs `frame_logic` and asks for a frame only when background work
  asked the bridge for a repaint. So: set a `wants_*` flag or call a state method in your event
  handler and `cx.notify()`; the next frame routes it as egui did.
- `root.show_page(Page::X, cx)` changes page. `root.modal_open(cx)` is egui's `modal_open` (close
  prompt, copy dialog, an open kit menu): Ctrl+S and other page keys wait for it.
- Global keys (actions in `gui/mod.rs`): Ctrl+K Lookup, Ctrl+J notes, Ctrl+T Trackers, Ctrl+\ the
  session list, Escape (an open menu, then the notes panel, then Lookup). A page binds its own keys
  with `actions!` + `KeyBinding` in its `init` (`cx.bind_keys`) and `.on_action(cx.listener(..))`
  on its element, with its own `key_context`.

### How a page hooks in

Your folder `src/gui/pages/<area>/mod.rs` already has these, called by the shell; fill them in and
keep the names and signatures (`R` below is `crate::gui::SplatRoot`):

| Hook | Signature | Called |
| --- | --- | --- |
| `State` | `#[derive(Default)] pub struct State` | kept in `root.pages.<area>` |
| `init` | `fn(&mut R, &mut Window, &mut Context<R>)` | once, when the root is built |
| `poll` | same | each frame, before anything draws |
| `render` | `fn(&mut R, &mut Window, &mut Context<R>) -> AnyElement` | the page, inside the page's `M`/`S` padding |

Extra hooks the shell already calls: `run::render_strip` (above other pages while a run is open),
`party::render_bar` (`Option`, on the Session page; it already sets `sessions.party_hidden`),
`sessions::render_banner` (`Option`, at the window's bottom under the page, as egui's bottom
panel), `sessions::render_notes_panel` and
`lookup::render_panel` (the side panel, `PANEL_MIN_WIDTH` wide; Lookup calls
`trackers::render_view` while `lookup.on_trackers()`), `library::render_copy_dialog` (while
`app.copy_dialog` is open), `settings::render_account`, `render_recording`,
`render_add_campaign_form`, and `campaigns::render_packages`, `render_run_settings`, `render_link`.
The shell draws the Settings and Campaign headers and tabs, Settings → Touchpad, the no-campaign
state of pages that need a campaign, the rail and the close prompt. To add a hook, add it to your
folder and one call in `gui/mod.rs`, and say so in your final message.

Event handlers: `cx.listener(|root: &mut SplatRoot, event, window, cx| { …; cx.notify() })`. Kit
parts take `Fn(&ClickEvent, &mut Window, &mut App)`, which `cx.listener` makes. Keep logic in the
state types; the view reads state and calls the methods the egui code calls.

### The theme (`gui::theme`)

`colors()` (every token of `design::tokens`, plus `backdrop`, `shadow`, `selection`), spaces `XS S M L
XL XXL`, `ICON_SLOT`, `RADIUS_CONTROL`, `RADIUS_LAYER`, `HAIRLINE_WIDTH`, `FOCUS_WIDTH`, `MARK_WIDTH`,
`LINE_GAP`, `MIN_TARGET`, the rail, panel and measure widths (`READER_MEASURE`, `LIST_MEASURE`,
`FORM_WIDTH`, …), `Density::{Glance, Scan, Read}.control()/.row()`, `RunScale` (`RUN_COMPACT`,
`RUN_REGULAR`, `RUN_LARGE`, `fit`), and `Type` (the 19 roles). Set text with `.ty(Type::Body)` and
icons with `.icon_font(Type::Body)` (trait `TypeExt`). Fonts: IBM Plex Sans, Source Serif 4 (Text,
Subhead, SmText) and Phosphor, loaded by `theme::install`, which also sets gpui-component's theme
from the tokens. A test keeps the GPUI tokens equal to the egui tokens.

Icons: `gui::icons::*` has the design system's names (`icons::RUN`, `icons::WARN`, …) and
`icons::phosphor::*` the raw glyphs egui code names. Same code points as egui-phosphor.

### The kit (`gui::kit`)

Every part once, with its signature. Each part's doc comment says more. Ids are `impl Into<ElementId>`,
words `impl Into<SharedString>`, handlers `Fn(&ClickEvent, &mut Window, &mut App)` unless the row
says otherwise (`cx.listener` makes them).

| Part | Call | Use |
| --- | --- | --- |
| Text (`text`) | `text(Type, Hsla, words) -> Div`, `one_line(Type, Hsla, words)`, `icon(glyph, Type, Hsla)`, `label(words, count)`, `heading(Heading::{Group{title,count}, Section, Panel, Name}, first: bool)`, `rule()`, `gap(Pixels)`, `text::any(element)` | All words; `Label` is set in capitals for you |
| Button (`button`) | `Button::{primary, secondary, quiet, destructive}(id, label)` then `.icon(glyph)`, `.key("Ctrl+S")`, `.hint(words)`, `.blocked(Option<reason>)`, `.density(Density)`, `.selected(bool)`, `.plain()`, `.line_start()`, `.full_width()`, `.name(words)`, `.on_click(f)` | One primary per view; `line_start` for a quiet action that starts a line. Found by its words, its `.name`, or `id:<id>` (`button::element_name`) |
| Icon button | `IconButton::new(id, glyph, name).hint(words).toggled(bool).on_click(f)` | `⋯`, `×`, a view toggle; never an action that needs words |
| Link (`link`) | `Link::new(id, words)` or `link(id, words)`, then `.small()`, `.wrap()`, `.hint(words)`, `.name(words)`, `.on_click(f)` | A move to another place (a note, a file, a page), never an act that changes data. Accent words, underlined, the focus colour on hover; at least `XL` wide and 24 px high. One line by default; `.wrap()` for words of unknown length (a note's name, a path) |
| Text field (`field`) | `TextField::new(id, value).placeholder(..).name(..).masked().disabled(bool).width(px).on_change(\|&str, window, cx\|).on_enter(f).on_escape(f).on_blur(f)`, `.focus(bool)` / `.focus_selected(bool)`, `.text_type(Type)` | One line; `text_type(Type::Heading)` for a name that heads its editor (one `Scan` control high); two-way sync with your `String` (give the value each frame, take edits in `on_change`). `focus(true)` takes the keyboard with the caret after the words (a field that opens for typing); `focus_selected(true)` takes it with all the words selected (a rename a press opened). Give either for one frame. `on_escape` stops the press there: the window's Escape does not also run |
| Editor | `TextEditor::new(id, value).placeholder(..).name(..).rows(n).mono().text_type(Type).fill().focus(bool).handle(&EditorHandle).on_change(f)` | Several lines that wrap; `text_type(Type::Prose)` for a note |
| Editor handle | `EditorHandle::new(placeholder, window, cx)`, `.caret(cx)`, `.set_caret(byte, cx)`, `.words(cx)`, `.show_words(words, window, cx)`, `.focus(window, cx)`, `.focused(window, cx)` | An editor state the page keeps (in its `State`, by what it edits) to read and place the caret and to write at it |
| Field block | `field(caption, Option<help>, Option<error>, control) -> Div`, `help(words) -> Div` | Caption, control, help, error, then `L` |
| Value block (`value_block`) | `ValueBlock::new(HeadingRow::new(title).note(HeadingNote::..).action(Button), first).field(..).save_row(..).messages(Vec<Message>)` or `.end()` | One settings value: heading row, field, save row, its messages (typed stages: a field comes before its save row) |
| Fact (`fact`) | `fact::fact_line(label, values)`, `fact::words(label, values)`, `fact::tile_label(..)`, `fact::roll_tile(label, link)` | `Label: a · b` on its own line (no line ends at a dot); a stat tile whose value rolls |
| Rows (`list`) | `Row::new(id, name).height(px).grows().selected(bool).on_click(f).child(..)`, `two_line_row(id, title, detail, selected)` | List rows with hover, selection and focus; `grows` makes `height` a minimum, for words that wrap |
| Tabs (`tabs`) | `Segmented::new(id, options, chosen).on_select(\|&usize, window, cx\|)` | Page tabs and small exclusive choices |
| Checkbox (`checkbox`) | `Checkbox::new(id, label, checked).mixed(bool).name(words).blocked(..).on_click(f)`, `Switch::new(id, label, on).blocked(..).on_click(f)` | Yes/no settings; `mixed` draws egui's partly-chosen dash |
| Menu (`menu`) | `MenuButton::new(id, Button).item(MenuItem::new(label).icon(..).checked(bool).blocked(Option<reason>).destructive().on_click(f))` | A row's `⋯`, `New ▾` |
| Status menu | `StatusMenu::new(id, Status).item(MenuItem)` | A status that opens a menu under it (`2 notes need fixing`) |
| Drop-down | `Dropdown::new(id, name, options, Option<chosen>).placeholder(..).blocked(..).width(px).density(..).command(MenuItem).on_select(\|&usize, window, cx\|)` | Choose one value |
| Popover | `Popover::new(id, Button, content).on_status(..)` | Content in a raised layer under a button |
| Creature filter (`creature_filter`) | `FilterBar::new(id, &Filter, \|\| choices, cx).on_change(\|&Filter, window, cx\|)`, `FilterBar::menu_id(&id)` | egui's `creature_filter::filter_bar`: the Filter menu (a submenu per fact, the challenge's Lowest and Highest rows), a chip per chosen value, Clear filters. `choices` runs only while the menu is open; the caller keeps the filter where its egui page keeps it |
| Menu layer | `menu::layer_for(&id, content)` with `menu::open_popup(id, window, cx)`, `menu::is_open(&id, cx)`, `menu::any_open(cx)`, `menu::close_open_popup(window, cx)` | A menu that code opens (a row's right-click menu): draw the layer after its element while `is_open` |
| Section (`section`) | `Section::new(id, title, open).blocked(Option<reason>).on_toggle(\|&bool, window, cx\|).child(..)` | egui's `design::section`: a `Strong` title after a caret. With `on_toggle` the caller keeps whether it is open; without it the section keeps it under its id, starting at `open` |
| Quiet section | `QuietSection::new(id, title).default_open(bool).child(..)`, or caller-kept `.open(bool).on_toggle(\|&bool, window, cx\|)` | egui's `design::quiet_section`: a caret and muted `Small` words that open what most readers skip (Technical details). It keeps its own open state, unless the caller gives `open` and `on_toggle` (to build a costly body only while open) |
| Dialog (`dialog`) | `Dialog::new(id, title).child(..).destructive(Button).action(safe).action(commit).on_escape(f)` | Modal tasks only; draw it while open; it covers the window |
| Message (`outcome`) | `Message::new(Tone::{Info, Success, Warning, Error}, words).small().detail(element).action(element).destructive(element)` | Results, reasons, notices; safe action first, `L` before the destructive one |
| Footer messages (`messages`) | Build `Messages` as egui does with `let frame = PaneFrame::plain();`, then `messages.into_plain() -> Vec<PlainLine<A>>` (also `RowMessages::into_plain()`, `KeptLine`, `FaultMessage` and `Notice::into_plain() -> PlainLine<A>`; an owner that feeds both a footer and a save row writes to `impl plain::Sink<'f, A>`); `kit::messages::lines(id, lines, cx.listener(\|root, pressed: &kit::messages::Pressed<A>, window, cx\| { if let Some(press) = pressed.take() { … } }))`, `kit::messages::message(id, line, on_press, small)`, `kit::messages::tone(design::Tone)` | Every pane footer and save row's messages. One order (`footer_order`), Undo first with its icon, safe choices bordered, a `Lose` choice red with `L` before it, locks (`blocked` with the reason), hover words, each choice's help under it, the file line with `Edit in Obsidian` and `Show file`. A press is a `Press { id, ticket }` (`PlainLine::press_undo()`, `press_choice(at)`): a kept line's ticket reaches only the entry it showed; give it to the store's `…press…` method as egui does. A locked control gives no press. A view's own choice look (a banner) takes `plain::ChoiceForm` |
| Status (`status`) | `Status::new(glyph, word, color).icon_color(c)`, `status(&Status)`, `status_link(id, Status).hint(..).on_click(f)` | Icon + word + colour |
| HP (`hp`) | `hp::readout(hp, max, current: Type, max: Type)`, `hp::bar(hp, max, temp, width)`, `hp::word(hp, max) -> Option<(icon, word, Hsla)>`, `hp::temp(temp, Type)`, `hp::hp_color(HpState)`, `hp::current_color(HpState)` | HP at a glance (`design::hp_readout`, `hp_bar`, `hp_word`); colour is never the only sign |
| Portrait (`portrait`) | `portrait(Option<&Path>, name, size) -> Div` | A round picture, or the name's initial on a quiet disc |
| Empty state (`empty`) | `empty_state(glyph, title, body, Option<action>)` | An empty pane or page |
| Page header (`title`) | `page_header(title, Option<context>, Vec<AnyElement>)`, `page_header_with(title, Option<AnyElement>, Vec<AnyElement>)`, `title::actions(items)` | A page's first line, rule and `L`; `_with` for a context that is an element (a status) |
| Scroll (`scroll`) | `scroll_column(id, content)`, `scroll::scroll_column_with(id, &ScrollHandle, content)`, `scroll::sideways(id, content)` | A column that keeps its bar's lane |
| Virtual list (`virtual_list`) | `uniform_column(id, &UniformListScrollHandle, count, cx.processor(\|root, range, window, cx\| Vec<row>))`, `measured_column(id, &ListState, \|at, window, cx\| AnyElement)` | A long list that builds only the rows in view, in a column that keeps its bar's lane. `uniform_column` when every row has one height (Library entries, Timeline events); `measured_column` when rows have their own heights (World's headings and notes; `splice` the `ListState` when the rows change). Keep the handle or state in your page's GPUI state |
| Panes (`panes`) | `Pane::new(id).header(..).footer(element).fixed().flush().child(..)`, `Columns::new(id).column(width, min..max, pane).rest(pane).on_resized(\|&[Pixels], window, cx\|)` | Task columns with splitters; messages in the footer. `on_resized` gives each column's width when a drag ends: keep a dragged width with `root.set_column_width(id, w)` (`root.column_width(id)` reads it back; `ui_state` saves it as egui's `column_widths`) |
| Comparison (`comparison`) | `comparison::DraftComparison::new(id, draft, disk)` | A draft beside its file (`design::draft_comparison_styled`): how many lines differ and under which heading, `Next difference`, the two texts aligned line by line (`design::aligned`), each differing line marked |
| Markdown (`markdown`) | `markdown(id, source)` / `Markdown::new(id, source)` then `.document(uuid)`, `.compact()`, `.fields()`, `.skills()`, `.row_words(muted)`, `.full_width()`, `.wide_tables()`, `.jump_in(&ScrollHandle)`, `.on_link(\|&str, window, cx\|)`, `.on_target(\|&LinkTarget, window, cx\|)` | Every Markdown document; see Reader parts |
| Nav (`nav`) | `NavItem::new(..).detail(..).selected(bool).hint(..).name(..).on_click(f)`, `NavToggle::new(..).hint(..).on_click(f)` | The rail only |
| Tooltip (`tooltip`) | `.with_tooltip(words)` on an element with an id (`WithTooltip`), `tooltip::tip(words)` | Hints and blocked reasons |
| Target | `kit::target(element, name)` | Name your own pressable element for the harness |

A control's harness names: its words, its accessible name when `.name(..)` says more (`Button`,
`Link`), and `id:<element id>`. `shell.click("Undo remove Ash")` or `shell.click("id:…")`.

The kit gallery (`src/gui/gallery.rs`, tests only) shows every part on one page:
`artifacts/screenshots/gpui/gpui-kit__01-gallery__1200x800.png`, `__02-menu`, `__03-dialog`. A
test sets `root.gallery = Some(Gallery::default())` to show it in place of the page, or
`root.pages.reader.preview = Some(Rc::new(|root, window, cx| element))` to show one part of its
own (the reader's tests do: an entry view without its page).

Menus, drop-downs and popovers keep which one is open in a GPUI global (`kit::menu`): no open flag
in your state. `kit::menu::any_open(cx)`, `open_popup(id, window, cx)` and `close_open_popup`
exist for keys.

### Reader parts

The Reader area (`kit/markdown/`, `pages/reader/`) gives every page the egui reader's Markdown
and entry views. Their state stays where egui keeps it: pressed links, rolls and roll tables go
through the app's egui context (`kit::markdown::Host`, installed by `pages::reader::init`), and
the frame logic routes them as it routes egui's (`Splat::route`, `poll_background`). So a page
calls the same state methods after a reader as its egui code did (`markdown_view::take_lookup_link`,
`rolls::take_ask`, …) and needs no handler for them.

**Markdown** (`kit::Markdown`): headings, paragraphs, emphasis, code, block quotes, rules, lists
with each mark in its own column and nested levels (#949), task boxes, tables, roll tables (`<!--
splat:rolls … -->`: **Roll** and a typed roll under the table, the rolled row marked), inventory
and `Field`/`Value` tables, figures (local art in place), and links: a note link (`[[Name]]`,
resolved in the link index) goes to Lookup, a bold name that resolves to one entry goes to
Lookup, a missing link shows its warning and goes nowhere, a web link opens the browser, a dice
link asks for its roll (Shift advantage, Ctrl disadvantage). Short links get `XL` of width. Long
words break between graphemes. Parsed blocks and table columns are kept under the element id and
made again only when the words, mode, document, link index, art or width change.

| Call | Egui | Use |
| --- | --- | --- |
| `Markdown::new(id, source)` | `show_reader`, `show_document` | Reader prose at `READER_MEASURE`; `ProseCompact` below `READER_MEASURE - 60` |
| `.document(entry.id)` | `reader_document` | The note shown: its links resolve from it, a followed note link says where it came from |
| `.compact()` | (narrow `prose_type`) | `ProseCompact` at any width |
| `.fields()` | `show_compact` | `Stat` field rows; `Field`/`Value` tables become label/value rows |
| `.skills()` | `show_skills` | A stat block's Skills row: each bonus's target covers its skill's name |
| `.row_words(muted)` | `show_row_words` | A plan row's words: `Body` on `XL` lines, no space after |
| `.full_width()` / `.wide_tables()` | `with_wide_tables` | Fill the parent / tables take the parent while prose keeps the measure |
| `.jump_in(&handle)` | `want_heading` | The scroll column a typed jump (`markdown_view::want_heading(ctx, Some(h))`) scrolls |
| `.on_link(f)` / `.on_target(f)` | `take_note_link` | Replace the reader's routing (a note's `splat-note:…`, a URL, a bold name / any `LinkTarget`) |

**Entry view** (`pages::reader::entry`): the one reader of a content entry (egui `library::view`),
for the Library, Lookup, the run's side panel and campaign entries.

```rust
use crate::gui::pages::reader::entry::{self, EntryView, Source};
// Links: what related, feature and spell links open (`entry::index_links(index)` or `Rc::new(NoLinks)`).
EntryView::new(id, &name, Some(&entry), entry.body.clone(), Source::from(&origin), links)
    .empty_value_block(false)            // egui `show_with_related`'s flag
    .run(RunValues { .. })               // egui `show_run`: live vitals, printed values under them
    .related(element)                    // egui `show_with_related`'s closure
    .on_open(|index: usize, window, cx| …)   // a followed related/feature/spell link (egui's `Option<usize>`)
    .jump_in(&scroll);                   // typed jumps to section headings
entry::title_with_art(title, subtitle, Some(&entry), cx)  // name, subtitle and picture
```

It draws line warnings, a reading's summary, a creature's stat block (identity, vitals, the
ability table whose modifiers and saves roll, detail rows, rules prose with actions as tiles and
Spellcasting lists as spell tiles), or the common layout (item summary, spell or encounter facts,
source facts and prose, sections under headings), class traits and features, related entries,
then the closed end sections. Its parts are public for pages that draw pieces of an entry:
`entry::parts::{title, subtitle, heavy_rule, identity, section_heading, detail, stat_detail,
detail_row, facts, help, source_prose, item}` and `entry::tiles::tile_row` (action tiles from
`rolls::plain::roll_tiles`).

### Keys that an editor would take

Key bindings dispatch before `on_key_down` listeners, so a menu over a text editor (the note's
`/` menu) cannot take Up, Down, Enter or Escape from a `capture_key_down`. Use
`cx.intercept_keystrokes` (kept as a `Subscription` in the page's state) and
`cx.stop_propagation()` (`pages/sessions/slash.rs`).

### Screenshots and interaction tests

`src/gui/harness.rs` (`#[cfg(test)]`): `Shell` runs the whole app headless, with only Splat's
fonts (deterministic text), the real Vulkan renderer, at scale 1.

```rust
use crate::gui::harness::Shell;
use crate::visual_tests::fixture::Seed;

#[test]
fn library_browse() {
    let mut shell = Shell::new("library-browse", Seed::Campaign);       // 1200×800
    shell.click("Library");                    // a kit target by its words
    shell.click("id:library-new");             // or by its element id
    shell.type_text("Ash");                    // into the focused field
    shell.press("ctrl-s escape");              // keys, as GPUI writes them
    shell.capture("library-browse__01-list__1200x800");  // artifacts/screenshots/gpui/
    assert_eq!(shell.read(|root| root.app.page), crate::Page::Library);
}
```

- `Shell::new(name, Seed)` builds the same fixture as the egui scenario of that seed
  (`visual_tests::fixture::World`); its ids and dice come from `name`. `Shell::with_size(name, seed,
  size(px(w), px(h)))` for another window size (a window cannot be resized after it opens).
  `Shell::over(app, tempdir, size)` for a `Splat` you built.
- `update(|root, window, cx| …)` changes state as a test's setup; `read(|root| …)` reads it;
  `settle()` runs frames until idle; `wait(Duration)` advances the clock (timers, the poll);
  `bounds(name)` and `click_at(point)` for a place; `scroll(name, dy)` turns the wheel over a
  target (a negative `dy` shows what is below), for a row a scrolled layer hides.
- Name captures `<scenario>__<step>__<w>x<h>` like the egui references, and compare them with
  `artifacts/screenshots/egui-reference/` by eye.
- Put your tests in your folder (`pages/<area>/tests.rs`, `#[cfg(test)] mod tests;`). Run them with
  `cargo.sh test --lib gui::pages::<area>`.

### GPUI gotchas found in wave 1

- gpui turns on serde_json's `preserve_order` for any build that includes it (the app, `--workspace`
  tests), so a `serde_json::Map` keeps insertion order instead of sorting its keys. A build of one
  crate without gpui (`cargo test -p splat-character`) still sorts, so a test of order must run in
  a build with gpui. Code that iterates a map, compares key lists, hashes serialized bytes or writes
  a map to a file and needs an order must sort itself with `Value::sort_all_objects` or
  `Map::sort_keys`. Sorted now: `content::contracts::fingerprint`, D&D Beyond snapshots on read
  (`character::Source`), D&D Beyond pins (`character::pin`), importer notes (`rules::written`),
  campaign package keys, coverage metadata, combat sync writes and `execution-proof` output. Map
  equality is not affected.
- `ElementId::NamedChild` takes an `Arc<ElementId>`. Tuple ids exist for `(&str, u32/u64/usize…)`,
  not `(&str, &str)`; use `ElementId::Name(format!(..).into())`.
- `anchored()` starts at its parent's origin, not where it sits in a flex column: put it in a
  zero-height box after the trigger (as `kit::menu` does) to open under it.
- `Keystroke::parse` needs a `&'static str`; use `parse_with_os` for built strings.
- `window.focus(&handle, cx)` takes `cx`. `focus_handle(cx)` on an input state needs `gpui::Focusable`
  in scope.
- `window.use_keyed_state(id, cx, init)` keeps per-element state between frames (the kit's text
  fields and dialogs use it); its key joins the ids of the elements around it.
- The headless window draws at scale 2 unless told otherwise; the harness sets 1.
- Test windows do not resize: `window.resize` leaves the viewport as it was.
- `debug_selector` bounds are not reachable from `HeadlessAppContext`, hence `kit::target`.
- There is no letter spacing: `Type::Label`'s +0.6 tracking is not drawn.

### Known gaps after wave 1

- The touchpad speed is chosen and saved, but GPUI has no hook to scale scroll events; GPUI's own
  touchpad scrolling applies.
- The rail is not draggable and its width is not saved (136 px, 188 px from a 1400 px window).
- Dialogs ask for the keyboard when they open but do not keep Tab inside them; Escape reaches them
  through the root (`kit::dialog::escape_open_dialog`) wherever the keyboard is.
- A kit menu may reach past its pane's edge (egui keeps it inside the pane, one lane clear).
- Reader (wave 2): reader text cannot be selected or copied, and links cannot be reached with Tab
  (GPUI's `InteractiveText` has neither); a dice link or tile has no right-click mode menu (Shift
  and Ctrl still pick advantage and disadvantage); a wide table's header does not stay at the
  top while its rows scroll; images do not open full size; a lone action tile keeps 208 px
  instead of the row's width; Markdown source (`markdown_source`: reader faces, folded
  `splat:` marks, section rules where a note is typed) is not ported, as the kit's
  `TextEditor` has one font size per editor (`text_type`).
- The entry view (`pages::reader::entry::EntryView`) is not yet drawn by the Library reader,
  Lookup or the run's side panel: they show the entry's Markdown (`kit::Markdown`) and their
  own vitals. Moving them onto `EntryView` changes what they show, so it is a page task.
- The legacy host freeze (`src/design/legacy_hosts.tsv`) has three GPUI rows (`footer-feed` in
  `pages/settings/mod.rs` `campaign_lines`, `account_lines`, `pages/party/sheet.rs`
  `sync_footer`): the GPUI view reads the same egui footer feeders. Its growth check passes
  only with their raise lines in `SPLAT_STILLNESS_RAISE`; the cleanup wave deletes this lint.
