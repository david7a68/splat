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

#[derive(Clone, Debug)]
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
    Undo,
    Redo,
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

/// The app's write from `before` to `after` as the reference takes it: the app gives the new
/// words and no range (a named limit of `write_under_history`), so its write is the smallest
/// change between the two texts, at the last place it can stand. `(at, removed, new words)`, in
/// characters. The reference's own code: it calls nothing of the editor's.
fn smallest_change(before: &str, after: &str) -> (usize, usize, String) {
    let (old, new): (Vec<char>, Vec<char>) = (before.chars().collect(), after.chars().collect());
    let head = old
        .iter()
        .zip(&new)
        .take_while(|(one, other)| one == other)
        .count();
    let tail = old
        .iter()
        .rev()
        .zip(new.iter().rev())
        .take_while(|(one, other)| one == other)
        .count()
        .min(old.len().min(new.len()) - head);
    (
        head,
        old.len() - head - tail,
        new[head..new.len() - tail].iter().collect(),
    )
}

/// What an Undo's text is checked for.
#[derive(Clone, Copy, PartialEq)]
enum Order {
    /// The reference's exact text: traces in which each of the DM's steps is one edit.
    Exact,
    /// The reference's characters, in any order: traces in which a step holds several edits.
    /// The editor's steps are whole texts, so equal letters can then hide where the DM typed,
    /// and a word the DM gets back can stand at another place among equal letters
    /// (`write_under_history`'s named limit). No character may be lost or added.
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
        match step {
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
                let (head, old, new) = smallest_change(&before, &after);
                if apart && old > 0 {
                    return ended(failures, before);
                }
                let ctx = harness.ctx.clone();
                layout::write_under_history(
                    &ctx,
                    FIELD.id(),
                    &mut harness.state_mut().draft,
                    &after,
                );
                harness.step();
                for model in &mut models {
                    model.foreign(head, old, &new);
                    model.open = None;
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
                    if !model.crossed && !apart {
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
    // One edit per step: the exact text. Several edits in a step: the characters (`Order`).
    for (one_edit, order) in [(true, Order::Exact), (false, Order::Characters)] {
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
    // The app adds words after the DM's typing: Undo takes the typing back and keeps them.
    assert_eq!(
        run("one", &[type_at(0, "X"), app(4, 0, " summary"), Step::Undo]),
        Ok("one summary".to_owned())
    );
    // PR 825-b: the app changes words before the DM's typing. Undo takes the typing back and
    // keeps the app's words (a step placed by the words after the change could not take this
    // one, and the history ended there: `APP1`).
    assert_eq!(
        run("x", &[type_at(1, "1"), app(0, 1, "APP"), Step::Undo]),
        Ok("APP".to_owned())
    );
    // An expected outcome, not a fault (egui's steps are whole texts): the DM deletes `a` with
    // no pause, the app writes `a`, the DM types `a`. The kept step took the app's write and is
    // `aa`, the same words as the text now, so Undo has nothing to go to. The same holds with no
    // app write: a letter deleted and typed again with no pause. No word is lost.
    let delete = Step::Delete {
        at: 0,
        len: 0,
        settle: false,
    };
    let typed = Step::Type {
        at: 0,
        words: "a".to_owned(),
        settle: false,
    };
    assert_eq!(
        run("a", &[delete, app(0, 0, "a"), typed, Step::Undo]),
        Ok("aa".to_owned())
    );
    // PR 825-a: a reload ends the history: Undo never brings back the words before it.
    let reload = Step::Reload {
        words: "OUT".to_owned(),
    };
    assert_eq!(
        run("ab", &[type_at(2, "1"), reload, Step::Undo]),
        Ok("OUT".to_owned())
    );
}

/// Astra's trace (the note, section 3, v3): `aa`; the DM deletes the second `a`; the app changes
/// the first `a` to `b`; Undo.
#[test]
fn astras_trace() {
    let delete = Step::Delete {
        at: 1,
        len: 0,
        settle: true,
    };
    let result = run("aa", &[delete, app(0, 1, "b"), Step::Undo]);
    // Issue #900 (F2): the DM's `a` comes back where it was, after the app's `b`. A step placed
    // by the words after the change took the change at its end, which gave `ab`.
    assert_eq!(result, Ok("ba".to_owned()));
    let mut model = Reference::set("aa", false);
    model.typed(1, 1, "");
    model.foreign(0, 1, "b");
    assert_eq!(model.text(1), "ba");
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

/// The app writes `APP ` at the start of the text, under the typing history.
fn app_writes_at_the_start(harness: &mut Harness<'_, Page>) {
    let after = format!("APP {}", shown(harness));
    let ctx = harness.ctx.clone();
    layout::write_under_history(&ctx, FIELD.id(), &mut harness.state_mut().draft, &after);
    harness.step();
}

/// Ctrl+Z, then Ctrl+Y: after each, the app's words and every older word are there.
fn undo_and_redo_keep(harness: &mut Harness<'_, Page>, stages: &mut Vec<String>) {
    for key_of in [egui::Key::Z, egui::Key::Y] {
        key(harness, key_of, egui::Modifiers::COMMAND);
        stages.push(shown(harness));
        let text = shown(harness);
        assert!(
            text.starts_with("APP ") && text.contains("one") && text.ends_with(" two"),
            "the app's write and the older words stay: {stages:?}"
        );
    }
}

/// An input method's composition and the app's write (the note, section 3, v3 point 2; review
/// of PR 910, P1 910-a): real egui composition events; the app writes before the composition's
/// place while it is open, or right after its commit. No character is lost, the DM's committed
/// words are never lost or moved, and the app's write is never left out, also after Ctrl+Z and
/// Ctrl+Y.
#[test]
fn an_app_write_at_a_composition_loses_no_word() {
    use egui::ImeEvent::{Commit, Disabled, Enabled, Preedit};
    let preedit = |words: &str| Preedit(words.to_owned());
    let commit = || Commit("か".to_owned());
    // Each trace: the events before the app's write, the events after it, and the text after
    // each of those. The write lands while the composition is open in all but the last.
    let traces: [(&str, Vec<egui::ImeEvent>, Vec<egui::ImeEvent>, Vec<&str>); 6] = [
        (
            "more open words, an empty Preedit, then the Commit (as a platform sends them)",
            vec![Enabled, preedit("k")],
            vec![preedit("ka"), preedit(""), commit(), Disabled],
            vec![
                "APP oneka two",
                "APP one two",
                "APP oneか two",
                "APP oneか two",
            ],
        ),
        (
            "a direct Commit (910-a)",
            vec![Enabled, preedit("k")],
            vec![commit()],
            vec!["APP oneか two"],
        ),
        (
            "a direct Commit, with no Enabled before the composition",
            vec![preedit("k")],
            vec![commit()],
            vec!["APP oneか two"],
        ),
        (
            "Disabled, then the Commit",
            vec![Enabled, preedit("k")],
            vec![Disabled, commit()],
            vec!["APP onek two", "APP oneか two"],
        ),
        (
            "Disabled, Enabled, then the Commit",
            vec![Enabled, preedit("k")],
            vec![Disabled, Enabled, commit()],
            vec!["APP onek two", "APP onek two", "APP oneか two"],
        ),
        (
            "the write right after the Commit",
            vec![Enabled, preedit("k"), preedit(""), commit(), Disabled],
            vec![],
            vec![],
        ),
    ];
    for (name, before, after, texts) in traces {
        let mut harness = page("one two");
        select(&harness, 3, 3);
        for event in before {
            ime(&mut harness, event);
        }
        let mut stages = vec![shown(&harness)];
        app_writes_at_the_start(&mut harness);
        stages.push(shown(&harness));
        assert_eq!(
            stages[1],
            format!("APP {}", stages[0]),
            "{name}: the app's write stands before the words"
        );
        let after: Vec<String> = after
            .into_iter()
            .map(|event| {
                ime(&mut harness, event);
                shown(&harness)
            })
            .collect();
        assert_eq!(
            after, texts,
            "{name}: the composition goes on in its own words after the app's write"
        );
        stages.extend(after);
        assert_eq!(
            shown(&harness),
            "APP oneか two",
            "{name}: the DM's committed words stand where the DM put them: {stages:?}"
        );
        undo_and_redo_keep(&mut harness, &mut stages);
        assert_eq!(
            shown(&harness),
            "APP oneか two",
            "{name}: the committed words are there after Ctrl+Z and Ctrl+Y: {stages:?}"
        );
    }
}
