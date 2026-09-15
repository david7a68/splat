---
splat_schema: 1
splat_id: a13cbe49-3f0c-5ea5-aef8-a35899999be3
splat_kind: creature
splat_key: dnd2024:creatures/bugbear_warrior
splat_name: Bugbear Warrior
splat_origin:
  key: dnd2024:reference/creature/bugbear_warrior
splat_sources:
- title: SRD 5.2.1
  locator: pp. 272
  url: https://www.dndbeyond.com/srd
---

# Bugbear Warrior

## Source text

<!-- SRD page 272 -->

Medium Fey (Goblinoid), Chaotic Evil\
AC 14 Initiative +2 (12)\
HP 33 (6d8 + 6)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 14 +2 +2 Con 13 +1 +1\
Int 8 −1 −1 Wis 11 +0 +0 Cha 9 −1 −1\
Skills Stealth +6, Survival +2\
Gear Hide Armor, Light Hammers (3)\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Common, Goblin\
CR 1 (XP 200; PB +2)\

### Traits

Abduct. The bugbear needn’t spend extra movement to\
move a creature it is grappling.\

### Actions

Grab. Melee Attack Roll: +4, reach 10 ft. Hit: 9 (2d6\
+ 2) Bludgeoning damage. If the target is a Medium\
or smaller creature, it has the Grappled condition (es-\
cape DC 12).\
Light Hammer. Melee or Ranged Attack Roll: +4 (with\
Advantage if the target is Grappled by the bugbear),\
reach 10 ft. or range 20/60 ft. Hit: 9 (3d4 + 2) Blud-\
geoning damage.\

Original layout: [[../../srd-5.2.1/pages/272|page 272]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` fey ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Goblinoid) ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Stealth +6, Survival +2 ` |
| ` gear_text ` | ` Hide Armor, Light Hammers (3) ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Common, Goblin ` |
| ` cr_text ` | ` 1 (XP 200; PB +2) ` |
| ` hit_points_text ` | ` 33 (6d8 + 6) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 15 ` | ` 2 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 8 ` | ` -1 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 9 ` | ` -1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 33 ` |  |
| ` initiative ` | ` 2 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/bugbear_warrior ` |
| ` implementation ` | ` dnd2024:creature_bugbear_warrior ` |

### Parameters

#### Features

##### Grab

<!-- splat:yaml path=splat_rule/parameters/features/grab -->

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
  damage_type: bludgeoning
- op: if
  test:
    target_kind: creature
    target_size_max: medium
  then:
  - op: condition
    condition: grappled
    escape_dc: 12
    source: self
reach_feet: 10
```

##### Light hammer

<!-- splat:yaml path=splat_rule/parameters/features/light_hammer -->

```yaml
economy: action
op: attack
attack_modes:
- melee
- ranged
attack_bonus: 4
targets:
  count: 1
hit:
- op: damage
  average: 9
  amount:
    dice:
      count: 3
      sides: 4
    add: 2
  damage_type: bludgeoning
reach_feet: 10
range_feet: 20
long_range_feet: 60
advantage_when:
  target_grappled_by: self
```

##### Abduct

<!-- splat:values path=splat_rule/parameters/features/abduct -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` move_grappled_creature_extra_cost ` | ` 0 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 6 ` |
| ` survival ` | ` 2 ` |

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
| ` row_1 ` | ` Hide Armor ` | ` 1 ` |
| ` row_2 ` | ` Light Hammers ` | ` 3 ` |

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
