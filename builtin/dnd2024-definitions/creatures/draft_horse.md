---
splat_schema: 1
splat_id: d56719fd-ca46-59a2-afb6-162639274fc9
splat_kind: creature
splat_key: dnd2024:creatures/draft_horse
splat_name: Draft Horse
splat_origin:
  key: dnd2024:reference/creature/draft_horse
splat_sources:
- title: SRD 5.2.1
  locator: pp. 347, 348
  url: https://www.dndbeyond.com/srd
---

# Draft Horse

## Source text

<!-- SRD page 347 -->

Large Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 15 (2d10 + 4)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 10 +0 +0 Con 15 +2 +2\
Int 2 −4 −4 Wis 11 +0 +0 Cha 7 −2 −2\
Senses Passive Perception 10\
Languages None\
CR 1/4 (XP 50; PB +2)

<!-- SRD page 348 -->

### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 6 (1d4 + 4) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/347|page 347]], [[../../srd-5.2.1/pages/348|page 348]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 40 ft. ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 15 (2d10 + 4) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 10 ` | ` 0 ` |
| ` Constitution ` | ` 15 ` | ` 2 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 7 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 10 ` |  |
| ` max_hp ` | ` 15 ` |  |
| ` initiative ` | ` 0 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/draft_horse ` |
| ` implementation ` | ` dnd2024:creature_draft_horse ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

#### Senses

<!-- splat:yaml path=splat_rule/parameters/senses -->

```yaml
[]
```

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Hooves

<!-- splat:yaml path=splat_rule/parameters/features/hooves -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 6
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 1
        sides: 4
      add: 4
    average: 6
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
