---
splat_schema: 1
splat_id: ead59709-203b-5520-be75-a6ad1a7e69e4
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_high_altitude
splat_name: High Altitude
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
---

# High Altitude

## Source text

<!-- SRD page 195 -->

Traveling at altitudes of 10,000 feet or higher above sea level is taxing for most creatures because of the reduced amount of oxygen in the air. Each hour such a creature spends traveling at high altitude counts as 2 hours for the purpose of determining how long that creature can travel (see “Travel Pace”).

Creatures can become acclimated to a high altitude by spending 30 days or more at this elevation. Creatures can’t become acclimated to elevations above 20,000 feet unless they are native to such environments.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/environmental_effects_high_altitude ` |
| ` implementation ` | ` dnd2024:rules_environmental_effects_high_altitude ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` altitude_feet_minimum ` | ` 10000 ` |
| ` travel_hours_multiplier ` | ` 2 ` |

#### Acclimation

<!-- splat:values path=splat_rule/parameters/acclimation -->

| Field | Value |
| --- | --- |
| ` days_minimum ` | ` 30 ` |
| ` above_altitude_feet ` | ` 20000 ` |
| ` above_altitude_requires_native ` | ` true ` |
| ` acclimated_exempt_from_multiplier ` | ` true ` |
