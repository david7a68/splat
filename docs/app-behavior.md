# App behavior

This reference describes how Splat behaves across pages. The
[design system](design-system.md) defines the layout rules. The
[user guide](user-guide.md) gives the steps for each task.

## Navigation and narrow windows

- The campaign switcher stays above the sidebar pages. The Run slot shows the
  open run or the unfinished runs. Each page keeps its state when you leave it.
  Splat reopens on the last page or the open run.
- **Lookup** (Ctrl+K) opens beside any page. It keeps up to five results above
  the reader and marks the open one. **Show all** opens the full list in place.
  Use Alt+Left and Alt+Right, or the mouse Back and Forward buttons, in Lookup
  and Library.
- While a fight is open, other pages show the turn, **Next turn** and
  **Back to run**.
- Drag a pane divider to resize it. With its divider focused, use Left or Right
  to resize it in 16 px steps or Home to reset it. Splat remembers your widths.
- At narrow widths, the Session plan and note have separate tabs. The same
  **More** menu holds overflow actions on Session and Timeline. Party tabs and
  Library reader actions also move into **More** when space runs out.

## Notes, readers and search

- Read and Write share a left-aligned text measure of at most 75 characters.
  Read hides empty headings and links already shown in the plan. Stored line
  breaks and the visible words of links stay intact.
- A scene is a plan task. Checking it saves the note. Scenes fold together
  when they would push the next fight out of view. A scene linked to a creature
  or NPC can start a fight when no run is open. A checked scene linked to a
  World entry can add a line to its Log, with Undo.
- A bold phrase opens Lookup when it resolves to an entry. Markdown and wiki
  links keep their destinations. Tab reaches each link. Enter or Space opens it.
  Each link has its own target, at least 24 px square.
- Creature filters are available in Library browsing, Library search and
  encounter search. Filter by habitat, type, size, alignment, challenge, movement,
  senses, languages, resistances, immunities or vulnerabilities. Remove a filter
  by selecting its chip. Search also accepts terms such as `darkvision`,
  `habitat:forest`, `type:dragon`, `cr:5-10`, `cr:5+`, `immune:fire` and
  `lang:"deep speech"`. Habitat `Any` matches every habitat.
- Stat block readers show each set value once. A creature's CR supplies missing
  XP and proficiency bonus. Mismatches get a warning unless they describe a
  lair. Built-in creatures get habitat data from `scripts/srd_inputs/habitats.yaml`.

## Fights and rests

- Splat counts missing initiative and ties across the whole roster. Shared
  rolls need no tie ruling. Unread roster rows block **Start fight** and
  **Begin combat**, including starts from the plan and resumed setups.
- **Correct** opens a participant's values and leaves sync as it was. Narrow
  run rows keep the first condition. A shortened turn heading keeps the
  creature's copy number.
- Session and Timeline use the same fight and rest states. An open fight's round
  links back to the run. A long rest's row shows **Everyone at full** or
  **Not at full**. Change the ruling on either page, with Undo.
- After a long rest, saved fights past Setup keep their HP. A resumed Setup
  starts at full HP. **Use the saved HP** restores its previous values and
  offers **Undo use saved HP**.

## Recordings and messages

- The sidebar has one slot for recording or processing. Select it to open the
  recording. The recording row keeps a failure or stop after you dismiss a
  banner. A missing input links to microphone settings.
- **Continue processing** and **Process again…** say what they send. If a
  transcript is complete, they can reuse it. If it is partial, they name the
  remaining parts. Replacing a processed result requires confirmation.
- The sidebar keeps **Processed** and any warning until a later job replaces the
  result. Opening the session clears the unread notice.
- The run notice leaves room for Undo. The short label keeps the action, amount
  and copy number. Hover for the full text. Sync conflicts appear before
  recording silence, which appears before connection notices.

## Files and recovery

- On window focus, Splat checks campaign notes and registered packages for disk
  changes. Library, Lookup and encounter search reload changed content. A draft
  opens a comparison at the first changed line, with two lines of context.
- A broken or missing package manifest leaves the package's entries available.
  The warning stays on **About**. Campaign names the fault and links to the
  Library repair. A file Splat cannot read at all offers **Show file** and
  **Try again**. Splat keeps the file and any open draft.
- A missing roster creature keeps its last known name, or shows **Missing creature**.
  **Replace…** offers a creature search with CR, HP and AC. **Replace all** starts
  on for rows that use the same missing entry. Replacement keeps row names and
  starting values and offers Undo.
- Typed World notes keep their identity when moved. Links by name also work
  after a plain note moves. If several files match, choose one or select
  **Always use this one** to remember it.
- Before a World group change, Splat checks **Part of** on disk. If the field is
  unreadable, it shows the source line. Saving other fields keeps that group
  text intact.
- Repairing duplicate IDs names the file and the encounters that change source.
  A copied package or campaign can register with a new identity. Copy
  registration can also switch encounter and group sources to the copy, keeping
  their names, counts and changes. Each repair offers Undo.
- A file-writing result loses its Undo when the file changes on disk. A copy
  registration result can stay if it has another action, with an explanation
  for the disabled Undo.
