---
splat_schema: 1
splat_id: 20810ca1-9835-5698-ae23-9f0cccaaae71
splat_kind: rule
splat_key: dnd2024:rules/travel_pace_special_movement
splat_name: Special Movement
splat_origin:
  key: dnd2024:reference/pages/192
splat_sources:
- title: SRD 5.2.1
  locator: pp. 192
  url: https://www.dndbeyond.com/srd
---

# Special Movement

## Source text

<!-- SRD page 192 -->

If a party can travel at a high Speed for an extended time, as with a spell such as Wind Walk or a magic item such as a Carpet of Flying, translate the party’s Speed into travel rates using these rules:

**Travel Terrain**

| Terrain | Maximum Pace | Encounter Distance | Foraging DC | Navigation DC | Search DC |
| --- | --- | --- | --- | --- | --- |
| Arctic | Fast* | 6d6 × 10 feet | 20 | 10 | 10 |
| Coastal | Normal | 2d10 × 10 feet | 10 | 5 | 15 |
| Desert | Normal | 6d6 × 10 feet | 20 | 10 | 10 |
| Forest | Normal | 2d8 × 10 feet | 10 | 15 | 15 |
| Grassland | Fast | 6d6 × 10 feet | 15 | 5 | 15 |
| Hill | Normal | 2d10 × 10 feet | 15 | 10 | 15 |
| Mountain | Slow | 4d10 × 10 feet | 20 | 15 | 20 |
| Swamp | Slow | 2d8 × 10 feet | 10 | 15 | 20 |
| Underdark | Normal | 2d6 × 10 feet | 20 | 10 | 20 |
| Urban | Normal | 2d6 × 10 feet | 20 | 15 | 15 |
| Waterborne | Special† | 6d6 × 10 feet | 15 | 10 | 15 |

*Appropriate equipment (such as skis) is necessary to keep up a Fast pace in Arctic terrain. †Characters’ rate of travel while waterborne depends on the vehicle carrying them; see “Vehicles.”

Miles per hour = Speed ÷ 10

Miles per day (Normal pace) = Miles per hour × number of hours traveled (typically 8)

Fast pace = Miles per day × 1⅓ (round down)

Slow pace = Miles per day × 2/3 (round down)

If the characters are flying or their special movement allows them to ignore Difficult Terrain, they can move at a Fast pace regardless of the terrain.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/travel_pace_special_movement ` |
| ` implementation ` | ` dnd2024:rule_travel_pace_special_movement ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` eligibility ` | ` sustained_special_movement_speed ` |
| ` typical_hours_traveled ` | ` 8 ` |
| ` fast_pace_regardless_of_terrain_when_any ` | ` ["flying", "special_movement_ignores_difficult_terrain"] ` |
| ` arctic_fast_requires ` | ` appropriate_equipment ` |
| ` waterborne_speed_source ` | ` vehicle ` |

#### Miles per hour

<!-- splat:yaml path=splat_rule/parameters/miles_per_hour -->

```yaml
op: divide
args:
- variable: speed_feet
- 10
```

#### Normal miles per day

<!-- splat:values path=splat_rule/parameters/normal_miles_per_day -->

| Field | Value |
| --- | --- |
| ` op ` | ` multiply ` |

##### Args

<!-- splat:table
path: splat_rule/parameters/normal_miles_per_day/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Variable:
    path: variable
    type: symbol
-->

| Key | Variable |
| --- | --- |
| ` row_1 ` | ` Miles per hour ` |
| ` row_2 ` | ` Hours traveled ` |

#### Fast miles per day

<!-- splat:yaml path=splat_rule/parameters/fast_miles_per_day -->

```yaml
op: floor
args:
- op: multiply
  args:
  - variable: normal_miles_per_day
  - numerator: 4
    denominator: 3
```

#### Slow miles per day

<!-- splat:yaml path=splat_rule/parameters/slow_miles_per_day -->

```yaml
op: floor
args:
- op: multiply
  args:
  - variable: normal_miles_per_day
  - numerator: 2
    denominator: 3
```

#### Terrain table

<!-- splat:table
path: splat_rule/parameters/terrain_table
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
- row_10
- row_11
columns:
  Key:
    type: key
  Terrain:
    path: terrain
    type: symbol
  Maximum pace:
    path: maximum_pace
    type: symbol
  Encounter distance feet op:
    path: encounter_distance_feet/op
    type: symbol
  Foraging dc:
    path: foraging_dc
    type: integer
  Navigation dc:
    path: navigation_dc
    type: integer
  Search dc:
    path: search_dc
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Terrain | Maximum pace | Encounter distance feet op | Foraging dc | Navigation dc | Search dc | Details |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Arctic ` | ` Fast ` | ` Multiply ` | ` 20 ` | ` 10 ` | ` 10 ` | [Rules](#^splat-b87cf5e4-row_1 "section:row_1") |
| ` row_2 ` | ` Coastal ` | ` Normal ` | ` Multiply ` | ` 10 ` | ` 5 ` | ` 15 ` | [Rules](#^splat-b87cf5e4-row_2 "section:row_2") |
| ` row_3 ` | ` Desert ` | ` Normal ` | ` Multiply ` | ` 20 ` | ` 10 ` | ` 10 ` | [Rules](#^splat-b87cf5e4-row_3 "section:row_3") |
| ` row_4 ` | ` Forest ` | ` Normal ` | ` Multiply ` | ` 10 ` | ` 15 ` | ` 15 ` | [Rules](#^splat-b87cf5e4-row_4 "section:row_4") |
| ` row_5 ` | ` Grassland ` | ` Fast ` | ` Multiply ` | ` 15 ` | ` 5 ` | ` 15 ` | [Rules](#^splat-b87cf5e4-row_5 "section:row_5") |
| ` row_6 ` | ` Hill ` | ` Normal ` | ` Multiply ` | ` 15 ` | ` 10 ` | ` 15 ` | [Rules](#^splat-b87cf5e4-row_6 "section:row_6") |
| ` row_7 ` | ` Mountain ` | ` Slow ` | ` Multiply ` | ` 20 ` | ` 15 ` | ` 20 ` | [Rules](#^splat-b87cf5e4-row_7 "section:row_7") |
| ` row_8 ` | ` Swamp ` | ` Slow ` | ` Multiply ` | ` 10 ` | ` 15 ` | ` 20 ` | [Rules](#^splat-b87cf5e4-row_8 "section:row_8") |
| ` row_9 ` | ` Underdark ` | ` Normal ` | ` Multiply ` | ` 20 ` | ` 10 ` | ` 20 ` | [Rules](#^splat-b87cf5e4-row_9 "section:row_9") |
| ` row_10 ` | ` Urban ` | ` Normal ` | ` Multiply ` | ` 20 ` | ` 15 ` | ` 15 ` | [Rules](#^splat-b87cf5e4-row_10 "section:row_10") |
| ` row_11 ` | ` Waterborne ` | ` Vehicle speed ` | ` Multiply ` | ` 15 ` | ` 10 ` | ` 15 ` | [Rules](#^splat-b87cf5e4-row_11 "section:row_11") |

##### Row 1

^splat-b87cf5e4-row_1

###### Args

<!-- splat:yaml path=splat_rule/parameters/terrain_table/@row_1/encounter_distance_feet/args -->

```yaml
- dice:
    count: 6
    sides: 6
- 10
```

##### Row 2

^splat-b87cf5e4-row_2

###### Args

<!-- splat:yaml path=splat_rule/parameters/terrain_table/@row_2/encounter_distance_feet/args -->

```yaml
- dice:
    count: 2
    sides: 10
- 10
```

##### Row 3

^splat-b87cf5e4-row_3

###### Args

<!-- splat:yaml path=splat_rule/parameters/terrain_table/@row_3/encounter_distance_feet/args -->

```yaml
- dice:
    count: 6
    sides: 6
- 10
```

##### Row 4

^splat-b87cf5e4-row_4

###### Args

<!-- splat:yaml path=splat_rule/parameters/terrain_table/@row_4/encounter_distance_feet/args -->

```yaml
- dice:
    count: 2
    sides: 8
- 10
```

##### Row 5

^splat-b87cf5e4-row_5

###### Args

<!-- splat:yaml path=splat_rule/parameters/terrain_table/@row_5/encounter_distance_feet/args -->

```yaml
- dice:
    count: 6
    sides: 6
- 10
```

##### Row 6

^splat-b87cf5e4-row_6

###### Args

<!-- splat:yaml path=splat_rule/parameters/terrain_table/@row_6/encounter_distance_feet/args -->

```yaml
- dice:
    count: 2
    sides: 10
- 10
```

##### Row 7

^splat-b87cf5e4-row_7

###### Args

<!-- splat:yaml path=splat_rule/parameters/terrain_table/@row_7/encounter_distance_feet/args -->

```yaml
- dice:
    count: 4
    sides: 10
- 10
```

##### Row 8

^splat-b87cf5e4-row_8

###### Args

<!-- splat:yaml path=splat_rule/parameters/terrain_table/@row_8/encounter_distance_feet/args -->

```yaml
- dice:
    count: 2
    sides: 8
- 10
```

##### Row 9

^splat-b87cf5e4-row_9

###### Args

<!-- splat:yaml path=splat_rule/parameters/terrain_table/@row_9/encounter_distance_feet/args -->

```yaml
- dice:
    count: 2
    sides: 6
- 10
```

##### Row 10

^splat-b87cf5e4-row_10

###### Args

<!-- splat:yaml path=splat_rule/parameters/terrain_table/@row_10/encounter_distance_feet/args -->

```yaml
- dice:
    count: 2
    sides: 6
- 10
```

##### Row 11

^splat-b87cf5e4-row_11

###### Args

<!-- splat:yaml path=splat_rule/parameters/terrain_table/@row_11/encounter_distance_feet/args -->

```yaml
- dice:
    count: 6
    sides: 6
- 10
```
