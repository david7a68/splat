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
    /// them (else before them). Both orders keep every write; the run allows either.
    after_gone: bool,
    /// A foreign write came since the text was set: the field's steps were changed under it, and
    /// H2 holds no more (the field may then offer fewer steps, as H1 allows).
    crossed: bool,
}

impl Reference {
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

/// Whether the editor maps a kept step by its operations (the note, section 3, v3; issue #900).
/// Off for today's code: `write_under_history` places the app's change in a kept step by the
/// words after it, and a step can hold those words at two places. Two faults follow, each a
/// named test below:
/// - **F2** (`astras_trace`): a word the DM gets back stands on the wrong side of the app's
///   words. While this is off, an Undo's text is compared by its characters, not their order.
/// - **F3** (`a_replaced_word_takes_an_older_equal_word`): when the app REPLACES words the DM
///   just typed and equal words stand directly before them, Undo takes the older words out.
///   While this is off, the generated app writes only insert.
///
/// The Undo fix turns this on and flips both named tests.
const BY_OPERATIONS: bool = false;

/// Whether the editor's text `now` is the reference's text `allowed` (`BY_OPERATIONS`).
fn same(now: &str, allowed: &str) -> bool {
    if BY_OPERATIONS {
        return now == allowed;
    }
    let sorted = |text: &str| {
        let mut chars: Vec<char> = text.chars().collect();
        chars.sort_unstable();
        chars
    };
    sorted(now) == sorted(allowed)
}

/// Run `steps` from `start` in the real editor and in the reference. `Err` names the first
/// step at which the editor shows a text the reference does not allow.
fn run(start: &str, steps: &[Step]) -> Result<String, String> {
    let mut harness = page(start);
    // The reference in both orders of a tie; one that the editor leaves is dropped.
    let mut models = vec![Reference::set(start, false), Reference::set(start, true)];
    for (index, step) in steps.iter().enumerate() {
        let before = shown(&harness);
        let chars = before.chars().count();
        let fail = |what: &str, harness: &Harness<'_, Page>, models: &[Reference]| {
            Err(format!(
                "step {index} {step:?}: {what}\n  before: {before:?}\n  editor: {:?}\n  allowed: {:?}",
                shown(harness),
                models.iter().map(Reference::reachable).collect::<Vec<_>>(),
            ))
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
                if shown(&harness) != models[0].text(0) {
                    return fail("typing gave another text", &harness, &models);
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
                let at = usize::from(*at) % chars;
                let len = 1 + usize::from(*len) % (chars - at);
                select(&harness, at, at + len);
                key(&mut harness, egui::Key::Backspace, egui::Modifiers::NONE);
                models.iter_mut().for_each(|model| model.typed(at, len, ""));
                if shown(&harness) != models[0].text(0) {
                    return fail("a delete gave another text", &harness, &models);
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
                for model in &mut models {
                    model.foreign(at, len, words);
                    model.open = None;
                }
                if shown(&harness) != models[0].text(models[0].undone) {
                    return fail("the app's write gave another text", &harness, &models);
                }
            }
            Step::Reload { words } => {
                let _ = harness.state_mut().draft.replace(words.clone());
                harness.step();
                // Equal words are no change (the guard's one documented case): the DM's steps
                // stay, and they take nothing of the outside text away.
                if *words != before {
                    models = vec![Reference::set(words, false), Reference::set(words, true)];
                }
                if shown(&harness) != *words {
                    return fail("the reload gave another text", &harness, &models);
                }
            }
            Step::Undo | Step::Redo => {
                let undo = matches!(step, Step::Undo);
                if undo {
                    key(&mut harness, egui::Key::Z, egui::Modifiers::COMMAND);
                } else {
                    key(&mut harness, egui::Key::Y, egui::Modifiers::COMMAND);
                }
                let now = shown(&harness);
                // H1: the text is the reference's for some count of undone groups, on the side
                // the key goes to, in one order of a tie at least.
                let found = |model: &Reference| {
                    if undo {
                        (model.undone..=model.groups.len()).find(|&k| same(&now, &model.text(k)))
                    } else {
                        (0..=model.undone)
                            .rev()
                            .find(|&k| same(&now, &model.text(k)))
                    }
                };
                if !models.iter().any(|model| found(model).is_some()) {
                    return fail("H1: a text the reference does not allow", &harness, &models);
                }
                models.retain(|model| found(model).is_some());
                let mut one_group = true;
                for model in &mut models {
                    let k = found(model).unwrap_or_default();
                    // H2: no foreign write since the text was set: Undo goes to the next text
                    // that differs. egui's steps are whole texts: a group that changed nothing
                    // in sum (a letter typed and deleted) is no step.
                    if undo && !model.crossed {
                        let here = model.text(model.undone);
                        let next = (model.undone + 1..=model.groups.len())
                            .map(|k| model.text(k))
                            .find(|text| *text != here)
                            .unwrap_or(here);
                        one_group &= next == now;
                    }
                    // A key that changed nothing leaves an open group open (egui's undoer keeps
                    // it).
                    if k != model.undone {
                        model.open = None;
                    }
                    model.undone = k;
                }
                if !one_group {
                    return fail("H2: Undo did not go to the next text", &harness, &models);
                }
                // The same characters in another order (`BY_OPERATIONS` is off): the reference's places
                // are not the editor's from here, so the trace ends.
                if models.iter().all(|model| model.text(model.undone) != now) {
                    return Ok(now);
                }
                // Only the orders of a tie that the editor took go on.
                models.retain(|model| model.text(model.undone) == now);
                harness.step();
            }
        }
    }
    Ok(shown(&harness))
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

fn steps() -> impl Strategy<Value = Vec<Step>> {
    let step = prop_oneof![
        4 => (any::<u16>(), typed_words(), any::<bool>())
            .prop_map(|(at, words, settle)| Step::Type { at, words, settle }),
        2 => (any::<u16>(), any::<u8>(), any::<bool>())
            .prop_map(|(at, len, settle)| Step::Delete { at, len, settle }),
        2 => (any::<u16>(), 0..3u8, words()).prop_map(|(at, len, words)| Step::App {
            at,
            // Today's code can lose a word under a replacing write (F3): inserts only.
            len: if BY_OPERATIONS { len } else { 0 },
            words,
        }),
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
    let result = runner(1500).run(&(words(), steps()), |(start, steps)| {
        run(&start, &steps).map(|_| ()).map_err(TestCaseError::fail)
    });
    if let Err(failed) = result {
        panic!("{failed}");
    }
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
    // PR 825-b: the app changes words that a kept step cannot take (the step has no `1` after
    // the changed word). The step is not kept: Undo changes nothing, and the app's words stay.
    assert_eq!(
        run("x", &[type_at(1, "1"), app(0, 1, "APP"), Step::Undo]),
        Ok("APP1".to_owned())
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
    // Finding F2: the reference gives `ba` (the DM's `a` comes back where it was, after the
    // app's `b`). Today the kept step `aa` takes the app's change at its end, which gives `ab`.
    // With `BY_OPERATIONS` on, this is `Ok("ba")`.
    assert_eq!(
        result,
        Ok(if BY_OPERATIONS { "ba" } else { "ab" }.to_owned())
    );
    let mut model = Reference::set("aa", false);
    model.typed(1, 1, "");
    model.foreign(0, 1, "b");
    assert_eq!(model.text(1), "ba");
}

/// Finding F3 (issue #900): `éa`; the DM types `é` after the first `é`, with no pause; the app
/// changes the DM's new `é` to `a`; Undo.
#[test]
fn a_replaced_word_takes_an_older_equal_word() {
    let typed = Step::Type {
        at: 1,
        words: "é".to_owned(),
        settle: false,
    };
    let result = run("éa", &[typed, app(1, 1, "a"), Step::Undo]);
    // The reference gives `éaa`: the DM's `é` is gone with the app's change, the older `é` and
    // the app's `a` stay.
    let mut model = Reference::set("éa", false);
    model.typed(1, 0, "é");
    model.foreign(1, 1, "a");
    assert_eq!(model.reachable(), ["éaa", "éaa"]);
    if BY_OPERATIONS {
        assert_eq!(result, Ok("éaa".to_owned()));
    } else {
        // Today the kept step `éa` loses its own `é`: the editor shows `aa`, which is no text
        // of the reference (H1).
        let failed = result.expect_err("today's code loses the older word");
        assert!(
            failed.contains("H1") && failed.contains("editor: \"aa\""),
            "{failed}"
        );
    }
}
