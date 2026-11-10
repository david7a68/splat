# Rolling from the stat block and the sheet

Status: implemented, after three reviews of the design. The notes under "Engine
and data" and "As built" say where the build differs from the earlier text.

The DM clicks a number on a stat block or a character sheet, and Splat rolls it.
This covers ability checks, saving throws, skills, attacks, damage, and calls
for a save against a DC. Like D&D Beyond and similar tools, the numbers in the
text are the controls. Splat already has the parts: a deterministic dice stream
(`crates/core/src/execution/dice.rs`), the run's dice field and Log
(`src/encounter_run/view.rs` `dice`), structured attack data in converted
creatures (`splat_rule` parameters), and D&D Beyond computed values for
characters (SCDS `toHit`, `damage`, `saveInfo`, skills and saves).

The glossary already names the two kinds of roll this design uses:

- A **standalone roll** in a run. It shows a result and goes to the Log. It does
  not use the ability, spend a resource, or change a participant.
- A **preview roll** outside a run, while the DM reads a definition. It changes
  nothing and is not saved.

## What other tools do

| Tool | What we keep | What we do not copy |
| --- | --- | --- |
| D&D Beyond sheet and encounter builder | Every bonus and damage value in the text is a button. Right-click gives advantage, disadvantage and critical. The game log keeps the detail | A floating result that covers the sheet; 3D dice |
| Roll20 character and NPC sheets | Critical and fumble marks | Roll templates written as text macros; rolling damage with every attack by default |
| Foundry (dnd5e system) | A modifier key skips the dialog | A dialog before every roll |
| Avrae, Shieldmaiden, Improved Initiative | Dice in a stat block's prose are rollable. Rolls are labelled with the creature and the action | Automatic hit decisions and automatic damage to HP |

The common pattern: the number on the page is the button, one click rolls it,
a modifier or a menu changes how it rolls, and the result shows next to the
click. Splat keeps that pattern and drops the automatic rulings, because of
principle 7, "Record, do not rule" (`docs/ux-intent.md`).

## Principles for this feature

1. **The number is the control.** A rollable value in the reader is a **dice
   link**: accent text and an underline, as reader links already are. It looks
   like a link and is drawn like one, so the reading surface gets no new kind
   of visible control (principle 5). Hover says what the click does, for example
   `Roll 1d20 + 4: Grab attack. Shift: advantage. Ctrl: disadvantage.`
2. **The printed text is the base.** A dice link takes its numbers from the
   printed text. A standalone roll only shows a result, so reading its numbers
   from prose does not interpret the prose to use an ability. Structured data,
   where it exists, adds the feature's label, the damage type, and the run's
   changes. The glossary's structure-only rule stays for controls that use an
   ability or change state (see "Glossary change").
3. **A roll changes nothing.** No roll changes HP, a condition, a resource or a
   turn. A result can offer a follow-up, such as `Use 9`, which fills the Amount
   field. The DM still picks the target and presses Damage, and Undo names that
   step as it does now.
4. **Standalone rolls are history, not steps.** A standalone roll is not an Undo
   step, and Undo never removes it from the Log. This fixes the problem noted in
   `docs/ux-intent.md` ("A dice roll takes an undo step"). Rolls that belong to a
   step stay part of it, as now: initiative, group arrivals, marked-table rolls
   and loot rolls.
5. **The pointer is complete.** Every modifier key has a menu item, because the
   DM may have one hand on physical dice. The keyboard covers the run's flow
   after a roll (D, T and Enter). Focus on the dice links themselves is later
   work (see "Later").

## What becomes a dice link

### In feature text

In each named feature paragraph (the run-in title, such as `Grab.`), these
printed forms are dice links:

| Printed form | Roll |
| --- | --- |
| `Attack Roll: +4` | 1d20 + 4, an attack |
| `9 (2d6 + 2) Bludgeoning damage` | 2d6 + 2, damage of that type. Several groups in one sentence are one damage roll with parts |
| `Dexterity Saving Throw: DC 14` | Not rolled. Starts a call for a save (see below) |
| Any other dice expression, such as `1d4` or `2d6 + 2` | Those dice, with no attack or damage meaning |

Other DCs, such as `escape DC 12` in Grab, are later checks by the target. They
stay plain text. `(Recharge 5–6)` stays plain text in the first version.

Most converted 2024 creatures print these forms: 333 of the 336 built-in
creatures have `Attack Roll:`, and 139 have `Saving Throw: DC`. Their structured
data comes in several shapes (114 have no `attack_bonus:` line; the Aboleth
uses `supplied_attack_total`). So the printed text is what finds the link, and
structured data is matched to it by feature name where an adapter exists.
Hand-written creatures get dice links from their prose with no extra work.

When the run changed a value (a creature's `attack_bonus` or `damage_bonus`
change), the link rolls the run's value, and the result says so:
`+5 (changed from +4)`. This follows the existing "printed" pattern in
`src/library/creature.rs`. When structured data and the printed text disagree
with no run change, roll the printed value and show the difference in hover.

### In tables and on the sheet

| Source | Creature (schema-2 stat block) | Character (D&D Beyond SCDS) |
| --- | --- | --- |
| Ability check | `Mod` column | Ability modifier |
| Saving throw | `Save` column; an empty cell falls back to `Mod` and says so | Saving throw bonus |
| Skill | Each entry in the Skills row | Each skill; the sheet's advantage marks show in hover |
| Attack | From feature text | `toHit` of each attack and spell attack. Save attacks have no `toHit` and show their DC as a call for a save |
| Damage | From feature text | `damage` value and type. A fixed number is not a link |
| Death save | Not used | A d20 on the sheet's death-save line |

Initiative is not a dice link. In setup, the existing Roll control already sets
it as one step. In live play, a new initiative roll would change state, so it
stays in Correct.

A character's result states the sheet's age when the cached record is not live,
such as `Sheet from 3 hours ago` (principle 6). Spell damage that appears only
in prose uses the plain dice rule above. Upcasting is not covered.

## How a roll is made

| Input | Result |
| --- | --- |
| Click | Normal roll |
| Shift+click | Advantage: roll two d20s and keep the higher |
| Ctrl+click | Disadvantage: roll two d20s and keep the lower |
| Right-click | Menu: Normal, Advantage, Disadvantage, Critical (damage only) |

Alt is not used, because Alt+Left is Back in the reader and in Lookup. For a
one-off bonus, such as `1d4` for Bless, the DM types the expression in the
run's dice field.

### Keys in a run

Two new single keys act on the roll strip. They follow the same rules as N and
U, and are added to `KEYS` (`src/encounter_run/view.rs:205`). Neither `Key::D`
nor `Key::T` is used today, and Amount accepts only digits.

| Key | Action |
| --- | --- |
| D | `Roll damage` (or `Roll critical damage`) for the strip's attack |
| T | `Use` the strip's total: put it in Amount |

Run keys stop while any widget other than Amount has focus
(`live_keys`, `src/encounter_run/view.rs:403`). So a click on a dice link in a
run gives focus back to Amount after the roll, as `follow_with_keyboard` does
after other run steps. Then the flow is: click the attack, Down or Up to the
target, D, T, Enter.

### Escape

Escape closes one layer at a time. The existing layers keep their order, and two
new layers join it:

1. A dialog above the run (it takes the keyboard, as now).
2. **New:** an open roll popover.
3. Lookup and Notes, as now.
4. A footer's second step, such as ending the encounter, as now.
5. A row's initiative field, as now.
6. **New:** the call view. Its ticks are kept until Next turn.
7. The selection: select the current creature again, as now.

The popover must take Escape before Lookup does, because a roll from a stat
block in Lookup shows its popover inside Lookup.

## The result

### Beside the link

Every roll shows its result at the link that was clicked, in a small popover:
the total in heading type, then the parts in Small. Each die is listed, and a
dropped die is struck through. Escape, or a click elsewhere in the same reader,
closes it. This is the same in Library, Lookup, the Party sheet and the run,
because they share one reader (T6.4). Outside a run that is all a roll does.

A natural 20 or a natural 1 says `Natural 20` or `Natural 1` in words, with an
icon, only on attack rolls and death saves, where it has a rule effect. On
checks and saves the result shows the die and no mark, so it does not imply a
ruling.

### In a run

#### One selection, two jobs

The run has one selection. It chooses whose stat block or sheet the panel shows
(`reference_tab(ui, self.selected)`), and who takes the amount when the DM
presses Damage (with `also` for more targets and `halved` for half damage).
Next turn selects the creature whose turn it is. So when the DM reads the
attacker's stat block, the attacker is the selected row, and clicking the
target's row changes the panel to the target.

This design keeps that model. The DM rolls from the attacker's stat block, then
chooses the target by clicking its row, as for any damage today. The roll's
result must therefore not belong to the panel's content.

#### The roll strip

In live play, the last roll stays in a two-line **roll strip** at the very top
of the side panel, above the participant's title and tabs. Line one is the
result. Line two is its actions. It does not change when the selection changes,
when the panel scrolls, or when the DM opens another tab. It belongs to the
roll's roller, not to the selected row. It holds the roller by row index, as
`selected` and `also` do, and moves with them when rows move.

Its lifetime:

- Next turn clears it, by either path (the run's own Next turn and the strip on
  other pages). Clear it in `apply`, where `follow_turn` is handled
  (`src/encounter_run.rs:1715`), not in the view.
- Undo keeps it. Today Undo clears `last_roll` (`src/encounter_run.rs:1953`).
  The strip must survive, so that "wrong target, Undo, choose the right row,
  Use again" works.
- If Undo removes the roller's row (an arrival taken back,
  `forget_removed_participants`), the strip keeps its result and drops
  `Back to` and `Roll damage`.

A roll in a run from a definition in Lookup has no roller. It goes to the Log
under the entry's name, and the strip shows it with `Use` and `Roll damage`,
but no `Back to`.

The strip replaces the dock's `= N` result while the side panel is shown. A
roll from the dock's dice field shows in the strip with no roller. So the DM
sees the result in one place in the run, plus the popover at a link.

When the side panel is hidden (Lookup or Notes open at a small width;
`hide_panel` in `src/lib.rs`), the strip is hidden too. Then:

- The dock keeps its `= N` result, as today, for rolls from the dice field.
- The popover at a link carries the strip's actions (`Roll damage`, `Use`).
  It stays open across a click on a roster row, so the DM can roll, choose the
  target, then use. A click elsewhere in the reader, or Escape, closes it.

In setup there is no dock and no strip. Rolls in setup show in the popover and
go to the Log.

An attack, step by step, on the Bugbear Warrior's turn:

1. Click `+4` in Grab. The strip shows
   `Bugbear Warrior 2 · Grab · 17 to hit (d20 13 + 4)`.
2. Click Oren's row. The panel shows Oren's sheet. The strip now adds Oren's
   current AC: `· AC 15 Oren`. It shows no "hit" or "miss" word. The DM
   decides, because a reaction such as Shield can still change the result.
3. Click `Roll damage` in the strip, or press D. It rolls the damage of the
   same feature, by the roller's values and run changes, although the panel
   shows Oren. After a natural 20 the button is `Roll critical damage`, which
   rolls the damage dice twice and adds bonuses once (2024 rule; `Dice::roll`
   already takes a `critical` flag).
4. Click `Use 9`, or press T. It puts 9 in Amount and moves focus there.
5. Press Damage, or Enter. Undo names it as now: `Undo: 9 damage to Oren`.

That is five clicks from the stat block to applied damage.

Rules for the strip:

- The AC shows only for an attack, and only when the selected row is not the
  roller. It is the selected row's AC. With several targets, hover lists the
  others.
- `Use` on damage from the roller's own attack or save feature is blocked while
  the roller is the only selected row, and says `Choose a target first.` This
  stops damage to the attacker by mistake. Other rolls (plain dice, healing,
  the dice field) are never blocked. Choosing a row clears Amount, as it does
  today, so `Use` comes after the target. The strip keeps `Use` until the next
  roll, so the DM can press it again after choosing another row.
- A damage result shows each part with its type, such as `14: 7 piercing,
  7 fire`. `Use` puts the total in Amount. For one part, or for half, the DM
  types the number, because Splat does not apply resistance.
- `Back to Bugbear Warrior 2` shows only when the roller is not the current
  creature and is not selected. When the roller has the turn, Escape already
  does the same, and the strip says so in hover. It selects the roller again,
  so the DM can make the next attack of a Multiattack.

```
Bugbear Warrior 2 · Grab · 17 to hit (d20 13 + 4) · AC 15 Oren
[Roll damage]
```

```
Bugbear Warrior 2 · Grab · 14 damage: 7 piercing, 7 fire
[Use 14]
```

At 960×640 the panel is about 346 px wide (`src/design/components.rs:2181`).
Line one keeps the total and the AC. It drops the dice parts first, then the
feature name, then shortens the roller's name, keeping its copy number. Line
two has one or two buttons, which fit. Hover and accessibility give the full
lines.

Every standalone roll writes one Log line, for example
`Bugbear Warrior 2, Grab: to hit 1d20 + 4 with advantage: [13, 6] = 17.`

## Calls for a save

A save DC link (`Dexterity Saving Throw: DC 14`) starts a call for a save. The
DC is on the caster's stat block, so the caster is the selected row when the DM
clicks it. Choosing targets with the roster would change the panel. So a call
for a save has its own list of targets, and it uses the run's selection only at
the end.

The **call view** opens in the side panel, as Arrivals does: it has no
participant tab, and it stays open when the selection changes. Its title names
the feature and the DC, such as `Fire Breath · DC 14 Dexterity save`, with
Close. Below the title it shows the feature's whole paragraph, so the DM can
read the failure and success effects while calling the save.

1. Click the DC. Opening a call clears `also` and `halved`. The call view lists
   every present participant except the caster, in roster order, with side
   marks and tick boxes. Nothing is ticked. Tick the targets. Ticks are view
   state, never saved, like `also`.
2. Each ticked creature row has `Roll`, with its save bonus for that ability.
   `Roll all creatures` rolls every ticked creature row. Each ticked character
   row has an optional number field for the player's own roll, because players
   often roll their own saves at the table. Each roll and each typed total
   writes one Log line.
3. Each row with a total shows it and the DC, and says `Below DC` or
   `At or above DC` in words.
4. Each row has a `Half` toggle. It works with no total, so the DM can set it
   when a player just says "I made it". When the feature does half damage on a
   success, `Half for those at or above DC` turns it on for the rows with a
   total at or above the DC. The DM can change any tick or toggle.
5. The call view shows the feature's damage links. The damage is one roll for
   all targets, as the 2024 rules give for most area effects (the glossary's
   *roll scope*). It shows in the roll strip as usual. While the call view is
   open, the strip hides its own `Use`, so there is one `Use` button.
6. `Use 28 on 3 targets` sets the run's selection from the call. It calls
   `select` first, because `select` clears `also`, `halved` and Amount
   (`src/encounter_run/view.rs:333`). Then it puts the other ticked rows in
   `also`, the rows with Half in `halved`, and 28 in Amount. The DM presses
   Damage, which is one Undo step for all targets (`Command::Several`). The
   dock names targets in turn order (`targets()`), not in tick order.
   - With one target and Half on, the run's half set does not apply, because
     `amount_command` uses `halved` only when `also` has rows
     (`src/encounter_run/view.rs:318`). So the button puts the halved number,
     rounded down, in Amount and says `Use 14 (half) on Oren`.
   - With no target ticked, the button is blocked and says
     `Tick a target first.`

While the call view is open:

- A plain click on a roster row changes the selection and the dock, as with
  Arrivals. The panel keeps the call view.
- A Ctrl+click on a roster row ticks or unticks that row in the call, because
  Ctrl+click already means "add a target". It does not change `also`.
- A click on Stat block, Conditions, Correct, Log or Prep, or Ctrl+G for
  Arrivals, replaces the call view. Its ticks, totals and Half toggles are
  kept until Next turn. A click on the same DC link opens it again.
- Next turn closes it and forgets its ticks. The ticks also stay through a
  reaction or a legendary action, because those happen within the same turn.

Nothing in the call changes a participant until that last Damage.

## Engine and data

- Extend `Dice` with keep-highest and keep-lowest (`2d20kh1`, `2d20kl1`), so
  advantage is an ordinary expression. The dice field accepts it too.
- Add a labelled roll request: the roller (participant or entry), the source
  (ability, save, skill, attack, damage, dice), the mode (normal, advantage,
  disadvantage, critical). The result keeps every die, which were dropped, the
  natural d20, and the total. The Log line, the popover and the strip are all
  made from this one result.
- Today `Command::Roll` goes through `apply`, takes one of the 100 history
  slots, and writes to `state.log`, and Undo restores `state.random`.
  Standalone rolls change:
  - They do not go through `apply` and take no history slot.
  - Their Log line goes in `state.log` and in every state in `history`, and the
    dice's new position goes in every state's `random` too (`Run::record`, from
    the dock's `roll_dice` on `main`). So Undo keeps the line in its place, and
    undoing and rolling again does not give the same number. This replaces the
    separate stream that the earlier text proposed.
  - The Arrivals notice reads `state.log.last()` to find the last addition
    (`src/encounter_run/view.rs:3050`). It must skip roll lines.
  - The test `undo_takes_a_group_back_whole_and_its_dice_with_it` still holds:
    a group's own roll lines go with its step.
  - The DM's dice field becomes a standalone roll too.
- Feature text matching runs on the reader's existing paragraph pass
  (`design::reader::run_in_title`), so the Library, Lookup and run readers find
  the same links. A test checks every built-in creature: each `Attack Roll:` and
  each damage group gets a link.
- Dice links are ranges in the reader's laid-out text, hit-tested by pointer
  position, as reader links are today (`src/markdown_view.rs:1003`). They need
  a right-click menu, which reader links do not have yet.

## As built

- Parsing is `splat::content::dice_text`; rolling and the result beside a link
  are `src/rolls.rs`; the strip and the call view are
  `src/encounter_run/rolling.rs`.
- An empty `Save` cell has no link. A call for a save uses the save bonus, or
  else the ability modifier, from the creature's stat block.
- Character rows in a call have a typed total, `Half`, and `Roll for them` when
  the participant's entry has the bonus.
- The `run-rolls` visual scenario measures T16.1 to T16.5, T16.7 and T16.8.
  T16.6 has no script yet.

## Glossary change

- Add **Dice link**: "A number or dice expression in an entry's text that makes
  a standalone or preview roll from its printed value. Structured data can add
  its label, type and run changes."
- Change **Inline control** to say that it covers controls that use an ability
  or change state, and that a dice link is the exception for rolls that change
  nothing.
- Widen **Standalone roll**: "A roll of an ability's dice expression, or of any
  dice expression during a run, that displays a result without using the
  ability, spending resources, or changing a participant's state."

## Plan

1. **1a, engine, tables and the strip.**
   - Keep-highest and keep-lowest; roll requests; standalone rolls outside Undo
     with their own stream; Log lines kept across Undo.
   - Dice links on the ability table and Skills row, in every reader, with the
     popover, the right-click menu and its Escape layer. Preview rolls come with
     this step, so the shared reader never shows a link that does nothing.
   - The roll strip, `Use`, the T key, and the dice field's result in the strip.
2. **1b, feature text.** Dice links for attacks, damage and plain dice;
   `Roll damage`, the D key, the AC beside an attack, and `Back to`.
3. **Calls for a save.** Save DC links, the call view and its Escape layer,
   `Half`, and `Use … on … targets`. Character rows have only the typed total
   and `Half` in this step.
4. **Sheets.** The same links on the run's Sheet tab and the Party sheet,
   including spell attacks and death saves, and `Roll` for character rows in a
   call for a save.
5. **Later.** See below.

Each step also updates `docs/design-system.md` ("Header, dock and panels" for
the strip, the dock result and the call view) and the `KEYS` list, and adds
visual-test scenarios, a 960×640 view check, a keys check, and these task costs
to `docs/ux-intent.md`:

| ID | Task | Target |
| --- | --- | --- |
| T16.1 | Roll the current creature's first attack | 1 |
| T16.2 | Choose a target and see its AC beside the attack total, with the strip unchanged | 1 more |
| T16.3 | Roll the damage and apply it to that target | 3 more (5 in all) |
| T16.4 | Make the second attack of a Multiattack against the same target, with damage | 6 |
| T16.5 | Three creatures (no characters) save against an area effect: open the call, tick them, roll the saves, set Half for those at or above the DC, roll damage, use it, apply | 9 |
| T16.6 | A creature's area effect against three characters, two of whom say they made the save: open the call, tick them, turn on Half for two, roll damage, use it, apply | 9 |
| T16.7 | Roll a character's skill with advantage, by pointer only | 2 |
| T16.8 | After a pointer roll of an attack, choose the target, roll damage, use it and apply it by keyboard only: Down or Up, D, T, Enter | 4 keys, when the target is the next row |

## Later

- Keyboard focus on dice links: one tab stop per group (the ability table, the
  Skills row, each feature paragraph), Left and Right inside a group, Enter to
  roll, Shift+Enter and Ctrl+Enter for advantage and disadvantage, and the
  context-menu key. Reader links cannot take focus today.
- `Add a bonus…` on a dice link, and keeping it for later rolls by the same
  creature, such as Bless.
- Recharge links: `(Recharge 5–6)` rolls a d6 against the range.
- `Use` for one damage part, and `Use half`, on the strip.
- `Untick those at or above DC`, for features that do no damage on a success.
- Earlier preview rolls, and `Add to notes` for a session.
- Reminders of conditions that can change a roll. These need a table of which
  conditions affect which rolls, and come close to a ruling.
- `Roll all` for Multiattack. Its structured data has at least three shapes.
- A separate actor and target: the panel stays on the creature whose turn it is,
  and a row click only chooses the target. This makes Multiattack faster, but
  changes the run's selection model. Consider it if T16.4 is slow in real play.
- Upcast spell damage.
- Sending rolls to the D&D Beyond game log, so players see them in their app.
  The write path is not verified (`docs/research/dndbeyond-api.md` covers only
  reading). If it is built, it is opt-in, with the DM's rolls hidden by default.

## Out of scope

Automatic hit and miss decisions, automatic damage, resistance and condition
rules, spending resources, 3D dice and sound, and a player-facing view. These
belong to automatic combat resolution, which remains later work.
