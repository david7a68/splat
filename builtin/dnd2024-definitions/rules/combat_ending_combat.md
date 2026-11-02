---
splat_schema: 1
splat_id: 985f5870-52a3-5c24-beca-6d3d6c8111d2
splat_kind: rule
splat_key: dnd2024:rules/combat_ending_combat
splat_name: Ending Combat
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14
  url: https://www.dndbeyond.com/srd
---

# Ending Combat

## Source text

<!-- SRD page 14 -->

Combat ends when one side or the other is defeated, which can mean the creatures are killed or knocked out or have surrendered or fled. Combat can also end when both sides agree to end it.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_ending_combat ` |
| ` implementation ` | ` dnd2024:rules_combat_ending_combat ` |

### Parameters

#### End conditions

<!-- splat:values path=splat_rule/parameters/end_conditions -->

| Field | Value |
| --- | --- |
| ` any ` | ` ["side_killed", "side_unconscious", "side_surrendered", "side_fled", "mutual_agreement"] ` |
