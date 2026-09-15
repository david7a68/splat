# D&D 2024 representative creature and rule coverage

Status: design fixture specification, prepared under the user's delegation to
choose representative examples. No data-model implementation is authorized.
The user no longer wants an approval round for each example mechanic.

The first [complete schema examples](examples/README.md) and
[hand-worked encounter](encounter-trace.md) now exercise the common cases.
[Schema coverage](schema-coverage.md) assesses all families below against that
draft and identifies remaining contracts; none are executable test results.

This expands [the guard example](creature-example.md). The coverage target is
D&D 2024 (also called 5.5e), using the revised rules and current public SRD.
Keep source-specific exceptions explicit. Do not silently import 2014 behavior.

## What the example represents

Use one recognizable homebrew **Test Warden**, with named variants and companion
participants for interactions. This is a deliberately artificial rules fixture;
it is not a balanced monster, an official stat block, or a class-legal build.
Variants are independent authored examples or campaign copies under the existing
ownership model. Their names do not introduce a new inheritance/overlay system.

The base is the existing guard, retaining its UUID, local action keys, Mend,
and Protect Ally. Retain its existing resource key to avoid an incidental key
migration; the display name can change independently. The variants exercise
mechanical alternatives without requiring all of them to be active together.
Companions supply independent casters, allies, hostile targets, and objects.

Coverage means an explicit scenario and expected outcome for a mechanic family.
A feature name on the creature alone is insufficient. The rows below are design
acceptance cases, not passing software tests. Complete coverage of every printed
feature still requires a source-by-source exception audit; this document does
not claim that audit has been completed.

## Base Test Warden

| Property | Fixture value |
| --- | --- |
| Size and type | Medium Humanoid; encounter side is separate |
| Ability scores | STR 16, DEX 14, CON 14, INT 12, WIS 12, CHA 10 |
| Proficiency bonus | +2 |
| AC | Supplied 16, including scale mail and Dexterity; no shield initially equipped |
| HP | Supplied maximum 30; Hit Dice 4d8, stored separately from maximum HP |
| Speed | 30 feet; reach 5 feet; geometry resolved only by declared assumptions |
| Proficiencies | Strength and Constitution saves; Athletics and Perception |
| Strike | One action; +5 melee attack; one creature in reach; 1d8 + 3 piercing |
| Mend | Agreed one-action, one-use, touch healing for 1d8 + 2; three uses, long-rest recovery |
| Protect Ally | Agreed reaction: +2 AC for another ally against the triggering attack only |
| Inventory | Scale mail worn, spear wielded, shield carried, three healing potions, two distinct charged wands, worn backpack with nested pouch |

The supplied maximum HP is intentional, not a derived average from Hit Dice.
Equipment bookkeeping identifies the contributions already included in AC.
Wand A starts with four fire charges and two healing charges; Wand B starts
with one of each. Their common homebrew definition has separate fire/healing
pools, each with maximum four and its own recovery rules. They remain separate
quantity-one rows. A resource cost binds to the wand actually used.

Keep the authoritative Markdown inventory-table convention, including named
container tables and supplementary resource state. The lists here specify the
fixture's intended content, not a second encoded inventory for an executable
note. Detailed YAML operation fields remain design work.

## Variant catalogue

| Variant | Abilities and setup | Principal purpose |
| --- | --- | --- |
| Martial | Extra Attack case; separate monster Multiattack case; ranged weapon; unarmed strike; mastery-enabled weapons | Attack composition, substitutions, target changes, hands and equipment |
| Arcane | Cantrip, spell attack, save spell, multi-target blast, Concentration spell, reaction spell; slots and limited free casts | Spell legality, components, scaling, interruptions, shared/per-target rolls |
| Support | Mend, temporary HP grant, bonus-action healing, condition removal, healing blocked by a specific effect | HP changes, choices, recovery, timing |
| Controller | Grapple/shove, fear/charm tied to a source, save-to-end effects, aura, recurring damage | Relationships, durations, entry/exit, per-target state |
| Defender | Protect Ally, another timed reaction, resistance, immunity, vulnerability, damage reduction | Reaction order, typed damage, suppression and stacking |
| Boss | Recharge action, Legendary Resistance, source-defined out-of-turn actions, regeneration, phase change | Monster exceptions, limited uses, scheduler and encounter completion |
| Shapeshifter | Alternate form; summon with controller and expiry; replacement versus retained stats | Identity, equipment, HP/resource retention, dependent actors |
| Quartermaster | Multiple charged items, consumables, attunement, mundane/magical containers, standalone supply pack | Item identity, multiple pools, capacity, transfers and access |
| Player-state | Heroic Inspiration, death saves, Hit Dice, class-like limited uses and once-per-turn features | Same core model for party members and creatures |

These variants are fixture choices, not assertions that every real creature
can acquire every feature. Each special ability still states its eligibility,
costs, trigger, targets, operations, and end conditions.

## Coverage cases and acceptance checks

Each case must eventually record its selected rule references, fixture variant,
starting state, controlled rolls or reproducible seed, decisions, targeting
assumptions, and observable results. Fixed rolls are a future test-harness need;
they do not change the agreed standalone-roll controls in play.

| ID | Mechanic family | Required cases and observable result |
| --- | --- | --- |
| R01 | Supplied and calculated stats | Base AC 16; equipping the shield yields 18 once. A calculated-AC variant agrees. Unknown imported contributions prevent unsupported recalculation. |
| R02 | D20 tests | Attack, ability check, and each of six save abilities; proficient/untrained/expertise; natural 1/20 behavior follows the test kind, not one universal critical rule. |
| R03 | Advantage and Disadvantage | Normal, advantage, disadvantage, multiple causes, and both together. Cancellation is not a vote by number of sources. Preserve which dice were used. |
| R04 | Roll modification | Flat and rolled bonuses, replacement and reroll, minimum result, automatic success/failure, and critical-range exception. Preserve original and replacement results and their causes. |
| R05 | Action economy | Action, Bonus Action, Reaction, no-action trigger, extra/restricted action, and Ready. Costs are paid at the specified stage; Incapacitated removes forbidden choices. |
| R06 | Turn timing | Start/end of actor's turn, another creature's turn, round, once per turn, once on own turn, and once per round. Reaction refresh follows its rule. Group initiative does not merge participants' turns. |
| R07 | Attack composition | Melee/ranged, weapon/spell/unarmed, reach and range assumptions, hit/miss/critical, on-hit rider and on-miss effect. Extra Attack and monster Multiattack have distinct granting rules; revised Multiattack operates as part of the Attack action. |
| R08 | Multiple attacks | Two attacks may choose different targets as rules allow. First target falls; second attack can select another. Attack substitution and extra-attack restrictions belong to the granting feature; action cost is not charged twice. |
| R09 | Weapon properties/mastery | Finesse, versatile, light, thrown, ammunition/loading/two-handed and all eight mastery properties: Cleave, Graze, Nick, Push, Sap, Slow, Topple, Vex. Validate feature access, action/turn limits, target relations, saves, and stacking. |
| R10 | Damage components | Mixed piercing/fire in one hit, independent later damage, flat and rolled damage, immunity/resistance/vulnerability/reduction, rounding, and zero damage. Per-type adjustments preserve the distinction between a component and an event. |
| R11 | Damage arithmetic | Under ordinary resistance, 9 fire becomes 4. Resistance plus vulnerability becomes 8 when those are the only modifiers. A separate 5 piercing stays 5. Repeat with zero, immunity, and a component-specific exception. |
| R12 | Saves and area effects | Save for none/half/partial outcome, damage plus condition, successful save with rider removed, shared damage roll against multiple independent saves, per-target rolls when specified. Effect branches remain explicit. |
| R13 | HP and survival | Healing capped at maximum; temporary HP choice and non-addition; temporary HP do not revive; maximum-HP reduction and restoration; zero HP, stabilization, death-save success/failure, damage at zero, massive damage, death, revival, and melee knockout. Distinguish normal monster death from selected death-save behavior. |
| R14 | Conditions | Apply and remove every 2024 condition: Blinded, Charmed, Deafened, Exhaustion, Frightened, Grappled, Incapacitated, Invisible, Paralyzed, Petrified, Poisoned, Prone, Restrained, Stunned, Unconscious. Check immunities, source relationships, linked conditions, automatic saves, and attack effects. |
| R15 | Condition lifetime | Two independent sources of one condition; ending one leaves the other. Exhaustion accumulates by its own rules. Ending Unconscious does not automatically remove Prone. |
| R16 | Spell casting | Spell level versus slot level, cantrip scaling, prepared/available spell, slot versus free/innate/item cast, component and hand requirements, consumed/costly materials, target type, and cast-time legality. |
| R17 | Spell costs and interrupts | Revised per-turn spell-slot expenditure restriction; a slot-free cast; a reaction on a different turn; a readied spell that is never released; interrupted casting and rule-specific refund. Do not infer all restrictions from spell level alone. |
| R18 | Concentration | Begin a new Concentration effect, damage check, multiple damage sources, Incapacitated, death, voluntary ending, and duration expiry. Losing shared Concentration ends dependent effects on all targets without deleting independent effects. |
| R19 | Effects and stacking | Same spell from two casters, same mechanic via campaign copy, stronger/weaker/equal applications, overlapping target sets, distinct compatible bonuses, temporary immunity after a successful save, removal by action, and exceptions. Suppressed effects keep time and can resume. |
| R20 | Reactions | Protect Ally turns a noncritical hit into a miss; natural-20 critical stays a hit under ordinary rules. A later guard rechecks eligibility. Include before/after-trigger timing, nested reaction, cancellation, duplicate-event prevention, and chain-limit diagnostics. |
| R21 | Grapple and shove | Revised unarmed save choice, size/hand requirement, escape check, multiple grapplers, grappler incapacity, release, and out-of-reach ending. A monster's automatic-on-hit grapple is separately defined. |
| R22 | Resources and rests | Per-use/turn/rest/day pools, short/long rest, recharge die, partial fixed/dice recovery, multiple recovery mechanisms, start-time versus elapsed-time reset, and interrupted rest. Current amounts remain instance state. |
| R23 | Items and consumables | Use Wand A without spending Wand B; drain either named pool; attempt with insufficient charges; split compatible stacks; consume one potion at its rule-defined stage; attune/equip/unequip and update granted actions. |
| R24 | Containers | Standalone barrel, carried backpack, nested pouch, magical weight contribution, independent copies of a supply pack, and transfer preserving contents. Reject cycles, invalid capacities, and duplicate content authorities. |
| R25 | Monster exceptions | Legendary Resistance changes a failed save using its own use pool; recharge succeeds/fails at specified timing; regeneration is stopped by the specified damage until its specified expiry; threshold/form change fires once unless repeatable. Include damage absorption, Evasion, Magic Resistance, Undead Fortitude, spell reflection, and swarms that cannot regain HP or gain temporary HP. |
| R26 | Additional actor timing | Revised legendary-action timing, uses, option lockouts and in-lair use changes; restricted extra reactions; reaction-per-turn refresh; summons with controller and initiative rules; companion requiring a command. No generic rule that all extra actions use a Reaction. Legacy lair/mythic actions require their own source audit. |
| R27 | Transformation | Same participant changes form without acquiring a second turn. Explicitly state retained/replaced stats, HP, equipment, Concentration, and restoration on reversion; do not impose one universal form-change policy. |
| R28 | Target relations and perception | Self/ally/enemy/willing creature/object; creature type, size, seen/heard, hidden/invisible and special senses. Record assumed eligibility; unsupported visibility or geometry cannot silently become true. |
| R29 | Encounter phases | Arrival, exit, return with retained state, replacement by a new actor, overlapping triggers, delayed reinforcement, death-triggered effect, manual end, and incomplete trial. Use the agreed stable ordering after rule-required choices. |
| R30 | Reproducibility and choices | Same resolved inputs/seed/strategy yield the same automatic choices; changed input fingerprints prevent a claim of exact replay. A manual decision remains separate from a random seed. |
| R31 | Movement-dependent mechanics | Opportunity attacks, teleport/forced-movement exemptions, reach, prone/standing, speed zero, flight/hover/fall, push, aura entry/exit and cover-dependent eligibility. Use explicit scripted events or assumptions; geometry and pathfinding remain deferred. |
| R32 | Non-damage resolution | Help, Dodge, Disengage, Dash, Hide, Search, Study, Influence, Utilize and Magic action eligibility; ability checks/passive checks and noncombat uses. Social interpretation and exploration simulation beyond declared checks remain outside the encounter fixture. |
| R33 | Identity exceptions | Split produces new actors; possession changes control without conflating possessor and host; swallow retains inside/outside relationships and escape rules; charm/domination can change choices or sides without changing template ownership. |
| R34 | Fixed damage and hidden eligibility | Monster fixed damage is an alternative to rolling, never added to it. A spell cast at an invalid target follows its own outcome and resource rules; undisclosed invalidity is not automatically prevented by the editor or revealed to the controller. |

## Concrete interaction scenes

1. **Two protectors:** Warden A protects an AC 16 ally from an attack total 17.
   The ally's effective AC becomes 18 for that attack, which misses. Warden B
   retains its reaction because Protect Ally is no longer eligible. A total
   from a natural 20 remains a hit under the core critical rule. The next attack
   uses the ally's normal AC. No roll is repeated merely to recheck the hit.
2. **Split blast:** Arcane Warden affects three explicit eligible targets with
   one rolled fire-damage value of 9. One fails its save, one succeeds for half,
   one fails but has fire resistance. Expected damage is 9, 4, and 4. Each target
   has its own outcome record linked to the one use and shared roll.
3. **Reaction within a sequence:** Martial Warden makes two attacks. The first
   triggers Protect Ally; resolve that reaction, then the attack, then choose
   the next attack's target. One parent action was spent. Test a separate
   sequence whose second attack is forbidden after a specified interruption.
4. **Zero HP:** Player-state Warden takes damage through its temporary HP to
   zero HP. Run separate death-save, stabilized, damage-at-zero, instant-death,
   and healing variants. Support Warden grants temporary HP at zero; the target
   remains at zero. Mend can restore actual HP if no rule blocks that healing.
5. **Shared effect:** Two Arcane Wardens apply the same Concentration effect to
   overlapping target sets. End one target's effect, then one caster's
   Concentration. Verify the other caster's valid effects and independently
   timed effects survive; no entire target set is discarded by deduplication.
6. **Supply transfer:** Two Quartermasters instantiate Supply Pack separately.
   Emptying one does not change the other. Transfer the existing pack, then use
   one contained charged item after satisfying access requirements. Its runtime
   identity, remaining charges, and nested contents persist.
7. **Boss sequence:** Spend a limited save override; fail and succeed separate
   recharge checks; suppress regeneration; cross a phase threshold; leave and
   return; process a pending reinforcement. No step grants free rest, duplicate
   threshold firing, refreshed uses, or early victory.
8. **Revised counterspell:** Arcane Warden starts a slot spell. The opposing
   caster uses Counterspell and the Warden fails its Constitution save. The
   casting action is wasted but the interrupted spell slot remains. Run the
   successful-save branch and a separate invalid-target casting case whose
   slot is spent. Mend's prevalidation policy cannot override these rules.
9. **Extra reactions:** Give the Boss variant one ordinary reaction plus two
   extra opportunity-attack-only reactions. Spending the ordinary reaction
   prevents Protect Ally but leaves the restricted reactions available for
   valid opportunity attacks. Separately use a reaction-per-turn variant and
   verify refresh on each turn rather than only its own turn.
10. **Monster survival and identity:** A regeneration variant takes its specified
    suppressing damage; a fortitude variant reaches zero with and without an
    exception such as a critical hit; a splitting variant creates two new
    actors at the prescribed trigger. Retained identity, replacement actors,
    concentration ownership, and initiative treatment follow each feature.

## Revised-rule checks

These checks use the official public rules glossary. They are especially useful
for detecting accidental 2014 assumptions:

- Surprise gives Disadvantage on Initiative; it does not impose the old
  surprised-turn behavior.
- Heroic Inspiration rerolls a die immediately after rolling and uses the new
  result. It is not a generic instruction to give Advantage before a test.
- Unarmed grapple/shove uses the target's choice of Strength or Dexterity save;
  escape from a grapple uses its specified Athletics/Acrobatics check.
- Exhaustion applies a penalty of twice the level to D20 Tests and reduces
  Speed by 5 feet per level; six levels cause death.
- Concentration's damage save DC is the greater of 10 and half damage rounded
  down, capped at 30. Starting another Concentration effect ends the old one
  at the specified start point.
- Invisible benefits depend on whether the other creature can see the subject.
  Being Incapacitated blocks actions, Bonus Actions and Reactions, and breaks
  Concentration; it does not by itself mean every save automatically fails.
- Readying a spell pays its casting resources when readied and requires
  Concentration while held; an unused release does not imply a refund.
- Simultaneous effects on a turn can require the current turn's controller to
  choose their order. In simulation the strategy makes that rule-permitted
  choice; the agreed stable fallback applies only where rules leave a tie.
- The revised monster Multiattack description makes its listed attacks and
  additional abilities part of the Attack action (SRD p257). Feature-specific
  substitutions still require explicit permission.
- Revised legendary actions spend one use, occur immediately after another
  creature's turn, and refresh at the start of the monster's turn. Incapacitated
  blocks them. Individual options may also have a next-turn lockout (p257 and
  adult dragon entries). Extra reactions can be restricted to opportunity
  attacks (Hydra p300) or refresh every turn (Marilith p306).
- Recharge X–Y checks a d6 at the start of the monster's turn and also recovers
  on a Short or Long Rest. X/Day recovers on a Long Rest, not automatically at
  midnight (p257). Clock-based recovery is a separate explicit feature.
- One spell slot may be expended to cast a spell per turn. Slot-free casting
  and casting on another creature's turn require separate cases. Revised
  Counterspell uses a Constitution save and does not spend the interrupted
  spell slot on failure, although the casting action is wasted (p120).
- Invalid spell targeting can still expend the spell slot. The homebrew Mend
  example's prevalidation is not a universal rule for all actions or spells.
- A qualifying melee knockout leaves the target at 1 HP, Unconscious and
  starting a Short Rest; it is not the older zero-HP knockout behavior.
- A healing potion can be drunk or administered to a creature within 5 feet
  as a Bonus Action. Heavy weapons use Strength 13 for melee or Dexterity 13
  for ranged eligibility rather than the old blanket Small-creature penalty.
  Nick moves the Light property's extra attack into the Attack action rather
  than granting a second extra attack.
- A completed Long Rest recovers all spent Hit Point Dice. Interruption and
  resumption still follow the rest's explicit conditions.

## Monster exception anchors

The official SRD provides concrete anchors for variants without copying whole
stat blocks. Page numbers are printed SRD 5.2.1 pages.

| Source entry | Page | Fixture exception |
| --- | --- | --- |
| Aboleth | 258 | Control, repeated save on damage, healing restriction, in-lair use changes |
| Black Pudding | 265 | Split and Bloodied/damage-dependent triggers; new actor identities |
| Ghost | 287–288 | Possessor, host, and controller are distinct relationships |
| Hydra | 300 | Head count changes attacks and restricted reaction capacity |
| Marilith | 306 | Reaction refresh on every turn |
| Tarrasque | 331 | Spell reflection and retargeting |
| Troll | 333 | Regeneration suppressed by specified damage |
| Zombie | 343 | Zero-HP replacement save with critical/damage exceptions |

These anchors expand the case catalogue. Each actual fixture ability must
still preserve its source's exact restrictions and timing before that case is
considered specified or supported.

## Coverage boundaries and completion rule

Spatial families stay visible in the checklist, with reachability, visibility,
movement events and affected target sets supplied explicitly. No maps, tokens,
cover geometry, or pathfinding are added. A case that cannot be represented with
the allowed assumptions is marked blocked/deferred, never passed by omission.

Public rules establish the core baseline. Individual class/subclass features,
feats, species traits, spells, magic items, and monster abilities need exception
cases when their mechanics differ. Legacy, optional, and homebrew alternatives
must declare a distinct selected rule choice where incompatible; they do not
silently become the D&D 2024 default. Lair or unusual boss mechanics must cite
their specific source instead of being assumed universal to revised monsters.

The design fixture is complete for a mechanic family only when its source,
representative setup, edge cases, and expected observations are written down.
The current table is a coverage specification. It does not assert implemented
simulation support, exhaustive sourcebook coverage, or passing executable tests.
Keep refinements in these documents; the earlier ban on model implementation,
runtime persistence, and D&D Beyond data snapshots remains in force.

## Primary references

- [D&D Beyond Rules Glossary](https://www.dndbeyond.com/sources/dnd/free-rules/rules-glossary), accessed 2026-09-15. Covers D20 tests, action definitions, conditions, recovery, monster stat notation, and timing.
- [Playing the Game](https://www.dndbeyond.com/sources/dnd/free-rules/playing-the-game): attack resolution, damage, zero HP, initiative and combat.
- [Spells](https://www.dndbeyond.com/sources/dnd/free-rules/spells): casting, components, spell resources, and individual spell exceptions.
- [Equipment](https://www.dndbeyond.com/sources/dnd/free-rules/equipment): weapon properties/mastery, armor, consumables and carrying rules.
- [Official SRD 5.2.1 PDF](https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf), accessed 2026-09-15; printed pages cited above.
- [Official SRD index](https://www.dndbeyond.com/srd).

The public rule sections and SRD were checked on 2026-09-15 for the constraints
listed here. An exhaustive audit of all published options remains open.
