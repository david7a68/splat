---
splat_schema: 1
splat_id: 0f27f279-f32f-5eea-8c1d-292d9f6b9101
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_damage_threshold
splat_name: Damage Threshold
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
---

# Damage Threshold

## Source text

<!-- SRD page 101 -->

If a vehicle has a damage threshold (see “Rules Glossary”), it’s noted in the table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/mounts_and_vehicles_damage_threshold ` |
| ` implementation ` | ` dnd2024:rules_mounts_and_vehicles_damage_threshold ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` threshold_source ` | ` vehicle_table ` |

#### Threshold rule

<!-- splat:values path=splat_rule/parameters/threshold_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/damage_threshold ` |

#### Vehicle thresholds

<!-- splat:values path=splat_rule/parameters/vehicle_thresholds -->

| Field | Value |
| --- | --- |
| ` airship ` | ` null ` |
| ` galley ` | ` 20 ` |
| ` keelboat ` | ` 10 ` |
| ` longship ` | ` 15 ` |
| ` rowboat ` | ` null ` |
| ` sailing_ship ` | ` 15 ` |
| ` warship ` | ` 20 ` |
