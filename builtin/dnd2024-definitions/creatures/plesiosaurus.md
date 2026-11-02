---
splat_schema: 1
splat_id: d7a5ef27-6d5b-5565-a145-b8e10c96067f
splat_kind: creature
splat_key: dnd2024:creatures/plesiosaurus
splat_name: Plesiosaurus
splat_origin:
  key: dnd2024:reference/creature/plesiosaurus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 358, 359
  url: https://www.dndbeyond.com/srd
---

# Plesiosaurus

## Source text

<!-- SRD page 358 -->

Large Beast (Dinosaur), Unaligned\
AC 13  Initiative +2 (12)\
HP 68 (8d10 + 24)\
Speed 20 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3\
Int 2 −4 −4 Wis 12 +1 +1 Cha  5 −3 −3\
Skills Perception +3, Stealth +4\
Senses Passive Perception 13\
Languages None\
CR 2 (XP 450; PB +2)

### Traits

Hold Breath. The plesiosaurus can hold its breath for 1 hour.

<!-- SRD page 359 -->

### Actions

Bite. Melee Attack Roll: +6, reach 10 ft. Hit: 11 (2d6 + 4) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/358|page 358]], [[../../srd-5.2.1/pages/359|page 359]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` tags_text ` | ` (Dinosaur) ` |
| ` movement_text ` | ` 20 ft., Swim 40 ft. ` |
| ` skills_text ` | ` Perception +3, Stealth +4 ` |
| ` senses_text ` | ` Passive Perception 13 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 68 (8d10 + 24) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 5 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 68 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |
| ` swim_speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/plesiosaurus ` |
| ` implementation ` | ` dnd2024:creature_plesiosaurus ` |

### Parameters

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 6
targets:
  count: 1
hit:
- op: damage
  average: 11
  amount:
    dice:
      count: 2
      sides: 6
    add: 4
  damage_type: piercing
reach_feet: 10
```

##### Hold breath

<!-- splat:yaml path=splat_rule/parameters/features/hold_breath -->

```yaml
economy: passive
duration:
  hours: 1
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |
| ` stealth ` | ` 4 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 13 ` |

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
