---
splat_schema: 1
splat_id: 2f497705-dfff-5dc6-a052-42a1327dcf87
splat_kind: creature
splat_key: dnd2024:creatures/lizard
splat_name: Lizard
splat_origin:
  key: dnd2024:reference/creature/lizard
splat_sources:
- title: SRD 5.2.1
  locator: pp. 357
  url: https://www.dndbeyond.com/srd
---

# Lizard

## Source text

<!-- SRD page 357 -->

Tiny Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 2 (1d4)\
Speed 20 ft., Climb 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 2 −4 −4 Dex 11 +0 +0 Con 10 +0 +0\
Int 1 −5 −5 Wis 8 −1 −1 Cha 3 −4 −4\
Senses Darkvision 30 ft.; Passive Perception 9\
Languages None\
CR 0 (XP 10; PB +2)

### Traits

Spider Climb. The lizard can climb difficult surfaces, including along ceilings, without needing to make an ability check.

### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing damage.

Original layout: [[../../srd-5.2.1/pages/357|page 357]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` tiny ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 20 ft., Climb 20 ft. ` |
| ` senses_text ` | ` Darkvision 30 ft.; Passive Perception 9 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 0 (XP 10; PB +2) ` |
| ` hit_points_text ` | ` 2 (1d4) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 2 ` | ` -4 ` |
| ` Dexterity ` | ` 11 ` | ` 0 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 8 ` | ` -1 ` |
| ` Charisma ` | ` 3 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 10 ` |  |
| ` max_hp ` | ` 2 ` |  |
| ` initiative ` | ` 0 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |
| ` climb_speed ` | ` 20 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/lizard ` |
| ` implementation ` | ` dnd2024:creature_lizard ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 9 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 30 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Spider climb

<!-- splat:values path=splat_rule/parameters/features/spider_climb -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` movement.climb ` |
| ` difficult_surfaces ` | ` true ` |
| ` ceilings ` | ` true ` |
| ` ability_check_required ` | ` false ` |

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 2
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: piercing
    amount:
      constant: 1
    average: 1
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
