---
splat_schema: 2
splat_id: 9e1ea0e7-23c3-5691-a57c-b9970b82c0bb
splat_kind: creature
splat_key: dnd2024:creatures/pegasus
splat_name: Pegasus
splat_origin: dnd2024:reference/creature/pegasus
splat_sources:
- SRD 5.2.1, pp. 313, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 235, Habitat
---

# Pegasus


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Celestial |
| Alignment | Chaotic Good |
| Armor Class | 12 |
| Hit Points | 59 (7d10 + 21) |
| Initiative | +2 (12) |
| Speed | 60 ft., Fly 90 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Perception +6 |
| Senses | Passive Perception 16 |
| Languages | Understands Celestial, Common, Elvish, and Sylvan but can’t speak |
| Habitat | Forest, Grassland, Hill, Planar (Upper Planes) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 15 | +2 | +4 |
| Con | 16 | +3 | +5 |
| Int | 10 | +0 | +0 |
| Wis | 15 | +2 | +4 |
| Cha | 13 | +1 | +3 |


## Source text

<!-- SRD page 313 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Celestial, Chaotic Good\
> AC 12 Initiative +2 (12)\
> HP 59 (7d10 + 21)\
> Speed 60 ft., Fly 90 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 15 +2 +4 Con 16 +3 +5\
> Int 10 +0 +0 Wis 15 +2 +4 Cha 13 +1 +3\
> Skills Perception +6\
> Senses Passive Perception 16\
> Languages Understands Celestial, Common, Elvish, and Sylvan but can’t speak\
> CR 2 (XP 450; PB +2)


### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 7 (1d6 + 4) Bludgeoning damage plus 5 (2d4) Radiant damage.

Original layout: [[../../srd-5.2.1/pages/313|page 313]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/pegasus ` |
| ` implementation ` | ` dnd2024:creature_pegasus ` |

### Parameters

#### Features

##### Hooves

<!-- splat:values path=splat_rule/parameters/features/hooves -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/hooves/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 6
reach_ft: 5
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/hooves/hit/damage
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
| ` row_1 ` | ` 7 ` | ` 1 ` | ` 6 ` | ` 4 ` | ` Bludgeoning ` |
| ` row_2 ` | ` 5 ` | ` 2 ` | ` 4 ` | ` 0 ` | ` Radiant ` |

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
understands:
- celestial
- common
- elvish
- sylvan
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
>   value: 90
> ```

