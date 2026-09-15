# SRD contract source audit

Audit date: 2026-09-15. Scope: Counterspell, Polymorph, Creation, Wish,
Wand of Wonder, Bag of Holding, Apparatus of the Crab, Ring of Invisibility,
and Mysterious Deck.

Two source-mechanics issues were found and corrected. The other seven contracts have no
material discrepancy identified in this pass. This is a clause-by-clause source
review of the research contracts, including their tables and referenced spell
effects. It is not a statement that an engine adapter exists or that all
possible interactions have been tested.

The authority is the bundled official [SRD 5.2.1 PDF][pdf], whose SHA-256 is
`8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87`.

## Corrected findings

### 1. Apparatus claw damage needs a roll for each attack

Resolution: the final contract uses `scope: attack`. A regression check covers this field.

The [Apparatus contract][apparatus-contract] gives lever 5 Up's `2d6` damage
`scope: use`. The same lever specifies `scope: each_extended_claw`, so one
Utilize action can cause two separate melee attacks. Sharing one damage roll
across that action would not represent those attacks correctly.

The [original page 210 table][apparatus-page] says, “Each extended claw makes
the following melee attack,” followed by its attack modifier and hit damage.
The [schema's roll scopes][roll-scopes] distinguish `use` from `attack`.
Use `scope: attack` for that damage roll, with a separate roll and critical
handling for each claw attack. This does not require a GM choice.

The full page was checked because the named source extraction splits the table:
the Down column appears in the Armor of Resistance note. The contract correctly
restores all ten Up/Down pairs. The finding concerns damage roll scope, not the
table reconstruction.

### 2. Wand petrification must test whether its restraint still applies

Resolution: the final repeat-save trigger requires active Restrained from `this_effect`. A regression check covers that guard.

The [Wand of Wonder contract][wand-contract], percentile result 98–100, assigns
Restrained and schedules `repeat_at: target.next_turn_end`. It does not specify
the source restriction on that scheduled saving throw.

The [original wand text, page 252][wand-source], says: “While Restrained in this
way, the creature repeats the save at the end of its next turn.” The repeat
must require the target still to have **this effect's** Restrained condition.
If that condition has ended before the scheduled event, the repeat must not
turn the creature to stone. A separate Restrained effect must not satisfy the
guard. Add an explicit source-effect guard to the repeat-save trigger. This is
a fixed rule, not a generic manual-resolution case.

## Results for the other contracts

| Contract | Original source | Result of source comparison |
| --- | --- | --- |
| Counterspell | [p. 120][counterspell-source] | Correctly requires seeing a creature casting with a listed component, uses a Constitution save, wastes the triggering action economy on failure, and preserves the triggering spell slot. It does not import 2014's spell-level check or automatic cancellation. |
| Polymorph | [pp. 153–154][polymorph-source] | Correctly retains alignment, personality, creature type, HP and Hit Point Dice; grants the Beast's HP as Temporary HP; ends when no Temporary HP remain; and preserves the anatomy, speech, spellcasting and equipment restrictions. The CR/level ceiling is present. |
| Creation | [p. 121][creation-source] | All six material duration categories, shortest-duration handling for mixtures, seen-form/material restrictions, component-causes-failure rule and higher-slot size increase are present. The unlisted-mineral duration is a genuine source gap: the prose permits mineral matter including metal, but the table lists precious metals and two special metals without assigning a duration to ordinary metal. A narrow explicit input for that gap is justified. |
| Wish | [p. 175][wish-source], [Greater Restoration p. 137][restoration-source] | All eight modes and all stress rules are present, including Sudden Learning, reroll acceptance choice, nonpreventable damage, Strength reduction, light-activity recovery and permanent loss chance. Instant Health expands the linked Greater Restoration categories. Reshape Reality's GM resolution is expressly authorized by the source. |
| Bag of Holding | [p. 212][bag-source] | Weight, both capacity limits, retrieval economy, all destruction triggers, inversion, shared air supply and the complete nesting hazard are represented. The apparently inconsistent approximate interior dimensions and 64-cubic-foot limit both remain. The hazard is a Sphere and does not add another item's Total Cover exception. |
| Ring of Invisibility | [p. 238][ring-source] | Correctly uses a Magic action to activate, ends on removal or the wearer's Bonus Action, and does not add an attack/casting termination rule absent from this 2024 item. Attunement and wearing requirements are present. |
| Mysterious Deck | [pp. 231–232][deck-source] | Both complete percentile tables, 22 card effects, draw timing, extra-draw exceptions, Fool/Jester removal and Avatar of Death are represented. The Avatar's automatic-hit attack, summoner-dependent statistics, immunity list and one-avatar-per-creature rule are retained. GM choices correspond to source-authorized choices such as the granted weapon, knight statistics and prison location. |

The Wand's remaining 17 branches and nested spell/creature tables match the
source in this pass. In particular, the contract preserves its spell range
floor, source-specific Line origins, self-targeting branches, object restrictions,
repeat saves for blindness, and gem damage division. Its application of the
general round-down rule to divided gem damage is consistent with the system
rule, rather than a missing-engine fallback.

The generic optional context inputs on these contracts do not themselves
establish permission to replace fixed rule clauses with GM decisions. The
specific manual choices examined above are justified by the original text.

[pdf]: https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf
[apparatus-contract]: srd-conversion/items_2.md
[apparatus-page]: ../../builtin/srd-5.2.1/pages/210.md
[roll-scopes]: ../design/mechanics-schema.md#rolls-and-tests
[wand-contract]: srd-conversion/items_3.md
[wand-source]: ../../builtin/srd-5.2.1/magic_item/wand_of_wonder.md
[counterspell-source]: ../../builtin/srd-5.2.1/spell/counterspell.md
[polymorph-source]: ../../builtin/srd-5.2.1/spell/polymorph.md
[creation-source]: ../../builtin/srd-5.2.1/spell/creation.md
[wish-source]: ../../builtin/srd-5.2.1/spell/wish.md
[restoration-source]: ../../builtin/srd-5.2.1/spell/greater_restoration.md
[bag-source]: ../../builtin/srd-5.2.1/magic_item/bag_of_holding.md
[ring-source]: ../../builtin/srd-5.2.1/magic_item/ring_of_invisibility.md
[deck-source]: ../../builtin/srd-5.2.1/magic_item/mysterious_deck.md
