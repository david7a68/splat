---
splat_schema: 1
splat_id: 54f0ed25-a085-5b50-b211-31b98aca035c
splat_kind: creature
splat_key: dnd2024:creatures/constrictor_snake
splat_name: Constrictor Snake
splat_origin:
  key: dnd2024:reference/creature/constrictor_snake
splat_sources:
- title: SRD 5.2.1
  locator: pp. 346, 347
  url: https://www.dndbeyond.com/srd
---

# Constrictor Snake

## Source text

<!-- SRD page 346 -->

Large Beast, Unaligned\
AC 13  Initiative +2 (12)\
HP 13 (2d10 + 2)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 14 +2 +2 Con 12 +1 +1\
Int 1 −5 −5 Wis 10 +0 +0 Cha 3 −4 −4\
Skills Perception +2, Stealth +4\
Senses Blindsight 10 ft.; Passive Perception 12\
Languages None\
CR 1/4 (XP 50; PB +2)\

<!-- SRD page 347 -->


### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2)\
Piercing damage.\
Constrict. Strength Saving Throw: DC 12, one Medium\
or smaller creature the snake can see within 5 feet.\
Failure: 7 (3d4) Bludgeoning damage, and the target has\
the Grappled condition (escape DC 12).\

Original layout: [[../../srd-5.2.1/pages/346|page 346]], [[../../srd-5.2.1/pages/347|page 347]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 30 ft., Swim 30 ft. ` |
| ` skills_text ` | ` Perception +2, Stealth +4 ` |
| ` senses_text ` | ` Blindsight 10 ft.; Passive Perception 12 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 13 (2d10 + 2) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 15 ` | ` 2 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 3 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 13 ` |  |
| ` initiative ` | ` 2 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/constrictor_snake ` |
| ` implementation ` | ` dnd2024:creature_constrictor_snake ` |

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
| ` stealth ` | ` 4 ` |

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
attack_bonus: 4
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 8
      add: 2
    average: 6
```

##### Constrict

<!-- splat:yaml path=splat_rule/parameters/features/constrict -->

```yaml
economy: action
op: saving_throw
ability: strength
dc: 12
targets:
  kind: creature
  count: 1
  range_ft: 5
  maximum_size: medium
  visible_to_actor: true
failure:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 3
        sides: 4
      add: 0
    average: 7
- op: grapple
  escape_dc: 12
success: []
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
