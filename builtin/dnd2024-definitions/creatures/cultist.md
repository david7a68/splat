---
splat_schema: 2
splat_id: 90ed0908-d71a-533a-b9e2-616435f0b171
splat_kind: creature
splat_key: dnd2024:creatures/cultist
splat_name: Cultist
splat_origin: dnd2024:reference/creature/cultist
splat_sources:
- SRD 5.2.1, pp. 278, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 84, Habitat
---

# Cultist


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Humanoid |
| Alignment | Neutral |
| Armor Class | 12 |
| Hit Points | 9 (2d8) |
| Initiative | +1 (11) |
| Speed | 30 ft. |
| Challenge | 1/8 (XP 25; PB +2) |
| Skills | Deception +2, Religion +2 |
| Senses | Passive Perception 10 |
| Languages | Common |
| Gear | Leather Armor, Sickle |
| Habitat | Any |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 11 | +0 | +0 |
| Dex | 12 | +1 | +1 |
| Con | 10 | +0 | +0 |
| Int | 10 | +0 | +0 |
| Wis | 11 | +0 | +2 |
| Cha | 10 | +0 | +0 |


## Source text

<!-- SRD page 278 -->

Medium or Small Humanoid, Neutral\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 12 Initiative +1 (11)\
> HP 9 (2d8)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 11 +0 +0 Dex 12 +1 +1 Con 10 +0 +0\
> Int 10 +0 +0 Wis 11 +0 +2 Cha 10 +0 +0\
> Skills Deception +2, Religion +2\
> Gear Leather Armor, Sickle\
> Senses Passive Perception 10\
> Languages Common\
> CR 1/8 (XP 25; PB +2)


### Actions

Ritual Sickle. Melee Attack Roll: +3, reach 5 ft. Hit: 3 (1d4 + 1) Slashing damage plus 1 Necrotic damage.

Original layout: [[../../srd-5.2.1/pages/278|page 278]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/cultist ` |
| ` implementation ` | ` dnd2024:creature_cultist ` |

### Parameters

#### Features

##### Ritual sickle

<!-- splat:yaml path=splat_rule/parameters/features/ritual_sickle -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 3
targets:
  count: 1
hit:
- op: damage
  average: 3
  amount:
    dice:
      count: 1
      sides: 4
    add: 1
  damage_type: slashing
- op: damage
  average: 1
  amount:
    constant: 1
  damage_type: necrotic
reach_feet: 5
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 2 ` |
| ` religion ` | ` 2 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

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
| ` row_1 ` | ` Leather Armor ` | ` 1 ` |
| ` row_2 ` | ` Sickle ` | ` 1 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common"] ` |
| ` understands ` | ` ["common"] ` |

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
