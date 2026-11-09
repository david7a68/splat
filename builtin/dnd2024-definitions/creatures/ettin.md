---
splat_schema: 2
splat_id: fc14b342-51b7-59a4-9a62-b0680cb2fdd5
splat_kind: creature
splat_key: dnd2024:creatures/ettin
splat_name: Ettin
splat_origin: dnd2024:reference/creature/ettin
splat_sources:
- SRD 5.2.1, pp. 284, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 116, Habitat
---

# Ettin


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Giant |
| Alignment | Chaotic Evil |
| Armor Class | 12 |
| Hit Points | 85 (10d10 + 30) |
| Initiative | -1 (9) |
| Speed | 40 ft. |
| Challenge | 4 (XP 1,100; PB +2) |
| Skills | Perception +4 |
| Senses | Darkvision 60 ft.; Passive Perception 14 |
| Languages | Giant |
| Gear | Battleaxe, Morningstar |
| Immunities | Blinded, Charmed, Deafened, Frightened, Stunned, Unconscious |
| Habitat | Hill, Mountain, Underdark |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 21 | +5 | +5 |
| Dex | 8 | -1 | -1 |
| Con | 17 | +3 | +3 |
| Int | 6 | -2 | -2 |
| Wis | 10 | +0 | +0 |
| Cha | 8 | -1 | -1 |


## Source text

<!-- SRD page 284 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Giant, Chaotic Evil\
> AC 12  Initiative −1 (9)\
> HP 85 (10d10 + 30)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 21 +5 +5 Dex 8 −1 −1 Con 17 +3 +3\
> Int 6 −2 −2 Wis 10 +0 +0 Cha 8 −1 −1\
> Skills Perception +4\
> Immunities Blinded, Charmed, Deafened, Frightened, Stunned, Unconscious\
> Gear Battleaxe, Morningstar\
> Senses Darkvision 60 ft.; Passive Perception 14\
> Languages Giant\
> CR 4 (XP 1,100; PB +2)


### Actions

Multiattack. The ettin makes one Battleaxe attack and one Morningstar attack.

Battleaxe. Melee Attack Roll: +7, reach 5 ft. Hit: 14 (2d8 + 5) Slashing damage. If the target is a Large or smaller creature, it has the Prone condition.

Morningstar. Melee Attack Roll: +7, reach 5 ft. Hit: 14 (2d8 + 5) Piercing damage, and the target has Disadvantage on the next attack roll it makes before the end of its next turn.

Original layout: [[../../srd-5.2.1/pages/284|page 284]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/ettin ` |
| ` implementation ` | ` dnd2024:creature_ettin ` |

### Parameters

#### Features

##### Battleaxe

<!-- splat:yaml path=splat_rule/parameters/features/battleaxe -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 7
targets:
  count: 1
hit:
- op: damage
  average: 14
  amount:
    dice:
      count: 2
      sides: 8
    add: 5
  damage_type: slashing
- op: if
  test:
    target_kind: creature
    target_size_max: large
  then:
  - op: condition
    condition: prone
reach_feet: 5
```

##### Morningstar

<!-- splat:yaml path=splat_rule/parameters/features/morningstar -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 7
targets:
  count: 1
hit:
- op: damage
  average: 14
  amount:
    dice:
      count: 2
      sides: 8
    add: 5
  damage_type: piercing
- op: disadvantage
  roll: attack
  count: 1
  ends:
    event: turn.end
    participant: target
    occurrence: next
reach_feet: 5
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: battleaxe
  count: 1
- op: use_feature
  feature: morningstar
  count: 1
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` conditions ` | ` ["blinded", "charmed", "deafened", "frightened", "stunned", "unconscious"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Item name:
    path: item_name
    type: string
  Quantity:
    path: quantity
    type: integer
-->

| Key | Item name | Quantity |
| --- | --- | --- |
| ` row_1 ` | ` Battleaxe ` | ` 1 ` |
| ` row_2 ` | ` Morningstar ` | ` 1 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["giant"] ` |
| ` understands ` | ` ["giant"] ` |

#### Stat block conventions

<!-- splat:yaml path=splat_rule/parameters/stat_block_conventions -->

```yaml
attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
save_dc: supplied_total
damage_mode_choice:
- fixed_average
- rolled_dice
condition_source: self
multiattack_child_economy_cost: 0
multiattack_child_resource_costs_retained: true
spellcasting:
  default_level: spell_minimum
  spell_slots_expended: false
  required_materials: available
  casting_time_at_least_one_minute:
    uses_normal_casting_duration: true
    magic_action_each_turn: true
    concentration_required: true
geometry_assumptions:
  chooser: gm
  supplies:
  - range
  - visibility
  - line_of_sight
  - area_membership
  - reachable_locations
  - unoccupied_locations
  - terrain
  - plane
  - form_appearance
```
