---
splat_schema: 1
splat_id: eb0e0267-2c45-5fe5-b03e-29f5af103f9c
splat_kind: creature
splat_key: dnd2024:creatures/violet_fungus
splat_name: Violet Fungus
splat_origin:
  key: dnd2024:reference/creature/violet_fungus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 286
  url: https://www.dndbeyond.com/srd
---

# Violet Fungus

## Source text

<!-- SRD page 286 -->

Medium Plant, Unaligned\
AC 5 Initiative −5 (5)\
HP 18 (4d8)\
Speed 5 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 3 −4 −4 Dex 1 −5 −5 Con 10 +0 +0\
Int 1 −5 −5 Wis 3 −4 −4 Cha 1 −5 −5\
Immunities Blinded, Charmed, Deafened, Frightened\
Senses Blindsight 30 ft.; Passive Perception 6\
Languages None\
CR 1/4 (XP 50; PB +2)

### Actions

Multiattack. The fungus makes two Rotting Touch attacks.

Rotting Touch. Melee Attack Roll: +2, reach 10 ft. Hit: 4 (1d8) Necrotic damage.

Original layout: [[../../srd-5.2.1/pages/286|page 286]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` plant ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 5 ft. ` |
| ` immunities_text ` | ` Blinded, Charmed, Deafened, Frightened ` |
| ` senses_text ` | ` Blindsight 30 ft.; Passive Perception 6 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 18 (4d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 3 ` | ` -4 ` |
| ` Dexterity ` | ` 1 ` | ` -5 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 3 ` | ` -4 ` |
| ` Charisma ` | ` 1 ` | ` -5 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 5 ` |  |
| ` max_hp ` | ` 18 ` |  |
| ` initiative ` | ` -5 ` |  |
| ` speed ` | ` 5 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/violet_fungus ` |
| ` implementation ` | ` dnd2024:creature_violet_fungus ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition_immunities ` | ` ["blinded", "charmed", "deafened", "frightened"] ` |
| ` passive_perception ` | ` 6 ` |

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

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

##### Rotting touch

<!-- splat:yaml path=splat_rule/parameters/features/rotting_touch -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 2
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: necrotic
    amount:
      dice:
        count: 1
        sides: 8
      add: 0
    average: 4
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: rotting_touch
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
