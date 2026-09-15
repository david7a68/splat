---
splat_schema: 1
splat_id: 01df29a2-b902-5f07-8b69-1bfb56abf561
splat_kind: creature
splat_key: dnd2024:creatures/wyvern
splat_name: Wyvern
splat_origin:
  key: dnd2024:reference/creature/wyvern
splat_sources:
- title: SRD 5.2.1
  locator: pp. 343
  url: https://www.dndbeyond.com/srd
---

# Wyvern

## Source text

<!-- SRD page 343 -->

Large Dragon, Unaligned\
AC 14 Initiative +0 (10)\
HP 127 (15d10 + 45)\
Speed 30 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 10 +0 +0 Con 16 +3 +3\
Int 5 −3 −3 Wis 12 +1 +1 Cha 6 −2 −2\
Skills Perception +4\
Senses Darkvision 120 ft.; Passive Perception 14\
Languages None\
CR 6 (XP 2,300; PB +3)\

### Actions

Multiattack. The wyvern makes one Bite attack and\
one Sting attack.\
Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 13 (2d8 + 4)\
Piercing damage.\
Sting. Melee Attack Roll: +7, reach 10 ft. Hit: 11 (2d6\
+ 4) Piercing damage plus 24 (7d6) Poison damage, and\
the target has the Poisoned condition until the start of\
the wyvern’s next turn.\

Original layout: [[../../srd-5.2.1/pages/343|page 343]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 30 ft., Fly 80 ft. ` |
| ` skills_text ` | ` Perception +4 ` |
| ` senses_text ` | ` Darkvision 120 ft.; Passive Perception 14 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 6 (XP 2,300; PB +3) ` |
| ` hit_points_text ` | ` 127 (15d10 + 45) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 19 ` | ` 4 ` |
| ` Dexterity ` | ` 10 ` | ` 0 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 5 ` | ` -3 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 127 ` |  |
| ` initiative ` | ` 0 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` fly_speed ` | ` 80 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/wyvern ` |
| ` implementation ` | ` dnd2024:creature_wyvern ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 7
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
        sides: 8
      add: 4
    average: 13
```

##### Sting

<!-- splat:yaml path=splat_rule/parameters/features/sting -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 7
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 6
      add: 4
    average: 11
  - type: poison
    amount:
      dice:
        count: 7
        sides: 6
      add: 0
    average: 24
- op: condition.apply
  condition: poisoned
  duration:
    until: actor.next_turn_start
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 1
  feature: bite
- count: 1
  feature: sting
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
