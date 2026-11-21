# Encounters and runs

## Build an encounter

1. Open **Encounters**, select **New encounter**, and enter a name.
2. Select **Add party** or search for creatures. Search also finds NPCs with
   a stat block. Select a name to read its stat block in Lookup.
3. Add participants and choose their sides. **Party** and **Enemies** are
   hostile to each other. Choose **Other…** to name your own side.
4. Select **Save encounter**. This writes a Markdown template in `Encounters/`
   in the campaign folder.

Use a roster row's **⋯** menu to move, duplicate, remove or **Make it yours**.
Open **Edit** to set a display name, stats, starting HP, temporary HP,
conditions or a note for that participant. These changes stay with the roster
entry. [Roster change fields](design/campaign-schema.md#overrides)

After refreshing a linked **Party**, use **Reload** in Encounters to pick up
new local copies. If a roster entry cannot be read, follow its repair link or
remove the entry before starting.

## Prepare an encounter group

An encounter group is a set of creatures you can add together during a fight.

- Select **New group** in Encounters, or select **Group** in the Session plan
  and then **New group**. Add creatures and set their counts and changes.
- Use **Belongs to** to assign the group to one encounter or make it available
  to any encounter. A group added to the plan stays linked there.
- If some members are missing or invalid, check **Cannot join** for the
  reason and repair link. Arrivals can still add the usable members.
- Use **Edit** to change a saved group. In the encounter editor,
  **Move to…** or **Make available to any encounter** changes its owner on
  the next save. [Encounter group format](design/campaign-schema.md#encounter-groups)

## Running an encounter as DM

### Set initiative and begin combat

1. Close any other open run. Select **Start fight** or **Resume** beside the
   encounter. **Start fight** copies the roster and its values into the run.
   Saving the encounter later updates its open run’s **Prep** text. Combat
   values stay as they were.
2. Enter or roll initiative. Use **Tie** and the arrows to order tied
   participants. A shared roll needs no tie ruling.
3. Use **Correct…** in the row menu to fill in missing HP.
4. Select **Begin combat**. Select **Next turn** to advance. Participants at
   zero HP stay in the order until you mark them absent. Absent participants
   keep their initiative and state when they return.

To change initiative during combat, select the total, type the new value and
press Enter. Escape keeps the old value. The current participant keeps its turn.
Moving the current participant takes effect after **Next turn**.

### Record damage, conditions and effects

- Select a row, type the final amount and press Enter to apply damage.
  **Heal** and **Temp HP** are beside it. Ctrl+click adds targets. Use a row's
  half mark to round its damage down for a save.
- Select **Conditions** to set official conditions. Select a condition's
  name on the row to open its rule. Use **Correct** for HP and initiative
  corrections, notes or sync. At zero HP, party members can record death
  saves on their cards.
- Use **Effect** or **Other effect…** to write a reminder such as
  `Bless 10 rounds`. A duration counts down at the end of that participant's
  turn. `on its turn` repeats a reminder. You choose when to remove the effect.
- Enter a dice expression such as `2d6 + 3` in **Dice**, then select **Roll**.
  The result goes in the log. Undo applies to the preceding change. The roll
  stays in the log.

You resolve attacks, damage modifiers, condition effects, Concentration,
death and recovery. Splat records your decisions.

| Key | Command |
| --- | --- |
| N | Next turn |
| Enter or Shift+Enter | Damage or heal by the typed amount |
| U or Ctrl+Z | Undo |
| Up or Down | Select another row |
| Ctrl+click | Add a row to the targets |
| Escape | Select the current turn |
| A | Focus the amount |
| C | Conditions |
| E | Effect |
| Ctrl+G | Add to fight |
| Ctrl+K | Toggle Lookup |
| Ctrl+T | Toggle Trackers |

### Add a creature or group

Open **Add to fight**. Search for a creature or enter a name and HP for a
creature that exists only in this run. For a group, choose **Add** or drag it
onto the initiative list. Enter a total to use it for the whole group. Clear
the field to roll using the campaign's initiative rule.

A tied arrival offers **Move before**. The current turn stays active.
Undo removes a group as one step.

### Close, end or continue a fight

- **Close run** returns to the page where you opened the fight. If a save is
  pending or a change is unsent, review the choice in the dialog. Continue
  an unfinished fight with **Resume**, in the sidebar or in **In progress**.
- **End encounter…** records your outcome. Open **History** to read an ended
  run, or follow **Whole run** from the Session plan.
- Use **Undo: end encounter** while the ending is the latest step. Use
  **Continue fight** on the ended view or in History to return to the last
  turn and keep recorded loot. **Undo: continue fight** restores the ending.

**Continue fight** requires a readable Timeline, the fight's own Timeline
event, and no later fight or long rest. Close other runs first. If a later
fight or rest has happened, start a new run to play this one again.

## Run storage and campaign initiative

In **Campaign → Rules and runs**, choose the run folder and the NPC
initiative rule. The folder defaults to `Encounter Runs`. Close the run before
changing the folder. If the old folder contains runs, choose **Move runs and save**
to keep them available. **Save without moving** leaves the files in the old
folder and takes those runs out of the list.

Runs save as JSON. If the run file has changed on disk, choose **Replace the
file with this run** or **Save as a new run**. The new copy keeps your changes
without replacing the disk version. Undo usually survives close and restart.
If old steps cannot be read, the run still opens and tells you that Undo
starts again from there.

For NPC initiative, choose identical NPC groups, individual rolls, one
group per side, or the encounter's initiative groups. **Roll NPC initiative**
uses the campaign's choice and leaves player totals in place. Use the arrow
beside it to change the rule for one roll.

## D&D Beyond combat sync

### Choose what goes to character sheets

**Sync party with D&D Beyond** is on by default for encounters in a linked
campaign. With sync on, players start from their sheets. Changes to current
and temporary HP, death saves, official conditions, spent spell slots,
Heroic Inspiration and supported limited uses can go back to those sheets.
Concentration, custom effects, initiative, inventory charges and notes stay
in Splat.

For a test or flashback, turn the encounter option off. The run then starts
from local copies and starting overrides. In a run, select **Sync party**
to start sync for linked players or **Stop sync** to stop it. The **Correct**
tab has controls for each player. An ended run can finish pending sync
work. If you continue the fight, sync resumes if it was on.

### Review conflicts and unsent changes

- **Sync read-only** means D&D Beyond allows reads for that character. Your
  changes stay in Splat. To change Maximum HP in a run, stop sync first.
- On the first read after starting, reconnecting or reopening Splat, an
  unchanged run value takes the sheet value. When you changed a value and
  the sheet differs, open the conflict and choose which value to keep.
- A sheet change to HP or conditions can remove Undo steps for that character.
  The log names removed steps when an earlier action can no longer be undone.
- If the connection ends, **Not sent** marks pending changes. Wait for sync
  before closing, or turn it off to cancel queued work. Requests already
  sent can still finish. Unsent spell slots and limited uses can be lost
  when you close the run.

Live writes have yet to be verified with an authenticated character.

[Back to the user guide](user-guide.md).
