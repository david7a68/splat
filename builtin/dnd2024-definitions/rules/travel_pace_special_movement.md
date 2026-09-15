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

If a party can travel at a high Speed for an extended\
time, as with a spell such as Wind Walk or a magic\
item such as a Carpet of Flying, translate the party’s\
Speed into travel rates using these rules:\
Travel Terrain\
Terrain Maximum Pace Encounter Distance\
Arctic Fast* 6d6 × 10 feet\
Coastal Normal 2d10 × 10 feet\
Desert Normal 6d6 × 10 feet\
Forest Normal 2d8 × 10 feet\
Grassland Fast 6d6 × 10 feet\
Hill Normal 2d10 × 10 feet\
Mountain Slow 4d10 × 10 feet\
Swamp Slow 2d8 × 10 feet\
Underdark Normal 2d6 × 10 feet\
Urban Normal 2d6 × 10 feet\
Waterborne Special† 6d6 × 10 feet\
*Appropriate equipment (such as skis) is necessary to keep up a Fast pace in Arctic terrain.\
†Characters’ rate of travel while waterborne depends on the vehicle carrying them; see “Vehicles.”\
Miles per hour = Speed ÷ 10\
Miles per day (Normal pace) = Miles per hour\
× number of hours traveled (typically 8)\
Fast pace = Miles per day × 1⅓ (round down)\
Slow pace = Miles per day × 2/3 (round down)\
If the characters are flying or their special move-\
ment allows them to ignore Difficult Terrain, they\
can move at a Fast pace regardless of the terrain.\

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

<!-- splat:yaml path=splat_rule/parameters/normal_miles_per_day -->

```yaml
op: multiply
args:
- variable: miles_per_hour
- variable: hours_traveled
```

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

<!-- splat:rows path=splat_rule/parameters/terrain_table -->

| Row | terrain | maximum_pace | foraging_dc | navigation_dc | search_dc |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` arctic ` | ` fast ` | ` 20 ` | ` 10 ` | ` 10 ` |
| ` 1 ` | ` coastal ` | ` normal ` | ` 10 ` | ` 5 ` | ` 15 ` |
| ` 2 ` | ` desert ` | ` normal ` | ` 20 ` | ` 10 ` | ` 10 ` |
| ` 3 ` | ` forest ` | ` normal ` | ` 10 ` | ` 15 ` | ` 15 ` |
| ` 4 ` | ` grassland ` | ` fast ` | ` 15 ` | ` 5 ` | ` 15 ` |
| ` 5 ` | ` hill ` | ` normal ` | ` 15 ` | ` 10 ` | ` 15 ` |
| ` 6 ` | ` mountain ` | ` slow ` | ` 20 ` | ` 15 ` | ` 20 ` |
| ` 7 ` | ` swamp ` | ` slow ` | ` 10 ` | ` 15 ` | ` 20 ` |
| ` 8 ` | ` underdark ` | ` normal ` | ` 20 ` | ` 10 ` | ` 20 ` |
| ` 9 ` | ` urban ` | ` normal ` | ` 20 ` | ` 15 ` | ` 15 ` |
| ` 10 ` | ` waterborne ` | ` vehicle_speed ` | ` 15 ` | ` 10 ` | ` 15 ` |

##### Encounter distance feet

<!-- splat:yaml path=splat_rule/parameters/terrain_table/0/encounter_distance_feet -->

```yaml
op: multiply
args:
- dice:
    count: 6
    sides: 6
- 10
```

##### Encounter distance feet

<!-- splat:yaml path=splat_rule/parameters/terrain_table/1/encounter_distance_feet -->

```yaml
op: multiply
args:
- dice:
    count: 2
    sides: 10
- 10
```

##### Encounter distance feet

<!-- splat:yaml path=splat_rule/parameters/terrain_table/2/encounter_distance_feet -->

```yaml
op: multiply
args:
- dice:
    count: 6
    sides: 6
- 10
```

##### Encounter distance feet

<!-- splat:yaml path=splat_rule/parameters/terrain_table/3/encounter_distance_feet -->

```yaml
op: multiply
args:
- dice:
    count: 2
    sides: 8
- 10
```

##### Encounter distance feet

<!-- splat:yaml path=splat_rule/parameters/terrain_table/4/encounter_distance_feet -->

```yaml
op: multiply
args:
- dice:
    count: 6
    sides: 6
- 10
```

##### Encounter distance feet

<!-- splat:yaml path=splat_rule/parameters/terrain_table/5/encounter_distance_feet -->

```yaml
op: multiply
args:
- dice:
    count: 2
    sides: 10
- 10
```

##### Encounter distance feet

<!-- splat:yaml path=splat_rule/parameters/terrain_table/6/encounter_distance_feet -->

```yaml
op: multiply
args:
- dice:
    count: 4
    sides: 10
- 10
```

##### Encounter distance feet

<!-- splat:yaml path=splat_rule/parameters/terrain_table/7/encounter_distance_feet -->

```yaml
op: multiply
args:
- dice:
    count: 2
    sides: 8
- 10
```

##### Encounter distance feet

<!-- splat:yaml path=splat_rule/parameters/terrain_table/8/encounter_distance_feet -->

```yaml
op: multiply
args:
- dice:
    count: 2
    sides: 6
- 10
```

##### Encounter distance feet

<!-- splat:yaml path=splat_rule/parameters/terrain_table/9/encounter_distance_feet -->

```yaml
op: multiply
args:
- dice:
    count: 2
    sides: 6
- 10
```

##### Encounter distance feet

<!-- splat:yaml path=splat_rule/parameters/terrain_table/10/encounter_distance_feet -->

```yaml
op: multiply
args:
- dice:
    count: 6
    sides: 6
- 10
```
