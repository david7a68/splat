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

## Next iteration

Visual review found a moderate issue in the Wizard class table at the minimum
window size: words such as “Memorize” and “Subclass” break into a few letters per
line. The table needs readable column widths and horizontal scrolling when it
cannot fit. Evidence: `library-browse__06-class-scrolled__960x640.png` in the
before captures. Review wide tables in both Library and Lookup.

## Limits

All app data is synthetic and kept in temporary folders. Live D&D Beyond,
microphone hardware and remote transcription services are not part of this
audit. The existing hermetic tests cover their UI states and failure paths.
This is an expert audit; no human table session was observed.
