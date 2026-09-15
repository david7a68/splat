---
splat_schema: 1
splat_id: c69dc034-79a5-5200-8bc8-1c29a6586988
splat_kind: creature
splat_key: dnd2024:creatures/tough
splat_name: Tough
splat_origin:
  key: dnd2024:reference/creature/tough
splat_sources:
- title: SRD 5.2.1
  locator: pp. 332
  url: https://www.dndbeyond.com/srd
---

# Tough

## Source text

<!-- SRD page 332 -->

Medium or Small Humanoid, Neutral\
AC 12 Initiative +1 (11)\
HP 32 (5d8 + 10)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 12 +1 +1 Con 14 +2 +2\
Int 10 +0 +0 Wis 10 +0 +0 Cha 11 +0 +0\
Gear Heavy Crossbow, Leather Armor, Mace\
Senses Passive Perception 10\
Languages Common\
CR 1/2 (XP 100; PB +2)\

### Traits

Pack Tactics. The tough has Advantage on an attack\
roll against a creature if at least one of the tough’s allies\
is within 5 feet of the creature and the ally doesn’t have\
the Incapacitated condition.\

### Actions

Mace. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6\
+ 2) Bludgeoning damage.\
Heavy Crossbow. Ranged Attack Roll: +3, range\
100/400 ft. Hit: 6 (1d10 + 1) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/332|page 332]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` gear_text ` | ` Heavy Crossbow, Leather Armor, Mace ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` Common ` |
| ` cr_text ` | ` 1/2 (XP 100; PB +2) ` |
| ` hit_points_text ` | ` 32 (5d8 + 10) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 15 ` | ` 2 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 11 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 32 ` |  |
| ` initiative ` | ` 1 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/tough ` |
| ` implementation ` | ` dnd2024:creature_tough ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Item:
    path: item
    type: symbol
  Count:
    path: count
    type: integer
-->

| Key | Item | Count |
| --- | --- | --- |
| ` row_1 ` | ` Heavy crossbow ` | ` 1 ` |
| ` row_2 ` | ` Leather armor ` | ` 1 ` |
| ` row_3 ` | ` Mace ` | ` 1 ` |

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

#### Features

##### Pack tactics

<!-- splat:yaml path=splat_rule/parameters/features/pack_tactics -->

```yaml
economy: passive
op: roll.advantage
rolls:
- attack
target_kind: creature
when:
  ally_within_target_ft: 5
  ally_excluded_conditions:
  - incapacitated
```

##### Mace

<!-- splat:yaml path=splat_rule/parameters/features/mace -->

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
  - type: bludgeoning
    amount:
      dice:
        count: 1
        sides: 6
      add: 2
    average: 5
```

##### Heavy crossbow

<!-- splat:yaml path=splat_rule/parameters/features/heavy_crossbow -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 3
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 100
  long: 400
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 10
      add: 1
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
