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

A ship sailing against a strong wind moves at half\
speed. In a dead calm (no wind), waterborne ships\
can’t move under sail and must be rowed. Keelboats\
and Rowboats are used on lakes and rivers. If going\
downstream, add the speed of the current (typically\
3 miles per hour) to the speed of the vehicle. These\
vehicles can’t be rowed against any significant cur-\
rent, but they can be pulled upstream by draft an-\
imals on the shores. A Rowboat can be carried and\
weighs 100 pounds.\

<!-- SRD page 101 -->

Airborne and Waterborne Vehicles\
Ship Speed Crew Passengers Cargo (Tons)\
Airship 8 mph 10 20 1\
Galley 4 mph 80 — 150\
Keelboat 1 mph 1 6 1/2\
Longship 3 mph 40 150 10\
Rowboat 1½ mph 1 3 —\
Sailing Ship 2 mph 20 20 100\
Warship 2½ mph 60 60 200\

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

<!-- splat:rows path=splat_rule/parameters/vehicles -->

| Row | id | speed_mph | crew | passengers | cargo_tons | ac | hp | damage_threshold | cost_gp |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` airship ` | ` 8 ` | ` 10 ` | ` 20 ` | ` 1 ` | ` 13 ` | ` 300 ` | ` null ` | ` 40000 ` |
| ` 1 ` | ` galley ` | ` 4 ` | ` 80 ` | ` 0 ` | ` 150 ` | ` 15 ` | ` 500 ` | ` 20 ` | ` 30000 ` |
| ` 2 ` | ` keelboat ` | ` 1 ` | ` 1 ` | ` 6 ` | ` 0.5 ` | ` 15 ` | ` 100 ` | ` 10 ` | ` 3000 ` |
| ` 3 ` | ` longship ` | ` 3 ` | ` 40 ` | ` 150 ` | ` 10 ` | ` 15 ` | ` 300 ` | ` 15 ` | ` 10000 ` |
| ` 4 ` | ` rowboat ` | ` 1.5 ` | ` 1 ` | ` 3 ` | ` 0 ` | ` 11 ` | ` 50 ` | ` null ` | ` 50 ` |
| ` 5 ` | ` sailing_ship ` | ` 2 ` | ` 20 ` | ` 20 ` | ` 100 ` | ` 15 ` | ` 300 ` | ` 15 ` | ` 10000 ` |
| ` 6 ` | ` warship ` | ` 2.5 ` | ` 60 ` | ` 60 ` | ` 200 ` | ` 15 ` | ` 500 ` | ` 20 ` | ` 25000 ` |

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
