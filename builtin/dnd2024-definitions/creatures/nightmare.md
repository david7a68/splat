---
splat_schema: 2
splat_id: 9ba2827f-99b6-59da-a8c3-d724f26f2d8b
splat_kind: creature
splat_key: dnd2024:creatures/nightmare
splat_name: Nightmare
splat_origin: dnd2024:reference/creature/nightmare
splat_sources:
- SRD 5.2.1, pp. 311, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 226, Habitat
---

# Nightmare


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Fiend |
| Alignment | Neutral Evil |
| Armor Class | 13 |
| Hit Points | 68 (8d10 + 24) |
| Initiative | +2 (12) |
| Speed | 60 ft., Fly 90 ft. (hover) |
| Challenge | 3 (XP 700; PB +2) |
| Senses | Passive Perception 11 |
| Languages | Understands Abyssal, Common, and Infernal but can’t speak |
| Immunities | Fire |
| Habitat | Planar (Lower Planes) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 15 | +2 | +2 |
| Con | 16 | +3 | +3 |
| Int | 10 | +0 | +0 |
| Wis | 13 | +1 | +1 |
| Cha | 15 | +2 | +2 |


## Source text

<!-- SRD page 311 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Fiend, Neutral Evil\
> AC 13 Initiative +2 (12)\
> HP 68 (8d10 + 24)\
> Speed 60 ft., Fly 90 ft. (hover)\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3\
> Int 10 +0 +0 Wis 13 +1 +1 Cha 15 +2 +2\
> Immunities Fire\
> Senses Passive Perception 11\
> Languages Understands Abyssal, Common, and Infernal but can’t speak\
> CR 3 (XP 700; PB +2)


### Traits

Confer Fire Resistance. The nightmare can grant Resistance to Fire damage to a rider while it is on the nightmare.

Illumination. The nightmare sheds Bright Light in a 10-foot radius and Dim Light for an additional 10 feet.

### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 13 (2d8 + 4) Bludgeoning damage plus 10 (3d6) Fire damage.

Ethereal Stride. The nightmare and up to three willing creatures within 5 feet of it teleport to the Ethereal Plane from the Material Plane or vice versa.

Original layout: [[../../srd-5.2.1/pages/311|page 311]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/nightmare ` |
| ` implementation ` | ` dnd2024:creature_nightmare ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |

#### Features

##### Confer fire resistance

<!-- splat:yaml path=splat_rule/parameters/features/confer_fire_resistance -->

```yaml
economy: trait
operation: passive_rule
optional: true
grant:
  damage_resistances:
  - fire
  target: rider
  while: mounted_on_self
```

##### Illumination

<!-- splat:values path=splat_rule/parameters/features/illumination -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` bright_radius_ft ` | ` 10 ` |
| ` additional_dim_radius_ft ` | ` 10 ` |

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
| ` row_1 ` | ` 13 ` | ` 2 ` | ` 8 ` | ` 4 ` | ` Bludgeoning ` |
| ` row_2 ` | ` 10 ` | ` 3 ` | ` 6 ` | ` 0 ` | ` Fire ` |

##### Ethereal stride

<!-- splat:yaml path=splat_rule/parameters/features/ethereal_stride -->

```yaml
economy: action
operation: planar_teleport
targets:
  self: true
  additional:
    count_maximum: 3
    willing: true
    range_ft: 5
plane_pairs:
- - material
  - ethereal
- - ethereal
  - material
```

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands:
- abyssal
- common
- infernal
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

