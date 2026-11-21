# Sessions and the timeline

Use **Session** to write tonight's notes and plan fights. Use **Timeline** to track the order and duration of events.

## Plan a session

- Open **Session**. The plan shows completed events and fights still to come.
- Select **Fight** to add or create an encounter. Splat adds a link such as
  `- [[Bridge Ambush]]` under `Plan for tonight` in the note.
- Select **Start fight** to enter setup or **Resume** to open a paused fight.
  **Close run** returns to the page where you opened it.
- Open a plan row to read the event's title, length, place and notes or the
  fight's final HP and log.
- Use **Scene** in the Add strip's **More** menu to add a task line. Checking a
  scene saves the note.

**Fight**, **Group** and **Long rest** stay on the Add strip. At narrow widths,
**More Add actions** holds **Day**, **Short rest**, **Travel** and **Major event**.
Timeline uses the same strip without **Group**.

**Last time** opens the previous session. It shows the first line of
`What happened`, the recording recap, or the first line of the note, in that
order. Before tonight has any events or runs, select **Carry over** to bring
forward the recap link, unplayed plan lines and open threads. **Undo carry over** restores the
note. Read view shows the carried recap under `From <session>`.

The **Playing** label marks the active session. From another session, choose
**Return to the session being played**. If two notes share a session number,
Splat names both files.

### Starting HP and rests

A new fight starts each player at the HP from their last fight in this
session. A fight resumed in a later session also counts there. Conditions
stay with the fight. Select **Use the sheets' HP** to use sheet values instead.

Adding **Long rest** sets the next fight to start at full HP. To keep the
rest without that ruling, open it and choose **Mark not at full**. Starting
HP set in an encounter takes precedence when sync is off. When sync is on,
the first read can use the sheet's HP or raise a conflict if it differs from
your full-HP ruling.

## Edit notes and templates

Select **Read** or **Write** in the session note. At narrow widths the plan
and note have separate tabs. Read view hides empty headings and plan links
that already appear beside the note. Open **Templates** from the page header
and use **← Sessions** to return. Press **Ctrl+J** to open the current note
beside any page.

Splat saves notes and templates as Markdown in `Sessions` by default. To move
the folder, open **Campaign → Link and folder**. Splat counts the files that
would stay behind. Choose **Move files and save** to move notes, templates,
events, recordings and processing files, or **Save without moving**. The move
keeps existing files at the destination.

### Insert a link with a slash command

Type `/` at the start of a line or word. Choose **Search** to find an entry,
World note or heading. Choose **Generate NPC** and then **Save NPC** to create
and link a note. Escape closes the command without inserting a link.

### Manage planned fights

Use **Resume** to reopen a saved run. Its menu offers **Start a new run**
and **Delete the saved run**, with Undo for deletion. **Remove from tonight**
removes the plan link and offers **Undo remove**. Open a played fight for
its final HP, **Log**, **Whole run** and **Timeline** links.

## Adjust the timeline

**Timeline** saves events in story order in `Timeline.md` in the session folder.
The clock starts on the first **Day**. **Day** moves it to the start of the
next day. A long rest adds 8 hours and a short rest adds 1 hour by default.
Use a duration such as `3 days` or `an hour` to set another length. An unreadable
length shows a warning and leaves the clock in place. A fight leaves the clock
in place too.

- Open an event in the plan to edit **Title** and **Notes**, or follow
  **Timeline** to edit **How long** and **Where**. A new plan event starts closed
  and uses its kind as its title. A new Timeline event opens with **Title** focused.
- Use **Place after** to set the order. A Day shows only its start time.
- Use **Adds to** to choose a session. New events and fights go to the newest
  session by default. A fight adds its event when combat begins.
- If you start a session during a fight, choose **Move it here** to move that
  fight's Timeline event. **Undo move** stays until the next note or plan edit.
- Use **Copy link** to paste a reference such as
  `[[Timeline#^t-1a2b3c4d5e|Rest at the mill]]` in any note. Splat lists the notes
  that link to the event. After a rename, **Update links in notes** updates
  aliases that still use the old title. The Timeline list offers one Undo
  until the next note or event edit, or until you leave Timeline.

In Read view, select bold words that match an entry to open it in Lookup.
Bold words inside an explicit link follow that link. An empty note offers
**Write**.

## Resolve a changed or missing file

When a note or Timeline file changes on disk, Splat opens your draft beside
the file at the first difference. Use **Next difference** to move between
groups of changed lines. Choose **Overwrite with my draft** to keep your edits,
or **Reload and lose my draft** to use the disk version. You can still record
during the conflict. After a Timeline repair, Splat reads the file again and
applies the open fight's latest result.

Timeline saves preserve unknown properties and fields and comments outside the
event list. Saving may remove comments inside the rewritten list. See
[Timeline's file format](obsidian-file-layouts.md#timeline).

A moved draft can save at its new path. If you copy a session file, choose
**Make <file> a separate entry**. If a session or run folder is gone, choose its
folder before creating more files.

## Record and process a session

1. Tell the table before you record.
2. Select **Record** on the session. Select **Stop recording** to finish.
3. In **Settings**, choose a speech service and a service for **Transcript and summary**.
4. On the recording's row, select **Process**. Check the service destination
   below the action or on hover before you send it.
5. Review the generated text and suggested events. Use **Add** beside a
   suggestion to put it on the Timeline.

Splat sends audio for processing when you select **Process**. It uses
`pw-record`, `parecord`, or `arecord` to write 16 kHz mono WAV files under
`Sessions/Recordings/<session id>/`. If Splat closes unexpectedly, its recorder
stops and the file stays. A recording failure appears on its row. Hover for the
recorder's message. If the input is missing, choose **Choose an input**. If the
input is available, use **Record again**. When the WAV is missing, its results
stay listed as **Audio removed**.

If processing stops, select **Continue processing**. To replace a result,
choose **Process again…** and review the confirmation. A complete saved
transcript can be reused without sending audio again.

### What processing sends and saves

The speech service transcribes the audio and saves `<recording>.transcript.md`.
It needs an API key. If it stops partway through, **Continue processing**
resumes from the next part.

The transcript service cleans and checks the text, proposes Timeline events,
and writes the review and summary. The default uses the Codex CLI with your
Codex sign-in. Anthropic and OpenAI-compatible services are also available.
The raw transcript stays intact. Review the generated text before adding it
to your notes.

[Back to the user guide](user-guide.md).
