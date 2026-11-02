---
splat_schema: 1
splat_id: cb038f77-0dc2-56f4-a91d-e123d1acb532
splat_kind: creature
splat_key: dnd2024:creatures/vulture
splat_name: Vulture
splat_origin:
  key: dnd2024:reference/creature/vulture
splat_sources:
- title: SRD 5.2.1
  locator: pp. 363, 364
  url: https://www.dndbeyond.com/srd
---

# Vulture

## Source text

<!-- SRD page 363 -->

Medium Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 5 (1d8 + 1)\
Speed 10 ft., Fly 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 7 −2 −2 Dex 10 +0 +0 Con 13 +1 +1\
Int 2 −4 −4 Wis 12 +1 +1 Cha  4 −3 −3\
Skills Perception +3\
Senses Passive Perception 13\
Languages None\
CR 0 (XP 10; PB +2)

<!-- SRD page 364 -->

### Traits

Pack Tactics. The vulture has Advantage on an attack roll against a creature if at least one of the vulture’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

### Actions

Beak. Melee Attack Roll: +2, reach 5 ft. Hit: 2 (1d4) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/363|page 363]], [[../../srd-5.2.1/pages/364|page 364]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 10 ft., Fly 50 ft. ` |
| ` skills_text ` | ` Perception +3 ` |
| ` senses_text ` | ` Passive Perception 13 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 0 (XP 10; PB +2) ` |
| ` hit_points_text ` | ` 5 (1d8 + 1) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 7 ` | ` -2 ` |
| ` Dexterity ` | ` 10 ` | ` 0 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 4 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 10 ` |  |
| ` max_hp ` | ` 5 ` |  |
| ` initiative ` | ` 0 ` |  |
| ` speed ` | ` 10 ` | ` ft ` |
| ` fly_speed ` | ` 50 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/vulture ` |
| ` implementation ` | ` dnd2024:creature_vulture ` |

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

##### Beak

<!-- splat:yaml path=splat_rule/parameters/features/beak -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 2
targets:
  count: 1
hit:
- op: damage
  average: 2
  amount:
    dice:
      count: 1
      sides: 4
    add: 0
  damage_type: piercing
reach_feet: 5
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |

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
