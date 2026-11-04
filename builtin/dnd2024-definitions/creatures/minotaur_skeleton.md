---
splat_schema: 2
splat_id: 7a50a965-93f4-58ea-b359-afca41a8f7e5
splat_kind: creature
splat_key: dnd2024:creatures/minotaur_skeleton
splat_name: Minotaur Skeleton
splat_origin: dnd2024:reference/creature/minotaur_skeleton
splat_sources:
- SRD 5.2.1, pp. 326, https://www.dndbeyond.com/srd
---

# Minotaur Skeleton


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Undead |
| Alignment | Lawful Evil |
| Armor Class | 12 |
| Hit Points | 45 (6d10 + 12) |
| Initiative | +0 (10) |
| Speed | 40 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 9 |
| Languages | Understands Abyssal but can’t speak |
| Immunities | Poison; Exhaustion, Poisoned |
| Vulnerabilities | Bludgeoning |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 11 | +0 | +0 |
| Con | 15 | +2 | +2 |
| Int | 6 | -2 | -2 |
| Wis | 8 | -1 | -1 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 326 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Undead, Lawful Evil\
> AC 12 Initiative +0 (10)\
> HP 45 (6d10 + 12)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 11 +0 +0 Con 15 +2 +2\
> Int 6 −2 −2 Wis 8 −1 −1 Cha 5 −3 −3\
> Vulnerabilities Bludgeoning\
> Immunities Poison; Exhaustion, Poisoned\
> Senses Darkvision 60 ft.; Passive Perception 9\
> Languages Understands Abyssal but can’t speak\
> CR 2 (XP 450; PB +2)


### Actions

Gore. Melee Attack Roll: +6, reach 5 ft. Hit: 11 (2d6 + 4) Piercing damage. If the target is a Large or smaller creature and the skeleton moved 20+ feet straight toward it immediately before the hit, the target takes an extra 9 (2d8) Piercing damage and has the Prone condition.

Slam. Melee Attack Roll: +6, reach 5 ft. Hit: 15 (2d10 + 4) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/326|page 326]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/minotaur_skeleton ` |
| ` implementation ` | ` dnd2024:creature_minotaur_skeleton ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_vulnerabilities ` | ` ["bludgeoning"] ` |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "poisoned"] ` |

#### Features

##### Gore

<!-- splat:yaml path=splat_rule/parameters/features/gore -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 6
  reach_ft: 5
hit:
  damage:
  - average: 11
    roll:
      count: 2
      sides: 6
      modifier: 4
    type: piercing
  prone:
    condition: prone
    maximum_size: large
    requires:
      immediately_preceding_straight_movement_toward_target_ft: 20
  conditional_damage:
    requires:
      maximum_target_size: large
      immediately_preceding_straight_movement_toward_target_ft: 20
    damage:
    - roll:
        count: 2
        sides: 8
        modifier: 0
      type: piercing
```

##### Slam

<!-- splat:yaml path=splat_rule/parameters/features/slam -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 6
  reach_ft: 5
hit:
  damage:
  - average: 15
    roll:
      count: 2
      sides: 10
      modifier: 4
    type: bludgeoning
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 9 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands:
- abyssal
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
