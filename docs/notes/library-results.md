# Library results onto the typed footer (M-L0.4 design note, rulings M14, M14a)

Lane M-L0. Revised after Astra's REJECTs of e7ccab77 and ab9f6987. Base `origin/main` f21f2473 (line numbers below are
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
- Each kept result has a complete owner and named end events. One file is one `FileId` (owner kind, owner
  id, path): main's `FileKey` drops the kind, so a campaign and a package with the same UUID merge (§3).
  PR 4a fixes that with red witnesses (proof 4) and keeps every lifecycle of main exactly (no behaviour
  change in 4a; the existing gate stays green).
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
struct KeptOwner { kind: KeptKind, copy: Option<(FileId, String)> } // copy: linked file + copied path
enum OwnerKey { Builtin, Package(Uuid), Campaign(Uuid) }
struct FileId { owner: OwnerKey, path: String }                       // replaces `FileKey`
```

Each kind has `set_<kind>(data)` (stores the data and records its mark in one call), `end_<kind>()` (drops
both), a read `<kind>()`, and its Undo: `undo_<kind>(run)` in 4a (flag press), `undo(ticket, run)` in 4b
(`marks.undo_by`; the data is cleared only when `run` returns `Ok`). No code outside `kept.rs` can write a
field, so a writer cannot skip the mark (privacy is the static rung), and no sync pass is needed. Every kind
but Copy holds one result: its owner is its kind, a new `set` replaces the old result, and the old ticket is
`Gone`. Scope is `()`: today's slots are not hidden by a campaign switch; their own end events decide.
**One file identity.** Main's `FileKey` is `(Owner::id(), path)`: `Owner::id` drops the owner's kind, and
settings check package and campaign UUIDs apart (`persistence.rs:56-72`), so a campaign and a package may
share a UUID. `FileId` keeps the kind. PR 4a moves every file key in `library.rs` to it, also the ones that
do not use the `FileKey` alias:
- the kept payloads: Heading, Vital, Discarded (`:4635`, `:8358`, `discard_is_for`, `refresh_destination`
  `:4969`), Removed;
- navigation: `place()` (`:991-1000`) stores only the UUID and the path, and `Place::find` (`:4267-4270`)
  drops the kind. A campaign file and a package file with the same UUID and path: visit the campaign's, then
  B, then Back, and the package's file wins, because packages load first. `Place` holds a `FileId`;
- the reselection after a reload (`start_load` `:4992`, `:5112`), the new entry's selection (`new_entry`
  `:7207`), the duplicate batch (`duplicate_choices` `:666`, `:673`), the diagnostics (`file_key` `:1698`,
  `files_to_fix` `:1730`), and the footers' egui ids (`:8665`, `:8791`, `:8970`).
Named, outside 4a: the checks that ask whether a note belongs to a package by its UUID alone (`:1025`,
`:5096`, `:5494`, `:5632`, `:5934`, `:5988`, `:6998`, `:7842`) can take a campaign with that UUID for the
package. It is the same cause (an owner key without its kind), but it is about scope, not results: a later
PR, with its own witness.

## 4. Each kept result: payload, writer, end events, Undo

| Kind | Payload (file identity) | Writer | Ends (besides its Undo and a newer one) | Undo; failure |
|---|---|---|---|---|
| ScopeNotice | words | `open_entry` | new query or scope, `go`, a mode change | none (`Notice`) |
| Unregistered | `Module` | `unregister_dialog` | none | `Undo unregister <package>`; a failure keeps it |
| PackageUse | `SelectionUndo` (campaign) | Choose all | reload with other choices; that package's registration Undo | see below |
| Manifest | `ManifestWrite` (absolute path) | repair write | reload when the file changed | `Undo write <path>`; a failure keeps it |
| Identities | edits (root) | separate entries | reload when the edits are off disk | `Undo separate entries`; keeps on failure |
| Identity | root + path | `give_new_identity` | reload when the file changed | `Undo separate entry`; keeps on failure |
| Discarded | `FileId` + draft | reload, Discard | a new draft, file changed or unowned, campaign switch, a selection while Remove holds it | `Undo discard of <name>` |
| Removed | `LetGo` (`FileId`) | `reload_note_from_disk` | any selection, campaign switch, file changed | `Undo remove <name>` |
| Heading | `FileId` + before + after | heading fix | see below (main's rules, unchanged) | `Undo heading change` |
| Vital | `FileId` + before + after + words | stat fix | see below (main's rules, unchanged) | `Undo stat change` |
| Copy | root + path + copied path | copy dependency | reload when either file changed | `Undo copy <entry>`; keeps on failure |

**Heading and Vital: when shown, when ended (main's lifecycle, kept exactly in 4a).** Two separate slots:
one never ends the other (a heading fix after a stat fix keeps the stat Undo in its slot, hidden while the
draft is not its text, and the other way round).
- (a) SHOWN while the selected file's `FileId` equals the fix's AND the draft text equals the fix's text
  (`after`). It hides while either differs and comes back when both match again: on the exact text after an
  edit, an invalid draft, Discard or Undo discard (`package-rename`, `world_build.rs:6215-6243`), and after
  switching to another file and back. Its press acts only under the same two checks, in the same frame
  (`library.rs:9025`, `:9073`): typed text is never replaced.
- (b) ENDS when its Undo is applied (the draft becomes `before`), and when a newer fix of the SAME kind
  replaces it (one slot per kind, on any file: today's `heading_undo`/`vital_undo` overwrite). Nothing else
  ends it on main, and 4a adds nothing: a campaign switch, the owner removed or re-registered, the file gone or
  moved, a reload, a press that the checks turn away (no change; it stays) leave it in its slot, hidden unless
  (a) holds again. That is safe because (a) needs the full identity and the exact text. Ending it at those
  events is a lifecycle change; it is not in this plan (it needs its own ruling).
- The fault on main is only the identity: main compares a relative path (`library.rs:4103`, `:389`, `:8376`,
  `:9025`, `:9073`) and its reselection merges kinds, so B can show A's Undo and a press can replace B's
  draft. 4a fixes the identity only (proof 4).

**PackageUse failure (P2).** `apply_package_results` (`library.rs:7539`) takes `selection_undo` before it
loads, checks or saves the campaign, and drops it on failure. The policy, in PR 4b (4a keeps main's Undo
behaviour): the Undo clears its payload only after the save works (`undo_by` keeps an entry on `Err`). A failed read or save keeps the result and its Undo with the error (as Manifest's
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
  unchanged. It also carries the one-identity fix (`FileId`: kept payloads, reselection, diagnostics),
  which changes nothing but the cross-owner fault. Every lifecycle stays main's (no behaviour change; the
  existing gate stays green). Per kind it proves the visible order and the Undo unchanged (proofs 2, 3),
  and the identity fix with red witnesses (proof 4).
- **PR 4b, the typed footers (about 950).** Every host in §5 to `Messages` at once (a footer is all typed or
  all legacy in PR 452, and one kind shows in several hosts); presses by ticket through the draft guards;
  the M9 labels; PackageUse's failure policy (its one intended Undo change, proof 5). Counts: legacy footer calls → 0; `ByOwner` → 0; `reader_messages`, `add_results`,
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
4. 4a, one file identity (unit tests through the real fix writers, the real reload and the real Back; each
   case is set up so that ONLY its owner check stands between B and A's result):
   - Heading and Vital, each kind on its own: make A's fix; give B (same UUID and other kind, or same path and
     other owner) A's exact `after` text; select B. Then the show check: no Undo line for B. Then the press
     check, apart: a press on B (a ticket or flag aimed at A's slot) leaves B's draft as it is. Each assertion
     goes red when its own owner check is removed (the kind, or the owner id), with the text check left in.
   - Reload: same UUID, other kind, both files with A's `after` text: after the reload the selection is still
     A's file and A's Undo shows; B shows none. Red when the reselection drops the kind.
   - Back: visit the campaign's file, then B, then Back: the campaign's file opens, not the package's with the
     same UUID and path. Red when `Place` drops the kind.
   - A → B → A: A's Undo shows again on A's exact `after` text. Red when a selection ends it.
   - Two slots: make a stat fix, then a heading fix; restore the draft to the stat fix's `after`: the stat
     Undo shows; restore it to the heading fix's `after`: the heading Undo shows. Red when one kind's `set`
     ends the other.
5. 4b, PackageUse: Choose all, campaign file unwritable, Undo → the error shows and the Undo stays; access
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
