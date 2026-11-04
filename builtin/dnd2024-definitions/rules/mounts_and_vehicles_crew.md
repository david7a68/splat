---
splat_schema: 2
splat_id: 632b3c3b-7ae6-5085-885d-26575d315b13
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_crew
splat_name: Crew
splat_origin: dnd2024:reference/pages/101
splat_sources:
- SRD 5.2.1, pp. 101, https://www.dndbeyond.com/srd
---

# Crew

## Source text

<!-- SRD page 101 -->

A ship larger than a Keelboat or Rowboat needs a crew of skilled hirelings (see “Hirelings” later in “Equipment”) to function. The minimum number of skilled hirelings needed to crew a ship depends on the type of ship, as shown in the table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/mounts_and_vehicles_crew ` |
| ` implementation ` | ` dnd2024:rules_mounts_and_vehicles_crew ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` skilled_hirelings_required_except ` | ` ["keelboat", "rowboat"] ` |

#### Minimum crew

<!-- splat:values path=splat_rule/parameters/minimum_crew -->

| Field | Value |
| --- | --- |
| ` airship ` | ` 10 ` |
| ` galley ` | ` 80 ` |
| ` keelboat ` | ` 1 ` |
| ` longship ` | ` 40 ` |
| ` rowboat ` | ` 1 ` |
| ` sailing_ship ` | ` 20 ` |
| ` warship ` | ` 60 ` |
