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

Two single keys follow the same rules as N and U, and are in `KEYS`:

| Key | Action |
| --- | --- |
| D | `Roll damage` (or `Roll critical damage`) for the open attack, or the call's damage |
| T | Put the last roll's total in Amount |

Run keys stop while any widget other than Amount has focus (`live_keys`). So a
click on a dice link in a run gives focus back to Amount after the roll. Then the
flow is: click the attack, click the target's row, D, Enter.

### Escape

Escape closes one layer at a time. An open roll result or roll menu comes first,
before Lookup and Notes; closing a held result ends the hold and any call. The
run's own layers keep their order.

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

Revised after a design review (2026-11-10). The first build kept the last roll
in a strip above the side panel, with `Back to <roller>`, and called saves in a
view that replaced the panel. The user found that it used the side panel too
heavily: a click on the target's row replaced the attacker's text, `Use` was at
the top of the panel while Amount is under the roster, and the call view was a
second roster without HP. The strip, `Back to` and the call view are gone.

**An open roll holds the panel.** The run still has one selection, which chooses
the dock's targets. While a roll from a stat block or sheet has its result open,
the Stat block tab stays on the roller, and a click on a row only selects the
target. Conditions and Correct follow the selected row. Under the title, muted
Small says `Held on <roller> while its roll is open ·` with `Show <selected>`,
which closes the result. Escape, Next turn and a click on the roller's own row
also end the hold. The hold is tied to a visible result, so it is not a hidden
mode.

An attack, step by step, on the Bugbear Warrior's turn:

1. Click `+4` in Grab. The result opens beside it: `17 to hit`.
2. Click Oren's row. The result adds `AC 15 Oren`, with no verdict; the Grab text
   stays in the panel.
3. Click `Roll damage` in the result, or press D. After a natural 20 it is
   `Roll critical damage`. The damage fills Amount, because a target other than
   the roller is selected.
4. Press Enter, or Damage. Undo names it as before.

That is 3 clicks and Enter. The next attack of a Multiattack is the attack link,
`Roll damage`, and Enter. Damage never fills Amount while only the roller is
selected. The dock shows the last total as `= N`; a click on it, or T, puts it in
Amount, for dice from the dock or a roll whose target changed.

Every standalone roll writes one Log line, as before.

## Calls for a save

A save DC link (`Dexterity Saving Throw: DC 14`) starts a call. A call card opens
beside the DC, held like a roll: `DC 14 Dexterity save` and how many targets.

1. Click rows to choose targets. A click adds or removes one; the caster's row
   does nothing. These are the run's own targets (`selected` and `also`).
2. `Roll saves` rolls each creature target's save, by its Save bonus or else its
   ability modifier. Each result stands in the row's AC cell, `DEX 19`, beside
   the HP it will change. A linked player's sheet rolls its own save; the DM uses
   the row's half mark when the player says they made it.
3. `Half for those at or above DC` sets the rows' half marks, when the feature's
   text says a success takes half.
4. `Roll damage` rolls once for all targets (the glossary's *roll scope*) into
   Amount. Enter applies it as one Undo step, and the call ends.

## As built

- Parsing is `splat::content::dice_text`; rolling and the result beside a link
  are `src/rolls.rs`; the hold and calls are `src/encounter_run/rolling.rs`.
- An empty `Save` cell has no link.
- The `run-rolls` visual scenario measures T16.1 to T16.5, T16.7 and T16.8:
  1, 1, 2, 3, 7, 2 and 2.

## Engine and data

- Extend `Dice` with keep-highest and keep-lowest (`2d20kh1`, `2d20kl1`), so
  advantage is an ordinary expression. The dice field accepts it too.
- Add a labelled roll request: the roller (participant or entry), the source
  (ability, save, skill, attack, damage, dice), the mode (normal, advantage,
  disadvantage, critical). The result keeps every die, which were dropped, the
  natural d20, and the total. The Log line and the result beside the link are both
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

This was the build order. The strip and the call view it names were replaced
after review; see "In a run".

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
| T16.2 | Choose a target and see its AC beside the attack total, with the attack's text in view | 1 more |
| T16.3 | Roll the damage into Amount and apply it to that target | 2 more |
| T16.4 | Make the second attack of a Multiattack against the same target, with damage | 3 |
| T16.5 | Two creatures save against an area effect: open the call, choose them, roll the saves, set Half, roll damage, apply | 7 |
| T16.7 | Roll with disadvantage by pointer only | 2 |
| T16.8 | After a pointer roll and target, D and Enter apply the damage | 2 keys |

## Later

- Keyboard focus on dice links: one tab stop per group (the ability table, the
  Skills row, each feature paragraph), Left and Right inside a group, Enter to
  roll, Shift+Enter and Ctrl+Enter for advantage and disadvantage, and the
  context-menu key. Reader links cannot take focus today.
- `Add a bonus…` on a dice link, and keeping it for later rolls by the same
  creature, such as Bless.
- Recharge links: `(Recharge 5–6)` rolls a d6 against the range.
- Using one damage part, or half, without typing the number.
- Clearing the targets at or above the DC, for features that do no damage on a
  success.
- Earlier preview rolls, and `Add to notes` for a session.
- Reminders of conditions that can change a roll. These need a table of which
  conditions affect which rolls, and come close to a ruling.
- `Roll all` for Multiattack. Its structured data has at least three shapes.
- Upcast spell damage.
- Sending rolls to the D&D Beyond game log, so players see them in their app.
  The write path is not verified (`docs/research/dndbeyond-api.md` covers only
  reading). If it is built, it is opt-in, with the DM's rolls hidden by default.

## Out of scope

Automatic hit and miss decisions, automatic damage, resistance and condition
rules, spending resources, 3D dice and sound, and a player-facing view. These
belong to automatic combat resolution, which remains later work.
