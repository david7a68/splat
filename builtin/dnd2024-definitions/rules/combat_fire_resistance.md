---
splat_schema: 2
splat_id: 587f2495-bf73-54a8-aec7-8bfc62010237
splat_kind: rule
splat_key: dnd2024:rules/combat_fire_resistance
splat_name: Fire Resistance
splat_origin: dnd2024:reference/pages/016
splat_sources:
- SRD 5.2.1, pp. 16, https://www.dndbeyond.com/srd
---

# Fire Resistance

## Source text

<!-- SRD page 16 -->

Anything underwater has Resistance to Fire damage (explained in “Damage and Healing”).

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_fire_resistance ` |
| ` implementation ` | ` dnd2024:rules_combat_fire_resistance ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` predicate ` | ` underwater ` |
| ` damage_resistance ` | ` ["fire"] ` |
| ` applies_to ` | ` ["creatures", "objects"] ` |
