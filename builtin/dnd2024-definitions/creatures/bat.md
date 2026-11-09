---
splat_schema: 2
splat_id: 91576b90-7b8c-5204-a710-e0f7a3d88545
splat_kind: creature
splat_key: dnd2024:creatures/bat
splat_name: Bat
splat_origin: dnd2024:reference/creature/bat
splat_sources:
- SRD 5.2.1, pp. 345, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 349, Habitat
---

# Bat


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 1 (1d4 - 1) |
| Initiative | +2 (12) |
| Speed | 5 ft., Fly 30 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Senses | Blindsight 60 ft.; Passive Perception 11 |
| Languages | None |
| Habitat | Forest, Mountain, Underdark, Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 2 | -4 | -4 |
| Dex | 15 | +2 | +2 |
| Con | 8 | -1 | -1 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 4 | -3 | -3 |


## Source text

<!-- SRD page 345 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Beast, Unaligned\
> AC 12 Initiative +2 (12)\
> HP 1 (1d4 − 1)\
> Speed 5 ft., Fly 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 2 −4 −4 Dex 15 +2 +2 Con 8 −1 −1\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 4 −3 −3\
> Senses Blindsight 60 ft.; Passive Perception 11\
> Languages None\
> CR 0 (XP 10; PB +2)


### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Piercing damage.

Original layout: [[../../srd-5.2.1/pages/345|page 345]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/bat ` |
| ` implementation ` | ` dnd2024:creature_bat ` |

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
  supplied_attack_total: 4
  reach_ft: 5
hit:
  damage:
  - average: 1
    roll:
      constant: 1
    type: piercing
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 60 ` |
| ` passive_perception ` | ` 11 ` |

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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 30
> ```

