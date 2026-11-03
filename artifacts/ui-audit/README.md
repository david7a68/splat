# DM and homebrew UI audit

Started 2026-11-02. Initial revision: `94500abf`.

Stop when a complete code and visual audit finds no moderate or higher severity
issues, or after 24 hours. A minor cosmetic issue does not extend the loop.
Each completed fix iteration gets a reviewed PR and is merged after validation.

## Workflows

| User and task | Path and success condition | UI scenarios |
| --- | --- | --- |
| DM: prepare tonight | Check the party, create a session, plan fights, add read-aloud notes. Preparation survives page changes. | party, encounter-build, session-plan |
| DM: run a fight | Set initiative, apply damage and conditions, roll dice, add arrivals, undo a mistake. Keep the current turn visible. | encounter-run, run-large-roster, run-add-creature, encounter-groups |
| DM: answer a question | Read rules, a sheet or a world note beside combat. Return without losing the fight. | run-lookup, party-from-run, world |
| DM: finish the evening | End a fight, inspect its result, rest, carry HP into the next fight, and reopen a saved run. | session-plan, encounter-history, run-restart |
| Homebrewer: create content | Register a package, create a note, edit and save it, and find it again. | library-author, library-scope |
| Homebrewer: adapt content | Copy a book entry into a campaign or package. Review its dependencies and use the copy in an encounter. | dialogs, library-to-encounter, session-plan |
| Homebrewer: read related rules | Follow a reference across package and campaign boundaries. Back returns to the same owner, scope and search. | library-browse, library-navigation |
| Homebrewer: recover from an error | Keep an unsaved draft during navigation, reject an invalid save, resolve an external edit, and undo discard. | library-author, library-navigation, dialogs |

## Severity

- **High:** an ordinary action loses authored content or corrupts game state.
- **Moderate:** a main workflow gives the wrong content, cannot be completed, or
  requires a substantial workaround. Unreadable rules count here.
- **Minor:** cosmetic or small efficiency issues with a clear, usable path.

## Iteration 1: safe Library navigation

| Finding | Severity | Evidence and fix |
| --- | --- | --- |
| Back discards an unsaved homebrew draft. | High | `back_keeps_an_unsaved_homebrew_draft` failed before the fix. Back now disables while dirty, and its handler also protects the draft. |
| Back can open another owner's note with the same relative file path. | Moderate | `back_returns_to_the_right_owner_and_scope` returned note 0 instead of note 1 before the fix. History now records and checks the owner and restores the scope. |
| A reference outside the chosen package opens and then disappears on the next frame. | Moderate | The new `library-navigation` UI scenario could not reach Captain Varra from Ash Exposure. Following a reference now includes the destination in the scope; Back restores the old scope. |

The new UI workflow follows a package reference into a campaign, returns, edits
the target, saves it and reads it again. Unit checks also cover duplicate paths.

Visual review used real software-rendered app frames, not mockups, at 960×640,
1200×800 and 1920×1080. The first capture pass covers Library reading and editing,
external-edit recovery, package management, copying, encounters, live combat,
Party, the session plan and world notes. Captures are under the ignored
`artifacts/screenshots/ui-audit-before/` and `ui-audit-iteration-1/` folders.
The fixed Back control remains in its normal place while disabled. The draft,
Save and Discard controls remain visible at the minimum window size.

Validation: the complete locked test suite, formatting and Clippy with warnings
denied passed. All 41 UI scenarios passed. The new scenario adds only egui's
existing scroll-bar and resize-handle allowances to the ergonomics baseline;
it has no workflow, target-size, clipping or overlap failures in app controls.
Independent Standards and Spec reviews found no actionable issues.

Merged as [PR 35](https://github.com/david7a68/splat/pull/35).

## Iteration 2: readable tables

Visual review found a moderate issue in the Wizard class table at the minimum
window size: words such as “Memorize” and “Subclass” broke into a few letters per
line. Evidence: `library-browse__06-class-scrolled__960x640.png` in the before
captures. This affected both Library and Lookup because they share a reader.

Columns now keep their longest word whole. Tables that cannot fit scroll
sideways, with a short instruction. An ordinary wheel gesture still scrolls
the note down. A nested scrollbar stays hidden until its table reaches the view,
so it cannot cover earlier prose.

The `reader-tables` workflow creates and saves a homebrew rule with 14 columns,
reads its final column in Library, and reads that column again beside a live
fight. A rendered unit check covers 280, 360 and 488 pixel reader widths. It
failed before the fix because “Memorize Spell” became ten lines of fragments.
The new ergonomics check recognizes actual horizontal scrollbars, including
nested bars, and permits the side clipping they can reveal. Its regression test
also checks that clipping without such a bar still fails.

Visual review covers the Wizard table and both ends of the homebrew table in
Library and Lookup at all three window sizes. Words remain whole, the last
column can be read, prose after the table stays within the reader, and the
current turn and Next turn stay visible beside Lookup. Captures are under
`artifacts/screenshots/ui-audit-iteration-2/`.

Validation: all 651 tests passed (11 manual tests ignored), including all 42 UI
scenarios. Formatting and Clippy with warnings denied passed. The baseline adds
only egui scrollbar counts. No app-control or workflow allowance was added.
The check also has a regression for a thin, unnamed control inside scrollable
content, so that control cannot be mistaken for a scrollbar.

## Convergence review

The final visual pass covers the eight workflows above, plus empty states,
large rosters, long names, sync conflicts, saved-run recovery, session recording
states, and resized columns. Real app captures are under the ignored
`artifacts/screenshots/ui-audit-final/` folder. Seven contact sheets cover the
remaining 28 scenarios; the table fixes were also checked at full resolution.
The visual pass found no remaining moderate or higher issue.

The existing nine clipped-label reports occur only after resizing Library
columns at 960×640. They are list titles or metadata shortened with an ellipsis;
opening an entry shows its full title and content. They are minor, and the
baseline was not expanded for them. Independent code review and the final
baseline-enforced UI run must also pass before this iteration is ready to merge.

## Limits

All app data is synthetic and kept in temporary folders. Live D&D Beyond,
microphone hardware and remote transcription services are not part of this
audit. The existing hermetic tests cover their UI states and failure paths.
This is an expert audit; no human table session was observed.
