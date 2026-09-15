---
splat_schema: 1
splat_id: 174efe28-e206-514c-b0c9-ead3d93990bc
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_passengers
splat_name: Passengers
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
---

# Passengers

## Source text

<!-- SRD page 101 -->

The table lists the number of Small and Medium\
passengers the ship can accommodate using ham-\
mocks. A ship outfitted with private accommoda-\
tions can carry one-fifth as many passengers. A pas-\
senger usually pays 5 SP per day for a hammock, but\
prices can vary from ship to ship. A small private\
cabin usually costs 2 GP per day.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/mounts_and_vehicles_passengers ` |
| ` implementation ` | ` dnd2024:rule_mounts_and_vehicles_passengers ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passenger_sizes ` | ` ["small", "medium"] ` |
| ` hammock_capacity ` | ` vehicle_table_passengers ` |

#### Private capacity

<!-- splat:yaml path=splat_rule/parameters/private_capacity -->

```yaml
op: floor
args:
- op: divide
  args:
  - variable: vehicle_table_passengers
  - 5
```

#### Typical daily prices

<!-- splat:yaml path=splat_rule/parameters/typical_daily_prices -->

```yaml
hammock:
  currency: sp
  amount: 5
small_private_cabin:
  currency: gp
  amount: 2
```

#### Price override

<!-- splat:values path=splat_rule/parameters/price_override -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` scope ` | ` ship ` |
