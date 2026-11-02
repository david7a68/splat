---
splat_schema: 1
splat_id: 1d7d0518-86d7-59cf-8a19-085cea7e7829
splat_kind: creature
splat_key: dnd2024:creatures/rat
splat_name: Rat
splat_origin:
  key: dnd2024:reference/creature/rat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 359
  url: https://www.dndbeyond.com/srd
---

# Rat

## Source text

<!-- SRD page 359 -->

Tiny Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 1 (1d4 − 1)\
Speed 20 ft., Climb 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 2 −4 −4 Dex 11 +0 +0 Con 9 −1 −1\
Int 2 −4 −4 Wis 10 +0 +0 Cha  4 −3 −3\
Skills Perception +2\
Senses Darkvision 30 ft.; Passive Perception 12\
Languages None\
CR 0 (XP 10; PB +2)

### Traits

Agile. The rat doesn’t provoke an Opportunity Attack when it moves out of an enemy’s reach.

### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing damage.

Original layout: [[../../srd-5.2.1/pages/359|page 359]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` tiny ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 20 ft., Climb 20 ft. ` |
| ` skills_text ` | ` Perception +2 ` |
| ` senses_text ` | ` Darkvision 30 ft.; Passive Perception 12 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 0 (XP 10; PB +2) ` |
| ` hit_points_text ` | ` 1 (1d4 - 1) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 2 ` | ` -4 ` |
| ` Dexterity ` | ` 11 ` | ` 0 ` |
| ` Constitution ` | ` 9 ` | ` -1 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 4 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 10 ` |  |
| ` max_hp ` | ` 1 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/rat ` |
| ` implementation ` | ` dnd2024:creature_rat ` |

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
| ` 0 ` | ` darkvision ` | ` 30 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Agile

<!-- splat:values path=splat_rule/parameters/features/agile -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` opportunity_attack.suppress ` |
| ` trigger ` | ` actor.leaves_enemy_reach ` |
| ` movement_modes ` | ` ["all"] ` |

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
