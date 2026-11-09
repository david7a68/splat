---
splat_schema: 2
splat_id: 6ff31d77-aeac-5666-8066-a74ed07ab885
splat_kind: creature
splat_key: dnd2024:creatures/allosaurus
splat_name: Allosaurus
splat_origin: dnd2024:reference/creature/allosaurus
splat_sources:
- SRD 5.2.1, pp. 344, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 348, Habitat
---

# Allosaurus


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast (Dinosaur) |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 51 (6d10 + 18) |
| Initiative | +1 (11) |
| Speed | 60 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Perception +5 |
| Senses | Passive Perception 15 |
| Languages | None |
| Habitat | Grassland |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +4 |
| Dex | 13 | +1 | +1 |
| Con | 17 | +3 | +3 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 344 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast (Dinosaur), Unaligned\
> AC 13  Initiative +1 (11)\
> HP 51 (6d10 + 18)\
> Speed 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 19 +4 +4 Dex 13 +1 +1 Con 17 +3 +3\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
> Skills Perception +5\
> Senses Passive Perception 15\
> Languages None\
> CR 2 (XP 450; PB +2)


### Actions

Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 15 (2d10 + 4) Piercing damage.

Claws. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (1d8 +4) Slashing damage. If the target is a Large or smaller creature and the allosaurus moved 30+ feet straight toward it immediately before the hit, the target has the Prone condition, and the allosaurus can make one Bite attack against it.

Original layout: [[../../srd-5.2.1/pages/344|page 344]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/allosaurus ` |
| ` implementation ` | ` dnd2024:creature_allosaurus ` |

### Parameters

#### Features

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
  supplied_attack_total: 6
  reach_ft: 5
hit:
  damage:
  - average: 15
    roll:
      count: 2
      sides: 10
      modifier: 4
    type: piercing
```

##### Claws

<!-- splat:yaml path=splat_rule/parameters/features/claws -->

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
  - average: 8
    roll:
      count: 1
      sides: 8
      modifier: 4
    type: slashing
  prone:
    condition: prone
    maximum_size: large
    requires:
      immediately_preceding_straight_movement_toward_target_ft: 30
  additional_attack:
    feature: bite
    target: same
    requires:
      maximum_target_size: large
      immediately_preceding_straight_movement_toward_target_ft: 30
    optional: true
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
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
