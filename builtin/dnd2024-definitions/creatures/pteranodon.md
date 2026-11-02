---
splat_schema: 1
splat_id: 7220231a-57f4-5b70-8d4f-2e4bd68482ab
splat_kind: creature
splat_key: dnd2024:creatures/pteranodon
splat_name: Pteranodon
splat_origin:
  key: dnd2024:reference/creature/pteranodon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 359
  url: https://www.dndbeyond.com/srd
---

# Pteranodon

## Source text

<!-- SRD page 359 -->

Medium Beast (Dinosaur), Unaligned\
AC 13  Initiative +2 (12)\
HP 13 (3d8)\
Speed 10 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 15 +2 +2 Con 10 +0 +0\
Int 2 −4 −4 Wis 9 −1 −1 Cha  5 −3 −3\
Skills Perception +1\
Senses Passive Perception 11\
Languages None\
CR 1/4 (XP 50; PB +2)

### Traits

Flyby. The pteranodon doesn’t provoke an Opportunity Attack when it flies out of an enemy’s reach.

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/359|page 359]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` tags_text ` | ` (Dinosaur) ` |
| ` movement_text ` | ` 10 ft., Fly 60 ft. ` |
| ` skills_text ` | ` Perception +1 ` |
| ` senses_text ` | ` Passive Perception 11 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 13 (3d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 12 ` | ` 1 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 9 ` | ` -1 ` |
| ` Charisma ` | ` 5 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 13 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 10 ` | ` ft ` |
| ` fly_speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/pteranodon ` |
| ` implementation ` | ` dnd2024:creature_pteranodon ` |

### Parameters

#### Features

##### Flyby

<!-- splat:yaml path=splat_rule/parameters/features/flyby -->

```yaml
economy: passive
opportunity_attack_immunity:
  trigger: fly_out_of_enemy_reach
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

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
  average: 6
  amount:
    dice:
      count: 1
      sides: 8
    add: 2
  damage_type: piercing
reach_feet: 5
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 1 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

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
