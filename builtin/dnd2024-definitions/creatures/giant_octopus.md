---
splat_schema: 1
splat_id: 376d504e-1cfd-5dbc-946b-39e90cb4af9b
splat_kind: creature
splat_key: dnd2024:creatures/giant_octopus
splat_name: Giant Octopus
splat_origin:
  key: dnd2024:reference/creature/giant_octopus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 352
  url: https://www.dndbeyond.com/srd
---

# Giant Octopus

## Source text

<!-- SRD page 352 -->

Large Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 45 (7d10 + 7)\
Speed 10 ft., Swim 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 13 +1 +1 Con 13 +1 +1\
Int 5 −3 −3 Wis 10 +0 +0 Cha 4 −3 −3\
Skills Perception +4, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages None\
CR 1 (XP 200; PB +2)\

### Traits

Water Breathing. The octopus can breathe only under-\
water. It can hold its breath for 1 hour outside water.\

### Actions

Tentacles. Melee Attack Roll: +5, reach 10 ft. Hit: 10\
(2d6 + 3) Bludgeoning damage. If the target is a Me-\
dium or smaller creature, it has the Grappled condition\
(escape DC 13) from all eight tentacles. While Grap-\
pled, the target has the Restrained condition.\

### Reactions

Ink Cloud (1/Day). Trigger: The octopus takes damage\
while underwater. Response: The octopus releases ink\
that fills a 10-foot Cube centered on itself, and the oc-\
topus moves up to its Swim Speed. The Cube is Heavily\
Obscured for 1 minute or until a strong current or simi-\
lar effect disperses the ink.\

Original layout: [[../../srd-5.2.1/pages/352|page 352]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 10 ft., Swim 60 ft. ` |
| ` skills_text ` | ` Perception +4, Stealth +5 ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 14 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1 (XP 200; PB +2) ` |
| ` hit_points_text ` | ` 45 (7d10 + 7) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 17 ` | ` 3 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 5 ` | ` -3 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 4 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 45 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 10 ` | ` ft ` |
| ` swim_speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_octopus ` |
| ` implementation ` | ` dnd2024:creature_giant_octopus ` |

### Parameters

#### Features

##### Water breathing

<!-- splat:values path=splat_rule/parameters/features/water_breathing -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` breathing ` | ` ["water"] ` |
| ` hold_breath_outside_water_seconds ` | ` 3600 ` |

##### Tentacles

<!-- splat:yaml path=splat_rule/parameters/features/tentacles -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  reach_ft: 10
hit:
  damage:
  - average: 10
    roll:
      count: 2
      sides: 6
      modifier: 3
    type: bludgeoning
  grapple:
    condition: grappled
    source: self
    escape_dc: 13
    maximum_size: medium
    appendage_pool:
      count: 8
      type: tentacles
      cost: 8
    while_grappled:
      conditions:
      - restrained
```

##### Ink cloud

<!-- splat:yaml path=splat_rule/parameters/features/ink_cloud -->

```yaml
economy: reaction
resource:
  kind: daily
  uses: 1
  recovery: long_rest
operation: sequence
trigger: self.damage_taken
requires:
  self_underwater: true
effects:
- operation: create_area
  area:
    shape: cube
    size_ft: 10
    center: self
  obscurement: heavy
  duration_seconds: 60
  ends_on:
  - strong_current
  - similar_dispersal_effect
- operation: move
  maximum_ft:
    multiply:
    - stat: swim_speed
    - 1
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |
| ` stealth ` | ` 5 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 14 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands: []
```

#### Resolution conventions

<!-- splat:yaml path=splat_rule/parameters/resolution_conventions -->

```yaml
distance_unit: foot
duration_unit: second
damage_roll_mode:
  choice:
  - listed_average
  - roll_dice
  chooser: gm
supplied_totals: before_additional_effect_modifiers
multiattack_nested_economy_cost: 0
nested_feature_resource_cost: normal
spell_level_unless_specified: lowest_possible
spell_upcast_without_permission: false
spell_casting_time_at_least_one_minute: normal_casting_procedure
geometry:
  input: adjudication
  assumptions:
  - range_and_visibility_verified
  - area_membership_verified
  - path_and_destination_legal
unlisted_success_branch: no_effect
unlisted_duration: until_removed_by_applicable_rule
```
