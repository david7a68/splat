---
splat_schema: 1
splat_id: 4c98848f-6ea9-5194-b756-113d2f989412
splat_kind: creature
splat_key: dnd2024:creatures/hobgoblin_captain
splat_name: Hobgoblin Captain
splat_origin:
  key: dnd2024:reference/creature/hobgoblin_captain
splat_sources:
- title: SRD 5.2.1
  locator: pp. 299
  url: https://www.dndbeyond.com/srd
---

# Hobgoblin Captain

## Source text

<!-- SRD page 299 -->

Medium Fey (Goblinoid), Lawful Evil\
AC 17 Initiative +4 (14)\
HP 58 (9d8 + 18)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 14 +2 +2 Con 14 +2 +2\
Int 12 +1 +1 Wis 10 +0 +0 Cha 13 +1 +1\
Gear Greatsword, Half Plate Armor, Longbow\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Common, Goblin\
CR 3 (XP 700; PB +2)\

### Traits

Aura of Authority. While in a 10-foot Emanation origi-\
nating from the hobgoblin, the hobgoblin and its allies\
have Advantage on attack rolls and saving throws,\
provided the hobgoblin doesn’t have the Incapacitated\
condition.\

### Actions

Multiattack. The hobgoblin makes two attacks, using\
Greatsword or Longbow in any combination.\
Greatsword. Melee Attack Roll: +4, reach 5 ft. Hit: 9\
(2d6 + 2) Slashing damage plus 3 (1d6) Poison damage.\
Longbow. Ranged Attack Roll: +4, range 150/600\
ft. Hit: 6 (1d8 + 2) Piercing damage plus 5 (2d4) Poi-\
son damage.\

Original layout: [[../../srd-5.2.1/pages/299|page 299]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` fey ` |
| ` alignment ` | ` Lawful Evil ` |
| ` tags_text ` | ` (Goblinoid) ` |
| ` movement_text ` | ` 30 ft. ` |
| ` gear_text ` | ` Greatsword, Half Plate Armor, Longbow ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Common, Goblin ` |
| ` cr_text ` | ` 3 (XP 700; PB +2) ` |
| ` hit_points_text ` | ` 58 (9d8 + 18) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 15 ` | ` 2 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 12 ` | ` 1 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 13 ` | ` 1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 58 ` |  |
| ` initiative ` | ` 4 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/hobgoblin_captain ` |
| ` implementation ` | ` dnd2024:creature_hobgoblin_captain ` |

### Parameters

#### Features

##### Greatsword

<!-- splat:yaml path=splat_rule/parameters/features/greatsword -->

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
  average: 9
  amount:
    dice:
      count: 2
      sides: 6
    add: 2
  damage_type: slashing
- op: damage
  average: 3
  amount:
    dice:
      count: 1
      sides: 6
    add: 0
  damage_type: poison
reach_feet: 5
```

##### Longbow

<!-- splat:yaml path=splat_rule/parameters/features/longbow -->

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
  average: 6
  amount:
    dice:
      count: 1
      sides: 8
    add: 2
  damage_type: piercing
- op: damage
  average: 5
  amount:
    dice:
      count: 2
      sides: 4
    add: 0
  damage_type: poison
range_feet: 150
long_range_feet: 600
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` repeat_choice ` |
| ` count ` | ` 2 ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/multiattack/choices
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Feature:
    path: feature
    type: symbol
-->

| Key | Feature |
| --- | --- |
| ` row_1 ` | ` Greatsword ` |
| ` row_2 ` | ` Longbow ` |

##### Aura of authority

<!-- splat:yaml path=splat_rule/parameters/features/aura_of_authority -->

```yaml
economy: passive
requires:
  self_incapacitated: false
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    size_feet: 10
    origin: self
  relation:
  - self
  - ally
advantage:
  rolls:
  - attack
  - saving_throw
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
| ` row_1 ` | ` Greatsword ` | ` 1 ` |
| ` row_2 ` | ` Half Plate Armor ` | ` 1 ` |
| ` row_3 ` | ` Longbow ` | ` 1 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common", "goblin"] ` |
| ` understands ` | ` ["common", "goblin"] ` |

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
