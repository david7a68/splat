---
splat_schema: 1
splat_id: 0cdad0a1-f0bb-5c2a-859a-469173a32d0f
splat_kind: rule
splat_key: dnd2024:rules/travel_pace_vehicles
splat_name: Vehicles
splat_origin:
  key: dnd2024:reference/pages/192
splat_sources:
- title: SRD 5.2.1
  locator: pp. 192
  url: https://www.dndbeyond.com/srd
---

# Vehicles

## Source text

<!-- SRD page 192 -->

Characters traveling in a vehicle use the vehicle’s\
speed in miles per hour (as shown in “Equipment”)\
to determine their rate of travel, and they don’t\
choose a travel pace.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/travel_pace_vehicles ` |
| ` implementation ` | ` dnd2024:rules_travel_pace_vehicles ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` travel_rate ` | ` vehicle_speed_mph ` |
| ` travel_pace_choice ` | ` false ` |
