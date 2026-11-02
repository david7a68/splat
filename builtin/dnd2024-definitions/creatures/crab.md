---
splat_schema: 1
splat_id: eaba7510-634f-5ec3-ab74-10343020a7ba
splat_kind: creature
splat_key: dnd2024:creatures/crab
splat_name: Crab
splat_origin:
  key: dnd2024:reference/creature/crab
splat_sources:
- title: SRD 5.2.1
  locator: pp. 347
  url: https://www.dndbeyond.com/srd
---

# Crab

## Source text

<!-- SRD page 347 -->

Tiny Beast, Unaligned\
AC 11  Initiative +0 (10)\
HP 3 (1d4 + 1)\
Speed 20 ft., Swim 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 11 +0 +0 Con 12 +1 +1\
Int 1 −5 −5 Wis 8 −1 −1 Cha 2 −4 −4\
Skills Stealth +2\
Senses Blindsight 30 ft.; Passive Perception 9\
Languages None\
CR 0 (XP 10; PB +2)

### Traits

Amphibious. The crab can breathe air and water.

### Actions

Claw. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/347|page 347]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` tiny ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 20 ft., Swim 20 ft. ` |
| ` skills_text ` | ` Stealth +2 ` |
| ` senses_text ` | ` Blindsight 30 ft.; Passive Perception 9 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 0 (XP 10; PB +2) ` |
| ` hit_points_text ` | ` 3 (1d4 + 1) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 6 ` | ` -2 ` |
| ` Dexterity ` | ` 11 ` | ` 0 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 8 ` | ` -1 ` |
| ` Charisma ` | ` 2 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 3 ` |  |
| ` initiative ` | ` 0 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |
| ` swim_speed ` | ` 20 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/crab ` |
| ` implementation ` | ` dnd2024:creature_crab ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 9 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 2 ` |

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

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` breathing.allow ` |
| ` media ` | ` ["air", "water"] ` |

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

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
  - type: bludgeoning
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
