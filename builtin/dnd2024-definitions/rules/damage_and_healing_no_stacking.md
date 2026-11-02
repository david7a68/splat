---
splat_schema: 1
splat_id: fca817d4-2b03-5651-a738-ef18537f23b7
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_no_stacking
splat_name: No Stacking
splat_origin:
  key: dnd2024:reference/pages/017
splat_sources:
- title: SRD 5.2.1
  locator: pp. 17
  url: https://www.dndbeyond.com/srd
---

# No Stacking

## Source text

<!-- SRD page 17 -->

Multiple instances of Resistance or Vulnerability that affect the same damage type count as only one instance. For example, if you have Resistance to Necrotic damage as well as Resistance to all damage, Necrotic damage is reduced by half against you.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_no_stacking ` |
| ` implementation ` | ` dnd2024:rules_damage_and_healing_no_stacking ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` stacking_key ` | ` damage_type ` |
| ` resistance_applications_max ` | ` 1 ` |
| ` vulnerability_applications_max ` | ` 1 ` |
| ` broad_and_specific_overlap ` | ` one_application ` |
