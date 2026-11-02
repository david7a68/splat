---
splat_schema: 1
splat_id: 3bbfcc51-aff7-5988-8d2d-bed44121df53
splat_kind: creature
splat_key: dnd2024:creatures/giant_crab
splat_name: Giant Crab
splat_origin:
  key: dnd2024:reference/creature/giant_crab
splat_sources:
- title: SRD 5.2.1
  locator: pp. 350
  url: https://www.dndbeyond.com/srd
---

# Giant Crab

## Source text

<!-- SRD page 350 -->

Medium Beast, Unaligned\
AC 15  Initiative +1 (11)\
HP 13 (3d8)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 13 +1 +1 Con 11 +0 +0\
Int 1 −5 −5 Wis 9 −1 −1 Cha 3 −4 −4\
Skills Stealth +3\
Senses Blindsight 30 ft.; Passive Perception 9\
Languages None\
CR 1/8 (XP 25; PB +2)

### Traits

Amphibious. The crab can breathe air and water.

### Actions

Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1) Bludgeoning damage. If the target is a Medium or smaller creature, it has the Grappled condition (escape DC 11) from one of two claws.

Original layout: [[../../srd-5.2.1/pages/350|page 350]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 30 ft., Swim 30 ft. ` |
| ` skills_text ` | ` Stealth +3 ` |
| ` senses_text ` | ` Blindsight 30 ft.; Passive Perception 9 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/8 (XP 25; PB +2) ` |
| ` hit_points_text ` | ` 13 (3d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 13 ` | ` 1 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 11 ` | ` 0 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 9 ` | ` -1 ` |
| ` Charisma ` | ` 3 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 15 ` |  |
| ` max_hp ` | ` 13 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` swim_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_crab ` |
| ` implementation ` | ` dnd2024:creature_giant_crab ` |

### Parameters

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` breathing_media ` | ` ["air", "water"] ` |

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

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
  average: 4
  amount:
    dice:
      count: 1
      sides: 6
    add: 1
  damage_type: bludgeoning
- op: if
  test:
    target_kind: creature
    target_size_max: medium
  then:
  - op: condition
    condition: grappled
    escape_dc: 11
    source: self
    appendages:
      kind: claw
      available: 2
      used: 1
reach_feet: 5
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 3 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 9 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 30 ` |

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
