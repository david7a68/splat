---
splat_schema: 2
splat_id: 6fad22d3-75bf-5c1d-95a8-e2e367dd4121
splat_kind: creature
splat_key: dnd2024:creatures/pony
splat_name: Pony
splat_origin: dnd2024:reference/creature/pony
splat_sources:
- SRD 5.2.1, pp. 359, https://www.dndbeyond.com/srd
---

# Pony


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 10 |
| Hit Points | 11 (2d8 + 2) |
| Initiative | +0 (10) |
| Speed | 40 ft. |
| Challenge | 1/8 (XP 25; PB +2) |
| Senses | Passive Perception 10 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +4 |
| Dex | 10 | +0 | +0 |
| Con | 13 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 11 | +0 | +0 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 359 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 10  Initiative +0 (10)\
> HP 11 (2d8 + 2)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str  15 +2 +4 Dex 10 +0 +0 Con 13 +1 +1\
> Int 2 −4 −4 Wis 11 +0 +0 Cha 7 −2 −2\
> Senses Passive Perception 10\
> Languages None\
> CR 1/8 (XP 25; PB +2)


### Actions

Hooves. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/359|page 359]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/pony ` |
| ` implementation ` | ` dnd2024:creature_pony ` |

### Parameters

#### Features

##### Hooves

<!-- splat:yaml path=splat_rule/parameters/features/hooves -->

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
    type: bludgeoning
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

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
