# Data and state safety

For any change that reads, writes, converts, moves, deletes, syncs or undoes DM data. Splat never drops data
a DM wrote without telling them.

## Walk every path

List each entry point the change reaches (keyboard, search, live-run Add, drag and drop, menu, repair, Undo,
close dialog, campaign switch) and each risk state (read-only or unsent typed values, an outside edit, the file
moved, renamed or deleted while open, an unreadable or non-UTF-8 file, duplicate ids or names, two faults at
once). Give the result for each pair you can reach. A missing pair is a finding when it touches a save path.

## Files

- An unreadable file does not stop the rest from loading; the message names the file and the line, in plain
  words. A raw `io::Error` or UTF-8 error goes to technical details, never the main message.
- A writer of a file the DM can also edit keeps every key and field it does not own. Judge write claims by
  the raw saved bytes of the reported fixture, never by parse output. If parsing drops a field, editing that
  field is blocked with a reason and saving other fields keeps its raw text.
- Overwrite compares the disk with what Splat last wrote and shows a like-for-like comparison (same format,
  same normalisation), or it shows noise.
- A deleted manifest is offered back ("Write X again"), never silently replaced with defaults.
- Format migration happens only on a write the DM started, never at load, poll or touch, never in read-only
  or generated packages. The old reader stays and stays tested; a malformed line is flagged, not dropped;
  an edit changes one cell in place; generated packages stay byte-identical.
- A guard keys on a checksum, not on layout; the old bytes are archived before Replace.

## Undo, results and drafts

- One action gives one message and one named Undo where the DM acted, never a second bare Undo. Undo of a
  converting save gives back the old bytes.
- A result whose files changed loses its Undo. A newer result replaces an older one: check that an older
  Undo cannot hide it (PR 202: an edit's stale Undo hid `Note repaired.`).
- State kept for one object is cleared only for that object (PR 202: repairing note B cleared note A's
  rename Undo).
- A failed save, a conflict, Undo and Discard keep the typed draft. A Discard with no Undo is destructive.
- A background timer or scan never clears a pending Undo or reopens a draft under it (PR 202: the 5 s
  Sessions folder scan dropped `Undo discard`). Ask for a test that forces the event, not one that depends
  on timing.
- A value kept in egui temp data or other frame-wide state is keyed to the pass and panel that set it, or
  a reader drawn later inherits it (PR 204: a Sessions note took the document id that Lookup had set, and
  its link lost focus).

## Game state and sync

- A recorded value changes only through a conflict the DM settles or a named Undo. Trace close, reopen,
  crash and stop.
- Each part of a combined command passes the same guards as a single command (PR 197: the max-HP check ran
  only on a top-level `Health`, not inside `Several`). A refused step changes nothing.
- A message about an event uses the event's own id and session, not the selected one.
- An interrupted gesture (panel closed while dragging, focus moved, a sync arriving mid-gesture) commits or
  cancels on purpose; say which.
- A dialog promise such as "Nothing is lost" holds for every field.
