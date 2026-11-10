//! `/` in a session note opens a short menu of commands. Each command ends with a link
//! where the `/` was typed, so the DM keeps writing and never leaves the note.
use crate::{
    design::{self, Action, Density, Tone, Type, colors, icons},
    lookup::{self, ContentIndex},
    npc_generator::{self, Generated},
    persistence::Campaign,
    world,
};
use eframe::egui;
use uuid::Uuid;

const MENU_WIDTH: f32 = 320.0;
const ROW: f32 = design::XXL;
/// The search shows this many results. A longer list belongs in Lookup.
const RESULTS: usize = 8;
/// More letters than any command has: the `/` was not a command.
const LONGEST_TYPED: usize = 16;
pub const SEARCH_NAME: &str = "Search to link";

#[derive(Clone, Copy, Debug, PartialEq)]
enum Command {
    GenerateNpc,
    Search,
}

const COMMANDS: [Command; 2] = [Command::GenerateNpc, Command::Search];

impl Command {
    fn word(self) -> &'static str {
        match self {
            Self::GenerateNpc => "Generate NPC",
            Self::Search => "Search",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Self::GenerateNpc => icons::DICE,
            Self::Search => icons::SEARCH,
        }
    }

    fn hint(self) -> &'static str {
        match self {
            Self::GenerateNpc => "Roll a new NPC and link it",
            Self::Search => "Find an entry or a note and link it",
        }
    }

    /// The letters typed after the `/` begin the command, or one of its words.
    fn matches(self, typed: &str) -> bool {
        let typed = typed.to_lowercase();
        let word = self.word().to_lowercase();
        word.starts_with(&typed) || word.split(' ').any(|part| part.starts_with(&typed))
    }
}

enum Stage {
    Menu {
        highlight: usize,
        /// How many letters follow the `/`, as the editor last showed them.
        typed: usize,
        /// The DM moved through the list. Enter then chooses even with nothing typed.
        moved: bool,
    },
    Search {
        query: String,
        highlight: usize,
        focus: bool,
    },
    Npc {
        npc: Option<Generated>,
        problems: Vec<npc_generator::TableProblem>,
        sources: Vec<String>,
        error: Option<String>,
        /// The first roll ran, so a failed roll is not tried again each frame.
        rolled: bool,
    },
}

struct Open {
    /// The `/`, as a character position in the note. The link goes here.
    at: usize,
    session: Option<Uuid>,
    stage: Stage,
    /// The popup was on screen last frame. If it is closed now, a click outside closed it.
    shown: bool,
}

#[derive(Default)]
struct Keys {
    up: bool,
    down: bool,
    choose: bool,
    escape: bool,
}

/// What the command needs from the page.
pub struct Place<'a> {
    pub session: Option<Uuid>,
    pub campaign: Option<&'a Campaign>,
    pub index: Option<&'a ContentIndex>,
    pub run_open: bool,
}

/// What the command changed.
#[derive(Default)]
pub struct Done {
    /// The note's words changed.
    pub edited: bool,
    /// A file was written that the index does not hold yet.
    pub created: bool,
}

#[derive(Default)]
pub struct Slash {
    open: Option<Open>,
    keys: Keys,
}

impl Slash {
    #[cfg(test)]
    fn is_open(&self) -> bool {
        self.open.is_some()
    }

    /// Before the note's editor shows. While the menu is open, the arrows choose a command,
    /// and Enter and Tab take it once the DM has typed a letter or moved. With only the `/`,
    /// Enter still ends the line. Escape closes the menu. While the NPC card is open, the
    /// note takes no keys: Enter saves, and nothing typed goes in before the link.
    pub fn take_keys(&mut self, ui: &egui::Ui, editor: Option<egui::Id>) {
        self.keys = Keys::default();
        let Some(open) = &mut self.open else {
            return;
        };
        // A click outside closed it while the note was away. Its keys go on as usual.
        if open.shown
            && editor.is_none_or(|id| !egui::Popup::is_id_open(ui.ctx(), id.with("slash")))
        {
            self.open = None;
            return;
        }
        let in_editor = editor.is_some_and(|id| ui.memory(|memory| memory.has_focus(id)));
        let keys = &mut self.keys;
        ui.input_mut(|i| {
            keys.escape = plain_key(i, egui::Key::Escape);
            match &mut open.stage {
                Stage::Menu { typed, moved, .. } if in_editor => {
                    keys.up = plain_key(i, egui::Key::ArrowUp);
                    keys.down = plain_key(i, egui::Key::ArrowDown);
                    *moved |= keys.up || keys.down;
                    if *typed > 0 || *moved {
                        keys.choose = plain_key(i, egui::Key::Enter) | plain_key(i, egui::Key::Tab);
                    }
                }
                Stage::Npc { .. } if in_editor => {
                    keys.choose = plain_key(i, egui::Key::Enter);
                    i.events.retain(|event| {
                        !matches!(
                            event,
                            egui::Event::Text(_)
                                | egui::Event::Paste(_)
                                | egui::Event::Cut
                                | egui::Event::Ime(_)
                                | egui::Event::Key { .. }
                        )
                    });
                }
                _ => {}
            }
        });
    }

    /// After the note's editor shows: open the menu on a new `/`, and show what is open.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        output: &egui::text_edit::TextEditOutput,
        body: &mut String,
        place: &Place<'_>,
    ) -> Done {
        let mut done = Done::default();
        let editor = output.response.id;
        // `take_keys` finds the popup by this id, too.
        let popup = editor.with("slash");
        let caret = output
            .state
            .cursor
            .char_range()
            .map(|range| range.primary.index);
        if self
            .open
            .as_ref()
            .is_some_and(|open| open.session != place.session)
        {
            self.open = None;
        }
        if self.open.is_none() {
            if output.response.changed()
                && output.response.has_focus()
                && typed_slash(ui)
                && let Some(caret) = caret
                && starts_command(body, caret)
            {
                self.open = Some(Open {
                    at: caret - 1,
                    session: place.session,
                    stage: Stage::Menu {
                        highlight: 0,
                        typed: 0,
                        moved: false,
                    },
                    shown: false,
                });
            } else {
                return done;
            }
        }
        let keys = std::mem::take(&mut self.keys);
        let open = self.open.as_mut().expect("opened above");
        // A click outside closed it. What was typed stays as it is.
        if open.shown && !egui::Popup::is_id_open(ui.ctx(), popup) {
            self.open = None;
            return done;
        }
        if keys.escape {
            let at = open.at;
            let in_menu = matches!(open.stage, Stage::Menu { .. });
            self.close(ui, editor, (!in_menu).then_some(at));
            return done;
        }
        let anchor = output
            .galley
            .pos_from_cursor(egui::text::CCursor::new(open.at))
            .translate(output.galley_pos.to_vec2());
        let words = open.at + 1;
        let mut result = Outcome::Stay;
        match &mut open.stage {
            Stage::Menu {
                highlight,
                typed: typed_count,
                ..
            } => {
                let typed = caret
                    .filter(|caret| *caret >= words && body.chars().nth(open.at) == Some('/'))
                    .map(|caret| {
                        body.chars()
                            .skip(words)
                            .take(caret - words)
                            .collect::<String>()
                    })
                    .filter(|typed| {
                        !typed.contains('\n') && typed.chars().count() <= LONGEST_TYPED
                    });
                let commands: Vec<Command> = typed
                    .as_deref()
                    .map(|typed| COMMANDS.into_iter().filter(|c| c.matches(typed)).collect())
                    .unwrap_or_default();
                if commands.is_empty() {
                    self.open = None;
                    egui::Popup::close_id(ui.ctx(), popup);
                    return done;
                }
                *typed_count = typed.as_deref().map_or(0, |typed| typed.chars().count());
                *highlight = (*highlight).min(commands.len() - 1);
                if keys.down {
                    *highlight = (*highlight + 1).min(commands.len() - 1);
                }
                if keys.up {
                    *highlight = highlight.saturating_sub(1);
                }
                let mut chosen = keys.choose.then_some(commands[*highlight]);
                let highlight = *highlight;
                popup_at(ui, popup, anchor, open.shown).show(|ui| {
                    for (row, command) in commands.iter().enumerate() {
                        if command_row(ui, *command, row == highlight).clicked() {
                            chosen = Some(*command);
                        }
                    }
                });
                if let Some(command) = chosen {
                    // The command's words go. Its link takes their place.
                    let end = caret.unwrap_or(words).max(words);
                    remove_chars(body, open.at, end);
                    done.edited = true;
                    set_caret(ui.ctx(), editor, open.at);
                    open.stage = match command {
                        Command::Search => Stage::Search {
                            query: String::new(),
                            highlight: 0,
                            focus: true,
                        },
                        Command::GenerateNpc => Stage::Npc {
                            npc: None,
                            problems: Vec::new(),
                            sources: Vec::new(),
                            error: None,
                            rolled: false,
                        },
                    };
                    // A click on the list took the keyboard. The card keeps it in the note,
                    // where it takes no keys until the card closes.
                    if command == Command::GenerateNpc {
                        ui.memory_mut(|memory| memory.request_focus(editor));
                    }
                }
            }
            Stage::Search {
                query,
                highlight,
                focus,
            } => {
                popup_at(ui, popup, anchor, open.shown).show(|ui| {
                    result = search(ui, place.index, query, highlight, focus);
                });
            }
            Stage::Npc {
                npc,
                problems,
                sources,
                error,
                rolled,
            } => {
                if !*rolled && let Some(index) = place.index {
                    *rolled = true;
                    roll(index, npc, problems, sources, error);
                }
                // The note keeps the keyboard. Tab and the arrows must not take it away.
                ui.memory_mut(|memory| memory.set_focus_lock_filter(editor, LOCKED));
                popup_at(ui, popup, anchor, open.shown).show(|ui| {
                    let Some(generated) = npc.as_ref() else {
                        match error.as_deref() {
                            Some(problem) => {
                                design::message(ui, Tone::Error, problem);
                                if Action::quiet("Close")
                                    .density(Density::Read)
                                    .show(ui)
                                    .clicked()
                                {
                                    result = Outcome::Cancel;
                                }
                            }
                            None => reading(ui),
                        }
                        return;
                    };
                    let (save, again, discard) =
                        lookup::generated_card(ui, generated, problems, sources, place.run_open);
                    let save = save || keys.choose;
                    if let Some(error) = error.as_deref() {
                        design::message(ui, Tone::Error, error);
                    }
                    if save {
                        match place
                            .campaign
                            .map(|campaign| world::create_generated(campaign, generated))
                        {
                            Some(Ok(_)) => {
                                done.created = true;
                                result = Outcome::Link(world::new_entry_link(
                                    place.index,
                                    &generated.name,
                                    "npc",
                                ));
                            }
                            Some(Err(problem)) => *error = Some(problem),
                            None => *error = Some("Open a campaign first.".to_owned()),
                        }
                    }
                    if discard {
                        result = Outcome::Cancel;
                    }
                    if again && let Some(index) = place.index {
                        roll(index, npc, problems, sources, error);
                    }
                });
            }
        }
        open.shown = true;
        let at = open.at;
        match result {
            Outcome::Stay => {}
            Outcome::Cancel => self.close(ui, editor, Some(at)),
            Outcome::Link(link) => {
                let at = at.min(body.chars().count());
                body.insert_str(byte_of(body, at), &link);
                done.edited = true;
                self.close(ui, editor, Some(at + link.chars().count()));
            }
        }
        done
    }

    /// Back to the note. The caret goes to `caret`, or stays where it was.
    fn close(&mut self, ui: &egui::Ui, editor: egui::Id, caret: Option<usize>) {
        self.open = None;
        egui::Popup::close_id(ui.ctx(), editor.with("slash"));
        if let Some(caret) = caret {
            set_caret(ui.ctx(), editor, caret);
        }
        ui.memory_mut(|memory| memory.request_focus(editor));
    }
}

enum Outcome {
    Stay,
    Cancel,
    Link(String),
}

/// The editor keeps the keyboard, and Escape still leaves it.
const LOCKED: egui::EventFilter = egui::EventFilter {
    tab: true,
    horizontal_arrows: true,
    vertical_arrows: true,
    escape: false,
};

/// The key, pressed with no Shift, Ctrl or Alt. `consume_key` would also take Shift+Enter.
fn plain_key(input: &mut egui::InputState, key: egui::Key) -> bool {
    input.modifiers.is_none() && input.consume_key(egui::Modifiers::NONE, key)
}

/// The text typed this frame ends with a `/`.
fn typed_slash(ui: &egui::Ui) -> bool {
    ui.input(|i| {
        i.events.iter().rev().find_map(|event| match event {
            egui::Event::Text(text) | egui::Event::Ime(egui::ImeEvent::Commit(text)) => {
                Some(text.ends_with('/'))
            }
            _ => None,
        })
    })
    .unwrap_or(false)
}

/// The `/` before the caret starts a line or a word. In `and/or` or `1/2` it is only a slash.
fn starts_command(body: &str, caret: usize) -> bool {
    let mut before = body
        .chars()
        .take(caret)
        .collect::<Vec<_>>()
        .into_iter()
        .rev();
    before.next() == Some('/') && before.next().is_none_or(char::is_whitespace)
}

fn byte_of(body: &str, at: usize) -> usize {
    body.char_indices()
        .nth(at)
        .map_or(body.len(), |(byte, _)| byte)
}

fn remove_chars(body: &mut String, from: usize, to: usize) {
    let (from, to) = (byte_of(body, from), byte_of(body, to));
    body.replace_range(from..to, "");
}

fn set_caret(ctx: &egui::Context, editor: egui::Id, at: usize) {
    let mut state = egui::TextEdit::load_state(ctx, editor).unwrap_or_default();
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::one(
            egui::text::CCursor::new(at),
        )));
    egui::TextEdit::store_state(ctx, editor, state);
}

/// Under the `/`, or above it when the note has no room below.
fn popup_at(ui: &egui::Ui, id: egui::Id, anchor: egui::Rect, shown: bool) -> egui::Popup<'static> {
    egui::Popup::new(id, ui.ctx().clone(), anchor, ui.layer_id())
        .open_memory((!shown).then_some(egui::SetOpenCommand::Bool(true)))
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .align(egui::RectAlign::BOTTOM_START)
        .align_alternatives(&[egui::RectAlign::TOP_START])
        .width(MENU_WIDTH)
}

fn command_row(ui: &mut egui::Ui, command: Command, selected: bool) -> egui::Response {
    ui.set_width(MENU_WIDTH);
    design::row(ui, ROW, selected, command.word(), |ui, rect| {
        let left = rect.left() + design::S;
        ui.painter().text(
            egui::pos2(left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            command.icon(),
            Type::Body.font(ui),
            colors().muted,
        );
        let word = ui.painter().layout_no_wrap(
            command.word().to_owned(),
            Type::Body.font(ui),
            colors().text,
        );
        let text_left = left + design::XL;
        let hint = design::one_line(
            ui,
            &format!(" · {}", command.hint()),
            Type::Small.font(ui),
            colors().muted,
            (rect.right() - text_left - word.size().x).max(0.0),
        );
        let word_width = word.size().x;
        ui.painter().galley(
            egui::pos2(text_left, rect.center().y - word.size().y / 2.0),
            word,
            colors().text,
        );
        ui.painter().galley(
            egui::pos2(
                text_left + word_width,
                rect.center().y - hint.size().y / 2.0,
            ),
            hint,
            colors().muted,
        );
    })
}

fn reading(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        design::spinner(ui);
        ui.label(
            Type::Body
                .rich(ui, "Reading content…")
                .color(colors().muted),
        );
    });
}

fn roll(
    index: &ContentIndex,
    npc: &mut Option<Generated>,
    problems: &mut Vec<npc_generator::TableProblem>,
    sources: &mut Vec<String>,
    error: &mut Option<String>,
) {
    let tables = index.npc_tables();
    match npc_generator::roll(&tables) {
        Ok(rolled) => {
            *npc = Some(rolled);
            *error = None;
        }
        Err(problem) => *error = Some(problem),
    }
    *problems = tables.problems;
    *sources = tables.sources;
}

/// A search field with the best results under it. Enter or a click links the chosen one.
fn search(
    ui: &mut egui::Ui,
    index: Option<&ContentIndex>,
    query: &mut String,
    highlight: &mut usize,
    focus: &mut bool,
) -> Outcome {
    let field_id = design::focus_id(SEARCH_NAME);
    let in_field = ui.memory(|memory| memory.has_focus(field_id));
    let (up, down, enter) = if in_field {
        ui.input_mut(|i| {
            // Tab stays in the field. It must not walk into the note or the page behind.
            plain_key(i, egui::Key::Tab);
            (
                plain_key(i, egui::Key::ArrowUp),
                plain_key(i, egui::Key::ArrowDown),
                plain_key(i, egui::Key::Enter),
            )
        })
    } else {
        (false, false, false)
    };
    let before = query.clone();
    let field = design::search_field(
        ui,
        SEARCH_NAME,
        query,
        "Search entries and notes…",
        MENU_WIDTH,
    );
    if std::mem::take(focus) {
        field.request_focus();
    }
    ui.memory_mut(|memory| memory.set_focus_lock_filter(field.id, LOCKED));
    if *query != before {
        *highlight = 0;
    }
    let Some(index) = index else {
        reading(ui);
        return Outcome::Stay;
    };
    let wanted = query.trim().to_lowercase();
    if wanted.is_empty() {
        ui.label(
            Type::Small
                .rich(ui, "Type a name, a heading or words from a note.")
                .color(colors().muted),
        );
        return Outcome::Stay;
    }
    let all = index.search(&wanted);
    let found: Vec<usize> = all.iter().copied().take(RESULTS).collect();
    if found.is_empty() {
        ui.label(
            Type::Body
                .rich(ui, "Nothing found. Check the spelling, or try fewer words.")
                .color(colors().muted),
        );
        return Outcome::Stay;
    }
    *highlight = (*highlight).min(found.len() - 1);
    if down {
        *highlight = (*highlight + 1).min(found.len() - 1);
    }
    if up {
        *highlight = highlight.saturating_sub(1);
    }
    let mut chosen = enter.then_some(found[*highlight]);
    ui.spacing_mut().item_spacing.y = 0.0;
    for (row, position) in found.iter().enumerate() {
        let item = &index.items[*position];
        let (title, detail) = (item.entry.name.clone(), item.result_detail());
        let full = format!("{title} · {detail}");
        let response = design::row(ui, ROW, row == *highlight, &full, |ui, rect| {
            let name =
                ui.painter()
                    .layout_no_wrap(title.clone(), Type::Body.font(ui), colors().text);
            let tail = design::one_line(
                ui,
                &format!(" · {detail}"),
                Type::Small.font(ui),
                colors().muted,
                (rect.width() - name.size().x).max(0.0),
            );
            let name_width = name.size().x;
            ui.painter().galley(
                egui::pos2(rect.left(), rect.center().y - name.size().y / 2.0),
                name,
                colors().text,
            );
            ui.painter().galley(
                egui::pos2(
                    rect.left() + name_width,
                    rect.center().y - tail.size().y / 2.0,
                ),
                tail,
                colors().muted,
            );
        })
        .on_hover_text(&full);
        if response.clicked() {
            chosen = Some(*position);
        }
    }
    ui.spacing_mut().item_spacing.y = design::XS;
    if all.len() > found.len() {
        ui.label(
            Type::Small
                .rich(
                    ui,
                    format!(
                        "The best {} of {}. Type more words to narrow them.",
                        found.len(),
                        all.len()
                    ),
                )
                .color(colors().muted),
        );
    }
    chosen.map_or(Outcome::Stay, |position| {
        Outcome::Link(index.link_to(position))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::{Harness, kittest::Queryable};

    const EDITOR: &str = "slash_test_note";

    struct Note {
        body: String,
        slash: Slash,
        index: ContentIndex,
        campaign: Campaign,
        created: bool,
        focus: bool,
        _dir: tempfile::TempDir,
    }

    fn note(body: &str) -> Harness<'static, Note> {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("Powers")).unwrap();
        std::fs::write(
            dir.path().join("Powers/Verori.md"),
            "# Verori\n\n## Culture\n",
        )
        .unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let harness = Harness::builder()
            .with_size(egui::vec2(800.0, 600.0))
            .build_ui_state(
                |ui, note: &mut Note| {
                    let id = egui::Id::new(EDITOR);
                    if std::mem::take(&mut note.focus) {
                        ui.memory_mut(|memory| memory.request_focus(id));
                    }
                    note.slash.take_keys(ui, Some(id));
                    let output = egui::TextEdit::multiline(&mut note.body).id(id).show(ui);
                    let place = Place {
                        session: None,
                        campaign: Some(&note.campaign),
                        index: Some(&note.index),
                        run_open: false,
                    };
                    note.created |= note.slash.show(ui, &output, &mut note.body, &place).created;
                },
                Note {
                    body: body.to_owned(),
                    slash: Slash::default(),
                    index,
                    campaign,
                    created: false,
                    focus: true,
                    _dir: dir,
                },
            );
        crate::design::theme::install(&harness.ctx);
        harness
    }

    fn type_text(harness: &mut Harness<'_, Note>, text: &str) {
        harness.event(egui::Event::Text(text.to_owned()));
        harness.run_steps(3);
    }

    fn press(harness: &mut Harness<'_, Note>, key: egui::Key) {
        harness.key_press(key);
        harness.run_steps(3);
    }

    #[test]
    fn a_slash_is_a_command_only_where_a_word_starts() {
        assert!(starts_command("/", 1));
        assert!(starts_command("Met /", 5));
        assert!(starts_command("Met\n/", 5));
        assert!(!starts_command("and/", 4));
        assert!(!starts_command("Met ", 4));
        assert!(Command::GenerateNpc.matches(""));
        assert!(Command::GenerateNpc.matches("npc"));
        assert!(Command::Search.matches("SEA"));
        assert!(!Command::Search.matches("sea "));
        assert!(!Command::GenerateNpc.matches("x"));
    }

    #[test]
    fn search_links_the_chosen_entry_where_the_slash_was() {
        let mut harness = note("Met ");
        harness.run_steps(2);
        type_text(&mut harness, "/");
        assert!(harness.state().slash.is_open());
        harness.get_by_label("Generate NPC");
        type_text(&mut harness, "se");
        press(&mut harness, egui::Key::Enter);
        // The command's words are gone, and the search has the keyboard.
        assert_eq!(harness.state().body, "Met ");
        type_text(&mut harness, "verori");
        press(&mut harness, egui::Key::Enter);
        assert_eq!(harness.state().body, "Met [[Verori]]");
        assert!(!harness.state().slash.is_open());
        // The keyboard is back in the note, after the link.
        type_text(&mut harness, " today");
        assert_eq!(harness.state().body, "Met [[Verori]] today");
    }

    #[test]
    fn search_links_a_heading_in_its_note() {
        let mut harness = note("");
        harness.run_steps(2);
        type_text(&mut harness, "/");
        type_text(&mut harness, "search");
        press(&mut harness, egui::Key::Enter);
        type_text(&mut harness, "culture");
        harness.get_by_label_contains("Culture").click();
        harness.run_steps(3);
        assert_eq!(harness.state().body, "[[Verori#Culture]]");
    }

    #[test]
    fn a_saved_npc_is_written_and_linked() {
        let mut harness = note("Hired ");
        harness.run_steps(2);
        type_text(&mut harness, "/");
        type_text(&mut harness, "gen");
        press(&mut harness, egui::Key::Enter);
        assert_eq!(harness.state().body, "Hired ");
        // While the card is open, what is typed does not go in before the link.
        type_text(&mut harness, "Bob");
        press(&mut harness, egui::Key::Tab);
        press(&mut harness, egui::Key::Backspace);
        assert_eq!(harness.state().body, "Hired ");
        harness.get_by_label("Save NPC").click();
        harness.run_steps(3);
        let state = harness.state();
        assert!(state.created);
        assert!(!state.slash.is_open());
        let files: Vec<_> = std::fs::read_dir(state.campaign.path.join("NPCs"))
            .unwrap()
            .map(|file| file.unwrap().path())
            .collect();
        assert_eq!(files.len(), 1);
        let stem = files[0].file_stem().unwrap().to_string_lossy();
        assert_eq!(state.body, format!("Hired [[{stem}]]"));
    }

    #[test]
    fn escape_and_other_words_leave_the_slash_as_typed() {
        let mut harness = note("");
        harness.run_steps(2);
        type_text(&mut harness, "/");
        press(&mut harness, egui::Key::Escape);
        assert!(!harness.state().slash.is_open());
        type_text(&mut harness, "x");
        assert_eq!(harness.state().body, "/x");
        // No command starts with these letters, so the menu goes and the words stay.
        type_text(&mut harness, " /");
        assert!(harness.state().slash.is_open());
        type_text(&mut harness, "zz");
        assert!(!harness.state().slash.is_open());
        assert_eq!(harness.state().body, "/x /zz");
        // In a word, a slash is only a slash.
        type_text(&mut harness, " and/");
        assert!(!harness.state().slash.is_open());
    }

    #[test]
    fn enter_saves_the_npc_and_the_note_goes_on_after_its_link() {
        let mut harness = note("");
        harness.run_steps(2);
        type_text(&mut harness, "/");
        press(&mut harness, egui::Key::ArrowDown);
        press(&mut harness, egui::Key::ArrowUp);
        // Moving through the list means Enter chooses, even with nothing typed.
        press(&mut harness, egui::Key::Enter);
        harness.get_by_label("Save NPC");
        press(&mut harness, egui::Key::Enter);
        assert!(harness.state().created);
        assert!(!harness.state().slash.is_open());
        type_text(&mut harness, " joined");
        assert!(
            harness.state().body.ends_with("]] joined"),
            "{}",
            harness.state().body
        );
    }

    #[test]
    fn enter_after_only_a_slash_ends_the_line_and_shift_enter_never_chooses() {
        let mut harness = note("");
        harness.run_steps(2);
        type_text(&mut harness, "/");
        press(&mut harness, egui::Key::Enter);
        assert_eq!(harness.state().body, "/\n");
        assert!(!harness.state().slash.is_open());
        type_text(&mut harness, "/");
        type_text(&mut harness, "se");
        harness.key_press_modifiers(egui::Modifiers::SHIFT, egui::Key::Enter);
        harness.run_steps(3);
        assert_eq!(harness.state().body, "/\n/se\n");
        assert!(!harness.state().slash.is_open());
    }

    #[test]
    fn tab_in_the_search_stays_there() {
        let mut harness = note("Met ");
        harness.run_steps(2);
        type_text(&mut harness, "/");
        type_text(&mut harness, "se");
        press(&mut harness, egui::Key::Enter);
        type_text(&mut harness, "ver");
        press(&mut harness, egui::Key::Tab);
        press(&mut harness, egui::Key::Tab);
        type_text(&mut harness, "ori");
        assert_eq!(harness.state().body, "Met ");
        press(&mut harness, egui::Key::Enter);
        assert_eq!(harness.state().body, "Met [[Verori]]");
    }

    #[test]
    fn escape_after_a_command_goes_back_without_a_link() {
        let mut harness = note("Met ");
        harness.run_steps(2);
        type_text(&mut harness, "/");
        type_text(&mut harness, "se");
        press(&mut harness, egui::Key::Enter);
        press(&mut harness, egui::Key::Escape);
        assert!(!harness.state().slash.is_open());
        type_text(&mut harness, "x");
        assert_eq!(harness.state().body, "Met x");
    }
}
