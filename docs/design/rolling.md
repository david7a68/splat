# Rolling from the stat block and the sheet

Status: implemented. This is the fifth design; "History" says what the earlier
ones were and why they changed.

The DM rolls a creature's or a character's attacks, saves, checks and damage
without leaving the run, as in a tactics game. A stat block's or a sheet's
actions are **tiles**. In a run a tile arms its action: each row shows the
chance to hit it, and a click on a row rolls against it and puts the damage in
Amount, which Enter applies. Players roll their own dice: a party member's
attack only targets the row, and the DM types what the player says. Outside a
run a tile previews. The numbers in tables (an ability, a save, a skill) still
roll.

The glossary names the two kinds of roll:

- A **standalone roll** in a run. It goes to the Log. It does not use the
  ability, spend a resource, or change a participant by itself.
- A **preview roll** outside a run, while the DM reads a definition. It changes
  nothing and is not saved.

## Principles

1. **Act on the rows.** An armed action targets roster rows, and its results show
   on them: the chance to hit, `11 vs AC 13 · 10 damage`, or a save in the AC
   cell (ux-intent principle 4). Damage goes into the dock's Amount, and the dock
   applies it as it applies any typed amount.
2. **Nothing floats.** No card or popover covers text. The armed action's status
   and next steps are under its tiles.
3. **The printed text is the base.** A tile takes its numbers from the printed
   rules text (`splat::content::dice_text`): `Attack Roll: +4`, the damage groups
   of its hit, `Dexterity Saving Throw: DC 14`. Structured data is not consistent
   enough (114 of the 336 built-in creatures have no `attack_bonus:` line). A
   run's attack and damage changes are added.
4. **Record, do not rule.** A roll never changes HP; Enter does. There is no hit
   or miss verdict; the chance to hit is arithmetic. A rolled attack puts its
   total beside the target's AC, `17 vs AC 13 · 10 damage`, and Amount gets the
   damage either way, so the DM rules on cover, a Shield spell and the like.
5. **Players roll their own dice.** For a party member, a tile targets but does
   not roll; `Roll for them` rolls if the player wants. A save's character
   targets roll at the table, and the DM uses their half mark.
6. **Standalone rolls are history, not steps.** A roll is not an Undo step, and
   Undo never removes it from the Log.

## Tiles

In a stat block, each rules paragraph with an attack, a save DC or damage is a
tile, in printed order; paragraphs next to each other share one row of tiles.
The paragraph's text is in the tile's hover and keyboard focus, and in its
accessible description, not under it. Spellcasting stays prose, with its spell
links. A character's Actions page shows its attacks as tiles. A tile has an icon
(sword for an attack, shield for a save, fire for damage), its name, its number
in a run (Ctrl+1 to Ctrl+9 arm it), its numbers (`+4 · 2d4+2 piercing, 1d6 fire`
or `DC 12 Dexterity · 2d6 fire`), and, when the text says, where it lands and
what a save does (`Reach 5 ft`, `10-ft emanation · half on a success`). Damage
with a condition follows as `or 2d6+2 slashing` or `plus 1d4 slashing`. In a
run, a creature changed for the encounter shows the numbers Splat rolls (`+4 ·
1d6+3 slashing` for Attack +1 and Damage +1); hover adds the printed ones.

A right-click on a tile or on a number that rolls a d20 opens a menu under it,
headed with what it rolls (`Bite`, `Dexterity save`): `Roll`, `Roll with
advantage`, `Roll with disadvantage`, and, by a campaign rule that gives
advantage for inspiration, `Roll with inspiration (advantage)` when the
character has it.

### An attack

1. Click `Bite` (Shift: advantage, Ctrl: disadvantage), or press Ctrl+1. It is
   armed: its tile is marked, the panel stays on the attacker, and every other
   row shows the chance to hit it, such as `60% to hit`. Armed with advantage or
   disadvantage, the hint and each row say so: `With advantage. Click a target…`
   and `84% to hit · advantage`.
2. Click the target's row (Shift or Ctrl for advantage or disadvantage). The
   attack and its damage roll; a natural 20 rolls critical damage. The row says
   `17 vs AC 13 · 10 damage` (or `11 vs AC 13 · 10 damage`), the row
   becomes the dock's target, and Amount has 10. Enter applies it, as one named
   step of Undo.
3. The tile stays armed: click the next target for a Multiattack. Escape, `Stop`
   or Next turn disarms it.

A right-click on a target row opens a menu under the row, headed `Bite against
Cinder Wolf 2`: `Roll`, `Roll with advantage`, `Roll with disadvantage`. For a
save it rolls that creature's save another way; for a player's attack it rolls
for them. Shift+F10 on the focused row, tile or number opens the same menu, so
every choice in it, the inspiration reroll too, works from the keyboard.

### Damage that depends on a condition

The printed text can put a condition on damage: `or 6 (1d8 + 2) Piercing damage
if the target is Bloodied` replaces the damage, and `plus 2 (1d4) Slashing damage
if the attack roll had Advantage` adds to it. The run settles what it can tell:
the roll's advantage, and whether the target or the attacker is Bloodied. For
anything else, such as `if the goat moved 20+ feet straight toward the target`,
the click on the target opens a question under the row before anything rolls:
the condition, then `Yes · 2d8+2 bludgeoning`, `No · 2d6+2 bludgeoning` and
`Cancel`. In the built-in creatures, 16 attacks have such damage; 3 need the
question.

### A save's damage

A save's damage is what a failure takes at once: the damage in the first
sentence after `Failure:`. Damage in a later sentence is another effect at
another time (`takes 10 (3d6) Acid damage at the start of each of its turns`),
so it is not rolled with the save; it stays in the tile's hover. Before this,
26 of the 143 built-in saves with damage rolled something else, such as 6d6 for
the gelatinous cube's 3d6 Engulf; 124 saves have damage now. Older text without
the label uses the damage after the DC in its sentence, and never the damage
before it, which is the hit's.

One later form is read as a condition: `If the target has 20 Hit Points or
fewer, it drops to 0 Hit Points. Otherwise, the target takes 13 (3d8) Psychic
damage` rolls only for a target with more than 20 HP. `Roll damage` settles a
condition when every target gives the same answer. When they differ, or the
run cannot tell, the question opens under the first target's row, because one
roll of damage cannot be both. When the answer gives no damage, the rows say
`No damage · see the text` and Amount is cleared, so Enter cannot apply an
amount from before.

### Inspiration

No ruleset comes first. The engine runs inspiration contracts with the
parameters their rule entries give, and a campaign takes its rule from its
packages and notes (`splat::content::inspiration`):

- `dnd2024:glossary_heroic_inspiration` (SRD 5.2.1, Heroic Inspiration): reroll
  one die immediately after rolling it, and use the new roll.
- `dnd2014:inspiration` (SRD 5.1, Inspiration): advantage on an attack roll, a
  saving throw or an ability check, chosen before the roll. Its parameters are
  `expend: {timing: when_making_d20_test, grants: advantage, applies_to:
  [attack_roll, saving_throw, ability_check]}`.

With one such rule, the campaign uses it. With several, campaign settings shows
`Inspiration rule`, saved in `splat_rule_choices` under `splat:inspiration`. A
chosen rule that is gone, such as a deleted note, is no choice; the other
settings still load.
With none, a run offers no inspiration. A run keeps the rule it started with.

Only a player character with inspiration is offered it: by the synced D&D Beyond
sheet, or else by the last read of their sheet. The run remembers each spend, by
participant, in the run file, until D&D Beyond confirms that the sheet shows no
inspiration. A value that waits to be sent, or that a read-only sheet cannot
take, is not a confirmation. A synced sheet is sent `No inspiration` at once, or
when its sync next can take it (a sync that starts later, or reconnects). A read
that still shows inspiration is older than the spend and does not give it back;
after the sheet has shown none, a new grant counts again. A grant the DM made
in the panel that still waits to be sent is spent with it.

- **Reroll rule.** After the run rolls for a player (`Roll for them`, or a
  check, save, skill or dice from the sheet), a right-click on the row with the
  result shows `Inspiration: reroll one die` and each die of that roll: `4 on
  the d20`, `8 on 1d8+4 slashing`. Equal faces of one roll are one line, lowest
  first, at most 8. The new die stands, its keep and totals are made again
  (`Dice::reroll`), Amount takes the new damage, and the Log says `1 becomes 6`
  and only the roll that changed. On an attack, the damage rolls again only when
  the d20 begins or ends a critical hit. A save's damage rolled for a player is
  one roll for all of its targets: each target's row offers its dice, and the
  targets and half marks stay. That roll stands only while the same save is
  armed against the same targets; `Stop`, a change of targets or a selection by
  the keys ends it for good. The next step of the run that succeeds (an applied
  amount, Next turn) ends the chance to reroll.
- **Advantage rule.** The tile, number or target menu offers `with inspiration
  (advantage)`; the roll has advantage and the Log says inspiration was expended.
  It pays for one roll: the armed action's next roll has no advantage from it.

### A save

1. Click `Ember Burst`. Click each target's row: a creature rolls its save at
   once, shown where its AC is, `DEX 14`. A click on a target takes it away
   again, with its save; the last target stays. The row's right-click menu rolls
   that target's save again, with advantage or disadvantage, and never takes a
   target away.
2. `Half for those at or above DC` marks them to take half, and takes the mark
   from a target whose save is now under the DC. `Roll damage` rolls the damage
   once, for every target, into Amount; each target's row then says what Enter
   gives it, `Saved · 1 damage (half)` or `Failed · 3 damage`. Enter applies it,
   halving where marked, and each row keeps its own result.
3. The armed action keeps that rolled damage until the step that applies it, or
   until the action ends; another step, such as a note, leaves it. When a
   target is added or taken away, a save is rolled again, or the marks change,
   Amount is made again from it: Enter applies Amount as it is to one target, so
   one target that made its save has half in Amount. A save rolled again loses
   the half mark of the save before it. An Amount that the DM edited since the
   roll stays, the same digits typed again too; a new roll of the damage takes
   its place. Damage that a condition settled for the targets before is not
   kept for new targets: Amount is cleared, and `Roll damage` rolls it again. A
   selection made outside the targeting, such as by the arrow keys, ends the
   rolled damage; coming back to the same targets does not bring it back.
4. Enter finishes the save: its tile disarms, and the rows show their AC
   again beside their results. The panel and the dock go back to the caster.
   Undo of the damage arms the save again as it was, with each save, the
   damage in Amount and each row's result.

### A player's attack

A party member's tile targets without rolling: click the row, then type the
damage the player says, and press Enter. `Roll for them` rolls instead.

### Checks, saves and skills

A number in a table rolls at once in a run, and its result shows on the
actor's row. Outside a run, a tile or a number previews: its result shows under
the tiles, and nothing is saved.

## Engine and data

- `Dice` has keep-highest and keep-lowest (`2d20kh1`, `2d20kl1`), so advantage is
  an ordinary expression. The dock's dice field accepts it too.
- `rolls::make` rolls an action: an attack and its damage, a save's damage, or a
  single d20 or dice roll, with the run's changes and a bonus. Each roll keeps
  every die, which were dropped, the natural d20 and the total; the Log line and
  the row's result are made from it.
- `Run::record` writes a roll's Log lines in `state.log` and in every state in
  `history`, and moves the dice there too, so Undo keeps the lines and never
  repeats the dice.
- The run keeps the armed action (`Armed`), each row's result (`badges`) and each
  save (`saves`). Next turn, Escape or `Stop` disarms; Next turn clears the rows.
  A save's damage step keeps its applied results in the run's state
  (`roll_results`), and the state before it keeps the save as it stood
  (`Cast`), so Undo of that step arms the save again.
- Results are kept by the row's place in the roster. Rows join only at its end,
  and only Undo of their arrival takes them away, so a place is one participant
  for as long as it shows a result. A move changes the turn order, not the
  places.
- While an action is armed, the panel and its D&D Beyond sheet stay on the actor,
  so a player's tiles and `Roll for them` stay when the dock moves to the target.
- `dice_text` marks a damage part's condition (`When`: its words, and whether it
  replaces or adds); `dice_text::resolve` picks the parts whose conditions hold.
- `rolls::actions` is the one list of an entry's actions, remembered by its text,
  so a frame that draws the same text parses nothing. A tile's number is its
  place in the whole entry's list, in every section and in a searched sheet, and
  Ctrl+number arms from the same list. Spellcasting is not in it.
- Chips come from `rolls::chips` over the rules text: each paragraph with a
  run-in title, skipping printed quotes, tables and headings. A test checks every
  built-in creature: each `Attack Roll:` and each damage group is found.

## Transitions

What each event does to an armed action, the results on the rows, a save's
rolls in the AC cells, and Amount. Each line has a focused test in
`src/encounter_run/rolling.rs` (`a_save_finishes_…`, `an_attacks_results_end_…`,
`a_result_stays_with_its_participant_…`); the `run-rolls` scenario checks the
save's lines in the app.

| Event | Armed action | Results on rows | Saves in AC cells | Amount and targets |
| --- | --- | --- | --- | --- |
| Arm (tile, Ctrl+number, menu) | This action; the panel stays on the actor | The last action's go; each row shows its chance | Go | The actor is selected |
| Roll against a row (attack) | Stays armed | That row: `17 vs AC 13 · 10 damage`; others keep theirs | — | The row is the target; Amount has the damage |
| Click a save's target | Stays armed | — | Its save; a second click takes it away (the last target stays) | The row joins or leaves the targets |
| `Roll damage` (save) | Stays armed | Each target: what Enter gives it | Stay | Amount has the damage |
| Enter, attack | Stays armed, for the next target | Stay | — | Cleared; one Undo step |
| Enter, save | Disarms | Each target keeps its result | Go; AC shows | Cleared; the caster is selected; one Undo step |
| Undo of a save's damage | The save again | What Enter gives each target again | Back | Targets, halves and Amount as they were (beside Lookup or Notes, Amount stays empty, as after every step) |
| Undo of another step | Unchanged | Unchanged (rolls are history) | Unchanged | — |
| The next target or attack | Stays armed | The new row gets its own; others keep theirs | — | The new row |
| The next action | That action | The last action's go | Go | The actor is selected |
| `Stop` or Escape | Disarms | Stay until the next action or turn | Go with the action | Unchanged |
| Next turn | Disarms | Go | Go | Cleared |
| Undo of Next turn | Stays disarmed | A save's applied results come back with their step | — | — |
| A row moved (tie order, an arrival moved before) | Unchanged | Stay with their participant | Stay | Unchanged |
| A row taken away (Undo of its arrival) | Ends if it was the actor's | That row's go; others keep theirs | That row's go | That row leaves the targets |

## Tasks

The `run-rolls` visual scenario measures these:

| ID | Task | Target |
| --- | --- | --- |
| T16.1 | Arm the selected creature's attack; rows show the chance to hit | 1 |
| T16.2 | Roll against a target and apply the damage | 2 |
| T16.3 | A second attack against the same target, applied | 2 |
| T16.4 | An area save against two creatures, then its damage | ≤ 6 |
| T16.5 | Arm an action by Ctrl+number | 1 |
| T16.6 | A player's attack with their own typed damage | ≤ 4 |
| T16.7 | Preview an attack in the Library | 1 |
| T16.8 | Arm an attack with advantage from its menu | 2 |
| T16.9 | Roll against one target with disadvantage from its menu | 2 |
| T16.10 | Attack when its damage depends on a condition | 3 |
| T16.11 | Reroll one die of a player's roll with inspiration | 3 |

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
4. **A turn strip.** Two concept reviews (Fable and Opus) recommended taking
   rolling out of the prose and the panel; the user chose a strip of chips, first
   under the selected row, then one line above the dock. The user then asked for
   more of a game's feel.
5. **This design: action tiles.** The user chose tiles in the stat block with
   text in hover, arming and clicking rows, typed player rolls, the chance to hit
   on rows, and number keys (Ctrl+number, as plain digits type into Amount).

## Later

- Keyboard: a key to step through target rows while an action is armed.
- A situational bonus, such as Bless.
- Gaining or giving inspiration.
- Damage at a later time, such as at the start of each turn, as its own roll.
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
