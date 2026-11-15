# Retained results: projection, capability and owner revisions (layout PR 2a)

Ruling L9 and attack finding 2 (`migration-32/layout/attack.md`). This note comes before the code. It
defines what a pane may keep of its results across frames, what it needs in the current frame to act on
them, and which revisions say that a kept copy is stale. PR 2b (the retained footer) builds on it. This PR
changes no footer renderer.

## 1. Two types: kept data, and a capability for this frame

Today a result reaches a footer only as `KeptLine<'f, A>` (`Shown::outcome`): it holds the entry's
`Ticket`, borrows this frame's `PaneFrame`, is not `Clone`, and does not compile when kept in app state.
That stays exactly as it is, with all its `compile_fail` tests.

New, beside it, in `design::results`:

- **`Projection<O, A, I>`: the retained presentation, as data.** One owner key's result: words, tone,
  the Undo's words and lock, choices, a refused attempt's words, and its place in the footer's order. It
  holds no `Ticket`, no closure, no borrow, and no order that a caller can read. It has no draw method and
  no act method. Its fields are private, and only `Retained::refresh` makes one, from a `&Results`. The
  owner keeps it (in a `Retained<O, A, I>` beside its store) across frames.
- **`Results::present(&self, frame: &'f PaneFrame, projection, key, inputs) -> Result<Option<KeptLine<'f,
  A>>, Mismatch>`: the capability.** It needs this frame's `PaneFrame` and a borrow of the live store. It
  checks the binding (section 3) and only then mints the entry's `Ticket`, from the store, into a
  frame-bound `KeptLine`. A press comes back with that ticket, and the store acts by it (`undo_by`), as
  today. Kept data can therefore show a line, but only the current store, in the current frame, can make
  that line act.

New `compile_fail` proofs: a `Projection` made by hand; a ticket read from a projection; a line from
`present` kept in app state (the existing proof, repeated for this path). Each has its compiling twin.

In this PR the `KeptLine` still goes to the unchanged `Messages` footer, which needs owned data: `present`
copies the retained words into the line each frame. That copy is as large as the shown lines, not the
content (the event list, the notes); it is counted (`handed` bytes) and named as PR 2b's to remove.

## 2. Owner revisions

`Results` publishes `revision() -> memo::Revision`. A store gets a new one at `new` (so `new` stops being
`const`; no caller is `const`). Revisions are unique in the run, and `Results` is not `Clone`, so a
revision also names its store. It advances only when what a projection reads changes:

| Event | Calls | Advances when |
|---|---|---|
| Store mutation | `record`; `undo`, `undo_by`, `use_press`, `use_press_by` | always (an entry came or left) |
| Refusal change | `undo_or_refuse` (`Err` sets a refusal, `Ok` removes), `end_refusals` | a refusal was set or ended |
| Lock change | new `set_locks(|value| -> Option<Sentence>)`, per entry | an entry's lock differs |
| End events | `owner_changed`, `disk_changed`, `end_unreachable` | an entry left |
| In-place change | `source_changed`, `revise_by`, `revise`, `revise_where`, `reconcile` | they hand out `&mut T` and cannot see a change, so: they reached an entry. Their docs already say "at the change, never each frame". |
| Timer expiry | new `expire(ctx)` | an entry's life ended |

Reads (`get`, `newest_first`, `present`) never advance it. A ticket's order is not the content revision:
a refusal or a lock changes the words with the same ticket.

**Expiry.** `Kept` gets `const LIFE: Option<Duration> = None`; a timed kind sets it. `record` stores the
entry's end on the frame clock (`frame_clock::now() + life`: the act's time, not a key). The owner calls
`expire(ctx)` in its update, before any projection lookup. It ends each entry whose end passed, advances
the revision only when one ended, and asks egui for a repaint at the soonest remaining end
(`request_repaint_after(remaining)`), so the line ends with no input. No key holds `now()` or a frame
number: a projection's key is the store's revision and the owner's input revisions. `present` refuses an
entry whose end passed (the owner did not expire first): `Mismatch::Expired`, a debug assertion, nothing
drawn. Result life is frame-clock seconds only; reveal's frame-number bounds are not used here.

## 3. Checked bindings

A projection carries `Bound { revision, key: O, inputs: I }`: the store revision it was made from, the
owner key it shows, and `I: memo::Key`, the owner's other revisions that its words read (for both Add
results, `Book::revision()`). `present` compares all three with the store's revision and the key and
inputs the caller gives now. Any mismatch is `Err(Mismatch::{Revision, Key, Inputs})`: no line, no ticket.
It is not drawn and not silently rebuilt in the paint; the owner's next `refresh` rebuilds it. An entry
that the projection names but the store lacks while the revision matches is an internal fault (debug
assertion, rejected). For fields (PR 3 and PR 10), the same `Bound` binds a field buffer to its owner,
field key and revision; this PR uses it for result lines only.

## 4. The first owners

| Owner | State after this PR | Life | Expiry and end events run in | Projection key |
|---|---|---|---|---|
| Timeline Add | `Results<TimelineFile, JustAdded>` (its `at` goes) | `LIFE` 30 s | top of `Timeline::show`, before any footer: `expire`; another selection ends it (`owner_changed`) | (store revision, `book.revision()`) |
| Session plan Add | `Option<Added>` with its own `ResultOrder::issue` and staged `outcome_at` becomes `Results<OfSession, PlanAdded>` | `LIFE` 30 s | `Sessions::expire_timed(ctx, book)`, from `poll`: `expire`, then the events check (count, label) only when `book.revision()` changed | (store revision, `book.revision()`) |
| Recording words | `saved_at` and `Recorder::heard_at` | 8 s, 5 s | `Sessions::expire_timed` and `Recorder::expire`: the stored facts change; repaint at the remaining time | `Sessions::recording_revision()` (a `Revised` of the two timed facts) |

Footer construction (`list_messages`, `plan_messages`) keeps only `refresh` (a revision compare on an
unchanged frame) and `present`. The per-frame `day_numbers`, event searches, `fights_that_keep_hp` and the
30 s repaint request leave it; `plan_messages` computes `day_numbers` only for a shown removal. The plan's
Add loses one staged `ResultOrder::issue` and one `outcome_at` (their `STAGED_ALLOWS` rows shrink).
`just_saved()` and `heard_just_now()` read stored facts, never the clock.

## 5. Proof

- Extend the store's ticket tests: A, unrelated B, Undo A: B's projection still presents with B's words;
  A's is rejected. Record A again: A's old projection is rejected before it can act. Refusal and lock
  each change the revision and the rebuilt words. B's projection with A's key: `Mismatch::Key`.
- Extend `the_add_result_ends_by_the_frame_clock` (Timeline) and the plan's 29 s / 31 s test: an
  unchanged pane across the end with no input; in one frame the words, `Undo add` and the footer's
  height change together; the repaint delay asked before it equals the remaining time.
- Extend the recording word test (7.5 s / 8.5 s) and `hearing_ends_by_the_frame_clock` the same way.
- Work counter (`test_support::CachedWork`): projection builds counted inside `refresh` in entries and
  word bytes. Unchanged frames and a resize: zero; an edit of the book: a rebuild.
- Mutants, each red at its own assertion: no revision at expiry; no revision at a refusal; B's binding
  with A's key accepted; `refresh` rebuilding every frame.

## 6. Limits and questions

- Saved runs have no revision. The plan and Timeline Add words name the fights that keep their HP
  (`fights_that_keep_hp`); they are re-derived when the store or the book changes, not when only a saved
  run changes in the 30 s. Today they are re-derived each frame. The fix is a revision of the
  encounters' saved runs (outside this lane's files). Question for the review: is this limit accepted?
- The plan's Add words fix the session's name at the act, as the event's label already is.
- Every other result owner keeps the frame-bound path; the footer renderer is PR 2b.
