---
splat_schema: 1
splat_id: 7144c6ca-61be-5129-83eb-ffc371f30da6
splat_kind: creature
splat_key: dnd2024:creatures/hobgoblin_warrior
splat_name: Hobgoblin Warrior
splat_origin:
  key: dnd2024:reference/creature/hobgoblin_warrior
splat_sources:
- title: SRD 5.2.1
  locator: pp. 298, 299
  url: https://www.dndbeyond.com/srd
---

# Hobgoblin Warrior

## Source text

<!-- SRD page 298 -->

Medium Fey (Goblinoid), Lawful Evil\
AC 18 Initiative +3 (13)\
HP 11 (2d8 + 2)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 12 +1 +1 Con 12 +1 +1\
Int 10 +0 +0 Wis 10 +0 +0 Cha 9 −1 −1\
Gear Half Plate Armor, Longbow, Longsword, Shield\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Common, Goblin\
CR 1/2 (XP 100; PB +2)\

<!-- SRD page 299 -->


### Traits

Pack Tactics. The hobgoblin has Advantage on an at-\
tack roll against a creature if at least one of the hobgob-\
lin’s allies is within 5 feet of the creature and the ally\
doesn’t have the Incapacitated condition.\

### Actions

Longsword. Melee Attack Roll: +3, reach 5 ft. Hit: 12\
(2d10 + 1) Slashing damage.\
Longbow. Ranged Attack Roll: +3, range 150/600 ft.\
Hit: 5 (1d8 + 1) Piercing damage plus 7 (3d4) Poi-\
son damage.\

Original layout: [[../../srd-5.2.1/pages/298|page 298]], [[../../srd-5.2.1/pages/299|page 299]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` fey ` |
| ` alignment ` | ` Lawful Evil ` |
| ` tags_text ` | ` (Goblinoid) ` |
| ` movement_text ` | ` 30 ft. ` |
| ` gear_text ` | ` Half Plate Armor, Longbow, Longsword, Shield ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Common, Goblin ` |
| ` cr_text ` | ` 1/2 (XP 100; PB +2) ` |
| ` hit_points_text ` | ` 11 (2d8 + 2) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 13 ` | ` 1 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 9 ` | ` -1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 11 ` |  |
| ` initiative ` | ` 3 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/hobgoblin_warrior ` |
| ` implementation ` | ` dnd2024:creature_hobgoblin_warrior ` |

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
- row_4
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
| ` row_1 ` | ` Half plate armor ` | ` 1 ` |
| ` row_2 ` | ` Longbow ` | ` 1 ` |
| ` row_3 ` | ` Longsword ` | ` 1 ` |
| ` row_4 ` | ` Shield ` | ` 1 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common", "goblin"] ` |

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

##### Longsword

<!-- splat:yaml path=splat_rule/parameters/features/longsword -->

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
        count: 2
        sides: 10
      add: 1
    average: 12
```

##### Longbow

<!-- splat:yaml path=splat_rule/parameters/features/longbow -->

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
  normal: 150
  long: 600
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
  - type: poison
    amount:
      dice:
        count: 3
        sides: 4
      add: 0
    average: 7
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
