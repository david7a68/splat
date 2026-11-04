---
splat_schema: 2
splat_id: 4b98a7da-5bb9-5961-83d8-20881041149c
splat_kind: rule
splat_key: dnd2024:rules/lifestyle_expenses_wretched_free
splat_name: Wretched (Free)
splat_origin: dnd2024:reference/pages/101
splat_sources:
- SRD 5.2.1, pp. 101, https://www.dndbeyond.com/srd
---

# Wretched (Free)

## Source text

<!-- SRD page 101 -->

You survive via chance and charity. You’re often exposed to natural dangers as a result of sleeping outside.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/lifestyle_expenses_wretched_free ` |
| ` implementation ` | ` dnd2024:rules_lifestyle_expenses_wretched_free ` |

### Parameters

#### Lifestyle

<!-- splat:values path=splat_rule/parameters/lifestyle -->

| Field | Value |
| --- | --- |
| ` id ` | ` wretched ` |
| ` cost_gp_per_day ` | ` 0 ` |

#### Displaced vehicle table

<!-- splat:table
path: splat_rule/parameters/displaced_vehicle_table
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
| ` Airship ` | ` 10 ` | ` 20 ` | ` 13 ` | ` 300 ` | null | ` 40000 ` | [Rules](#^splat-7b0cb3fe-airship "section:airship") |
| ` Galley ` | ` 80 ` | ` 0 ` | ` 15 ` | ` 500 ` | ` 20 ` | ` 30000 ` | [Rules](#^splat-7b0cb3fe-galley "section:galley") |
| ` Keelboat ` | ` 1 ` | ` 6 ` | ` 15 ` | ` 100 ` | ` 10 ` | ` 3000 ` | [Rules](#^splat-7b0cb3fe-keelboat "section:keelboat") |
| ` Longship ` | ` 40 ` | ` 150 ` | ` 15 ` | ` 300 ` | ` 15 ` | ` 10000 ` | [Rules](#^splat-7b0cb3fe-longship "section:longship") |
| ` Rowboat ` | ` 1 ` | ` 3 ` | ` 11 ` | ` 50 ` | null | ` 50 ` | [Rules](#^splat-7b0cb3fe-rowboat "section:rowboat") |
| ` Sailing ship ` | ` 20 ` | ` 20 ` | ` 15 ` | ` 300 ` | ` 15 ` | ` 10000 ` | [Rules](#^splat-7b0cb3fe-sailing_ship "section:sailing_ship") |
| ` Warship ` | ` 60 ` | ` 60 ` | ` 15 ` | ` 500 ` | ` 20 ` | ` 25000 ` | [Rules](#^splat-7b0cb3fe-warship "section:warship") |

##### Airship

^splat-7b0cb3fe-airship

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@airship/speed_mph -->

```yaml
8
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@airship/cargo_tons -->

```yaml
1
```

##### Galley

^splat-7b0cb3fe-galley

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@galley/speed_mph -->

```yaml
4
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@galley/cargo_tons -->

```yaml
150
```

##### Keelboat

^splat-7b0cb3fe-keelboat

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@keelboat/speed_mph -->

```yaml
1
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@keelboat/cargo_tons -->

```yaml
0.5
```

##### Longship

^splat-7b0cb3fe-longship

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@longship/speed_mph -->

```yaml
3
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@longship/cargo_tons -->

```yaml
10
```

##### Rowboat

^splat-7b0cb3fe-rowboat

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@rowboat/speed_mph -->

```yaml
1.5
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@rowboat/cargo_tons -->

```yaml
0
```

##### Sailing ship

^splat-7b0cb3fe-sailing_ship

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@sailing_ship/speed_mph -->

```yaml
2
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@sailing_ship/cargo_tons -->

```yaml
100
```

##### Warship

^splat-7b0cb3fe-warship

###### Speed mph

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@warship/speed_mph -->

```yaml
2.5
```

###### Cargo tons

<!-- splat:yaml path=splat_rule/parameters/displaced_vehicle_table/@warship/cargo_tons -->

```yaml
200
```
