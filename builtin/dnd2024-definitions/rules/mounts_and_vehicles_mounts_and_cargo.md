---
splat_schema: 1
splat_id: 8897075f-89e4-5187-bf75-0a6426ea2771
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_mounts_and_cargo
splat_name: Mounts and Cargo
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
---

# Mounts and Cargo

## Source text

<!-- SRD page 100 -->

An animal pulling a carriage, cart, chariot, sled, or wagon can move weight up to five times its base carrying capacity, including the weight of the vehicle. If multiple animals pull the same vehicle, add their carrying capacities together.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/mounts_and_vehicles_mounts_and_cargo ` |
| ` implementation ` | ` dnd2024:rule_mounts_and_vehicles_mounts_and_cargo ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` vehicle_kinds ` | ` ["carriage", "cart", "chariot", "sled", "wagon"] ` |
| ` vehicle_weight_counts ` | ` true ` |

#### Maximum pulled weight

<!-- splat:yaml path=splat_rule/parameters/maximum_pulled_weight -->

```yaml
op: multiply
args:
- 5
- op: sum
  args:
  - variable: pulling_animals_base_carrying_capacities
```
