---
splat_schema: 1
splat_id: 9bb302df-2d68-5311-9182-d160654ad979
splat_kind: creature
splat_key: dnd2024:creatures/warhorse
splat_name: Warhorse
splat_origin:
  key: dnd2024:reference/creature/warhorse
splat_sources:
- title: SRD 5.2.1
  locator: pp. 364
  url: https://www.dndbeyond.com/srd
---

# Warhorse

## Source text

<!-- SRD page 364 -->

Large Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 19 (3d10 + 3)\
Speed 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 12 +1 +1 Con13 +1 +1\
Int 2 −4 −4 Wis 12 +1 +3 Cha 7 −2 −2\
Senses Passive Perception 11\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 9 (2d4 +\
4) Bludgeoning damage. If the target is a Large or smaller\
creature and the horse moved 20+ feet straight toward\
it immediately before the hit, the target takes an extra 5\
(2d4) Bludgeoning damage and has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/364|page 364]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 60 ft. ` |
| ` senses_text ` | ` Passive Perception 11 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/2 (XP 100; PB +2) ` |
| ` hit_points_text ` | ` 19 (3d10 + 3) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 12 ` | ` 3 ` |
| ` Charisma ` | ` 7 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 19 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/warhorse ` |
| ` implementation ` | ` dnd2024:creature_warhorse ` |

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

##### Hooves

<!-- splat:yaml path=splat_rule/parameters/features/hooves -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 6
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
        count: 2
        sides: 4
      add: 4
    average: 9
- op: conditional
  when:
    target_maximum_size: large
    actor_moved_straight_toward_target_ft_at_least: 20
    immediately_before_hit: true
    target_kind: creature
  effects:
  - op: damage
    components:
    - type: bludgeoning
      amount:
        dice:
          count: 2
          sides: 4
        add: 0
      average: 5
  - op: condition.apply
    condition: prone
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
