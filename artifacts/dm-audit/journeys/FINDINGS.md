# Faults the journey scripts found

Each journey script (`src/visual_tests/dm_audit/journeys.rs`) plays the real app. Where a step could not be
done as section 2 of `docs/design/session-cycle.md` walks it, or cost more, or the app showed a wrong state,
the fault is below, and the script takes the route a DM would have to take, so that the measure is true. No
app code changed in this pull request. "The driver's words" are what the driver printed when the designed act
found no control, or a fact that the journey's `summary.tsv` records.

## J2-1, J6-1. After `Roll for encounters`, the focus is not in `How long`

- **Journey and step:** J2 step 6 (two days overland), at 1200x800; and J6's Travel row with its checks.
- **What I did:** `More Add actions`, `Add Travel`, the row's menu, `Roll for encounters`, then typed `2 days`.
- **What I saw:** the typed words went nowhere; the block said `Cannot roll`, and `How long` stayed empty.
  Section 2.2 step 6 says `Roll for encounters` "puts focus in **How long**". With one marked table in the
  campaign, nothing has focus. (`travel-checks` tests the focus only with several tables and no table chosen.)
- **The driver's words:** `after Roll for encounters, the focus is in None` (J2 and J6 `summary.tsv`).
- **Route measured:** click `How long`, then type: 1 more act in each journey.

## J6-2. Lookup's reader of an NPC has no `Change` on Status

- **Journey and step:** J6 step 2 (Verori died here), at 1200x800 in the ended view, Lookup open.
- **What I did:** Ctrl+K, typed `verori`; looked for **Change** on Status, as step 2 says.
- **What I saw:** Lookup's reader is compact: it shows `NPC · alive · Campaign`, `Edit Verori`, `Add to Log` and
  `Add to fight`, and no Status row with `Change` (`src/world.rs`, the rows are drawn only when the reader is not
  compact). The status goes by `Edit Verori`, the Status field (click, select all, type) and `Save changes`.
- **The driver's words:** `journey-j6: no visible node for control "Change Status".` The fact `his note holds the
  status: true, and a Log line for it: true` shows that the Edit route still writes the Log line.
- **Route measured:** Edit: 7 against the design's 5 (2 more).

## J3-1. At 960x640 with Lookup open, an effect with a time closes Lookup

- **Journey and step:** J3 step 2 (pin `Poisoned 3 rounds` to the wolf), case D.
- **What I did:** selected Cinder Wolf 2 and looked for the dock's `Effect`, as step 2 says.
- **What I saw:** the dock's second line holds `Conditions`, `Concentrating` and `Absent`; `Effect` has no room
  (section 2.5 row 6: "the word, then the icon, then not shown"). `E` is off while Lookup is open
  (`src/encounter_run/view.rs`, `let letters = !self.lookup_open`). The one way left is the **Conditions** tab
  and its `Other effect` field, and opening that tab closes Lookup, with the note the DM had found in it.
- **The driver's words:** `journey-j3: no visible node for control "Effect".` and the fact `Lookup is open after
  the effect: false`.
- **Route measured:** select, **Conditions**, click `Other effect` and type, Enter: 5, where the walk of step 2
  implies 4 (select, `Effect`, type, Enter). The DM also loses the found note in Lookup; after the crash that
  follows, step 3 opens it again anyway, so the count of step 3 is as designed.

## J4-1. A fight started beside Trackers keeps Trackers open in the run

- **Journey and step:** J4, the arena fight (between steps 7 and 8), at 1200x800.
- **What I did:** with Trackers open beside the plan, `Start fight: Shrine of Cinders`.
- **What I saw:** the run opens with Lookup still on Trackers in the place of the fight's panel. Section 2.4 says
  "in a fight the run shows no tracker part". The DM presses Ctrl+T to see the fight's panel. A first try of the
  script pressed Ctrl+T for the tracker act and so closed the view instead of opening it.
- **The driver's words:** the fact `in the fight, before Ctrl+T: Trackers in view true, Lookup open true, on
  Trackers true` (an earlier run), then `journey-j4: no visible node for control "Crowd loves it".` after that
  Ctrl+T.
- **Route measured:** one Ctrl+T at the fight's start (Panel 1), then the ruled Ctrl+T, press, Ctrl+T.

## J6-3. The plan has no way into the encounter editor

- **Journey and step:** J6, the fight whose difficulty word the DM reads and changes (not counted in section 2.6;
  the task message asks for it).
- **What I saw:** the opened planned row's `Read encounter` opens the encounter in Lookup, a reader; the row's
  menu has `Remove from tonight` only. The DM goes to **Encounters**, `Edit` beside the encounter, searches (the
  search field has no focus, so click and type), `Add`, `Save encounter`, and **Session**: 7 acts and 2 page
  transitions. The word changed from `Low` to `Moderate` (fact: `the planned fights' rows say after the change:
  ["Moderate", "Low"]`).
- **Not a fault of the design:** section 2.6 does not walk this step. It is measured so that a later change can
  be weighed.
