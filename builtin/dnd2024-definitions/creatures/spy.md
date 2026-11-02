---
splat_schema: 1
splat_id: 553b6a42-8669-5cbc-87d0-f0d3f7c9e6ed
splat_kind: creature
splat_key: dnd2024:creatures/spy
splat_name: Spy
splat_origin:
  key: dnd2024:reference/creature/spy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 329
  url: https://www.dndbeyond.com/srd
---

# Spy

## Source text

<!-- SRD page 329 -->

Medium or Small Humanoid, Neutral\
AC 12 Initiative +4 (14)\
HP 27 (6d8)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 15 +2 +2 Con 10 +0 +0\
Int 12 +1 +1 Wis 14 +2 +2 Cha 16 +3 +3\
Skills Deception +5, Insight +4, Investigation +5, Perception +6, Sleight of Hand +4, Stealth +6\
Gear Hand Crossbow, Shortsword, Thieves’ Tools\
Senses Passive Perception 16\
Languages Common plus one other language\
CR 1 (XP 200; PB +2)

### Actions

Shortsword. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage plus 7 (2d6) Poison damage.

Hand Crossbow. Ranged Attack Roll: +4, range 30/120 ft. Hit: 5 (1d6 + 2) Piercing damage plus 7 (2d6) Poison damage.

### Bonus Actions

Cunning Action. The spy takes the Dash, Disengage, or Hide action.

Original layout: [[../../srd-5.2.1/pages/329|page 329]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft., Climb 30 ft. ` |
| ` skills_text ` | ` Deception +5, Insight +4, Investigation +5, Perception +6, Sleight of Hand +4, Stealth +6 ` |
| ` gear_text ` | ` Hand Crossbow, Shortsword, Thieves’ Tools ` |
| ` senses_text ` | ` Passive Perception 16 ` |
| ` languages_text ` | ` Common plus one other language ` |
| ` cr_text ` | ` 1 (XP 200; PB +2) ` |
| ` hit_points_text ` | ` 27 (6d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 0 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 12 ` | ` 1 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 16 ` | ` 3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 27 ` |  |
| ` initiative ` | ` 4 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` climb_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/spy ` |
| ` implementation ` | ` dnd2024:creature_spy ` |

### Parameters

#### Features

##### Shortsword

<!-- splat:yaml path=splat_rule/parameters/features/shortsword -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 4
targets:
  count: 1
hit:
- op: damage
  average: 5
  amount:
    dice:
      count: 1
      sides: 6
    add: 2
  damage_type: piercing
- op: damage
  average: 7
  amount:
    dice:
      count: 2
      sides: 6
    add: 0
  damage_type: poison
reach_feet: 5
```

##### Hand crossbow

<!-- splat:yaml path=splat_rule/parameters/features/hand_crossbow -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 4
targets:
  count: 1
hit:
- op: damage
  average: 5
  amount:
    dice:
      count: 1
      sides: 6
    add: 2
  damage_type: piercing
- op: damage
  average: 7
  amount:
    dice:
      count: 2
      sides: 6
    add: 0
  damage_type: poison
range_feet: 30
long_range_feet: 120
```

##### Cunning action

<!-- splat:values path=splat_rule/parameters/features/cunning_action -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` take_action ` |
| ` choice ` | ` ["dash", "disengage", "hide"] ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 5 ` |
| ` insight ` | ` 4 ` |
| ` investigation ` | ` 5 ` |
| ` perception ` | ` 6 ` |
| ` sleight_of_hand ` | ` 4 ` |
| ` stealth ` | ` 6 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 16 ` |

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
- row_3
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
| ` row_1 ` | ` Hand Crossbow ` | ` 1 ` |
| ` row_2 ` | ` Shortsword ` | ` 1 ` |
| ` row_3 ` | ` Thieves’ Tools ` | ` 1 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common"] ` |
| ` understands ` | ` ["common"] ` |

##### Additional languages

<!-- splat:values path=splat_rule/parameters/communication/additional_languages -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` chooser ` | ` gm ` |
| ` distinct_from_known ` | ` true ` |
| ` can_speak ` | ` true ` |
| ` can_understand ` | ` true ` |

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
