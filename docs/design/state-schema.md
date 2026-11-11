# Participant, health, item and relationship schema

> Character storage update: the [implemented character cache](character-cache.md)
> supersedes earlier memory-only restrictions for D&D Beyond character records.
> Encounter run state and encounter integration remain separate future work.

Status: candidate note-model contract, documentation only. This completes runtime
record shapes and authored policies alongside [Mechanics](mechanics-schema.md).
Runtime records described here stay in memory; they are not new saved files.

## Participant state

ParticipantState contains `id`, `origin` (entry UUID and roster placement),
`side`, `controller`, `membership` (optional party reference), `presence`,
`stats`, `health`, `resources`, `budgets`, `limits`, `inventory`, `applications`,
`relationships`, `form_layers` and `schedule`. `presence` is
`active|absent|removed`; dead is health state, not synonymous with absent.
`stats` retains resolved supplied definitions, known contributions and effective
values. Current health or stat adjustments never overwrite a shared template.

Controller is `{kind: manual, role: player|gm}` or `{kind: strategy,
strategy: StrategyRef, parameters: {...}, knowledge: controller|gm}`. The earlier
fixture `{strategy: manual}` is the manual-GM shorthand. A control effect adds
a scoped controller relationship, preserving original controller and actor ID.
Changing controller does not automatically change side, initiative, allegiance,
inventory owner or Concentration; the feature specifies each relevant change.

An authored creature can set `splat_initial` with `hp`, `temporary_hp`,
`resources`, `hit_dice_spent`, `conditions` and `equipment`. Encounter placement
initial fields override these starting values; omitted values use the creature
defaults, then full HP/pools and no temporary HP/conditions. Explicit initial
conditions reference definitions and carry source bindings or report a missing
source. Never merge two HP values or duplicate inventory quantities as YAML.

`splat_traits` maps local keys to `{rule: mechanic, parameters, source?: EntryRef}`
for passive source-defined traits. A trait that grants named budgets, modifiers,
relationships or operations must expose those grants in its registered contract;
it cannot execute arbitrary code or make a prose paragraph authoritative.

## Health state and operations

HealthState is `{hp, effective_max_hp, temporary_hp?, status, death_saves,
hit_dice_spent, recovery_clock?}`. `status` is `alive|dying|stable|dead`; knockout
is a source-linked Unconscious application while still alive at 1 HP. Counters
are `{successes: 0..3, failures: 0..3}` and are meaningful only under the selected
zero-HP policy. `splat_health` contains `zero_hp: dnd2024:monster|dnd2024:character`
and optional supported rule exceptions. Default a monster template to monster
and a live party character to character; the GM may explicitly choose character
treatment for a monster placement. This is independent of encounter side.

TemporaryHpState is `{amount, grant_id, source_use?, expiry?}`. A new grant asks
the eligible controller to keep the old grant or accept the new one; it does
not add them or automatically choose the maximum. The default expiry is depletion
or a Long Rest; a specific source can supply another lifetime. Damage absorbed
by temporary HP is still damage taken for applicable triggers. Zero HP plus
temporary HP is not revival or restoration of consciousness.

| Transition | Rule contract and required state result |
| --- | --- |
| `splat:damage` | Apply adjusted damage to temporary HP first, then HP. Record incoming/applied/excess amounts; do not create negative stored HP |
| HP reaches zero, monster policy | Death unless an explicit feature changes that outcome |
| HP reaches zero, character policy | Massive-damage check, otherwise dying and Unconscious under its source; do not erase other conditions |
| Damage while at zero | Apply the rule's death-save failures, critical exceptions and death threshold; end stability as appropriate |
| `death_save` | At start of turn for a dying subject: ordinary 10+ success, natural 1 two failures, natural 20 restores 1 HP; three successes stabilize, three failures kill |
| `stabilize` | End required death saves while retaining zero HP/Unconscious; clear counters, create the source-defined recovery clock |
| `heal` to positive HP | Cap at effective maximum; end zero-HP unconsciousness and reset death counters; do not remove unrelated Unconscious sources |
| `knockout` | Eligible melee attack that would reduce HP to zero may instead leave 1 HP, Unconscious, starting Short Rest; source-specific ending by rest/healing/Medicine |
| `revive` | Requires a source rule, eligibility and costs; restore specified health and source-specific consequences, not a universal full reset |
| Maximum HP changes | Source-linked modifier changes effective maximum, clamps current HP where required, and zero maximum invokes the death rule |

Health commands require `subject` and `rule`. `stabilize` can carry an already
resolved qualifying check or invoke the Help/Medicine DC 10 rule; no guaranteed
success by merely naming the command. Stable recovery uses the core 1d4-hour
clock if unhealed; an interrupted/invalid clock follows its rule. Regaining HP
or becoming Stable resets death-save counters. Positive HP does not itself end
death unless an authorized revival/zero-HP exception applies.

Damage replacement effects (for example Undead Fortitude) bind to the pending
zero-HP result and inspect critical/damage provenance before applying a
rule-authorized replacement. Do not kill, remove the actor, and later recreate
it with a new ID to represent that survival.

## Inventory instances and transitions

ItemState has `id`, `definition_id`, `source_address`, `holder`, `quantity`,
`equipment`, `attuned_to?`, `resources`, `contents`, `access_state`, and
`applications`. Holder is exactly one of `{participant: id}`, `{item: id}`, or
`{standalone: placement_id}`. Contents is an ordered set of child IDs, not copied
item records. A runtime invariant enforces one immediate holder and no cycles.
Authored row keys locate the starting placement; they do not identify ownership
after transfer. Empty consumed stacks are retired from inventories, while
historical use/event records keep their source ID.

EquipmentState includes `mode: carried|stored|worn|wielded`, assigned body slots,
occupied hands/body parts, and `melded: bool`. Attunement is a distinct relation
between participant and item, not just a boolean once instantiated. Template
`Attuned: true` binds to the initial bearer and validates limits. Item granted
actions/effects retain item instance ID and test current equipment/attunement.
Dropping an item does not universally end attunement; its rule governs ending.

`splat:inventory` command inputs:

| Command | Required fields | State semantics |
| --- | --- | --- |
| `transfer` | `item`, `to: Holder`, optional `quantity`, `rule` | Atomic remove/add; preserve individual identity and nested contents |
| `split` | `item`, `quantity` | Compatible stack split creates a new stack ID and preserves provenance; source quantity decreases |
| `merge` | `items`, `destination`, `rule` | Only identical definition and compatible state; retain one selected stack identity and retire merged IDs |
| `equip` | `item`, `bearer`, `mode`, optional `hands/slots`, `rule` | Validate availability/training and spend rule-defined interaction costs; rebuild known stats |
| `attune` / `unattune` | `item`, `bearer`, `rule` | Apply process, eligibility, limits, time and effects; no instantaneous bypass of rest requirements |
| `access` | `item`, `state`, `rule` | Open/close/unlock/retrieve as a supported rule-defined action |
| `consume` | `item`, `quantity`, `rule` | Debit the exact stack at the specified stage; quantity cost is this operation's compact form |

Mass/volume comparison uses exact conversion constants (1 lb = 0.45359237 kg,
1 ft3 = 28.316846592 l). Count limits count the items as the definition specifies,
not automatically table rows. `containment.capacity` accepts `mass`, `volume`,
and `count`, each optional. `containment.constraints` adds typed predicates for
allowed categories or forbidden nested definitions. `carried_mass` is
`self_and_contents`, `{fixed: Quantity}`, or `{rule: mechanic, parameters}`.
Unknown measurements remain unknown. Access and capacity checks can fail without
rolling back already paid action costs, but never partially relocate contents.

Effect-granted weight changes follow their source and holder; a moved container
does not copy a bearer-only modifier to its new bearer. Body-part occupancy is
shared with grapple/form relationships so a grappling hand cannot simultaneously
hold a weapon unless a feature permits that combination.

## Typed relationships

RelationshipState is `{id, kind, source_use, source, target, rule, parameters,
grants, expiry, status}`. Generic fields identify endpoints; each kind has a
typed parameter contract. Ending removes only grants owned by this relation.
Unsupported kinds are blocked, not treated as decorative tags.

| Kind | Additional parameters | Required lifetime behavior |
| --- | --- | --- |
| `grapple` | `gripping_part`, `reach`, `escape_dc`, `escape_options` | One target per gripping part; source incapacitation, voluntary release, escape or out-of-range ends the relation |
| `charm` / `fear` | Source actor, imposed condition, restrictions/check policy | Preserve source-relative restrictions and independent applications from different actors |
| `control` | Controller grant, permitted choices, optional side change, priority rule | Restore prior applicable control on end without destroying identity |
| `summon` | Summoner, created actors, command policy, initiative policy, expiry and end behavior | Created actors are independent; concentration may end them when the spell specifies |
| `possession` | Possessor and host, controlled body, stat bindings, targetability, end policy | Both identities remain; host state is not overwritten by the possessor definition |
| `swallow` | Swallower, swallowed actor, internal capacity, grants, recurring effects, exit/escape policy | Keep the swallowed participant and its timing; inside is a relationship, not deletion or death |
| `concentration` | Owner, dependent use/application IDs | End only registered dependents by the source-defined reasons |
| `attunement` | Bearer, item, restrictions and ending conditions | Transfer/drop is distinct from breaking this relation |

`splat_relationships` maps local keys to templates of these kinds. Create binds
source/target and parameters; end names a relation instance and rule reason.
Repeated relations do not overwrite by `(kind,target)`; ID and source matter.
Grapple initiation uses the revised save-choice rule for unarmed strikes;
monster on-hit grapples use their own rule. Escape is an action/check contract,
not automatically the same saving throw used at initiation.

Possession's `stat_bindings` maps each affected stat to host/possessor data;
unmapped fields follow an explicit `fallback` profile. A Ghost-like profile can
bind physical ability modifiers and Speed from the host while preserving its
other stated statistics. Do not silently recompute a host-derived modifier from
the possessor's score. Targetability and end triggers (including body zero HP)
are separate fields. Source-specific post-possession immunity is an effect
bound to that possessor, not immunity to all possession.

## Forms

`splat_forms` maps local keys to `FormDef`:

```yaml
splat_forms:
  beast:
    definition: {key: "my_homebrew:creatures/beast_shape"}
    rule: "dnd2024:polymorph"
    stats:
      replace: [strength, dexterity, constitution, intelligence, wisdom, charisma,
                armor_class, speed, senses, actions]
      retain: [identity, hp, hit_dice]
      remainder: rule
    equipment: {policy: meld}
    health:
      hp: retain
      temporary_hp:
        amount: {binding: form_max_hp}
        bind_to_form: true
        end_form_when: no_temporary_hp
        remove_on_revert: true
    ongoing: {policy: retain_unless_rule_ends}
    end: {rule: "dnd2024:polymorph_end"}
```

This is an embedded shape example, not an unresolved reference in an installed
package. The named D&D contract determines all retained/replaced exceptions;
explicit lists cannot contradict it. `remainder: rule` requires that contract
to supply a complete field policy, never 'copy everything else by default'.
`temporary_hp.end_form_when` follows the specific source's exact wording.
The Polymorph profile uses `no_temporary_hp`; other effects may instead refer
to depletion of their own grant or have no temporary-HP ending condition.

FormState has `id`, `source_use`, `definition`, `profile`, `retained_bindings`,
`overridden_bindings`, `equipment_state`, `granted_temp_hp_id?` and `expiry`.
Original HP stays attached to the actor for revised Polymorph, Wild Shape,
Shapechange and creature-to-creature True Polymorph; no generic beast HP pool
or 2014 damage-overflow restoration is imposed. Each source defines its
temporary-HP grant and end conditions. Wild Shape losing temporary HP is not
automatically a Polymorph-style reversion rule.

Form `stats.replace/retain` can select individual stat keys or declared groups
such as `senses`, `actions`, `proficiencies`, `languages`, `identity`, `hp`, and
`hit_dice`. Overlapping contradictory selectors are invalid. Identity is always
retained by a form operation; changing actor identity requires a lifecycle
operation. `health.hp: retain` and the HP group express the same policy and
must agree; they do not store two authoritative HP values.

Entering a new form is a rule-resolved change, not a new turn or actor ID.
Nested forms require an explicit profile for layering/ending; the engine does
not always push and later pop a stat snapshot. Reversion recomputes effective
stats from the retained actor and surviving effects. Death's true-form rule
and source-specific exceptions apply. Conditions and curses ordinarily carry
between forms unless the effect states otherwise; equipment melding and
attunement interactions follow the selected profile.

## Spawn, split, departure and replacement

`splat:participant` commands:

- `spawn`: `definition`, `count`, `initial`, `side`, `controller`, `initiative`,
  optional `relation` and `origin`; creates new actors with independent IDs.
- `depart` / `return`: `participants`, `reason`; retains actor IDs and all state.
- `replace`: `participants`, `with: [SpawnSpec,...]`, `rule`, explicit
  `transfer: {hp, resources, items, effects, relationships}` policies.

A split is a replacement/spawn rule with source-specific stat/health allocation.
Do not deep-copy Concentration, inventories or controllers into every child.
Every transfer field must specify `retain_original|move_to_one|divide_by_rule|
end|new_from_definition`, with an eligible rule for that field. No missing policy
silently duplicates state. Possession and ordinary form change do not use spawn.
Departed actors retain scheduled durations and required checks under the
previously agreed presence rules. A temporary absence grants no free rest or
turn reset. The encounter scheduler controls first eligible turns on entry.

## Sources and known scope choices

- [SRD 5.2.1](https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf): damage and zero HP pp16–18; Wild Shape p43; Polymorph p153; Shapechange pp161–162; True Polymorph pp170–171; grapple p182; possession p186; shape-shifting p187; Ghost p288.
- [Damage and Healing](https://www.dndbeyond.com/sources/dnd/free-rules/playing-the-game#DamageandHealing).
- [Rules Glossary](https://www.dndbeyond.com/sources/dnd/free-rules/rules-glossary).

Source checks performed 2026-09-15. Hydra next-turn reaction capacity and manual
readied-spell targeting keep the user's recorded decisions. Spatial predicates
are explicit facts, not a new map engine. Specialized source content still
needs a supported rule profile; the schema does not claim all books are encoded.
