---
splat_schema: 1
splat_id: 6675cf17-efcf-52a9-a89f-e9b7a8bac680
splat_kind: creature
splat_key: dnd2024:creatures/ankylosaurus
splat_name: Ankylosaurus
splat_origin:
  key: dnd2024:reference/creature/ankylosaurus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 344
  url: https://www.dndbeyond.com/srd
---

# Ankylosaurus

## Source text

<!-- SRD page 344 -->

Huge Beast (Dinosaur), Unaligned\
AC 15   Initiative +0 (10)\
HP 68 (8d12 + 16)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +6 Dex 11 +0 +0 Con 15 +2 +2\
Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
Senses Passive Perception 11\
Languages None\
CR 3 (XP 700; PB +2)

### Actions

Multiattack. The ankylosaurus makes two Tail attacks.

Tail. Melee Attack Roll: +6, reach 10 ft. Hit: 9 (1d10 + 4) Bludgeoning damage. If the target is a Huge or smaller creature, it has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/344|page 344]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` tags_text ` | ` (Dinosaur) ` |
| ` movement_text ` | ` 30 ft. ` |
| ` senses_text ` | ` Passive Perception 11 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 3 (XP 700; PB +2) ` |
| ` hit_points_text ` | ` 68 (8d12 + 16) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 19 ` | ` 6 ` |
| ` Dexterity ` | ` 11 ` | ` 0 ` |
| ` Constitution ` | ` 15 ` | ` 2 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 5 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 15 ` |  |
| ` max_hp ` | ` 68 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/ankylosaurus ` |
| ` implementation ` | ` dnd2024:creature_ankylosaurus ` |

### Parameters

#### Features

##### Tail

<!-- splat:yaml path=splat_rule/parameters/features/tail -->

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
  average: 9
  amount:
    dice:
      count: 1
      sides: 10
    add: 4
  damage_type: bludgeoning
- op: if
  test:
    target_kind: creature
    target_size_max: huge
  then:
  - op: condition
    condition: prone
reach_feet: 10
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: tail
  count: 2
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
