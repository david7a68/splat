# What the journey scripts found

Each journey script (`src/visual_tests/dm_audit/journeys.rs`) plays the real app. Where a step could not be
done as section 2 of `docs/design/session-cycle.md` walks it, or cost more, or the app showed a wrong state,
it is below, and the script takes the route a DM would have to take, so that the measure is true. No app code
changed in this pull request. The journeys were first played on `main` at `9d3e0677`, frozen after the
merge of `main` at `bd7ff4b0` (pull request 280), frozen again after review round 1, cost the same after the
merge of `main` at `19cec3d2` (pull request 282), and were frozen again after the merge of `main` at `53bb0156`
(pull requests 284, 285 and 288), where J4 costs 2 more (J4-2).

Each entry is one of three kinds:

- **App fault:** the app does something wrong, or the design's promise does not hold.
- **Limit the design accepts:** the cost is above the walk, but section 1.2 or 7 of the design states it. The
  cost stays measured and visible; it is not a fault to repair here.
- **Not walked by the design:** a task the journey needs that section 2 does not count. Measured, so that a
  later change can be weighed.

Every outcome that a journey states is an assertion on the saved file, read after the act (`Audit::outcome`).

## J4-2. After a press, a bell's first roll is below the view (app fault, since pull request 285)

- **Journey and step:** J4 steps 3 and 13, at 1200x800, Trackers open beside the Session page.
- **What I did:** `Enter room` (step 2), and `Collect soul` (step 12): each raises a bell, whose Due row lists
  `Roll Rival Track` for each rival. Then the three rolls and the wandering roll.
- **What I saw:** since pull request 285 the view holds the pressed action where it was after a press, so the
  new Due row's first `Roll Rival Track` is below the view. The driver scrolls it into view: 1 more act at each
  bell. On `19cec3d2` the bell's rolls cost 4 and 5, as the design says.
- **The driver's words:** `J4/steps.tsv`: the first `Roll Rival Track` of each bell takes 2 interactions.
- **Route measured:** 5 against 4 for step 3, and 6 against 5 for step 13: steps 1 to 13 cost 27 against the
  design's 25 and paper's 25. The design counts a scroll only at case D (section 2.4).

## J3-3. With unsaved words and a changed file, the comparison takes the note's place on the Session page (not walked by the design as measured)

- **Journey and step:** J3 step 6, at 960x640 with Lookup open on the note found by `miller`.
- **What I did:** during the fight the DM typed a line in the note (Ctrl+J, not saved yet); Obsidian then added
  a plan line. After the fight, on the Session page: `Add rewards: Wolf Pack`.
- **What I saw:** before Splat's save after a pause, the act says `The note changed on disk. Choose in the
  note first.` and writes nothing (asserted: the file is byte-equal to Obsidian's). Once that save meets the
  changed file, the rail says `Notes changed`, and the Session page shows the comparison with its choices in
  the plan's place; Lookup stays open on the found note. The DM chooses `Overwrite with my draft` and presses
  `Add rewards` again.
- **Route measured:** the refused act 1; the choice and the act again 2. The design walks this step in the run
  (Ctrl+J, which closes Lookup; one choice; Ctrl+K; the note again: 4). On the Session page the choice is in
  view and Lookup stays, so the measured route costs 2 after the refusal, not 4.
- **Known limit kept:** the choice keeps one version of the lines both sides changed (section 7). The saved note
  holds the DM's line and Obsidian's first edit, and not Obsidian's second line (asserted).

## J6-2. Lookup's reader of an NPC has no `Change` on Status (app fault against step 2)

- **Journey and step:** J6 step 2 (Verori died here), at 1200x800 in the ended view, Lookup open.
- **What I did:** Ctrl+K, typed `verori`; looked for **Change** on Status, as step 2 says.
- **What I saw:** Lookup's reader is compact: it shows `NPC · alive · Campaign`, `Edit Verori`, `Add to Log` and
  `Add to fight`, and no Status row with `Change` (`src/world.rs`, the rows are drawn only when the reader is not
  compact). The status goes by `Edit Verori`, the Status field (click, select all, type) and `Save changes`.
- **The driver's words:** `journey-j6: no visible node for control "Change Status".` The Edit route still
  writes the status and the stamped Log line (asserted on `NPCs/Verori.md`).
- **Route measured:** Edit: 7 against the design's 5 (2 more).

## J3-1. At 960x640 with Lookup open, an effect with a time goes by the Conditions tab (limit the design accepts)

- **Journey and step:** J3 step 2 (pin `Poisoned 3 rounds` to the wolf), case D.
- **What I saw:** the dock's second line has no room for `Effect`, and `E` is off while Lookup is open. The way
  is the **Conditions** tab and its `Other effect` field, and that tab takes the panel's place, so Lookup
  closes with the note the DM had found in it.
- **Not a fault:** section 7 states it: "`Effect` by the pointer costs 5 when the dock has no room for it and
  Lookup is open", and section 2.5 row 15 keeps `Other effect` for this case. Section 1.2 keeps Lookup closed
  by the DM's own act only; here the act is the tab.
- **Route measured:** select, **Conditions**, click `Other effect` and type, Enter: 5, where the walk of step 2
  implies 4. After the crash that follows, step 3 opens the note again anyway, so step 3 costs what it says.

## J4-1. A fight started beside Trackers keeps Trackers open in the run (limit the design accepts)

- **Journey and step:** J4, the arena fight's start, at 1200x800.
- **What I saw:** with Trackers open beside the plan, `Start fight: Shrine of Cinders` opens the run with Lookup
  still on Trackers, in the place of the fight's panel. The DM presses Ctrl+T to see the fight's panel.
- **Not a fault:** section 1.2 says a side panel stays until the DM closes it, and the owner's ruling (section
  7) puts trackers in a fight behind Ctrl+T. Closing Lookup on `Start fight` would break section 1.2, so this
  is not proposed. The cost stays visible: 1 more at the fight's start.
- **Route measured:** one Ctrl+T at the fight's start (Panel 1); in round 1 the ruled Ctrl+T, press, Ctrl+T;
  in round 2 the seal's notice is due (the rail says `Due 1`), and the Old Guard's group joins by `Add to fight`
  and `Add Old Guard, 1-2 seals to the initiative list` (2). After the fight, step 8's Ctrl+T, as the design
  says.

## J6-3. The plan has no way into the encounter editor (not walked by the design)

- **Journey and step:** J6, the fight whose difficulty word the DM reads and changes.
- **What I saw:** the opened planned row's `Read encounter` opens the encounter in Lookup, a reader; the row's
  menu has `Remove from tonight` only. The DM goes to **Encounters**, `Edit` beside the encounter, searches (the
  search field has no focus, so click and type), `Add`, `Save encounter`, and **Session**: 7 acts and 2 page
  transitions. The saved encounter holds the Oni, the row's word changes from `Low` to `Moderate`, and the
  next run holds the Oni (asserted).

## J2-1, J6-1. After `Roll for encounters`, the focus was not in `How long` (gone on `main` `bd7ff4b0`)

- **What I saw on `9d3e0677`:** with one marked table in the campaign, nothing had focus after `Roll for
  encounters`, and the DM clicked the field first: 1 more act in each journey.
- **Now:** after the merge of `main` at `bd7ff4b0` (pull requests 280 and 281), the focus is in `How long`, and
  both steps cost what the design says. The script keeps both routes, so `summary.tsv` says where the focus
  is, and G6 goes red if it moves again.

## J2-2. The fight that a road check names was on the day after the journey ended (repaired by 284)

- **What I saw on `19cec3d2`:** the Travel row of `2 days` starts on Day 1, and its check at `24h` names the Wolf
  Pack. `Plan “Wolf Pack”` and `Start fight` put the fight after the whole journey, so `Timeline.md` had it on
  Day 3.
- **Now:** on `53bb0156` (pull request 284), the fight is on Day 2, and J2 asserts it on the saved
  `Timeline.md`.

## J3-2. A removed scene's `Undo remove` went away when the next fight began (W6-D1; repaired by 280)

- **What I saw on `9d3e0677`:** the plan had no `Undo remove`, and the scene's line stayed out of the note.
- **Now:** on `bd7ff4b0` (pull request 280), `Undo remove` is on the Session page after the fight began: 2
  acts, with the transition back to the Session page. The plan section is byte-equal to what it was before
  the removal, with the scene at its old place and the lines Obsidian added (asserted).
