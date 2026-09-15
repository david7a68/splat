---
splat_schema: 1
splat_id: 59ec6b04-1e2e-5008-b0d9-b9324dd93e0e
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_ship_repair
splat_name: Ship Repair
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
---

# Ship Repair

## Source text

<!-- SRD page 101 -->

Repairs to a damaged ship can be made while the\
vessel is berthed. Repairing 1 Hit Point of damage\
requires 1 day and costs 20 GP for materials and la-\
bor. If the repairs are made in a location where sup-\
plies and skilled labor are abundant, such as a city\
shipyard, the repair time and cost are halved.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/mounts_and_vehicles_ship_repair ` |
| ` implementation ` | ` dnd2024:rules_mounts_and_vehicles_ship_repair ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` requires_berthed ` | ` true ` |

#### Per hp

<!-- splat:values path=splat_rule/parameters/per_hp -->

| Field | Value |
| --- | --- |
| ` days ` | ` 1 ` |
| ` cost_gp ` | ` 20 ` |

#### Abundant supplies and skilled labor

<!-- splat:values path=splat_rule/parameters/abundant_supplies_and_skilled_labor -->

| Field | Value |
| --- | --- |
| ` time_multiplier ` | ` 0.5 ` |
| ` cost_multiplier ` | ` 0.5 ` |
