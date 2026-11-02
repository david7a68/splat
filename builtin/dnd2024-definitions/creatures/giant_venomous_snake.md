---
splat_schema: 1
splat_id: 679b39f5-f8cc-5c41-88e4-10a1b44c8ef6
splat_kind: creature
splat_key: dnd2024:creatures/giant_venomous_snake
splat_name: Giant Venomous Snake
splat_origin:
  key: dnd2024:reference/creature/giant_venomous_snake
splat_sources:
- title: SRD 5.2.1
  locator: pp. 354
  url: https://www.dndbeyond.com/srd
---

# Giant Venomous Snake

## Source text

<!-- SRD page 354 -->

Medium Beast, Unaligned\
AC 14  Initiative +4 (14)\
HP 11 (2d8 + 2)\
Speed 40 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 18 +4 +4 Con 13 +1 +1\
Int 2 −4 −4 Wis 10 +0 +0 Cha 3 −4 −4\
Skills Perception +2\
Senses Blindsight 10 ft.; Passive Perception 12\
Languages None\
CR 1/4 (XP 50; PB +2)

### Actions

Bite. Melee Attack Roll: +6, reach 10 ft. Hit: 6 (1d4 + 4) Piercing damage plus 4 (1d8) Poison damage.

Original layout: [[../../srd-5.2.1/pages/354|page 354]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 40 ft., Swim 40 ft. ` |
| ` skills_text ` | ` Perception +2 ` |
| ` senses_text ` | ` Blindsight 10 ft.; Passive Perception 12 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 11 (2d8 + 2) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 0 ` |
| ` Dexterity ` | ` 18 ` | ` 4 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 3 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 11 ` |  |
| ` initiative ` | ` 4 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/giant_venomous_snake ` |
| ` implementation ` | ` dnd2024:creature_giant_venomous_snake ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 2 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

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
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 4
      add: 4
    average: 6
  - type: poison
    amount:
      dice:
        count: 1
        sides: 8
      add: 0
    average: 4
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
