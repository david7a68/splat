---
splat_schema: 1
splat_id: 7ee21a0e-cede-5627-82e9-5c97418aff3d
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_order_of_application
splat_name: Order of Application
splat_origin:
  key: dnd2024:reference/pages/017
splat_sources:
- title: SRD 5.2.1
  locator: pp. 17
  url: https://www.dndbeyond.com/srd
---

# Order of Application

## Source text

<!-- SRD page 17 -->

Modifiers to damage are applied in the following\
order: adjustments such as bonuses, penalties, or\
multipliers are applied first; Resistance is applied\
second; and Vulnerability is applied third.\
For example, a creature has Resistance to all dam-\
age and Vulnerability to Fire damage, and it’s within\
a magical aura that reduces all damage by 5. If it\
takes 28 Fire damage, the damage is first reduced\
by 5 (to 23), then halved for the creature’s Resis-\
tance (and rounded down to 11), then doubled for\
its Vulnerability (to 22).\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_order_of_application ` |
| ` implementation ` | ` dnd2024:rule_damage_and_healing_order_of_application ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ordered_operations ` | ` ["bonuses_penalties_multipliers", "resistance", "vulnerability"] ` |

#### Resistance

<!-- splat:values path=splat_rule/parameters/resistance -->

| Field | Value |
| --- | --- |
| ` divide ` | ` 2 ` |
| ` rounding ` | ` floor ` |

#### Vulnerability

<!-- splat:values path=splat_rule/parameters/vulnerability -->

| Field | Value |
| --- | --- |
| ` multiply ` | ` 2 ` |

#### Example

<!-- splat:values path=splat_rule/parameters/example -->

| Field | Value |
| --- | --- |
| ` initial ` | ` 28 ` |
| ` flat_adjustment ` | ` -5 ` |
| ` after_adjustment ` | ` 23 ` |
| ` after_resistance ` | ` 11 ` |
| ` after_vulnerability ` | ` 22 ` |
