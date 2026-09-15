---
splat_schema: 1
splat_id: 703a8a36-2e37-5b7f-bc16-0ae31fb12cc7
splat_kind: creature
splat_key: dnd2024:creatures/bandit
splat_name: Bandit
splat_origin:
  key: dnd2024:reference/creature/bandit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 261
  url: https://www.dndbeyond.com/srd
---

# Bandit

## Source text

<!-- SRD page 261 -->

Medium or Small Humanoid, Neutral\
AC 12 Initiative +1 (11)\
HP 11 (2d8 + 2)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 12 +1 +1 Con 12 +1 +1\
Int 10 +0 +0 Wis 10 +0 +0 Cha 10 +0 +0\
Gear Leather Armor, Light Crossbow, Scimitar\
Senses Passive Perception 10\
Languages Common, Thieves’ Cant\
CR 1/8 (XP 25; PB +2)\

### Actions

Scimitar. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6\
+ 1) Slashing damage.\
Light Crossbow. Ranged Attack Roll: +3, range 80/320\
ft. Hit: 5 (1d8 + 1) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/261|page 261]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` gear_text ` | ` Leather Armor, Light Crossbow, Scimitar ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` Common, Thieves’ Cant ` |
| ` cr_text ` | ` 1/8 (XP 25; PB +2) ` |
| ` hit_points_text ` | ` 11 (2d8 + 2) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 11 ` | ` 0 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 11 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/bandit ` |
| ` implementation ` | ` dnd2024:creature_bandit ` |

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
| ` row_1 ` | ` Leather armor ` | ` 1 ` |
| ` row_2 ` | ` Light crossbow ` | ` 1 ` |
| ` row_3 ` | ` Scimitar ` | ` 1 ` |

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
| ` known ` | ` ["common", "thieves_cant"] ` |

#### Features

##### Scimitar

<!-- splat:yaml path=splat_rule/parameters/features/scimitar -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 3
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 1
        sides: 6
      add: 1
    average: 4
```

##### Light crossbow

<!-- splat:yaml path=splat_rule/parameters/features/light_crossbow -->

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
  normal: 80
  long: 320
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 8
      add: 1
    average: 5
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
