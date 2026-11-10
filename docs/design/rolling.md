# Rolling from the stat block and the sheet

Status: implemented. This is the fourth design; "History" says what the earlier
ones were and why they changed.

The DM rolls a creature's or a character's attacks, saves, checks and damage
without leaving the run. In a run, the selected row's **action strip** is one
fixed line above the dock: one chip for each of its actions. A chip rolls an attack and its
damage together; a click on a target's HP applies the damage. A save rolls its
damage once and every creature's save, shown on the rows. Outside a run, a
creature's reader and a character's Actions page have the same line as a
preview. The numbers in tables (an ability, a save, a skill) still roll.

The glossary names the two kinds of roll:

- A **standalone roll** in a run. It goes to the Log. It does not use the
  ability, spend a resource, or change a participant by itself.
- A **preview roll** outside a run, while the DM reads a definition. It changes
  nothing and is not saved.

## Principles

1. **Act from the roster.** The actor's actions sit just under the roster, above
   the dock, and the result lands on the rows it concerns (ux-intent principle 4). The side panel stays a
   reader: no result, mode or card is drawn over it or in it.
2. **Nothing floats, nothing moves.** Results have a fixed place, the strip
   above the dock, which keeps one height whether or not a result is there. It
   does not follow the selection, so no row moves when the DM selects or rolls.
3. **The printed text is the base.** A chip takes its numbers from the printed
   rules text (`splat::content::dice_text`): `Attack Roll: +4`, the damage
   groups of its hit, `Dexterity Saving Throw: DC 14`. Structured data is not
   consistent enough (114 of the 336 built-in creatures have no
   `attack_bonus:` line). A run's attack and damage changes are added.
4. **A roll changes nothing; a click on HP does.** Rolling never changes HP. A
   damage roll waits until the DM clicks a target's HP, which is one named step
   of Undo, as a typed amount is (principle 7, record, do not rule: there is no
   hit or miss verdict, and no automatic damage).
5. **Standalone rolls are history, not steps.** A roll is not an Undo step, and
   Undo never removes it from the Log. Rolls that belong to a step stay part of
   it, as before: initiative, group arrivals, marked-table rolls and loot rolls.

## The action strip

In live play, one line above the dock holds the selected row's actions. Next turn
selects the current creature, so its actions are the ones in view; selecting
another row, for a reaction or a legendary action, gives that row's. The roster
leaves room for this line, so it never covers or moves a row.

- **Chips.** One chip for each named rules paragraph with an attack, a save DC,
  or damage, in printed order: `Bite +4`, `Ember Burst DC 12`, `Claw 2d6`. A
  linked character's chips come from its sheet's attacks. The chips that fit
  show; `More` holds the rest. Click rolls; Shift+click rolls with advantage;
  Ctrl+click with disadvantage.
- **The result.** After a roll only the rolled chip stays in view, marked, and the
  others go in `More`, so the result has the line: the totals in Strong, such as
  `11 to hit` and `10 damage: 6 piercing, 4 fire`, with damage shortened to its
  total when room is short (types and dice in hover); `Natural 20` or `Natural 1`
  on attacks and death saves; then `adv`, `dis` and `crit`, which roll the action
  again that way, or `Full` and `Half` for a save; then how to apply the damage,
  while it fits. With no result, the line says how to roll.
- **The bonus field** keeps the right end: `+ 1d4`. It adds to the creature's d20
  rolls (attacks, saves, checks) as its own part of the result, for Bless or a
  cover penalty such as `-2`. It stays until it is cleared. A bad bonus says
  `Use dice or a number, such as 1d4 or -2.` and is not used.

### An attack

1. Click `Bite +4`. The attack and its damage roll together; a natural 20 rolls
   the damage as a critical hit (the dice twice, the bonus once).
2. Click the target's HP. The damage applies as one step of Undo, named as any
   damage is. The attacker stays selected, so the next attack of a Multiattack
   is one chip and one HP away.

While damage waits, every other present row's HP has a quiet frame that turns
accent on hover, and says `Apply 10 damage to Oren`. An attack's damage lands
once; after that the frames go. The attacker's own HP never takes it.

### A save

1. Click `Ember Burst DC 12`. Its damage rolls once, for every target (roll
   scope). Every present creature other than the caster rolls its save, by its
   Save bonus or else its ability modifier, and the result stands where that
   row's AC is: `DEX 14`. A linked player's sheet rolls its own save at the table.
2. Click the HP of each target. `Full` or `Half` on the strip says what a click
   applies; Half rounds down. Each target takes it once.

### The table's numbers

The attribute table's Mod and Save cells, the Skills row, and a sheet's
abilities, saves and skills are accent numbers with a quiet underline, at least
24 px each way. In a run their result goes to the strip above the dock; outside
a run, to the entry's preview line. A death save has a `Roll` link beside its
counts. Prose has no dice links.

### Outside a run

A creature's reader shows the same line under its stat head, and a character's
Actions page shows it above the attacks. A roll there is a preview on its own
dice: it changes nothing and goes to no Log. There are no HP frames.

## Engine and data

- `Dice` has keep-highest and keep-lowest (`2d20kh1`, `2d20kl1`), so advantage is
  an ordinary expression. The dock's dice field accepts it too.
- `rolls::make` rolls an action: an attack and its damage, a save's damage, or a
  single d20 or dice roll, with the run's changes and a bonus. Each roll keeps
  every die, which were dropped, the natural d20 and the total; the Log line and
  the strip are made from it.
- `Run::record` writes a roll's Log lines in `state.log` and in every state in
  `history`, and moves the dice there too, so Undo keeps the lines and never
  repeats the dice.
- The run keeps the selected row's last action (`Turn`): what rolled, which rows
  took its damage, Full or Half, and each creature's save. Next turn, another
  selection, or Undo that removes its actor ends it. Bonuses are kept by row and
  are not saved with the run.
- Chips come from `rolls::chips` over the rules text: each paragraph with a
  run-in title, skipping printed quotes, tables and headings. A test checks every
  built-in creature: each `Attack Roll:` and each damage group is found.

## Tasks

The `run-rolls` visual scenario measures these:

| ID | Task | Target |
| --- | --- | --- |
| T16.1 | Roll the selected creature's attack and its damage | 1 |
| T16.2 | Apply the damage by the target's HP, with the attacker still selected | 1 |
| T16.3 | Make a second attack and apply its damage | 2 |
| T16.4 | An area save against two creatures, one taking half | 4 |
| T16.5 | Roll the attack again with advantage | 1 |
| T16.6 | Roll an attack with a Bless bonus | 3 |
| T16.7 | Preview an attack with advantage in the Library by pointer | 1 |

## History

1. **Dice links in prose, a roll strip above the side panel, and a call view.**
   Every printed number was a link; results popped over beside the link and
   stayed in a strip above the panel with `Use` and `Back to`; a save DC opened a
   view that replaced the panel. Review: a click on the target's row replaced the
   attacker's text, `Use` was far from Amount, and the call view was a second
   roster without HP.
2. **A held panel.** An open result held the panel on its roller; saves showed
   on the rows; damage filled Amount. Review: the result cards still covered
   the text they were about, the hold line moved the stat block, and one roll
   showed in three or four places.
3. **A richer right-click menu** (header, click hints, damage, bonus). The user
   found it worse and reverted it.
4. **This design.** Two independent concept reviews (Fable and Opus) both
   recommended taking rolling out of the prose and the panel. The user chose the
   turn strip. Built under the selected row, it moved the rows below it on each
   selection and took the room 12 rows need at 960x640, so it became one fixed
   line above the dock.

## Later

- Keyboard: focus on chips, and keys for the HP targets.
- Recharge: `(Recharge 5–6)` rolls a d6 against the range.
- Multiattack: one press for all of its attacks. Its structured data has at
  least three shapes.
- Clearing the targets at or above the DC, for features that do no damage on a
  success.
- Reminders of conditions that can change a roll. These need a table of which
  conditions affect which rolls, and come close to a ruling.
- Upcast spell damage.
- Sending rolls to the D&D Beyond game log, opt-in. The write path is not
  verified (`docs/research/dndbeyond-api.md` covers only reading).

## Out of scope

Automatic hit and miss decisions, automatic damage, resistance and condition
rules, spending resources, 3D dice and sound, and a player-facing view. These
belong to automatic combat resolution, which remains later work.
