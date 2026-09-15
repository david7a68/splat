---
splat_schema: 1
splat_id: 4b98a7da-5bb9-5961-83d8-20881041149c
splat_kind: rule
splat_key: dnd2024:rules/lifestyle_expenses_wretched_free
splat_name: Wretched (Free)
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
---

# Wretched (Free)

## Source text

<!-- SRD page 101 -->

You survive via chance and charity. You’re often\
exposed to natural dangers as a result of sleeping\
outside.\
AC HP Damage Threshold Cost\
13 300 — 40,000 GP\
15 500 20 30,000 GP\
15 100 10 3,000 GP\
15 300 15 10,000 GP\
11 50 — 50 GP\
15 300 15 10,000 GP\
15 500 20 25,000 GP\

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

<!-- splat:rows path=splat_rule/parameters/displaced_vehicle_table -->

| Row | id | speed_mph | crew | passengers | cargo_tons | ac | hp | damage_threshold | cost_gp |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` airship ` | ` 8 ` | ` 10 ` | ` 20 ` | ` 1 ` | ` 13 ` | ` 300 ` | ` null ` | ` 40000 ` |
| ` 1 ` | ` galley ` | ` 4 ` | ` 80 ` | ` 0 ` | ` 150 ` | ` 15 ` | ` 500 ` | ` 20 ` | ` 30000 ` |
| ` 2 ` | ` keelboat ` | ` 1 ` | ` 1 ` | ` 6 ` | ` 0.5 ` | ` 15 ` | ` 100 ` | ` 10 ` | ` 3000 ` |
| ` 3 ` | ` longship ` | ` 3 ` | ` 40 ` | ` 150 ` | ` 10 ` | ` 15 ` | ` 300 ` | ` 15 ` | ` 10000 ` |
| ` 4 ` | ` rowboat ` | ` 1.5 ` | ` 1 ` | ` 3 ` | ` 0 ` | ` 11 ` | ` 50 ` | ` null ` | ` 50 ` |
| ` 5 ` | ` sailing_ship ` | ` 2 ` | ` 20 ` | ` 20 ` | ` 100 ` | ` 15 ` | ` 300 ` | ` 15 ` | ` 10000 ` |
| ` 6 ` | ` warship ` | ` 2.5 ` | ` 60 ` | ` 60 ` | ` 200 ` | ` 15 ` | ` 500 ` | ` 20 ` | ` 25000 ` |
