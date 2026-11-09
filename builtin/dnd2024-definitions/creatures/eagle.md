---
splat_schema: 2
splat_id: 21e8a0b6-2688-5df4-947d-b78117ff85ed
splat_kind: creature
splat_key: dnd2024:creatures/eagle
splat_name: Eagle
splat_origin: dnd2024:reference/creature/eagle
splat_sources:
- SRD 5.2.1, pp. 348, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 353, Habitat
---

# Eagle


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 4 (1d6 + 1) |
| Initiative | +2 (12) |
| Speed | 10 ft., Fly 60 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Skills | Perception +6 |
| Senses | Passive Perception 16 |
| Languages | None |
| Habitat | Coastal, Grassland, Hill, Mountain |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 6 | -2 | -2 |
| Dex | 15 | +2 | +2 |
| Con | 12 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 14 | +2 | +2 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 348 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Beast, Unaligned\
> AC 12  Initiative +2 (12)\
> HP 4 (1d6 + 1)\
> Speed 10 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 6 −2 −2 Dex 15 +2 +2 Con 12 +1 +1\
> Int 2 −4 −4 Wis 14 +2 +2 Cha 7 −2 −2\
> Skills Perception +6\
> Senses Passive Perception 16\
> Languages None\
> CR 0 (XP 10; PB +2)


### Actions

Talons. Melee Attack Roll: +4, reach 5 feet. Hit: 4 (1d4 + 2) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/348|page 348]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/eagle ` |
| ` implementation ` | ` dnd2024:creature_eagle ` |

### Parameters

#### Features

##### Talons

<!-- splat:yaml path=splat_rule/parameters/features/talons -->

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
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 6 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 16 ` |

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
>   value: 60
> ```

