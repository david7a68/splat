# Library results onto the typed footer (M-L0.4 design note, ruling M14)

Lane M-L0. Base: `origin/main` f21f2473 (PR 457 in). PR 463 (M-L0.3) no longer holds the three M9 Undo
labels (Astra's BLOCK, lead's ruling): they move here, onto the typed footer (§1, §7 proofs 6 to 8). Built
against PR 452's typed footer (head c8921177). Code starts after PR 452 lands and this note is accepted.

## 1. What must hold

- `ResultOrder::issue(` in `src/library.rs` 14 → 0 (13 production, 1 test fixture); `OrderKey`,
  `ResultOrders`, `order_of` and `result_orders` go.
- Every Library footer host draws through `Messages`; the legacy closure calls (`footer.result`,
  `legacy_reason`, `legacy_notice`, `legacy_kept`, `legacy_kept_line`) in `library.rs` reach 0.
- No new `ByOwner` row (M12). The existing ones (`apply_page_press` 3, `undo_package_registration` 1) go to
  0, because every press comes back with its `Ticket`.
- Words, tones and footer order stay as the DM sees them now. Each changed line has its witness.
- M9: the three Library Undos that name no object today gain it, through the typed footer's kept Undo
  (`UndoStep::of` in `outcome_of`; `Messages::kept` fits it with `alone` or `fit` in the Undo's own room,
  `undo_room`, and keeps the whole label as its name): `Undo copy <entry>` (Copy), `Undo unregister
  <package>` (Unregistered), `Undo remove <name>` (Removed). No label is fitted in `library.rs`.

## 2. Parts of PR 452 this relies on

`Messages::{new, outcome, answer, kept, pane, pane_editor}` with typed lines (`FooterLine`, no closure);
`Shown::outcome(frame, make) -> KeptLine` and `Ticket`; `Results::{record, get, newest_first, undo_by,
use_press_by, owner_changed, disk_changed, reconcile}`; `KeptOutcome::{Notice, Fault}`, `Said`, `Notice`,
`FaultMessage` with a fault type, `Choice` / `MessageChoice` / `choice!`, `UndoStep::of(...).pressed_as`,
`locked_when`; `Footer::planned` (the pane places the footer from its plan); `Ban::Footer` with its exact
per-file counts. If PR 452's repair renames any of these, the code follows; the design does not change.

## 3. The store: one mark store, as Encounters' `KeptResult` (PR 452)

```rust
enum KeptKind { ScopeNotice, Unregistered, PackageUse, Manifest, Identities, Identity,
                Discarded, Removed, Heading, Vital, Copy }
struct KeptOwner { kind: KeptKind, copy: Option<(FileKey, String)> }  // copy: linked file, copied path
struct KeptMark(KeptOwner);          // impl Kept: ENDS per kind below (one ENDS: see §6, Q1)
kept: design::Results<KeptOwner, KeptMark>
```

The store holds each result's existence, order and ticket. Its data stays in the field that holds it today
(`identity_undo`, `discarded`, `dependency_undos`, …). Every writer that today issues an order records the
mark in the same statement (`self.stamp(kind, file)`). One sync (`kept_marks`, as Encounters'
`kept_results`) runs before any footer line is made. It ends (`owner_changed`) every mark whose data is
gone, so a mark never outlives its data. Its work is O(kinds + copies) per frame, with no file read and no
words. A field without a mark has no line (it cannot happen: §7 test 3). The four stores that exist
(`registrations`, `rosters`, `renamed_links`, `link_edits`) stay as they are.

Why marks, and not the data inside the store, for now: in the first PR (§5) the presses stay on today's
flag path. A payload store would need `undo(owner)` there, which is a new `ByOwner` row. Once every press is
by ticket (PR 4b), moving each field's data into a payload variant is a pure refactor (optional PR 4c).

Scope is `()`: today's slots are not hidden at a campaign switch, and their own end events below decide. A
campaign scope would make an Undo reappear after a switch back, which Remove's rule forbids
(`note_campaign`).

## 4. Each kept result: owner, writer, end events, Undo

Every kind but Copy holds one result at a time (one field), so its owner is its kind: a new act of that kind
replaces the older result (`record`), whose ticket is then `Gone`. Copies are several per file: their owner
holds the linked file and the copied path. Where a line shows (its file, its pane) the data decides, as now.

| Kind | Writer (today's issue site) | Ends (besides its own Undo, and a newer result of its kind) | Undo (typed) |
|---|---|---|---|
| ScopeNotice | `open_entry` | a new query or scope (`show`), `go`, a mode change (`editor_pane`) | none: `Notice` |
| Unregistered | `unregister_dialog` | nothing else (an Undo that fails keeps it) | `Undo unregister <package>` |
| PackageUse | Choose all (`editor_pane`) | reload when the campaign's choices differ (`forget_changed_undos`); a registration Undo of that package | `Undo package choice` |
| Manifest | repair write (`editor_pane`) | reload when the file changed (`forget_if_changed`) | `Undo write <path>`, locked while a draft waits |
| Identities | separate entries (`editor_pane`) | reload when the edits are not on disk | `Undo separate entries`, locked as above |
| Identity | `give_new_identity` | reload when the file changed | `Undo separate entry`, locked as above |
| Discarded | `reload_note_from_disk`, Discard (`editor_pane`) | a new draft (`draft != text`), `refresh_destination` (file changed or unowned), campaign switch and any selection while Remove holds it | `Undo discard of <name>` |
| Removed | `reload_note_from_disk` | any selection (`end_remove_on_selection`, `open_entry`), campaign switch (`note_campaign`), file changed (`refresh_destination`) | `Undo remove <name>` |
| Heading | heading fix (`editor_pane`) | shown for its file only | `Undo heading change` |
| Vital | stat fix (`editor_pane`) | shown for its file only | `Undo stat change` |
| Copy | copy dependency (`editor_pane`) | reload when either file changed | `Undo copy <entry>` (`alone` / `fit`) |

A, unrelated B, Undo A: for every kind with an Undo the press carries A's ticket. B's later record of
another owner leaves A's entry and order alone. Undo A acts through `undo_by(A's ticket)`, which is `Gone`
(nothing happens) if A was replaced or ended. A's words leave with A's data (the sync), and B's line stays.
Existing proofs that keep this: `iter-26-L-copy-owner` (copy A, copy B, Undo A), `iter-25-L-undo-remove-after-b`
(Remove A, choice B, Undo A), `iter14_e_changed_files_end_every_undo_result`.

## 5. Every footer host that moves

| Host | Today | Lines it holds |
|---|---|---|
| Form footer (New entry, package form, empty scope) | `form_footer` (2 callers) | page results, the page error |
| Empty reader / removed entry | `show` (no entry open) | Removed, all results, the page error |
| Reader (Read) | `reader_messages` | page results, slot results, heading fix (`Notice` + choice) and its Undo, copies, duplicate notice (choices, blocked with reasons), dependency notices (choice locked by the draft), page error |
| Editor (Edit Markdown, its save row) | `editor_pane` | Discard/Heading/Vital, all results, repair fault and its recovery choice, Save failure, link repair, broken links, heading fix, stat fixes (`Use <value>` choices), duplicates, dependencies, copies, page error |
| Comparisons (disk change: Read and Edit Markdown) | the same `pane_footer` in `pane` / `pane_editor` | as Reader / Editor |

New typed vocabulary (in `library.rs`, a caller's types): `enum LibraryPress` (one id per act: `Undo`,
`UseLinks`, `UseCopy`, `FixHeading`, `UseValue(key)`, `NewIdentity(at)`, `NewIdentities`,
`CopyDependency(id)`, `Recover`, …). `enum LibraryFault` (page error with its code spans, repair problem
in its tone, Save failure, read fault) implements the fault trait. Notices use `sentence!` / `choice!`
(no new `Sentence::outside`: dynamic words are format arguments). `outcome_of(kind, &self) ->
KeptOutcome<LibraryPress>` gives each mark's line; the four stores get theirs from their own values.

## 6. The PR cut (each leaves main green)

- **PR 4a, the store (about 450 changed lines).** `kept` with `KeptMark`; all 13 production
  `ResultOrder::issue` sites → `stamp`, and the test fixture to the store. The legacy hosts draw every kept
  result through the one existing `legacy_kept_line` call (`add_reader_result`), made from `Shown::footer`.
  So the `footer.result` calls in the file go, and no legacy count rises. Presses stay as today (flags and
  the two existing by-owner paths). Counts: `ResultOrder::issue` library.rs 14 → 0; `.result(` STAGED
  lowered; `Ban::Results` rows of `give_new_identity`, `open_entry`, `reload_note_from_disk`,
  `unregister_dialog` go.
- **PR 4b, the typed footers (about 950).** Every host in §5 to `Messages` at once (a footer is all typed or
  all legacy in PR 452, and one kind shows in several hosts); presses by ticket; `LibraryFault`,
  `LibraryPress`, `outcome_of`. Counts: legacy footer calls → 0; `ByOwner` rows → 0; `reader_messages`,
  `add_results`, `show_result`, `form_footer` and `page_error` go with their rows. The M2 rows inside messages
  (`locked`, `.blocked` of M0283–M0352) become typed locks and their counts fall.
- **PR 4c (optional, about 300).** Data from the fields into the store's payload; the sync goes.

No single PR of 1,500 lines is needed. Q1 for review: one `ENDS` for every mark kind (`owner_changed: Ends`,
`disk_changed: Stays`, `press: Undo`; ScopeNotice ends by the sync and is never pressed), or a store per
kind group. I propose one, as Encounters does.

## 7. Proofs (each must go red once)

1. 4a: `kept_result_is_used_only_where_counted` and `staged_calls_only_shrink` with the lowered counts; a
   mutant `ResultOrder::issue()` back in `open_entry` → STAGED "occurs 1 times, 0 allowed".
2. 4a: footer order stays newest first: extend `iter14_e_changed_files_end_every_undo_result` to record two
   kinds and read the footer's first line; mutant: `stamp` issued before the earlier result → wrong first
   line.
3. 4a: a mark never outlives its data, and no field lives without its mark: a unit test over every
   `KeptKind` (set the data, stamp; drop the data, sync; the mark is gone); mutant: the sync skips one kind.
4. 4b: A, unrelated B, Undo A by ticket: extend `iter-26-L-copy-owner` with a press whose ticket is `Gone`
   (A replaced) → nothing written; mutant: undo by owner → A's newer copy undone.
5. 4b: words and places at 960, 1200 and 1920: the existing Library states (library-navigation,
   library-author, package-delete-entry, remove-package-in-use, library-file-fault) are the witnesses;
   any moved line is named from the head PNG beside the base PNG.

The three M9 labels (4b), with Astra's three failing cases of PR 463 as their proofs:

6. Boundary width: in a pane whose footer line fits the whole label but whose Undo room (the message
   inset, the Undo icon and padding, the `LIST_MEASURE` cap) does not, the visible label is the fitted form
   and the paint cuts nothing (no `…`). The test sweeps the width across that boundary for
   `Undo unregister Ashen Marches`. Mutant: fit against the footer's full width (PR 463's way) → the full
   label is kept and the paint cuts it.
7. Accessible name after shortening: at the narrowest width the visible words drop the object
   (`Undo unregister`), and the AccessKit name stays `Undo unregister Ashen Marches`, as does the hover.
   Mutant: name the action by its visible words → the name loses the package.
8. Quiet frames: with all three Undos shown and nothing changed, a frame clones no name, builds no step and
   measures no label: PR 452's footer work counters (plans, sorted lines, named bytes, probes) stay 0, and
   `outcome_of` runs only when its kind's data or the store's revision changed. Mutant: build the label in
   the draw → the counters rise.
   The visible witnesses stay in their states: `dependency-copy-reconnected` (library-navigation),
   `package-unregistered-with-undo` (remove-package-in-use), `deleted-draft-let-go` (package-delete-entry),
   at 960, 1200 and 1920.

## 8. Risks named

The heading Undo shows in Read and Edit Markdown alike (two hosts, one mark: one ticket, both lines). Copies
are several per file (the owner holds the copied path). The duplicate notice's choices include blocked
choices with reasons (`Choice` locks). If PR 452's typed message cannot show a choice on its own line (its
FB3 note), the stat-fix and dependency notices keep their choices after the words, as now.
