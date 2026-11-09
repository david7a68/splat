---
splat_schema: 2
splat_id: 87b77368-acf9-5128-b017-ac51db4b16e8
splat_kind: creature
splat_key: dnd2024:creatures/giant_wolf_spider
splat_name: Giant Wolf Spider
splat_origin: dnd2024:reference/creature/giant_wolf_spider
splat_sources:
- SRD 5.2.1, pp. 355, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 362, Habitat
---

# Giant Wolf Spider


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 11 (2d8 + 2) |
| Initiative | +3 (13) |
| Speed | 40 ft., Climb 40 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Skills | Perception +3, Stealth +7 |
| Senses | Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 13 |
| Languages | None |
| Habitat | Coastal, Desert, Forest, Grassland, Hill |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 12 | +1 | +1 |
| Dex | 16 | +3 | +3 |
| Con | 13 | +1 | +1 |
| Int | 3 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 4 | -3 | -3 |


## Source text

<!-- SRD page 355 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 13  Initiative +3 (13)\
> HP 11 (2d8 + 2)\
> Speed 40 ft., Climb 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 12 +1 +1 Dex 16 +3 +3 Con 13 +1 +1\
> Int 3 −4 −4 Wis 12 +1 +1 Cha 4 −3 −3\
> Skills Perception +3, Stealth +7\
> Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 13\
> Languages None\
> CR 1/4 (XP 50; PB +2)


### Traits

Spider Climb. The spider can climb difficult surfaces, including along ceilings, without needing to make an ability check.

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3) Piercing damage plus 5 (2d4) Poison damage.

Original layout: [[../../srd-5.2.1/pages/355|page 355]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_wolf_spider ` |
| ` implementation ` | ` dnd2024:creature_giant_wolf_spider ` |

### Parameters

#### Features

##### Spider climb

<!-- splat:values path=splat_rule/parameters/features/spider_climb -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` climb_difficult_surfaces ` | ` true ` |
| ` climb_ceilings ` | ` true ` |
| ` ability_check_required ` | ` false ` |

##### Bite

<!-- splat:values path=splat_rule/parameters/features/bite -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/bite/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 5
reach_ft: 5
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/bite/hit/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Average:
    path: average
    type: integer
  Roll count:
    path: roll/count
    type: integer
  Roll sides:
    path: roll/sides
    type: integer
  Roll modifier:
    path: roll/modifier
    type: integer
  Type:
    path: type
    type: symbol
-->

| Key | Average | Roll count | Roll sides | Roll modifier | Type |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 5 ` | ` 1 ` | ` 4 ` | ` 3 ` | ` Piercing ` |
| ` row_2 ` | ` 5 ` | ` 2 ` | ` 4 ` | ` 0 ` | ` Poison ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |
| ` stealth ` | ` 7 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 10 ` |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 13 ` |

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
> climb_speed:
>   unit: ft
>   value: 40
> ```

