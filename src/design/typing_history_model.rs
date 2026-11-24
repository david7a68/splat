//! The typing history's model (ruling M29; the invariants note, section 3): a reference for what
//! Undo and Redo in a text field may give after writes that the field did not make, and the real
//! editor (`pane::editor_view` over a `TextDraft`, egui's own undoer, the field guard and
//! `layout::write_under_history`) run against it.
//!
//! **The reference.** A text is a row of characters. Each character knows which write put it in
//! and which writes took it out. A write is the DM's (a typed group: the edits egui keeps as one
//! Undo step) or foreign (the app's own write, with its exact range). "Undo of the newest `k`
//! groups" is the row read with those groups left out: their characters are not there, and the
//! characters they took out are back. A foreign write is never left out. A text set from outside
//! (a reload) starts a new row with no groups.
//!
//! **What is checked** (the note's H1 and H2):
//! - **H1.** Each text the real field shows after Undo or Redo is the reference's text for some
//!   `k`. The real field may offer fewer steps. It never shows another text: such a text has lost
//!   words of a foreign write, or has words back that a foreign write took out.
//! - **H2.** With no foreign write since the text was set, Undo goes to the reference's next
//!   text that differs from the one shown. egui's steps are whole texts, so a group that changed
//!   nothing in sum is no step, and a text equal to the newest step has nothing to undo.
//!
//! Time is the harness's own (a quarter second per frame): egui ends a typed group after one
//! second with no change, so a settled edit is one group, and edits with no pause between them
//! are one group. No test reads a clock. The traces come from a fixed seed.
use super::test_harness::*;
use super::{layout, pane, text_draft::TextDraft};
use egui_kittest::Harness;
use proptest::prelude::*;
use proptest::test_runner::{Config, RngAlgorithm, TestCaseError, TestRng, TestRunner};

const FIELD: layout::FieldId = crate::field_id!("typing_history_model").once(layout::Once::Example);

/// Who made a write.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Source {
    /// The DM's typed group, by its number.
    Group(usize),
    /// The app, or the text as an outside change set it.
    Foreign,
}

#[derive(Clone, Debug, PartialEq)]
struct Atom {
    ch: char,
    put: Source,
    taken: Vec<Source>,
}

/// The reference: the row of characters and the DM's groups since the last outside change.
#[derive(Clone, Debug, Default)]
struct Reference {
    atoms: Vec<Atom>,
    /// The groups, oldest first.
    groups: Vec<usize>,
    next: usize,
    /// The group that takes the next edit, while egui has not ended it.
    open: Option<usize>,
    /// How many of the newest groups the field has undone now.
    undone: usize,
    /// Where new characters stand when characters that are not shown lie at their place: after
    /// them (else before them). The editor may take either order (the note, section 3): its
    /// steps are whole texts, and a group of several edits shows only their sum, so equal
    /// letters can hide that the DM removed words exactly where the app then wrote. Both orders
    /// hold the same characters for every count of undone groups (`run` checks it), so no word
    /// is lost in either.
    after_gone: bool,
    /// A foreign write came since the text was set: the field's steps were changed under it, and
    /// H2 holds no more (the field may then offer fewer steps, as H1 allows).
    crossed: bool,
}

impl Reference {
    /// The reference in both orders of a tie.
    fn both(text: &str) -> Vec<Self> {
        vec![Self::set(text, false), Self::set(text, true)]
    }

    fn set(text: &str, after_gone: bool) -> Self {
        Self {
            after_gone,
            atoms: text
                .chars()
                .map(|ch| Atom {
                    ch,
                    put: Source::Foreign,
                    taken: Vec::new(),
                })
                .collect(),
            ..Self::default()
        }
    }

    fn shows(&self, atom: &Atom, out: &[usize]) -> bool {
        let live = |source: &Source| match source {
            Source::Foreign => true,
            Source::Group(group) => !out.contains(group),
        };
        live(&atom.put) && !atom.taken.iter().any(live)
    }

    /// The groups left out when the newest `k` are undone.
    fn out(&self, k: usize) -> &[usize] {
        &self.groups[self.groups.len() - k..]
    }

    /// The text with the newest `k` groups undone.
    fn text(&self, k: usize) -> String {
        let out = self.out(k);
        self.atoms
            .iter()
            .filter(|atom| self.shows(atom, out))
            .map(|atom| atom.ch)
            .collect()
    }

    /// The write `source` changes `len` characters at `at` of the text as the field shows it
    /// now to `words`. New characters stand between the shown characters around them, before or
    /// after the characters there that are not shown (`after_gone`).
    fn write(&mut self, source: Source, at: usize, len: usize, words: &str) {
        let out = self.out(self.undone).to_vec();
        let shown: Vec<usize> = (0..self.atoms.len())
            .filter(|&index| self.shows(&self.atoms[index], &out))
            .collect();
        for &index in &shown[at..at + len] {
            self.atoms[index].taken.push(source);
        }
        let place = if len > 0 {
            shown[at]
        } else if self.after_gone {
            shown.get(at).copied().unwrap_or(self.atoms.len())
        } else if at == 0 {
            0
        } else {
            shown[at - 1] + 1
        };
        for (offset, ch) in words.chars().enumerate() {
            self.atoms.insert(
                place + offset,
                Atom {
                    ch,
                    put: source,
                    taken: Vec::new(),
                },
            );
        }
    }

    /// The undone groups are gone for good: an edit after an Undo ends the Redo.
    fn drop_undone(&mut self) {
        let gone = self.out(self.undone).to_vec();
        self.atoms.retain(|atom| match atom.put {
            Source::Group(group) => !gone.contains(&group),
            Source::Foreign => true,
        });
        for atom in &mut self.atoms {
            atom.taken
                .retain(|source| !matches!(source, Source::Group(group) if gone.contains(group)));
        }
        let keep = self.groups.len() - self.undone;
        self.groups.truncate(keep);
        self.undone = 0;
    }

    fn typed(&mut self, at: usize, len: usize, words: &str) {
        self.drop_undone();
        let group = *self.open.get_or_insert_with(|| {
            self.next += 1;
            self.groups.push(self.next);
            self.next
        });
        self.write(Source::Group(group), at, len, words);
    }

    fn foreign(&mut self, at: usize, len: usize, words: &str) {
        self.write(Source::Foreign, at, len, words);
        self.crossed = true;
    }

    /// Every text an Undo or a Redo may show.
    fn reachable(&self) -> Vec<String> {
        (0..=self.groups.len()).map(|k| self.text(k)).collect()
    }
}

/// One step of a trace. A place is a number that the run folds into the text's length.
#[derive(Clone, Debug)]
enum Step {
    /// The DM types `words` at a place; `settle`: a pause follows, which ends the group.
    Type {
        at: u16,
        words: String,
        settle: bool,
    },
    /// The DM selects `len` characters at a place and presses Backspace.
    Delete {
        at: u16,
        len: u8,
        settle: bool,
    },
    /// The app writes `words` over `len` characters at a place, under the typing history.
    App {
        at: u16,
        len: u8,
        words: String,
    },
    /// The text comes again from outside (a reload).
    Reload {
        words: String,
    },
    /// The DM moves the caret and changes no word: by a key, or to a place (as a click does).
    Move(Caret),
    Undo,
    Redo,
}

/// A move of the caret.
#[derive(Clone, Copy, Debug)]
enum Caret {
    Left,
    Right,
    Home,
    End,
    To(u16),
}

struct Page {
    draft: TextDraft,
}

fn page(text: &str) -> Harness<'static, Page> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(400.0, 300.0))
        .with_step_dt(0.25)
        .themed_ui_state(
            |ui, page: &mut Page| {
                let face = pane::Face::Font(egui::TextStyle::Body.resolve(ui.style()));
                let _ = pane::editor_view(
                    ui,
                    pane::ViewHeight::Fills,
                    FIELD,
                    &mut page.draft,
                    face,
                    |edit| edit,
                );
            },
            Page {
                draft: TextDraft::new(text),
            },
        );
    harness.step();
    harness
        .ctx
        .memory_mut(|memory| memory.request_focus(FIELD.id()));
    settle(&mut harness);
    assert!(
        harness.ctx.memory(|memory| memory.has_focus(FIELD.id())),
        "the editor has the focus"
    );
    harness
}

/// More than egui's second with no change: a typed group ends.
fn settle(harness: &mut Harness<'_, Page>) {
    for _ in 0..6 {
        harness.step();
    }
}

fn select(harness: &Harness<'_, Page>, from: usize, to: usize) {
    let ctx = harness.ctx.clone();
    let mut state = egui::TextEdit::load_state(&ctx, FIELD.id()).unwrap_or_default();
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::two(
            egui::text::CCursor::new(from),
            egui::text::CCursor::new(to),
        )));
    egui::TextEdit::store_state(&ctx, FIELD.id(), state);
}

fn key(harness: &mut Harness<'_, Page>, key: egui::Key, modifiers: egui::Modifiers) {
    harness.input_mut().modifiers = modifiers;
    harness.input_mut().events.push(egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    });
    harness.step();
    harness.input_mut().modifiers = egui::Modifiers::NONE;
}

fn shown(harness: &Harness<'_, Page>) -> String {
    harness.state().draft.as_str().to_owned()
}

/// What an Undo's text is checked for.
#[derive(Clone, Copy, PartialEq)]
enum Order {
    /// The reference's exact text.
    Exact,
    /// The reference's characters, in any order (a fixed trace asks for no more).
    Characters,
}

/// A fault that a test puts into the run at an Undo or a Redo, to show that the run finds it
/// (the fixed mutant checks of this model).
#[derive(Clone, Copy)]
enum Fault {
    /// No fault at this key.
    Nothing,
    /// The key is not pressed.
    NoKey,
    /// After the key, the first two characters change places.
    Swap,
    /// After the key, the first character goes.
    Drop,
}

fn letters(text: &str) -> Vec<char> {
    let mut letters: Vec<char> = text.chars().collect();
    letters.sort_unstable();
    letters
}

/// Run `steps` from `start` in the real editor and in the reference. `Err` holds every failure,
/// one line each:
/// - a text with other CHARACTERS than the reference allows ends the run (H1);
/// - a text with the reference's characters in another ORDER is a failure under
///   [`Order::Exact`]; the run then goes on and checks characters, so a later loss shows too;
/// - an Undo or a Redo that does not go to the reference's next text (H2).
///
/// A run never ends as a pass after a failure. After an order difference the reference's places
/// are not the editor's, so the run ends at the next step that removes words by their place.
fn run_with(start: &str, steps: &[Step], order: Order, faults: &[Fault]) -> Result<String, String> {
    let mut harness = page(start);
    // The reference in both orders of a tie; one that the editor leaves is dropped.
    let mut models = Reference::both(start);
    let mut failures: Vec<String> = Vec::new();
    // The editor's text has the reference's characters in another order.
    let mut apart = false;
    // The DM moved the caret since the last change of the words: egui takes a caret at another
    // place for another state, so an Undo or a Redo may only move the caret (H2 is not checked
    // until the words change again; H1 always is).
    let mut caret_moved = false;
    let mut faults = faults.iter().copied();
    let ended = |failures: Vec<String>, text: String| {
        if failures.is_empty() {
            Ok(text)
        } else {
            Err(failures.join("\n"))
        }
    };
    for (index, step) in steps.iter().enumerate() {
        let before = shown(&harness);
        let chars = before.chars().count();
        let failure = |what: &str, harness: &Harness<'_, Page>, models: &[Reference]| {
            format!(
                "step {index} {step:?}: {what}; before: {before:?}; editor: {:?}; allowed: {:?}",
                shown(harness),
                models.iter().map(Reference::reachable).collect::<Vec<_>>(),
            )
        };
        // The editor's text is the reference's newest text (its characters, when apart).
        let follows = |harness: &Harness<'_, Page>, models: &[Reference], apart: bool| {
            let (now, model) = (shown(harness), models[0].text(models[0].undone));
            if apart {
                letters(&now) == letters(&model)
            } else {
                now == model
            }
        };
        if !matches!(step, Step::Move(_) | Step::Undo | Step::Redo) {
            caret_moved = false;
        }
        match step {
            Step::Move(caret) => {
                let pressed = match caret {
                    Caret::Left => Some(egui::Key::ArrowLeft),
                    Caret::Right => Some(egui::Key::ArrowRight),
                    Caret::Home => Some(egui::Key::Home),
                    Caret::End => Some(egui::Key::End),
                    Caret::To(at) => {
                        let at = usize::from(*at) % (chars + 1);
                        select(&harness, at, at);
                        harness.step();
                        None
                    }
                };
                if let Some(pressed) = pressed {
                    key(&mut harness, pressed, egui::Modifiers::NONE);
                }
                caret_moved = true;
                if !follows(&harness, &models, apart) {
                    failures.push(failure("a caret move changed the words", &harness, &models));
                    return ended(failures, shown(&harness));
                }
            }
            Step::Type {
                at,
                words,
                settle: pause,
            } => {
                let at = usize::from(*at) % (chars + 1);
                select(&harness, at, at);
                harness
                    .input_mut()
                    .events
                    .push(egui::Event::Text(words.clone()));
                harness.step();
                models
                    .iter_mut()
                    .for_each(|model| model.typed(at, 0, words));
                if !follows(&harness, &models, apart) {
                    failures.push(failure("typing gave another text", &harness, &models));
                    return ended(failures, shown(&harness));
                }
                if *pause {
                    settle(&mut harness);
                    models.iter_mut().for_each(|model| model.open = None);
                }
            }
            Step::Delete {
                at,
                len,
                settle: pause,
            } => {
                if chars == 0 {
                    continue;
                }
                if apart {
                    return ended(failures, before);
                }
                let at = usize::from(*at) % chars;
                let len = 1 + usize::from(*len) % (chars - at);
                select(&harness, at, at + len);
                key(&mut harness, egui::Key::Backspace, egui::Modifiers::NONE);
                models.iter_mut().for_each(|model| model.typed(at, len, ""));
                if !follows(&harness, &models, apart) {
                    failures.push(failure("a delete gave another text", &harness, &models));
                    return ended(failures, shown(&harness));
                }
                if *pause {
                    settle(&mut harness);
                    models.iter_mut().for_each(|model| model.open = None);
                }
            }
            Step::App { at, len, words } => {
                let at = usize::from(*at) % (chars + 1);
                let len = usize::from(*len) % (chars - at + 1);
                let after: String = before
                    .chars()
                    .take(at)
                    .chain(words.chars())
                    .chain(before.chars().skip(at + len))
                    .collect();
                let ctx = harness.ctx.clone();
                layout::write_under_history(
                    &ctx,
                    FIELD.id(),
                    &mut harness.state_mut().draft,
                    &after,
                );
                harness.step();
                // The typing history ends at an app write (ruling M32): the reference starts
                // again from the written text, as at a reload. The same words change nothing.
                if after != before {
                    models = Reference::both(&after);
                    apart = false;
                }
                if !follows(&harness, &models, apart) {
                    failures.push(failure(
                        "the app's write gave another text",
                        &harness,
                        &models,
                    ));
                    return ended(failures, shown(&harness));
                }
            }
            Step::Reload { words } => {
                let _ = harness.state_mut().draft.replace(words.clone());
                harness.step();
                // Equal words are no change (the guard's one documented case): the DM's steps
                // stay, and they take nothing of the outside text away.
                if *words != before {
                    models = Reference::both(words);
                    apart = false;
                }
                if shown(&harness) != *words {
                    failures.push(failure("the reload gave another text", &harness, &models));
                    return ended(failures, shown(&harness));
                }
            }
            Step::Undo | Step::Redo => {
                let undo = matches!(step, Step::Undo);
                let fault = faults.next();
                if !matches!(fault, Some(Fault::NoKey)) {
                    let pressed = if undo { egui::Key::Z } else { egui::Key::Y };
                    key(&mut harness, pressed, egui::Modifiers::COMMAND);
                }
                if let Some(fault @ (Fault::Swap | Fault::Drop)) = fault {
                    let mut text: Vec<char> = shown(&harness).chars().collect();
                    if matches!(fault, Fault::Swap) {
                        text.swap(0, 1);
                    } else {
                        text.remove(0);
                    }
                    let _ = harness
                        .state_mut()
                        .draft
                        .replace(text.into_iter().collect::<String>());
                    harness.step();
                }
                let now = shown(&harness);
                // The two orders of a tie differ only in order: the same characters for every
                // count of undone groups, so neither loses a word.
                let all = |model: &Reference| -> Vec<Vec<char>> {
                    (0..=model.groups.len())
                        .map(|k| letters(&model.text(k)))
                        .collect()
                };
                if models.windows(2).any(|pair| all(&pair[0]) != all(&pair[1])) {
                    failures.push(failure(
                        "the orders of a tie hold other characters",
                        &harness,
                        &models,
                    ));
                    return ended(failures, now);
                }
                // H1: the text is the reference's for some count of undone groups, on the side
                // the key goes to, in one order of a tie at least.
                let found = |model: &Reference, exact: bool| {
                    let same = |k: &usize| {
                        let text = model.text(*k);
                        if exact {
                            text == now
                        } else {
                            letters(&text) == letters(&now)
                        }
                    };
                    if undo {
                        (model.undone..=model.groups.len()).find(same)
                    } else {
                        (0..=model.undone).rev().find(same)
                    }
                };
                let exact = !apart && models.iter().any(|model| found(model, true).is_some());
                if !exact {
                    if !models.iter().any(|model| found(model, false).is_some()) {
                        failures.push(failure(
                            "H1: a text the reference does not allow (a character lost or added)",
                            &harness,
                            &models,
                        ));
                        return ended(failures, now);
                    }
                    if !apart && order == Order::Exact {
                        failures.push(failure(
                            "H1: the reference's characters in another order",
                            &harness,
                            &models,
                        ));
                    }
                    apart = true;
                }
                models.retain(|model| found(model, exact).is_some());
                let mut next_text = true;
                for model in &mut models {
                    let k = found(model, exact).unwrap_or_default();
                    // H2: no foreign write since the text was set: Undo goes to the next text
                    // that differs, and Redo to the next one toward the newest (a key that does
                    // nothing does not pass). egui's steps are whole texts: a group that changed
                    // nothing in sum (a letter typed and deleted) is no step.
                    if !model.crossed && !apart && !caret_moved {
                        let here = model.text(model.undone);
                        let mut older = model.undone + 1..=model.groups.len();
                        let mut newer = (0..model.undone).rev();
                        let next = if undo {
                            older.find(|&k| model.text(k) != here)
                        } else {
                            newer.find(|&k| model.text(k) != here)
                        };
                        next_text &= next.map_or(here, |k| model.text(k)) == now;
                    }
                    // A key that changed nothing leaves an open group open (egui's undoer keeps
                    // it).
                    if k != model.undone {
                        model.open = None;
                    }
                    model.undone = k;
                }
                if !next_text {
                    failures.push(failure(
                        "H2: Undo or Redo did not go to the next text",
                        &harness,
                        &models,
                    ));
                }
                harness.step();
            }
        }
    }
    ended(failures, shown(&harness))
}

/// [`run_with`] for a trace of one edit per step, with no fault put in: the exact text.
fn run(start: &str, steps: &[Step]) -> Result<String, String> {
    run_with(start, steps, Order::Exact, &[])
}

fn words() -> impl Strategy<Value = String> {
    // Few letters, so equal words meet often; two bytes, four bytes and a line end.
    prop::collection::vec(
        prop::sample::select(vec!["a", "b", " ", "é", "🙂", "\n"]),
        1..3,
    )
    .prop_map(|parts| parts.concat())
}

/// Words the DM types as text: egui takes a line end only from the Enter key, so none here (a
/// line end comes into the text by the app's writes and by reloads).
fn typed_words() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(vec!["a", "b", " ", "é", "🙂"]), 1..3)
        .prop_map(|parts| parts.concat())
}

/// Generated steps. `one_edit`: a pause follows each of the DM's edits, so each of the editor's
/// steps is one edit.
fn steps(one_edit: bool) -> impl Strategy<Value = Vec<Step>> {
    let step = prop_oneof![
        4 => (any::<u16>(), typed_words(), any::<bool>())
            .prop_map(move |(at, words, settle)| Step::Type {
                at,
                words,
                settle: settle || one_edit,
            }),
        2 => (any::<u16>(), any::<u8>(), any::<bool>())
            .prop_map(move |(at, len, settle)| Step::Delete {
                at,
                len,
                settle: settle || one_edit,
            }),
        2 => (any::<u16>(), 0..3u8, words())
            .prop_map(|(at, len, words)| Step::App { at, len, words }),
        1 => words().prop_map(|words| Step::Reload { words }),
        2 => prop_oneof![
            Just(Caret::Left),
            Just(Caret::Right),
            Just(Caret::Home),
            Just(Caret::End),
            any::<u16>().prop_map(Caret::To),
        ]
        .prop_map(Step::Move),
        3 => Just(Step::Undo),
        1 => Just(Step::Redo),
    ];
    prop::collection::vec(step, 1..14)
}

fn runner(cases: u32) -> TestRunner {
    TestRunner::new_with_rng(
        Config {
            cases,
            failure_persistence: None,
            ..Config::default()
        },
        TestRng::from_seed(RngAlgorithm::ChaCha, &[32; 32]),
    )
}

#[test]
fn the_reference_undoes_the_dms_groups_and_keeps_a_foreign_write() {
    // Astra's trace (the note, section 3, v3): `aa`; the DM deletes the second `a`; the app
    // changes the first `a` to `b`. Undo gives `ba`: the DM's `a` is back, the app's `b` stays.
    let mut model = Reference::set("aa", false);
    model.typed(1, 1, "");
    model.open = None;
    model.foreign(0, 1, "b");
    assert_eq!(model.reachable(), ["b", "ba"]);
    // Typing after the app's words, then Undo of both groups, oldest last.
    let mut model = Reference::set("x", false);
    model.typed(1, 0, "1");
    model.open = None;
    model.foreign(0, 1, "APP");
    model.typed(4, 0, "2");
    model.open = None;
    assert_eq!(model.reachable(), ["APP12", "APP1", "APP"]);
}

#[test]
fn undo_and_redo_show_only_texts_the_reference_allows() {
    // One edit per step, and several edits in a step: the exact text in both.
    for (one_edit, order) in [(true, Order::Exact), (false, Order::Exact)] {
        let result = runner(1000).run(&(words(), steps(one_edit)), |(start, steps)| {
            run_with(&start, &steps, order, &[])
                .map(|_| ())
                .map_err(TestCaseError::fail)
        });
        if let Err(failed) = result {
            panic!("one edit per step: {one_edit}: {failed}");
        }
    }
}

/// Fixed mutant checks of the model itself (Sol 2-022): a fault put into the run must show.
#[test]
fn the_run_finds_a_fault_put_into_it() {
    // A Redo that does nothing fails the Redo side of H2. With its key, the same trace passes.
    let trace = [type_at(1, "b"), Step::Undo, Step::Redo];
    assert_eq!(run("a", &trace), Ok("ab".to_owned()));
    let failed = run_with("a", &trace, Order::Exact, &[Fault::Nothing, Fault::NoKey])
        .expect_err("a Redo that does nothing");
    assert!(
        failed.contains("step 2 Redo: H2: Undo or Redo did not go to the next text"),
        "{failed}"
    );
    // An order difference, then a lost character: the run reports both. After the first Undo
    // two letters change places; after the second Undo one letter goes.
    let failed = run_with(
        "ab",
        &[type_at(2, "c"), Step::Undo, type_at(2, "d"), Step::Undo],
        Order::Exact,
        &[Fault::Swap, Fault::Drop],
    )
    .expect_err("an order difference and a lost character");
    let lines: Vec<&str> = failed.lines().collect();
    assert_eq!(lines.len(), 2, "{failed}");
    assert!(
        lines[0].contains("step 1 Undo: H1: the reference's characters in another order"),
        "{failed}"
    );
    assert!(
        lines[1].contains("step 3 Undo: H1: a text the reference does not allow"),
        "{failed}"
    );
}

fn type_at(at: u16, words: &str) -> Step {
    Step::Type {
        at,
        words: words.to_owned(),
        settle: true,
    }
}

fn app(at: u16, len: u8, words: &str) -> Step {
    Step::App {
        at,
        len,
        words: words.to_owned(),
    }
}

/// Fixed traces: each historical fault's shape, by name (the random traces need not meet them).
#[test]
fn fixed_traces_keep_every_foreign_write() {
    // The history ends at an app write (ruling M32): Undo after it changes nothing, and the
    // app's words and the DM's typing both stay.
    // The app adds words after the DM's typing.
    assert_eq!(
        run("one", &[type_at(0, "X"), app(4, 0, " summary"), Step::Undo]),
        Ok("Xone summary".to_owned())
    );
    // PR 825-b's shape: the app changes words before the DM's typing.
    assert_eq!(
        run("x", &[type_at(1, "1"), app(0, 1, "APP"), Step::Undo]),
        Ok("APP1".to_owned())
    );
    // Undo after the write takes back only what the DM types after it.
    let trace = [
        type_at(1, "1"),
        app(0, 1, "APP"),
        type_at(4, "2"),
        Step::Undo,
        Step::Undo,
    ];
    assert_eq!(run("x", &trace), Ok("APP1".to_owned()));
    // PR 825-a: a reload ends the history: Undo never brings back the words before it.
    let reload = Step::Reload {
        words: "OUT".to_owned(),
    };
    assert_eq!(
        run("ab", &[type_at(2, "1"), reload, Step::Undo]),
        Ok("OUT".to_owned())
    );
}

/// Astra's trace (issue #900, F2): `aa`; the DM deletes the second `a`; the app changes the
/// first `a` to `b`; Undo. And the same with the caret moved after the delete (review of PR 910,
/// P1 910-c): by Left, by Home, and to a place.
///
/// The step `aa` cannot say which `a` the DM deleted, and no caret is evidence of it (the DM can
/// move the caret after the delete). The app's write lies at the letters in question, so the
/// step is not kept: Undo changes nothing. A step placed by the words after the change gave
/// `ab`; one placed by the caret gave `ba` here and `ab` after Left.
#[test]
fn astras_trace() {
    let delete = || Step::Delete {
        at: 1,
        len: 0,
        settle: true,
    };
    let at_once = || Step::Delete {
        at: 1,
        len: 0,
        settle: false,
    };
    // The repeated letter, with the caret unmoved, moved left, moved right or to the end, and
    // set to a place (Sol 2-027): Undo gives back only an edit that the texts prove (`ba`), or
    // the step is dropped and the words stay (`b`). Never `ab`.
    let mut rows = vec![(
        "the caret unmoved".to_owned(),
        run("aa", &[delete(), app(0, 1, "b"), Step::Undo]),
    )];
    for caret in [
        Caret::Left,
        Caret::Home,
        Caret::Right,
        Caret::End,
        Caret::To(0),
        Caret::To(1),
    ] {
        for delete in [delete(), at_once()] {
            let trace = [delete, Step::Move(caret), app(0, 1, "b"), Step::Undo];
            rows.push((format!("{trace:?}"), run("aa", &trace)));
        }
    }
    let wrong: Vec<_> = rows
        .iter()
        .filter(|(_, text)| !matches!(text.as_deref(), Ok("ba" | "b")))
        .collect();
    assert!(
        wrong.is_empty(),
        "Undo gives back a proved edit (`ba`) or keeps the words (`b`), never another text: \
         {wrong:#?}"
    );
    // Here the texts prove no place, so every row keeps the words.
    assert!(
        rows.iter().all(|(_, text)| text.as_deref() == Ok("b")),
        "the step `aa` cannot say which `a` was deleted: every row keeps the words: {rows:#?}"
    );
    // The reference's own answer is `ba`: a record of each typed edit would give it.
    let mut model = Reference::set("aa", false);
    model.typed(1, 1, "");
    model.foreign(0, 1, "b");
    assert_eq!(model.text(1), "ba");
    // With no equal letter at the delete the texts would prove the place (`ac`; delete `c`;
    // Home; the app changes `a` to `b`), but no step is kept under a write at all: `b`.
    let delete = Step::Delete {
        at: 1,
        len: 0,
        settle: false,
    };
    assert_eq!(
        run(
            "ac",
            &[delete, Step::Move(Caret::Home), app(0, 1, "b"), Step::Undo]
        ),
        Ok("b".to_owned())
    );
}

/// Issue #900 (F3): `éa`; the DM types `é` after the first `é`, with no pause; the app changes
/// the DM's new `é` to `a`; Undo.
#[test]
fn a_replaced_word_leaves_an_older_equal_word() {
    let typed = Step::Type {
        at: 1,
        words: "é".to_owned(),
        settle: false,
    };
    // The DM's `é` is gone with the app's change, so Undo has nothing of the DM's to take
    // back: the older `é` and the app's `a` stay. A step placed by the words after the change
    // lost its own `é` (`aa`).
    let result = run("éa", &[typed, app(1, 1, "a"), Step::Undo]);
    assert_eq!(result, Ok("éaa".to_owned()));
}

fn ime(harness: &mut Harness<'_, Page>, event: egui::ImeEvent) {
    harness.input_mut().events.push(egui::Event::Ime(event));
    harness.step();
}

/// An input method's composition and the app's write: **a check that this write path changes
/// nothing there, not a check that the texts are right.** They are not: the app's write does
/// not move the field's caret or egui's own place for a composition, so the next composition
/// event acts at the old place (in the first row the committed words take the place of the
/// app's space, and the open `k` stays). That fault is on main, and it is not this change's to
/// fix (it has its own issue; three tries to fix it here with events put into the input each
/// lost text another way: reviews of PR 910).
///
/// One setup: `one two`, the caret after `one`, a composition open with `k`, then the app
/// writes `APP ` at the start. One row per way the composition ends; each row's texts after the
/// write, after every event, after Ctrl+Z and after Ctrl+Y are those of main's write path,
/// recorded by running this table on it (`MAIN_COMPOSITION`).
///
/// Ctrl+Z and Ctrl+Y give main's texts too: main ends the typing history at this write in every
/// row (its step has no place for the write), as this write path does at every write.
#[test]
fn an_app_write_at_a_composition_acts_as_on_main() {
    use egui::ImeEvent::{Commit, Disabled, Enabled, Preedit};
    let preedit = |words: &str| Preedit(words.to_owned());
    let commit = || Commit("か".to_owned());
    let open = || vec![Enabled, preedit("k")];
    // The events before the write, the events after it, and main's texts: after the write, after
    // each event, after Ctrl+Z, after Ctrl+Y.
    type Row = (
        &'static str,
        Vec<egui::ImeEvent>,
        Vec<egui::ImeEvent>,
        &'static [&'static str],
    );
    let rows: Vec<Row> = vec![
        (
            "a direct Commit",
            open(),
            vec![commit()],
            MAIN_COMPOSITION[0],
        ),
        (
            "an empty Preedit, the write, then the Commit",
            vec![Enabled, preedit("k"), preedit("")],
            vec![commit()],
            MAIN_COMPOSITION[1],
        ),
        (
            "a direct Commit, with no Enabled before the composition",
            vec![preedit("k")],
            vec![commit()],
            MAIN_COMPOSITION[2],
        ),
        (
            "more open words, an empty Preedit, then the Commit",
            open(),
            vec![preedit("ka"), preedit(""), commit(), Disabled],
            MAIN_COMPOSITION[3],
        ),
        (
            "ended with no Commit (Disabled)",
            open(),
            vec![Disabled],
            MAIN_COMPOSITION[4],
        ),
        (
            "Disabled, then the Commit",
            open(),
            vec![Disabled, commit()],
            MAIN_COMPOSITION[5],
        ),
        (
            "Disabled, Enabled, then the Commit",
            open(),
            vec![Disabled, Enabled, commit()],
            MAIN_COMPOSITION[6],
        ),
        (
            "the write right after the Commit",
            vec![Enabled, preedit("k"), preedit(""), commit(), Disabled],
            Vec::new(),
            MAIN_COMPOSITION[7],
        ),
    ];
    let mut differ = Vec::new();
    for (name, before, after, on_main) in rows {
        let mut harness = page("one two");
        select(&harness, 3, 3);
        for event in before {
            ime(&mut harness, event);
        }
        let write = format!("APP {}", shown(&harness));
        let ctx = harness.ctx.clone();
        layout::write_under_history(&ctx, FIELD.id(), &mut harness.state_mut().draft, &write);
        harness.step();
        let mut texts = vec![shown(&harness)];
        for event in after {
            ime(&mut harness, event);
            texts.push(shown(&harness));
        }
        for pressed in [egui::Key::Z, egui::Key::Y] {
            key(&mut harness, pressed, egui::Modifiers::COMMAND);
            texts.push(shown(&harness));
        }
        if texts != on_main {
            differ.push(format!("{name}: {texts:?}, on main {on_main:?}"));
        }
    }
    assert!(
        differ.is_empty(),
        "a composition and the app's write act as on main; the rows that differ:\n{}",
        differ.join("\n")
    );
}

/// Main's texts for `an_app_write_at_a_composition_acts_as_on_main`, row by row.
const MAIN_COMPOSITION: [&[&str]; 8] = [
    &[
        "APP onek two",
        "APPかonek two",
        "APP onek two",
        "APPかonek two",
    ],
    &[
        "APP one two",
        "APPか one two",
        "APP one two",
        "APPか one two",
    ],
    &[
        "APP onek two",
        "APPかonek two",
        "APP onek two",
        "APPかonek two",
    ],
    &[
        "APP onek two",
        "APPkaonek two",
        "APPonek two",
        "APPかonek two",
        "APPかonek two",
        "APP onek two",
        "APPかonek two",
    ],
    &[
        "APP onek two",
        "APP onek two",
        "APP onek two",
        "APP onek two",
    ],
    &[
        "APP onek two",
        "APP onek two",
        "APPかonek two",
        "APP onek two",
        "APPかonek two",
    ],
    &[
        "APP onek two",
        "APP onek two",
        "APP onek two",
        "APPかonek two",
        "APP onek two",
        "APPかonek two",
    ],
    &["APP oneか two", "APP oneか two", "APP oneか two"],
];

/// A write among typed words that equal letters hide (issue #900): `éa`; the DM types `é` after
/// the first `é`, then another `é` at the same place, with no pause; the app changes the second
/// typed `é` to `X`; Undo. The step holds two edits, and its caret names a wrong place for
/// them (the start). Read so, the write would stand at the older `é` and take it out (`Xa`).
/// The step is not kept: Undo changes nothing, and no letter is lost.
#[test]
fn a_write_among_typed_equal_letters_ends_the_history() {
    let typed = || Step::Type {
        at: 1,
        words: "é".to_owned(),
        settle: false,
    };
    let result = run_with(
        "éa",
        &[typed(), typed(), app(2, 1, "X"), Step::Undo],
        Order::Characters,
        &[],
    );
    assert_eq!(result, Ok("ééXa".to_owned()));
}

/// The Notes panel's sequence (`sessions::page`'s panel test), in letters: typing below, a
/// summary above it, Undo; then typing after the summary, a second summary below it, Undo.
#[test]
fn the_notes_panels_sequence() {
    let typed = |at: u16, words: &str| Step::Type {
        at,
        words: words.to_owned(),
        settle: false,
    };
    let start = "N.\n\n## T\n- b.\n";
    let result = run(
        start,
        &[
            typed(14, " Th."),
            app(2, 0, "\n\nS2."),
            Step::Undo,
            Step::Undo,
            typed(7, " Ni."),
            app(11, 0, "\n\nS4."),
            Step::Undo,
        ],
    );
    // Each summary ends the history: the typing before it stays at Ctrl+Z.
    assert_eq!(
        result,
        Ok("N.\n\nS2. Ni.\n\nS4.\n\n## T\n- b.\n Th.".to_owned())
    );
}

/// A write over a part of the typed words (the Notes panel's renamed link with a summary above
/// it: one change from the first changed letter to the last): the DM types a link at the end;
/// the app's write runs from the summary's place into the typed link. The step is not kept:
/// Undo changes nothing. Placed, it would take back half of the typing and leave a broken link.
#[test]
fn a_write_over_a_part_of_the_typed_words_ends_the_history() {
    let typed = Step::Type {
        at: 9,
        words: " [[Old p]].".to_owned(),
        settle: false,
    };
    let write = app(2, 13, "\n\nS3.\n\n## T\n [[New");
    let result = run("S.\n\n## T\n", &[typed, write, Step::Undo]);
    assert_eq!(
        result,
        Ok("S.\n\nS3.\n\n## T\n [[New p]].".to_owned()),
        "a write over a part of the typed words ends the history"
    );
}

/// Typed words at a place that equal letters hide, and a write that adds words there: `a`; the
/// DM types `a` after it; the app writes `b` between them; Undo. The typed `a` can be the first
/// or the second, and the two readings give other texts (`ba`, `ab`), so the step is not kept.
#[test]
fn typed_words_at_a_hidden_place_keep_no_uncertain_step() {
    let result = run("a", &[type_at(1, "a"), app(1, 0, "b"), Step::Undo]);
    assert_eq!(result, Ok("aba".to_owned()));
}

const FIELD_A: layout::FieldId =
    crate::field_id!("typing_history_model_a").once(layout::Once::Example);
const FIELD_B: layout::FieldId =
    crate::field_id!("typing_history_model_b").once(layout::Once::Example);

/// Two editors, one above the other.
struct Two {
    a: TextDraft,
    b: TextDraft,
    /// The focus goes to B in this frame, after A has drawn and before B draws.
    to_b: bool,
}

fn two_editors() -> Harness<'static, Two> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(400.0, 300.0))
        .with_step_dt(0.25)
        .themed_ui_state(
            |ui, two: &mut Two| {
                let face = pane::Face::Font(egui::TextStyle::Body.resolve(ui.style()));
                let height = pane::ViewHeight::AtMost(100.0);
                let _ =
                    pane::editor_view(ui, height, FIELD_A, &mut two.a, face.clone(), |edit| edit);
                if std::mem::take(&mut two.to_b) {
                    ui.memory_mut(|memory| memory.request_focus(FIELD_B.id()));
                }
                let height = pane::ViewHeight::AtMost(100.0);
                let _ = pane::editor_view(ui, height, FIELD_B, &mut two.b, face, |edit| edit);
            },
            Two {
                a: TextDraft::new("one two"),
                b: TextDraft::new("six ten"),
                to_b: false,
            },
        );
    harness.step();
    harness
        .ctx
        .memory_mut(|memory| memory.request_focus(FIELD_A.id()));
    for _ in 0..6 {
        harness.step();
    }
    harness
}

/// The selection of the field `id`: where it starts and its caret.
fn selection(ctx: &egui::Context, id: egui::Id) -> Option<(usize, usize)> {
    egui::TextEdit::load_state(ctx, id)
        .and_then(|state| state.cursor.char_range())
        .map(|range| (range.secondary.index, range.primary.index))
}

fn set_selection(ctx: &egui::Context, id: egui::Id, from: usize, to: usize) {
    let mut state = egui::TextEdit::load_state(ctx, id).unwrap_or_default();
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::two(
            egui::text::CCursor::new(from),
            egui::text::CCursor::new(to),
        )));
    egui::TextEdit::store_state(ctx, id, state);
}

/// Who owns the next edit after an app write (Sol 2-026; review of PR 910, P1 910-b): the app
/// writes into editor A; editor B has a selection; the focus goes to B before A's next frame, or
/// in that frame after A has drawn. A's write changes only A; B keeps its words and its
/// selection; the next typed letter is B's. (A fix that put an event for A into the frame's
/// input erased B's selected words.)
#[test]
fn an_app_write_to_one_editor_leaves_the_other_editors_selection() {
    for in_the_frame in [false, true] {
        let mut harness = two_editors();
        let ctx = harness.ctx.clone();
        set_selection(&ctx, FIELD_A.id(), 3, 3);
        set_selection(&ctx, FIELD_B.id(), 0, 3);
        harness.step();
        layout::write_under_history(
            &ctx,
            FIELD_A.id(),
            &mut harness.state_mut().a,
            "APP one two",
        );
        if in_the_frame {
            harness.state_mut().to_b = true;
        } else {
            ctx.memory_mut(|memory| memory.request_focus(FIELD_B.id()));
        }
        harness.step();
        let texts = |harness: &Harness<'_, Two>| {
            (
                harness.state().a.as_str().to_owned(),
                harness.state().b.as_str().to_owned(),
            )
        };
        assert_eq!(
            texts(&harness),
            ("APP one two".to_owned(), "six ten".to_owned()),
            "in the frame: {in_the_frame}: the write changes A only"
        );
        assert_eq!(
            selection(&ctx, FIELD_B.id()),
            Some((0, 3)),
            "in the frame: {in_the_frame}: B keeps its selection"
        );
        assert!(
            ctx.memory(|memory| memory.has_focus(FIELD_B.id())),
            "in the frame: {in_the_frame}: B has the focus"
        );
        harness
            .input_mut()
            .events
            .push(egui::Event::Text("X".to_owned()));
        harness.step();
        assert_eq!(
            texts(&harness),
            ("APP one two".to_owned(), "X ten".to_owned()),
            "in the frame: {in_the_frame}: the next typed letter is B's, over B's selection"
        );
    }
}

/// A selection the DM makes with the pointer in the frame right after an app write (Sol 2-026;
/// review of PR 910, P1 910-b): `one two` with the caret after `one`; the app writes `APP ` at
/// the start; in the next frame the DM presses Shift and clicks right of the text's end. The text
/// is whole, and the selection runs from the caret to the end. (egui applies a pointer selection
/// before a frame's events: an event put into that frame for the write deleted the new
/// selection.)
#[test]
fn a_pointer_selection_right_after_an_app_write_deletes_nothing() {
    let mut harness = page("one two");
    select(&harness, 3, 3);
    // The pointer comes to stand right of the text's end, on its line (egui knows what the
    // pointer is over from the frame before).
    let field = harness
        .ctx
        .read_response(FIELD.id())
        .expect("the editor drew")
        .interact_rect;
    let right_of_the_end = egui::pos2(field.right() - 4.0, field.top() + 8.0);
    harness
        .input_mut()
        .events
        .push(egui::Event::PointerMoved(right_of_the_end));
    harness.step();
    let ctx = harness.ctx.clone();
    layout::write_under_history(
        &ctx,
        FIELD.id(),
        &mut harness.state_mut().draft,
        "APP one two",
    );
    // No frame between the write and the click.
    harness.input_mut().modifiers = egui::Modifiers::SHIFT;
    harness.input_mut().events.push(egui::Event::PointerButton {
        pos: right_of_the_end,
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::SHIFT,
    });
    harness.step();
    harness.input_mut().events.push(egui::Event::PointerButton {
        pos: right_of_the_end,
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::SHIFT,
    });
    harness.step();
    harness.input_mut().modifiers = egui::Modifiers::NONE;
    harness.step();
    assert_eq!(shown(&harness), "APP one two", "the text is whole");
    assert_eq!(
        selection(&ctx, FIELD.id()),
        Some((3, 11)),
        "the selection runs from the caret to the text's end"
    );
}

/// A write beside typed words, among equal letters (review of PR 910, P1 910-c; found by the
/// generated traces): `ab`; the DM types `bb` at the end and a space after `a`, with no pause;
/// the app takes out the last `b`; Undo. The step `ab` and the text `a bbb` do not say which
/// `b` is the older one, so the app's write has no certain place in the step: the step is not
/// kept, and Undo changes nothing. Placed by the letters between the texts' common start and
/// end alone (` bb` typed after `a`), the write took out the step's own `b`: `a` for `ab`.
#[test]
fn a_write_beside_typed_equal_letters_ends_the_history() {
    let typed = |at: u16, words: &str| Step::Type {
        at,
        words: words.to_owned(),
        settle: false,
    };
    let result = run(
        "ab",
        &[typed(2, "bb"), typed(1, " "), app(4, 1, ""), Step::Undo],
    );
    assert_eq!(result, Ok("a bb".to_owned()));
}

/// A letter deleted and typed again in one step (review of PR 910, P1 910-d): `ab`; the DM
/// deletes `a` and types `ac` at the start, with no pause; the app changes the new `a` to `x`;
/// Undo. The texts `ab` and `acb` are equal at `a`, so no rule on whole texts can know that the
/// `a` is a new one: a step kept there lost the older `a` (`xb`). The history ends at the
/// write: Undo changes nothing. (A record of each typed edit would give `xab` or `axb`.)
#[test]
fn a_letter_typed_again_loses_nothing() {
    let delete = Step::Delete {
        at: 0,
        len: 0,
        settle: false,
    };
    let typed = Step::Type {
        at: 0,
        words: "ac".to_owned(),
        settle: false,
    };
    assert_eq!(
        run("ab", &[delete, typed, app(0, 1, "x"), Step::Undo]),
        Ok("xcb".to_owned())
    );
}
