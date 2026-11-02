---
splat_schema: 1
splat_id: 885c82de-0918-50a6-90e1-c501b866da58
splat_kind: creature
splat_key: dnd2024:creatures/warrior_infantry
splat_name: Warrior Infantry
splat_origin:
  key: dnd2024:reference/creature/warrior_infantry
splat_sources:
- title: SRD 5.2.1
  locator: pp. 336, 337
  url: https://www.dndbeyond.com/srd
---

# Warrior Infantry

## Source text

<!-- SRD page 336 -->

Medium or Small Humanoid, Neutral\
AC 13 Initiative +0 (10)\
HP 9 (2d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 11 +0 +0 Con 11 +0 +0\
Int 8 −1 −1 Wis 11 +0 +0 Cha 8 −1 −1\
Gear Chain Shirt, Spear\
Senses Passive Perception 10\
Languages Common\
CR 1/8 (XP 25; PB +2)

<!-- SRD page 337 -->

### Traits

Pack Tactics. The warrior has Advantage on an attack roll against a creature if at least one of the warrior’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

### Actions

Spear. Melee or Ranged Attack Roll: +3, reach 5 ft. or range 20/60 ft. Hit: 4 (1d6 + 1) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/336|page 336]], [[../../srd-5.2.1/pages/337|page 337]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` gear_text ` | ` Chain Shirt, Spear ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` Common ` |
| ` cr_text ` | ` 1/8 (XP 25; PB +2) ` |
| ` hit_points_text ` | ` 9 (2d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 13 ` | ` 1 ` |
| ` Dexterity ` | ` 11 ` | ` 0 ` |
| ` Constitution ` | ` 11 ` | ` 0 ` |
| ` Intelligence ` | ` 8 ` | ` -1 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 8 ` | ` -1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 9 ` |  |
| ` initiative ` | ` 0 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/warrior_infantry ` |
| ` implementation ` | ` dnd2024:creature_warrior_infantry ` |

### Parameters

#### Features

##### Pack tactics

<!-- splat:yaml path=splat_rule/parameters/features/pack_tactics -->

```yaml
economy: passive
advantage:
  roll: attack
  target_kind: creature
  requires:
    ally_distance_to_target_max_feet: 5
    ally_incapacitated: false
```

##### Spear

<!-- splat:yaml path=splat_rule/parameters/features/spear -->

```yaml
economy: action
op: attack
attack_modes:
- melee
- ranged
attack_bonus: 3
targets:
  count: 1
hit:
- op: damage
  average: 4
  amount:
    dice:
      count: 1
      sides: 6
    add: 1
  damage_type: piercing
reach_feet: 5
range_feet: 20
long_range_feet: 60
```

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
| ` row_1 ` | ` Chain Shirt ` | ` 1 ` |
| ` row_2 ` | ` Spear ` | ` 1 ` |

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
