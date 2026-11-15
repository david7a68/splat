# Library results onto the typed footer (M-L0.4 design note, rulings M14, M14a)

Lane M-L0. Revised after Astra's REJECT of e7ccab77. Base `origin/main` f21f2473 (line numbers below are
from it). Built against PR 452's typed footer (head c8921177). Code starts after PR 452 lands and this note
is accepted. PR 463 no longer holds the three M9 Undo labels: they move here (PR 4b).

## 1. What must hold

- `ResultOrder::issue(` in `src/library.rs` 14 → 0 (13 production, 1 test fixture). `OrderKey`,
  `ResultOrders`, `order_of` and `result_orders` go.
- Every Library footer host draws through `Messages`. The legacy closure calls (`footer.result`,
  `legacy_reason`, `legacy_notice`, `legacy_kept`, `legacy_kept_line`) in `library.rs` reach 0.
- No new `ByOwner` row (M12). The existing ones (`apply_page_press` 3, `undo_package_registration` 1) go to
  0, because every press comes back with its `Ticket`.
- ONE mark store (Astra's answer to Q1), behind ONE mutation boundary for data and mark (§3).
- Each kept result has a complete owner and named end events. Heading and Vital have neither on main
  today; PR 4a fixes that, with a red witness (§4, proof 4).
- A valid ticket never lets an Undo replace text typed after the footer was built. The same-frame draft
  guards of `editor_pane` (`library.rs:8998`, `:9025`, `:9073`) stay, and the ticket path goes through them.
- M9 (PR 4b): `Undo copy <entry>`, `Undo unregister <package>`, `Undo remove <name>` through
  `Messages::kept` with `UndoStep::alone` (or `fit` when two copies share a footer). No label is fitted in
  `library.rs`.

## 2. Parts this relies on

PR 452: `Messages::{new, outcome, answer, kept, pane, pane_editor}` with typed lines (`FooterLine`);
`Shown::outcome -> KeptLine`, `Ticket`, `Results::{record, get, newest_first, undo_by, owner_changed}`;
`KeptOutcome::{Notice, Fault}`, `Said`, `Notice`, `FaultMessage`, `Choice`/`MessageChoice`/`choice!`,
`UndoStep::{of, alone, fit}.pressed_as`, `locked_when`; `Footer::planned`; `Ban::Footer` with exact
per-file counts. L2a (PR 444, on main): `Retained`, `Projection`, `Results::present`. **Dependency:** proof
10 needs PR 452's repair to retain the Undo label's construction and measurement (`outcome.rs:3254`,
`:3521` on c8921177), which Astra is adding there.

## 3. The store and its one mutation boundary

```rust
// src/library/kept.rs: the fields are private to this module.
pub(super) struct Kept {
    marks: design::Results<KeptOwner, KeptMark>,   // one store: order, ticket, one ENDS for all kinds
    identity: Option<IdentityUndo>, identities: Option<IdentitiesUndo>, manifest: Option<ManifestWrite>,
    package_use: Option<SelectionUndo>, discarded: Option<Discarded>, removed: Option<LetGo>,
    heading: Option<FixUndo>, vital: Option<FixUndo>, copies: Vec<DependencyUndo>,
    scope_notice: Option<String>, unregistered: Option<Module>,
}
struct KeptOwner { kind: KeptKind, copy: Option<(FileKey, String)> } // copy: linked file + copied path
```

Each kind has `set_<kind>(data)` (stores the data and records its mark in one call), `end_<kind>()` (drops
both), a read `<kind>()`, and its Undo: `undo_<kind>(run)` in 4a (flag press), `undo(ticket, run)` in 4b
(`marks.undo_by`; the data is cleared only when `run` returns `Ok`). No code outside `kept.rs` can write a
field, so a writer cannot skip the mark (privacy is the static rung), and no sync pass is needed. Every kind
but Copy holds one result: its owner is its kind, a new `set` replaces the old result, and the old ticket is
`Gone`. Scope is `()`: today's slots are not hidden by a campaign switch; their own end events decide.
`FileKey` is `(owner id, path)` (`Loaded::file_key`), a complete file identity across packages.

## 4. Each kept result: payload, writer, end events, Undo

| Kind | Payload (file identity) | Writer | Ends (besides its Undo and a newer one) | Undo; failure |
|---|---|---|---|---|
| ScopeNotice | words | `open_entry` | new query or scope, `go`, a mode change | none (`Notice`) |
| Unregistered | `Module` | `unregister_dialog` | none | `Undo unregister <package>`; a failure keeps it |
| PackageUse | `SelectionUndo` (campaign) | Choose all | reload with other choices; that package's registration Undo | see below |
| Manifest | `ManifestWrite` (absolute path) | repair write | reload when the file changed | `Undo write <path>`; a failure keeps it |
| Identities | edits (root) | separate entries | reload when the edits are off disk | `Undo separate entries`; keeps on failure |
| Identity | root + path | `give_new_identity` | reload when the file changed | `Undo separate entry`; keeps on failure |
| Discarded | owner id + path + draft | reload, Discard | a new draft, file changed or unowned, campaign switch, a selection while Remove holds it | `Undo discard of <name>` |
| Removed | `LetGo` (owner + path) | `reload_note_from_disk` | any selection, campaign switch, file changed | `Undo remove <name>` |
| Heading | **`FileKey`** + before + after | heading fix | **another file selected; its draft leaves `after` (edit, Discard, reload); campaign switch** | `Undo heading change` |
| Vital | **`FileKey`** + before + after + words | stat fix | **as Heading** | `Undo stat change` |
| Copy | root + path + copied path | copy dependency | reload when either file changed | `Undo copy <entry>`; keeps on failure |

**Heading and Vital (P1, a fault on main).** `heading_undo`/`vital_undo` hold a relative path
(`library.rs:4103`), and the display and press checks compare that path and the draft text (`:389`,
`:8376`, `:9025`, `:9073`). A's heading fix in one package, then B with the same path and text in another:
A's Undo shows for B and can replace B's draft. PR 4a keys both by `FileKey` in the payload and in both
checks, and makes "shown for its file only" end events: selecting another file, its draft leaving `after`
(checked at the frame's end, as Discard's `draft != text` is today), a campaign switch. Behaviour change:
switching away and back no longer brings the Undo back. Witness: proof 4.

**PackageUse failure (P2).** `apply_package_results` (`library.rs:7539`) takes `selection_undo` before it
loads, checks or saves the campaign, and drops it on failure. The policy in 4a: the Undo clears its payload
only after the save works. A failed read or save keeps the result and its Undo with the error (as Manifest's
Undo does now), so the DM can retry. Choices changed since (the check) end it with the refusal's words,
as a reload would. Behaviour change; witness: proof 5.

## 5. Every footer host that moves (PR 4b)

| Host | Today | Lines |
|---|---|---|
| Form footer (New entry, package form, empty scope) | `form_footer` | page results, page error |
| Empty reader / removed entry | `show` | Removed, all results, page error |
| Reader (Read) | `reader_messages` | page and slot results, heading fix and Undo, copies, duplicate notice (choices, blocked with reasons), dependency notices (choice locked by the draft), page error |
| Editor (Edit Markdown, save row) | `editor_pane` | Discard/Heading/Vital, all results, repair fault and its choice, Save failure, link repair, broken links, heading fix, stat fixes (`Use <value>`), duplicates, dependencies, copies, page error |
| Comparisons (disk change) | the same `pane_footer` | as Reader / Editor |

New caller types in `library.rs`: `LibraryPress` (one id per act), `LibraryFault` (page error with its code
spans, repair problem in its tone, Save failure, read fault), `outcome_of(kind) -> KeptOutcome`, each kept
line a `Retained` projection. Words use `sentence!`/`choice!`; no new `Sentence::outside`.

## 6. The PR cut (each leaves main green)

- **PR 4a, the store (M14a; about 550 changed lines; count-neutral).** `kept.rs` with the boundary; the 13
  production issue sites become `set_*`, and the fixture moves to the store. The legacy hosts draw every kept
  result through the one existing `legacy_kept_line` call (`add_reader_result`), made from `Shown::footer`,
  so `.result(` falls and no count rises. Presses stay legacy flags and call `undo_<kind>`; `ByOwner`
  unchanged. It also carries the Heading/Vital identity fix and the PackageUse failure policy. Per kind it
  proves the visible order and the Undo unchanged (proofs 2, 3).
- **PR 4b, the typed footers (about 950).** Every host in §5 to `Messages` at once (a footer is all typed or
  all legacy in PR 452, and one kind shows in several hosts); presses by ticket through the draft guards;
  the M9 labels. Counts: legacy footer calls → 0; `ByOwner` → 0; `reader_messages`, `add_results`,
  `show_result`, `form_footer` and `page_error` go with their rows; the M2 locks inside messages become
  typed.

## 7. Proofs (each goes red once)

1. 4a counts: `staged_calls_only_shrink` with `ResultOrder::issue` library.rs 14 → 0. Mutant: one issue
   back in `open_entry` → "occurs 1 times, 0 allowed".
2. 4a, through the REAL writers (each extracted as a method that the UI and the test both call:
   `fix_heading`, `fix_vital`, `discard_draft`, `choose_all`, `separate_entries`, `copy_dependency`,
   `write_manifest`, with `give_new_identity`, `reload_note_from_disk`, `open_entry` and `unregister`):
   per kind, empty → present (a mark, newest first) and replacement (the newer order, one line). Mutant: drop
   the record in one `set_*` → that kind's case red.
3. 4a, the visible order and the Undo unchanged per kind, at 960/1200/1920, in the states that show them:
   Copy `library-navigation` (with `iter-26-L-copy-owner`), Removed `package-delete-entry`
   (`iter-25-L-undo-remove-after-b`), Unregistered `remove-package-in-use`, PackageUse and Manifest
   `package-about`, Identities `package-folder-copied`, Identity `package-duplicate-id`, Heading and Vital
   `package-rename`, Discarded `library-author`; ScopeNotice has no state: a new end check in
   `library-scope` (open an entry outside the scope).
4. 4a, Heading and Vital per file: A's fix in package 1 `Rules/X.md`, then B with the same path and text in
   package 2: no Undo line for B, and a press cannot replace B's draft. Red on main (relative path).
5. 4a, PackageUse: Choose all, campaign file unwritable, Undo → the error shows and the Undo stays; access
   back, Undo → choices restored. Red on main (the payload is gone).
6. 4b, A, unrelated B, Undo A by ticket: extend `iter-26-L-copy-owner`: a ticket of a replaced result is
   `Gone` and writes nothing. Mutant: undo by owner → B's copy undone.
7. 4b, same frame: the PR 343 P1 cases (`world_build.rs:5913`, `:6197`, `:6953`) press by ticket now;
   typed words stay. Mutant: the guard removed → the typed words are replaced.
8. 4b, boundary width: a pane where the footer line fits `Undo unregister Ashen Marches` but the Undo's
   room (the message inset, icon and padding, `LIST_MEASURE`) does not: shortened, never cut (`…`). Mutant:
   fit against the footer width → cut.
9. 4b, the accessible name after shortening stays the whole label (and its hover). Mutant: name by the
   visible words → the package is lost.
10. 4b, quiet frames: three Undos shown, nothing changed: no `outcome_of` call, no label built or measured,
    PR 452's footer counters 0. Mutant: `outcome_of` in the draw → counts rise. Depends on §2.

## 8. Risks named

The heading Undo shows in Read and Edit Markdown alike (one mark, one ticket, two lines). The duplicate
notice has blocked choices with reasons (`Choice` locks). If PR 452's typed message cannot put a choice on
its own line (its FB3 note), the stat-fix and dependency notices keep their choices after the words, as now.
