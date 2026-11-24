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
copies the retained message into the line each frame, and `Messages::kept_at` copies its words again.
**The cost of an unchanged frame is therefore O(all handed message bytes)**: every word of each presented
line, also clauses the footer does not show (a long rest that names N fights keeping their HP hands all N
sentences). It does not grow with the event list or the notes. It is counted (`handed` bytes) and is PR
2b's to remove. Everything else on an unchanged frame is constant and counted (`visits`): `refresh`
compares three values; `present` finds its entry by the place kept with the line (valid while the
revision stands), not by a search; `expire` keeps the oldest act for the store's revision and visits no
entry before the next end (at an end it visits each entry once, and finds the next end among those
that stay). `set_locks` visits every entry, so an
owner calls it when the lock's source changed, not each frame (no first owner has a lock). Other legacy
footer work of these panes (other results, the removal line) is outside this claim.

## 2. Owner revisions

`Results` publishes `revision() -> memo::Revision`. A store gets a new one at `new` (so `new` stops being
`const`; no caller is `const`). Revisions are unique in the run, and `Results` is not `Clone`, so a
revision also names its store. It advances only when what a projection reads changes:

| Event | Calls | Advances when |
|---|---|---|
| Store mutation | `record`, `record_timed`; `undo`, `undo_by`, `use_press`, `use_press_by` | an entry came or left (a failed plain Undo is no change) |
| Refusal change | `undo_or_refuse` (`Err` sets a refusal; `Ok` removes the entry, refused or not), `end_refusals` | a refusal was set or ended, or the entry left |
| Lock change | new `set_locks(|value| -> Option<Sentence>)`, per entry | an entry's lock differs |
| End events | `owner_changed`, `disk_changed`, `end_unreachable` | an entry left |
| In-place change | `source_changed`, `revise_by`, `revise`, `revise_where`, `reconcile` | they hand out `&mut T` and cannot see a change, so: they reached an entry. Their docs already say "at the change, never each frame". |
| Timer expiry | new `expire(ctx)` | an entry's life ended |

Reads (`get`, `newest_first`, `present`) never advance it. A ticket's order is not the content revision:
a refusal or a lock changes the words with the same ticket.

**Expiry.** `Kept` gets `const LIFE: Option<Duration> = None`; a timed kind sets it, and must be recorded
with `record_timed(value, ctx)` (`record` of a timed kind, and `record_timed` of an untimed one, do not
compile). It stores the act's time on the frame clock (not a key) and arms the end at once, so a result
recorded after the owner's update in the same frame still ends with no input. The owner calls
`expire(ctx)` in its update, before any projection lookup. It ends each entry whose end passed, advances
the revision only when one ended, and asks the state context for a repaint at the soonest remaining end
(`request_repaint_after(remaining)`), so the line ends with no input. No key holds `now()` or a frame
number: a projection's key is the store's revision and the owner's input revisions. `present` refuses an
entry whose end passed (the owner did not expire first): `Mismatch::Expired`, a debug assertion, nothing
drawn. Result life is frame-clock seconds only; reveal's frame-number bounds are not used here.

## 3. Checked bindings

A projection carries `Bound { revision, key: O, inputs: I }`: the store revision it was made from, the
owner key it shows, and `I: memo::Key`, the owner's other revisions that its words read (for both Add
results, `Book::revision()` and `Encounters::saved_runs_revision()`). `present` compares all three with
the store's revision and the key and inputs the caller gives now. The key is compared as the store
compares owners: scope and `O::eq` (an owner whose `PartialEq` compares an id only cannot pass B's key for
A's line). A store made by `new` or `Default` gets a revision of its own. The types cannot prove that
the caller gives the live key and the live input revisions: the caller passes the values it read this
frame (the owner's own `book.revision()`, `plan.saved_runs_revision`). Any mismatch is `Err(Mismatch::{Revision, Key, Inputs})`: no line, no ticket.
It is not drawn and not silently rebuilt in the paint; the owner's next `refresh` rebuilds it. An entry
that the projection names but the store lacks while the revision matches is an internal fault (debug
assertion, rejected). For fields (PR 3 and PR 10), the same `Bound` binds a field buffer to its owner,
field key and revision; this PR uses it for result lines only.

## 4. The first owners

| Owner | State after this PR | Life | Expiry and end events run in | Projection key |
|---|---|---|---|---|
| Timeline Add | `Results<TimelineFile, JustAdded>` (its `at` goes) | `LIFE` 30 s | `settle_added`, at the top of `Timeline::show`, before any footer: `expire`; when `book.revision()` changed, an Add whose event is gone ends (`disk_changed`, now `Checked`: a reload, a file event, an edit); another selection ends it at the press (`select`, used by every writer) and again in the update | (store revision, (`book.revision()`, saved-run facts)) |
| Session plan Add | `Option<Added>` with its own `ResultOrder::issue` and staged `outcome_at` becomes `Results<OfSession, PlanAdded>` | `LIFE` 30 s | `Sessions::settle_timed(ctx, book)`, at the end of `poll`, after `book.poll`: `expire`, then the events check (count, label) only when `book.revision()` changed; select, a ruling, a note insert end it (`end_plan_add`) | (store revision, (`book.revision()`, saved-run facts)) |
| Recording words | `saved_at` and the recorder's `hearing` | 8 s, 5 s | `Sessions::settle_words` (from `settle_timed`, and from the stop, which arms the end) and `Recorder::expire`, each frame, outside `poll`'s wall-clock pause | `recording_revision()` (a `Revised` of the two words); read only by tests until a retained reader exists |

**Unreadable or moved book (Timeline Add).** An unreadable book keeps the events it had, so its Add
stands until its life ends; the page then shows the problem in place of the list and its footer. A book of
another file is another owner key, so its Add shows no more and ends by its life.

**Saved-run facts (F1 of the early review).** `Encounters` notes, at the end of each `poll`, the facts
that `fights_that_keep_hp` reads: which read of the run list it is (a revision that `take_run_list`
advances: a reload, a deleted or restored run) and the open run's id and whether it is running. Their
revision is the second input of both Add keys (`Plan::saved_runs_revision`, `TimelinePage::set_saved_runs`).
The work is constant per frame (a revision, and the open run's id and running flag through
`Run::keeps_saved_hp`, with no string copied), never the run list.

Footer construction (`list_messages`, `plan_messages`) keeps only `refresh` (a revision compare on an
unchanged frame) and `present`. The per-frame `day_numbers`, event searches, `fights_that_keep_hp` and the
30 s repaint request leave it; `plan_messages` computes `day_numbers` only for a shown removal. The plan's
Add loses one staged `ResultOrder::issue` and one `outcome_at` (their `STAGED_ALLOWS` rows shrink).
`just_saved()` and `heard_just_now()` read stored facts, never the clock.

## 5. Proof

- Extend the store's ticket tests (one store revision, two stages): projections of A and B; Undo A; both
  old projections are refused (`Mismatch::Revision`); after the owner refreshes, A has no line and B
  presents its unchanged words with a ticket equal to B's. Record A again: A's first projection and its
  ticket stay unusable. Refusal and lock each change the revision and the rebuilt words; a failed plain
  Undo and an unchanged lock do not. With the `ById` owner and both entries recorded first, A's
  projection with B's equal-id key: `Mismatch::Key`; a projection of a `Default` store: `Revision`.
- Extend `the_add_result_ends_by_the_frame_clock` (Timeline) and the plan's 29 s / 31 s test: an
  unchanged pane across the end with no input; in one frame the words, `Undo add` and the footer's
  height change together; the repaint delay asked before it equals the remaining time.
- Extend the recording word test (7.5 s / 8.5 s) and `hearing_ends_by_the_frame_clock` the same way.
- Work counter (`test_support::CachedWork`): projection builds counted inside `refresh` in entries and
  word bytes. Unchanged frames and a resize: zero; an edit of the book: a rebuild.
- Extend the plan long-rest test: a deleted saved run (book revision fixed) removes the HP clause in the
  next frame while the rest's result and Undo stay; Undo of the deletion brings it back.
- Extend the Timeline test: a saved A, an unrelated B, and a reload without A: no A result or Undo on the
  first frame, B unchanged; an edit on disk that keeps A is the control.
- A timed store with 200 hidden entries and a long message: unchanged frames build nothing and visit no
  entry; the handed bytes are the shown line's words per frame; the end visits each entry once.
- Mutants, each red: no revision at expiry; no revision at a refusal; B's key accepted; the scope not
  compared; `refresh` rebuilding every frame; `expire` scanning every frame; the plan's key without the
  saved-run facts; Timeline without the lost-event check; the recording words not settled.
- A source scan counts the frame clock's reads (`frame_clock`): only where a timer starts or ends.

## 6. Limits

- The saved-run facts are noted at the end of `Encounters::poll`. A run list read at the end of a frame
  (`route`) is seen at the next frame's poll, before that frame's pages draw; the same frame drew before
  the read.
- The plan's Add words fix the session's name at the act, as the event's label already is.
- A frame-bound line ends the store's borrow; the types do not stop the pane's body from changing the
  store between `present` and the footer's paint (as today). The truth claim is for the frame's start.
- Every other result owner keeps the frame-bound path; the footer renderer is PR 2b.

## 7. The retained footer (layout PR 2b)

`Messages` keeps its lines as typed data (`FooterLine`: rank, act order, the typed message, a kept line's
ticket); nothing is lowered to a closure. The pane draws them through `TypedFooter`, which keeps a plan per
footer: the lines' one order (`footer_order`'s rule), the accessible name, each Undo's short form
(`undo_alone`), each kept Undo's words as fitted to its room (`forms`: the visible form, the whole form that
names it, the action's width), the room the lines take (`bare`), the room their open locks' reasons want
(`reserved`) and the room each of those reasons keeps (`rooms`).

- **When the plan is made.** By the pass that draws the lines, so its measure is the paint's own (the
  typography, wraps, actions and reasons that are drawn), and only when the lines changed (compared as data
  with the last ones, with the state their file lines draw from: a link pressed for a file that is gone adds a
  reason line), or the width, the scale or the style. An unchanged frame sorts, names, counts Undo verbs,
  builds and fits no Undo label (`Body::undo_form` takes the plan's) and lays out no reservation (an empty
  widget-like rect takes each reservation's kept room); `FooterWork` counts plans, sorted lines, named bytes,
  reason probes, verb comparisons, Undo labels fitted and reservation layouts. The pane knows an
  **unchanged** footer's demand before it draws (`Footer::planned`). A **changed** footer's plan is made before
  the pane places it (L1c): `TypedFooter::measure` draws its lines in a context of their own, with this
  context's fonts, style and scale, so the pane knows every footer's demand before it draws and draws the lines
  once, in their place, from the plan.
- **Reasons' room (R29), per footer.** Each pass decides `give_up` from its plan's numbers and the room where
  the footer stands; the plan measures every open lock's reason (L1c: made before the footer is placed). It
  keeps each reservation's room also when it gives that room up, so after the pane grows (the plan
  holds: height is not in its key) a quiet frame takes the room and lays out no reason. No state is shared between
  panes. A message outside a footer keeps its reasons' room and measures and counts nothing (`Reasons::Body`).
  The legacy footer's shared flags (`GIVE_UP`, `RESERVED`, `DRAWN`) went with it (L2c).
- **Lifetime.** The plans of one context live in one store (`FooterPlans`). A footer that no frame drew since
  the last one has ended (its pane closed, its owner changed), and its plan, with its lines, is dropped by the
  store's sweep, once per frame when the first footer draws: the store holds the footers of about two frames.
  The sweep visits each kept footer once (`FooterWork::swept`), and the store's spare room shrinks with it.
- **Presses.** One press slot per `Messages` per pass, taken by `pane`; nothing keeps a press across passes, so
  a press is acted on once, also when the frame is drawn again or not drawn.
- **The old path, removed (L2c).** Every app footer is typed. `Footer::legacy_kept`, `legacy_kept_line`,
  `FooterResult`, `Shown::footer`, `outcome::kept_result` and `KeptGroup` are gone; `Footer::result`,
  `legacy_reason` and `legacy_notice` are `#[cfg(test)]`: only the design system's own pane tests draw closure
  lines (listed per function, `Ban::Footer`, in `OWNED_ALLOWS`, and counted per file in
  `LEGACY_FOOTER_LINES`), so no app code can. The callers moved: Rules and runs (its discard joins the page's
  `kept` store), Settings and Link and folder (Campaigns, the account, D&D Beyond); then (L2c) the encounter
  editor's file flags (into the editor's one footer, with the file's line) and the creature pane's refusal;
  then the run's Arrivals footer (the newest arrival's result with its Move choice, and the totals' reasons).
  Campaign details (Settings: `pane_content_then`; Link and folder), the creature pane (its Save, also the
  page's Ctrl+S, is done in `then`, after the fields) and the encounter editor read their lines after the
  body's acts (L1c, `pane_then`), so the frame of an act shows the line it brought and never one it ended.
- **Limits.** Building `Messages` each frame (the caller's words) and comparing them stays O(the footer's
  words); a caller that keeps owner revisions could skip both, which is later work. Fonts changed with
  `set_fonts` are not part of the plan's key (only width, scale and style).
