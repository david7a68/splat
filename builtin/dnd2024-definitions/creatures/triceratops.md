---
splat_schema: 1
splat_id: 95a127ec-d2fb-5908-8987-dd8846ede461
splat_kind: creature
splat_key: dnd2024:creatures/triceratops
splat_name: Triceratops
splat_origin:
  key: dnd2024:reference/creature/triceratops
splat_sources:
- title: SRD 5.2.1
  locator: pp. 363
  url: https://www.dndbeyond.com/srd
---

# Triceratops

## Source text

<!-- SRD page 363 -->

Huge Beast (Dinosaur), Unaligned\
AC 14  Initiative −1 (9)\
HP 114 (12d12 + 36)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 22 +6 +6 Dex 9 −1 −1 Con 17 +3 +3\
Int 2 −4 −4 Wis 11 +0 +0 Cha  5 −3 −3\
Senses Passive Perception 10\
Languages None\
CR 5 (XP 1,800; PB +3)

### Actions

Multiattack. The triceratops makes two Gore attacks.

Gore. Melee Attack Roll: +9, reach 5 ft. Hit: 19 (2d12 + 6) Piercing damage. If the target is Huge or smaller and the triceratops moved 20+ feet straight toward it immediately before the hit, the target takes an extra 9 (2d8) Piercing damage and has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/363|page 363]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` tags_text ` | ` (Dinosaur) ` |
| ` movement_text ` | ` 50 ft. ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 114 (12d12 + 36) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 22 ` | ` 6 ` |
| ` Dexterity ` | ` 9 ` | ` -1 ` |
| ` Constitution ` | ` 17 ` | ` 3 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 5 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 114 ` |  |
| ` initiative ` | ` -1 ` |  |
| ` speed ` | ` 50 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/triceratops ` |
| ` implementation ` | ` dnd2024:creature_triceratops ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

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

##### Gore

<!-- splat:yaml path=splat_rule/parameters/features/gore -->

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
        count: 2
        sides: 12
      add: 6
    average: 19
- op: conditional
  when:
    target_maximum_size: huge
    actor_moved_straight_toward_target_ft_at_least: 20
    immediately_before_hit: true
  effects:
  - op: damage
    components:
    - type: piercing
      amount:
        dice:
          count: 2
          sides: 8
        add: 0
      average: 9
  - op: condition.apply
    condition: prone
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: gore
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
