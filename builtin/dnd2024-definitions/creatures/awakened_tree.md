---
splat_schema: 1
splat_id: 8a76ac78-98c3-5ce4-b9d8-7bf1312ce86f
splat_kind: creature
splat_key: dnd2024:creatures/awakened_tree
splat_name: Awakened Tree
splat_origin:
  key: dnd2024:reference/creature/awakened_tree
splat_sources:
- title: SRD 5.2.1
  locator: pp. 260
  url: https://www.dndbeyond.com/srd
---

# Awakened Tree

## Source text

<!-- SRD page 260 -->

Huge Plant, Neutral\
AC 13 Initiative −2 (8)\
HP 59 (7d12 + 14)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 6 −2 −2 Con 15 +2 +2\
Int 10 +0 +0 Wis 10 +0 +0 Cha 7 −2 −2\
Vulnerabilities Fire\
Resistances Bludgeoning, Piercing\
Senses Passive Perception 10\
Languages Common plus one other language\
CR 2 (XP 450; PB +2)

### Actions

Slam. Melee Attack Roll: +6, reach 10 ft. Hit: 14 (3d6 + 4) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/260|page 260]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` plant ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 20 ft. ` |
| ` resistances_text ` | ` Bludgeoning, Piercing ` |
| ` vulnerabilities_text ` | ` Fire ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` Common plus one other language ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 59 (7d12 + 14) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 19 ` | ` 4 ` |
| ` Dexterity ` | ` 6 ` | ` -2 ` |
| ` Constitution ` | ` 15 ` | ` 2 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 7 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 59 ` |  |
| ` initiative ` | ` -2 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/awakened_tree ` |
| ` implementation ` | ` dnd2024:creature_awakened_tree ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_vulnerabilities ` | ` ["fire"] ` |
| ` damage_resistances ` | ` ["bludgeoning", "piercing"] ` |
| ` passive_perception ` | ` 10 ` |

#### Senses

<!-- splat:yaml path=splat_rule/parameters/senses -->

```yaml
[]
```

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common"] ` |
| ` additional_choice_count ` | ` 1 ` |

#### Features

##### Slam

<!-- splat:yaml path=splat_rule/parameters/features/slam -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 6
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 3
        sides: 6
      add: 4
    average: 14
```

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |
