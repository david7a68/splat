# World notes, trackers and loot

## Find and edit a World note

- Open **World** to browse campaign notes. Press Ctrl+K to search from any page.
  Follow Markdown and Obsidian links in the reader. If several files match a
  link, choose one. If none match, Splat offers to create a note.
- Use **Edit in Obsidian** to edit a file. Splat reads changes when you return.
  For a typed entry, use **Edit** to change **Status**, **Stat block**, **In**,
  **Part of** and the text. **Save changes** offers **Undo changes**.
- If the file changed on disk, compare your draft with the disk version.
  **Overwrite with my draft** applies your changed fields over the disk version.
  **Reload and lose my draft** uses the disk version and offers **Undo reload**.
  If **Part of** is invalid, fix its group links in Obsidian. You can still save
  other fields.

World includes campaign Markdown notes. Session notes, templates, recordings,
tracker and Timeline files, and files in hidden folders have their own views or
stay out of World. Unreadable notes appear under **Cannot be read**.

## Create an NPC or location

- Select **New NPC** or **New location** in Lookup to use the search name.
  Splat saves the note in `NPCs/` or `Locations/`. To give a plain note a type,
  select **Make world entry**.
- Select **Generate NPC** to roll a name, look, manner and want. Use **Reroll**
  to try again. Check the path and select **Save NPC** to write the note.
- Use **Change** beside **Stat block** to give an NPC combat stats. An NPC
  with a stat block can join the open fight through **Add to fight**. Splat
  uses the NPC's name and leaves the encounter template in place.

To use your own generator tables, mark tables in World notes with the keys
`npc-name`, `npc-look`, `npc-manner` or `npc-want`. The first table in path
order wins. Missing or invalid tables fall back to the built-in lists.

## Make a table rollable

Put a roll marker above the table. Use numbers or ranges in the first column
and cover every result the dice can roll.

```markdown
<!-- splat:rolls key=gifts dice=1d6 -->

| d6 | The crowd throws |
| --- | --- |
| 1-3 | A knife |
| 4-6 | A potion |
```

Select **Roll** or type a result from your own dice. During a fight, the result
goes in the run log and can be undone. In Timeline, **Roll for encounters** on
a Travel row uses the campaign's marked encounter tables. Results stay with
that journey. If a result links an encounter, you can add it to the plan.

## Set up a house rule and its trackers

1. Create a `rule` note with `implementation: splat:house_rules`. Define the
   trackers in a `splat:yaml path=splat_members` block.
2. In **Campaign → Rules and runs**, enable the note under **House rules in
   play** and save.
3. Press Ctrl+T to open **Trackers** beside any page. Press it again to return
   to Lookup.

Use the quick actions to change a tracker. A tracker with `per` has one line
per name. A value that crosses a mark raises a **Due** notice. Review the
suggested rolls and changes. Select **Done** to apply the checked changes or
**Dismiss** from the menu. [Example house-rule note](design/world-examples/heralds-tournament.md)

Day marks use the Timeline clock. In `day_marks`, `at: 721` means the start of
Day 721. `every: 30, from: 2` means Day 32, Day 62 and so on. If several days
are due, **Next** handles the oldest. **Dismiss all** clears the row. Splat
pauses day notices if it cannot read the Timeline or if its file has changed
during an unsaved edit.

Tracker actions save to `Trackers.md`. Use **Undo** to reverse the latest action
in the session, even after restarting. To remove an older action, open the
tracker's history in its house-rule note. If a later notice depends on the old
action, remove the dependent entry first. If a save fails, resolve the error
before trying again.

## Prepare and award loot

1. In the encounter editor, use **Add loot** below the roster. Type a reward
   such as `30 gp`, choose a Library item, or select **Roll on a table**. For
   loot carried by one creature, open **Changes** on its roster or group row.
2. When a creature drops to 0 HP, open its loot from the run. After the fight,
   the ended view lists the encounter's loot and the loot of creatures at 0 HP.
   Select **Roll all** to make the remaining rolls as one Undo step.
3. Select **Copy loot** for a Markdown list. From the session plan, select
   **Add rewards** to add the fight link, XP and loot to **Loot and rewards**.
   **Undo rewards** removes that block. Delete the XP line from the latest
   block to leave XP out of future blocks.

Splat saves encounter loot in the note under `## Loot`.

```markdown
## Loot

- 30 gp
- [[Potion of Healing]]
- Roll 2× gems
```

[Back to the user guide](user-guide.md).
