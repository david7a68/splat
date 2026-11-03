# Design audit: consistency, density, access and effort

Started 2026-11-02 at `e3c292e7`, after a day of feature work (PRs 25 to 36).
The brief is `docs/ux-intent.md`. The rules are `docs/design-system.md`.
Whole-page and cross-page redesign is allowed.

Each iteration audits every workflow on real app captures, fixes the findings,
and is merged as one PR. The loop stops when a full audit finds no Moderate or
High finding. A Minor finding does not extend the loop.

## Metrics

Each finding names one metric. The checks in brackets are measured by the
visual suite; the others are judged on captures at 960x640, 1200x800 and
1920x1080.

| ID | Metric | Pass condition |
|---|---|---|
| M1 | **Design-system consistency** | Colour, type and spacing come from tokens [lint]. One component for one job: a page header, a list row, a search field, an add button and an empty state look and sit the same on every page. A status is an icon, a word and a colour, with no pill and no card. The same word means the same thing on every page |
| M2 | **Hierarchy** | At most one primary action in view, and it is the screen's primary action from brief section 4. What is most important is most prominent. Destructive and closing actions are apart from frequent ones |
| M3 | **Information density** | The screen's key information (brief section 4, in order) is in view at 960x640 with no scroll. No blank band wider than the content it separates. No control or text repeated on one screen. The density level (Glance, Scan, Read) suits the screen |
| M4 | **Actions per intention** | Interactions per task at or below the targets in brief section 8 [cost goals]. No page change for a task that the brief says happens beside the page. Each extra step is a click to find, not to do |
| M5 | **Accessibility** | Targets of 24 px or more, an accessible name for each control, keyboard reach, focus visible [ergonomics]. Contrast of 4.5:1 for text, 3:1 for large text and UI [tokens]. Colour is never the only signal. A disabled control says why. No text cut without a way to the full text [clipped] |
| M6 | **State visibility** | Unsaved, saving, saved, conflict, sync, recording, offline and read-only states show where the DM looks, as a word and an icon |
| M7 | **Aesthetic order** | Edges align to a shared column. Gutters and rhythm follow the 4 px grid. Few distinct styles of rule, border and fill. Nothing floats without an anchor. A wide window does not stretch a form or prose |
| M8 | **Error prevention and recovery** | An action that loses work has undo or a confirmation. A failure says what to do next |

## Severity

- **High:** an ordinary action loses work or corrupts game state, or a main
  task cannot be done.
- **Moderate:** a main workflow is hard to read, needs a workaround, breaks a
  design-system rule in a way the DM notices, or costs a page change or two or
  more extra interactions against its target.
- **Minor:** a cosmetic or small efficiency fault with a clear, usable path.

## Workflows

| Workflow | Scenarios |
|---|---|
| Prepare tonight | party, session-notes, session-plan, session-plan-groups, encounter-build, encounter-owned-groups |
| Start or resume a fight | prep-to-table, run-resume, run-restart, encounter-history |
| Run a fight | encounter-run, run-large-roster, run-long-names, run-add-creature, encounter-groups, run-overrides, run-edge-states, run-absent-player |
| Answer a question beside the fight | run-lookup, party-from-run, world, reader-tables |
| Keep D&D Beyond in step | run-sync-conflict, run-sync-states, run-roster-conflict, signed-in |
| Record and review the evening | session-recording, session-notes, timeline in session-plan |
| Read and write content | library-browse, library-navigation, library-scope, library-author, library-to-encounter, dialogs |
| Set up a campaign | empty, settings, campaign-content, column-resize, nav-state |

## Timing

Build and test times for each iteration are in `timing.md`.
