---
splat_schema: 1
splat_id: 288eb685-d6ab-5bec-8aa0-b56547a5433a
splat_kind: creature
splat_key: dnd2024:creatures/ghoul
splat_name: Ghoul
splat_origin:
  key: dnd2024:reference/creature/ghoul
splat_sources:
- title: SRD 5.2.1
  locator: pp. 288
  url: https://www.dndbeyond.com/srd
---

# Ghoul

## Source text

<!-- SRD page 288 -->

Medium Undead, Chaotic Evil\
AC 12 Initiative +2 (12)\
HP 22 (5d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 15 +2 +2 Con 10 +0 +0\
Int 7 −2 −2 Wis 10 +0 +0 Cha 6 −2 −2\
Immunities Poison; Charmed, Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Common\
CR 1 (XP 200; PB +2)\

### Actions

Multiattack. The ghoul makes two Bite attacks.\
Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2)\
Piercing damage plus 3 (1d6) Necrotic damage.\
Claw. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4\
+ 2) Slashing damage. If the target is a creature that\
isn’t an Undead or elf, it is subjected to the following\
effect. Constitution Saving Throw: DC 10. Failure: The\
target has the Paralyzed condition until the end of its\
next turn.\

Original layout: [[../../srd-5.2.1/pages/288|page 288]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` undead ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` movement_text ` | ` 30 ft. ` |
| ` immunities_text ` | ` Poison; Charmed, Exhaustion, Poisoned ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Common ` |
| ` cr_text ` | ` 1 (XP 200; PB +2) ` |
| ` hit_points_text ` | ` 22 (5d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 13 ` | ` 1 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 7 ` | ` -2 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 22 ` |  |
| ` initiative ` | ` 2 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/ghoul ` |
| ` implementation ` | ` dnd2024:creature_ghoul ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["charmed", "exhaustion", "poisoned"] ` |

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: bite
  count: 2
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 4
  reach_ft: 5
hit:
  damage:
  - average: 5
    roll:
      count: 1
      sides: 6
      modifier: 2
    type: piercing
  - average: 3
    roll:
      count: 1
      sides: 6
      modifier: 0
    type: necrotic
```

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 4
  reach_ft: 5
hit:
  damage:
  - average: 4
    roll:
      count: 1
      sides: 4
      modifier: 2
    type: slashing
  save:
    operation: saving_throw
    ability: constitution
    dc: 10
    targets:
      kind: creature
      exclude:
      - undead
      - elf
    failure:
      conditions:
      - condition: paralyzed
        until: target.next_turn_end
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
understands: []
```

#### Resolution conventions

<!-- splat:yaml path=splat_rule/parameters/resolution_conventions -->

```yaml
distance_unit: foot
duration_unit: second
damage_roll_mode:
  choice:
  - listed_average
  - roll_dice
  chooser: gm
supplied_totals: before_additional_effect_modifiers
multiattack_nested_economy_cost: 0
nested_feature_resource_cost: normal
spell_level_unless_specified: lowest_possible
spell_upcast_without_permission: false
spell_casting_time_at_least_one_minute: normal_casting_procedure
geometry:
  input: adjudication
  assumptions:
  - range_and_visibility_verified
  - area_membership_verified
  - path_and_destination_legal
unlisted_success_branch: no_effect
unlisted_duration: until_removed_by_applicable_rule
```
