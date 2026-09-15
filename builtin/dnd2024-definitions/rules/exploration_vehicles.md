---
splat_schema: 1
splat_id: f8425ca4-ea07-5bf6-89bb-a6cadc0a1575
splat_kind: rule
splat_key: dnd2024:rules/exploration_vehicles
splat_name: Vehicles
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
---

# Vehicles

## Source text

<!-- SRD page 12 -->

Travelers in wagons, carriages, or other land vehi-\
cles choose a pace as normal. Characters in a water-\
borne vessel are limited to the speed of the vessel,\
and they don’t choose a travel pace. Depending on\
the vessel and the size of the crew, ships might be\
able to travel for up to 24 hours per day. “Equip-\
ment” includes vehicles for sale.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/exploration_vehicles ` |
| ` implementation ` | ` dnd2024:rules_exploration_vehicles ` |

### Parameters

#### Land vehicle

<!-- splat:values path=splat_rule/parameters/land_vehicle -->

| Field | Value |
| --- | --- |
| ` pace_choice ` | ` true ` |

#### Water vehicle

<!-- splat:values path=splat_rule/parameters/water_vehicle -->

| Field | Value |
| --- | --- |
| ` pace_choice ` | ` false ` |
| ` speed_source ` | ` vehicle ` |
| ` travel_hours_per_day_maximum ` | ` 24 ` |
| ` daily_hours_authority ` | ` gm ` |
| ` daily_hours_factors ` | ` ["vessel", "crew_size"] ` |
