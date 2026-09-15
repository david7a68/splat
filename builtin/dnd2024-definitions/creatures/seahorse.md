---
splat_schema: 1
splat_id: 4716d703-8690-5db4-80ac-cd5400ee9f4b
splat_kind: creature
splat_key: dnd2024:creatures/seahorse
splat_name: Seahorse
splat_origin:
  key: dnd2024:reference/creature/seahorse
splat_sources:
- title: SRD 5.2.1
  locator: pp. 361
  url: https://www.dndbeyond.com/srd
---

# Seahorse

## Source text

<!-- SRD page 361 -->

Tiny Beast, Unaligned\
AC 12  Initiative +1 (11)\
HP 1 (1d4 − 1)\
Speed 5 ft., Swim 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 1 −5 −5 Dex 12 +1 +1 Con 8 −1 −1\
Int 1 −5 −5 Wis 10 +0 +0 Cha 2 −4 −4\
Skills Perception +2, Stealth +5\
Senses Passive Perception 12\
Languages None\
CR 0 (XP 0; PB +2)\

### Traits

Water Breathing. The seahorse can breathe only\
underwater.\

### Actions

Bubble Dash. While underwater, the seahorse moves\
up to its Swim Speed without provoking Opportu-\
nity Attacks.\

Original layout: [[../../srd-5.2.1/pages/361|page 361]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` tiny ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 5 ft., Swim 20 ft. ` |
| ` skills_text ` | ` Perception +2, Stealth +5 ` |
| ` senses_text ` | ` Passive Perception 12 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 0 (XP 0; PB +2) ` |
| ` hit_points_text ` | ` 1 (1d4 - 1) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 1 ` | ` -5 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 8 ` | ` -1 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 2 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 1 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 5 ` | ` ft ` |
| ` swim_speed ` | ` 20 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/seahorse ` |
| ` implementation ` | ` dnd2024:creature_seahorse ` |

### Parameters

#### Features

##### Water breathing

<!-- splat:values path=splat_rule/parameters/features/water_breathing -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` breathing_media ` | ` ["water"] ` |

##### Bubble dash

<!-- splat:yaml path=splat_rule/parameters/features/bubble_dash -->

```yaml
economy: action
requires:
  underwater: true
op: move
distance_max:
  multiply:
  - stat: swim_speed
  - 1
provokes_opportunity_attacks: false
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 2 ` |
| ` stealth ` | ` 5 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

#### Communication

##### Speaks

<!-- splat:yaml path=splat_rule/parameters/communication/speaks -->

```yaml
[]
```

##### Understands

<!-- splat:yaml path=splat_rule/parameters/communication/understands -->

```yaml
[]
```

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
