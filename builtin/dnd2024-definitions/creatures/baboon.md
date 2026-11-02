---
splat_schema: 1
splat_id: e05c3bfb-3cc6-5042-a194-c0ff7ddc6f96
splat_kind: creature
splat_key: dnd2024:creatures/baboon
splat_name: Baboon
splat_origin:
  key: dnd2024:reference/creature/baboon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 345
  url: https://www.dndbeyond.com/srd
---

# Baboon

## Source text

<!-- SRD page 345 -->

Small Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 3 (1d6)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 8 −1 −1 Dex 14 +2 +2 Con 11 +0 +0\
Int 4 −3 −3 Wis 12 +1 +1 Cha 6 −2 −2\
Senses Passive Perception 11\
Languages None\
CR 0 (XP 10; PB +2)

### Traits

Pack Tactics. The baboon has Advantage on an attack roll against a creature if at least one of the baboon’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

### Actions

Bite. Melee Attack Roll: +1, reach 5 ft. Hit: 1 (1d4 − 1) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/345|page 345]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` small ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 30 ft., Climb 30 ft. ` |
| ` senses_text ` | ` Passive Perception 11 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 0 (XP 10; PB +2) ` |
| ` hit_points_text ` | ` 3 (1d6) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 8 ` | ` -1 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 11 ` | ` 0 ` |
| ` Intelligence ` | ` 4 ` | ` -3 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 3 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` climb_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/baboon ` |
| ` implementation ` | ` dnd2024:creature_baboon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

#### Senses

<!-- splat:yaml path=splat_rule/parameters/senses -->

```yaml
[]
```

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

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 1
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
        sides: 4
      add: -1
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
