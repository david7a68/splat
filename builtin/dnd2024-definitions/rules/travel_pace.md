---
splat_schema: 1
splat_id: e29ea5b8-5087-51fb-aa95-c7e475fb5152
splat_kind: rule
splat_key: dnd2024:rules/travel_pace
splat_name: Travel Pace
splat_origin:
  key: dnd2024:reference/pages/192
splat_sources:
- title: SRD 5.2.1
  locator: pp. 192
  url: https://www.dndbeyond.com/srd
---

# Travel Pace

## Source text

<!-- SRD page 192 -->

A group of characters can travel overland at a Nor-\
mal, Fast, or Slow pace, as described in “Playing the\
Game.” During any journey stage, the predominant\
terrain determines the characters’ maximum travel\
pace, as shown in the Maximum Pace column of the\
Travel Terrain table. Certain factors can affect a\
group’s travel pace.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/travel_pace ` |
| ` implementation ` | ` dnd2024:rules_travel_pace ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` paces ` | ` ["normal", "fast", "slow"] ` |
| ` maximum_determined_by ` | ` predominant_terrain_per_journey_stage ` |
| ` arctic_fast_pace_requires_appropriate_equipment ` | ` true ` |
| ` waterborne_uses_vehicle_speed ` | ` true ` |

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

##### Encounter distance ft

<!-- splat:yaml path=splat_rule/parameters/terrain_table/0/encounter_distance_ft -->

```yaml
dice:
  count: 6
  sides: 6
multiplier: 10
```

##### Encounter distance ft

<!-- splat:yaml path=splat_rule/parameters/terrain_table/1/encounter_distance_ft -->

```yaml
dice:
  count: 2
  sides: 10
multiplier: 10
```

##### Encounter distance ft

<!-- splat:yaml path=splat_rule/parameters/terrain_table/2/encounter_distance_ft -->

```yaml
dice:
  count: 6
  sides: 6
multiplier: 10
```

##### Encounter distance ft

<!-- splat:yaml path=splat_rule/parameters/terrain_table/3/encounter_distance_ft -->

```yaml
dice:
  count: 2
  sides: 8
multiplier: 10
```

##### Encounter distance ft

<!-- splat:yaml path=splat_rule/parameters/terrain_table/4/encounter_distance_ft -->

```yaml
dice:
  count: 6
  sides: 6
multiplier: 10
```

##### Encounter distance ft

<!-- splat:yaml path=splat_rule/parameters/terrain_table/5/encounter_distance_ft -->

```yaml
dice:
  count: 2
  sides: 10
multiplier: 10
```

##### Encounter distance ft

<!-- splat:yaml path=splat_rule/parameters/terrain_table/6/encounter_distance_ft -->

```yaml
dice:
  count: 4
  sides: 10
multiplier: 10
```

##### Encounter distance ft

<!-- splat:yaml path=splat_rule/parameters/terrain_table/7/encounter_distance_ft -->

```yaml
dice:
  count: 2
  sides: 8
multiplier: 10
```

##### Encounter distance ft

<!-- splat:yaml path=splat_rule/parameters/terrain_table/8/encounter_distance_ft -->

```yaml
dice:
  count: 2
  sides: 6
multiplier: 10
```

##### Encounter distance ft

<!-- splat:yaml path=splat_rule/parameters/terrain_table/9/encounter_distance_ft -->

```yaml
dice:
  count: 2
  sides: 6
multiplier: 10
```

##### Encounter distance ft

<!-- splat:yaml path=splat_rule/parameters/terrain_table/10/encounter_distance_ft -->

```yaml
dice:
  count: 6
  sides: 6
multiplier: 10
```
