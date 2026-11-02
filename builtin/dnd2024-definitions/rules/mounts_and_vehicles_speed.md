---
splat_schema: 1
splat_id: 35bed7e8-3ea2-55b1-a6ff-f772be00fc71
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_speed
splat_name: Speed
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100, 101
  url: https://www.dndbeyond.com/srd
---

# Speed

## Source text

<!-- SRD page 100 -->

A ship sailing against a strong wind moves at half speed. In a dead calm (no wind), waterborne ships can’t move under sail and must be rowed. Keelboats and Rowboats are used on lakes and rivers. If going downstream, add the speed of the current (typically 3 miles per hour) to the speed of the vehicle. These vehicles can’t be rowed against any significant current, but they can be pulled upstream by draft animals on the shores. A Rowboat can be carried and weighs 100 pounds.

<!-- SRD page 101 -->

**Airborne and Waterborne Vehicles**

| Ship | Speed | Crew | Passengers | Cargo (Tons) | AC | HP | Damage Threshold | Cost |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Airship | 8 mph | 10 | 20 | 1 | 13 | 300 | — | 40,000 GP |
| Galley | 4 mph | 80 | — | 150 | 15 | 500 | 20 | 30,000 GP |
| Keelboat | 1 mph | 1 | 6 | 1/2 | 15 | 100 | 10 | 3,000 GP |
| Longship | 3 mph | 40 | 150 | 10 | 15 | 300 | 15 | 10,000 GP |
| Rowboat | 1½ mph | 1 | 3 | — | 11 | 50 | — | 50 GP |
| Sailing Ship | 2 mph | 20 | 20 | 100 | 15 | 300 | 15 | 10,000 GP |
| Warship | 2½ mph | 60 | 60 | 200 | 15 | 500 | 20 | 25,000 GP |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/mounts_and_vehicles_speed ` |
| ` implementation ` | ` dnd2024:rules_mounts_and_vehicles_speed ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` strong_headwind_speed_multiplier ` | ` 0.5 ` |
| ` river_boats ` | ` ["keelboat", "rowboat"] ` |
| ` rowboat_weight_lb ` | ` 100 ` |

#### Vehicles

<!-- splat:table
path: splat_rule/parameters/vehicles
key: Variant
order:
- airship
- galley
- keelboat
- longship
- rowboat
- sailing_ship
- warship
columns:
  Variant:
    path: id
    type: symbol
  Crew:
    path: crew
    type: integer
  Passengers:
    path: passengers
    type: integer
  Ac:
    path: ac
    type: integer
  Hp:
    path: hp
    type: integer
  Damage threshold:
    path: damage_threshold
    type: integer
    nullable: true
  Cost gp:
    path: cost_gp
    type: integer
  Details:
    type: section
    optional: true
-->

| Variant | Crew | Passengers | Ac | Hp | Damage threshold | Cost gp | Details |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ` Airship ` | ` 10 ` | ` 20 ` | ` 13 ` | ` 300 ` | null | ` 40000 ` | [Rules](#^splat-e0c0ed34-airship "section:airship") |
| ` Galley ` | ` 80 ` | ` 0 ` | ` 15 ` | ` 500 ` | ` 20 ` | ` 30000 ` | [Rules](#^splat-e0c0ed34-galley "section:galley") |
| ` Keelboat ` | ` 1 ` | ` 6 ` | ` 15 ` | ` 100 ` | ` 10 ` | ` 3000 ` | [Rules](#^splat-e0c0ed34-keelboat "section:keelboat") |
| ` Longship ` | ` 40 ` | ` 150 ` | ` 15 ` | ` 300 ` | ` 15 ` | ` 10000 ` | [Rules](#^splat-e0c0ed34-longship "section:longship") |
| ` Rowboat ` | ` 1 ` | ` 3 ` | ` 11 ` | ` 50 ` | null | ` 50 ` | [Rules](#^splat-e0c0ed34-rowboat "section:rowboat") |
| ` Sailing ship ` | ` 20 ` | ` 20 ` | ` 15 ` | ` 300 ` | ` 15 ` | ` 10000 ` | [Rules](#^splat-e0c0ed34-sailing_ship "section:sailing_ship") |
| ` Warship ` | ` 60 ` | ` 60 ` | ` 15 ` | ` 500 ` | ` 20 ` | ` 25000 ` | [Rules](#^splat-e0c0ed34-warship "section:warship") |

##### Airship

^splat-e0c0ed34-airship

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/vehicles/@airship/speed_mph -->

```yaml
8
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/vehicles/@airship/cargo_tons -->

```yaml
1
```

##### Galley

^splat-e0c0ed34-galley

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/vehicles/@galley/speed_mph -->

```yaml
4
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/vehicles/@galley/cargo_tons -->

```yaml
150
```

##### Keelboat

^splat-e0c0ed34-keelboat

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/vehicles/@keelboat/speed_mph -->

```yaml
1
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/vehicles/@keelboat/cargo_tons -->

```yaml
0.5
```

##### Longship

^splat-e0c0ed34-longship

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/vehicles/@longship/speed_mph -->

```yaml
3
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/vehicles/@longship/cargo_tons -->

```yaml
10
```

##### Rowboat

^splat-e0c0ed34-rowboat

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/vehicles/@rowboat/speed_mph -->

```yaml
1.5
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/vehicles/@rowboat/cargo_tons -->

```yaml
0
```

##### Sailing ship

^splat-e0c0ed34-sailing_ship

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/vehicles/@sailing_ship/speed_mph -->

```yaml
2
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/vehicles/@sailing_ship/cargo_tons -->

```yaml
100
```

##### Warship

^splat-e0c0ed34-warship

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/vehicles/@warship/speed_mph -->

```yaml
2.5
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/vehicles/@warship/cargo_tons -->

```yaml
200
```

#### No wind

<!-- splat:values path=splat_rule/parameters/no_wind -->

| Field | Value |
| --- | --- |
| ` sailing_movement ` | ` false ` |
| ` rowing_required ` | ` true ` |

#### Downstream

<!-- splat:values path=splat_rule/parameters/downstream -->

| Field | Value |
| --- | --- |
| ` add_current_speed ` | ` true ` |
| ` typical_current_mph ` | ` 3 ` |

#### Significant upstream current

<!-- splat:values path=splat_rule/parameters/significant_upstream_current -->

| Field | Value |
| --- | --- |
| ` rowing_allowed ` | ` false ` |
| ` shore_draft_animal_towing_allowed ` | ` true ` |
