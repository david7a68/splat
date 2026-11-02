---
splat_schema: 1
splat_id: 642cf218-adec-5322-a5c7-2c40ca2cee34
splat_kind: creature
splat_key: dnd2024:creatures/reef_shark
splat_name: Reef Shark
splat_origin:
  key: dnd2024:reference/creature/reef_shark
splat_sources:
- title: SRD 5.2.1
  locator: pp. 360
  url: https://www.dndbeyond.com/srd
---

# Reef Shark

## Source text

<!-- SRD page 360 -->

Medium Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 22 (4d8 + 4)\
Speed 5 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  14 +2 +2 Dex 15 +2 +2 Con 13 +1 +1\
Int 1 −5 −5 Wis 10 +0 +0 Cha  4 −3 −3\
Skills Perception +2\
Senses Blindsight 30 ft.; Passive Perception 12\
Languages None\
CR 1/2 (XP 100; PB +2)

### Traits

Pack Tactics. The shark has Advantage on an attack roll against a creature if at least one of the shark’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

Water Breathing. The shark can breathe only underwater.

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/360|page 360]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 5 ft., Swim 30 ft. ` |
| ` skills_text ` | ` Perception +2 ` |
| ` senses_text ` | ` Blindsight 30 ft.; Passive Perception 12 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/2 (XP 100; PB +2) ` |
| ` hit_points_text ` | ` 22 (4d8 + 4) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 14 ` | ` 2 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 4 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 22 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 5 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/reef_shark ` |
| ` implementation ` | ` dnd2024:creature_reef_shark ` |

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
| ` 0 ` | ` blindsight ` | ` 30 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

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

##### Water breathing

<!-- splat:values path=splat_rule/parameters/features/water_breathing -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` breathing.allow ` |
| ` media ` | ` ["water"] ` |
| ` exclusive ` | ` true ` |

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
        count: 2
        sides: 4
      add: 2
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
