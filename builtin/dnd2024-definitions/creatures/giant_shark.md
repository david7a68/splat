---
splat_schema: 1
splat_id: 4bab219d-c9cc-5920-8bcc-1ba356ce8b64
splat_kind: creature
splat_key: dnd2024:creatures/giant_shark
splat_name: Giant Shark
splat_origin:
  key: dnd2024:reference/creature/giant_shark
splat_sources:
- title: SRD 5.2.1
  locator: pp. 353
  url: https://www.dndbeyond.com/srd
---

# Giant Shark

## Source text

<!-- SRD page 353 -->

Huge Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 92 (8d12 + 40)\
Speed 5 ft., Swim 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 11 +0 +0 Con 21 +5 +5\
Int 1 −5 −5 Wis 10 +0 +0 Cha 5 −3 −3\
Skills Perception +3\
Senses Blindsight 60 ft.; Passive Perception 13\
Languages None\
CR 5 (XP 1,800; PB +3)\

### Traits

Water Breathing. The shark can breathe only underwater.\

### Actions

Multiattack. The shark makes two Bite attacks.\
Bite. Melee Attack Roll: +9 (with Advantage if the target\
doesn’t have all its Hit Points), reach 5 ft. Hit: 22 (3d10\
+ 6) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/353|page 353]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 5 ft., Swim 60 ft. ` |
| ` skills_text ` | ` Perception +3 ` |
| ` senses_text ` | ` Blindsight 60 ft.; Passive Perception 13 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 92 (8d12 + 40) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 23 ` | ` 6 ` |
| ` Dexterity ` | ` 11 ` | ` 0 ` |
| ` Constitution ` | ` 21 ` | ` 5 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 5 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 92 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 5 ` | ` ft ` |
| ` swim_speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_shark ` |
| ` implementation ` | ` dnd2024:creature_giant_shark ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 13 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 60 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

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
attack_bonus: 9
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
        count: 3
        sides: 10
      add: 6
    average: 22
advantage_when:
  target_hp_below_maximum: true
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: bite
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
