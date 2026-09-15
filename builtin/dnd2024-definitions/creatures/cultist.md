---
splat_schema: 1
splat_id: 90ed0908-d71a-533a-b9e2-616435f0b171
splat_kind: creature
splat_key: dnd2024:creatures/cultist
splat_name: Cultist
splat_origin:
  key: dnd2024:reference/creature/cultist
splat_sources:
- title: SRD 5.2.1
  locator: pp. 278
  url: https://www.dndbeyond.com/srd
---

# Cultist

## Source text

<!-- SRD page 278 -->

Medium or Small Humanoid, Neutral\
AC 12 Initiative +1 (11)\
HP 9 (2d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 12 +1 +1 Con 10 +0 +0\
Int 10 +0 +0 Wis 11 +0 +2 Cha 10 +0 +0\
Skills Deception +2, Religion +2\
Gear Leather Armor, Sickle\
Senses Passive Perception 10\
Languages Common\
CR 1/8 (XP 25; PB +2)\

### Actions

Ritual Sickle. Melee Attack Roll: +3, reach 5 ft. Hit: 3\
(1d4 + 1) Slashing damage plus 1 Necrotic damage.\

Original layout: [[../../srd-5.2.1/pages/278|page 278]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Deception +2, Religion +2 ` |
| ` gear_text ` | ` Leather Armor, Sickle ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` Common ` |
| ` cr_text ` | ` 1/8 (XP 25; PB +2) ` |
| ` hit_points_text ` | ` 9 (2d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 11 ` | ` 0 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 11 ` | ` 2 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 9 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |

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
