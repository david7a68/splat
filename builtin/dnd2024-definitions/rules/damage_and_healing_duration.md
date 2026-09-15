---
splat_schema: 1
splat_id: 49bb4aef-a5f3-596e-bad1-fac089164d71
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_duration
splat_name: Duration
splat_origin:
  key: dnd2024:reference/pages/018
splat_sources:
- title: SRD 5.2.1
  locator: pp. 18
  url: https://www.dndbeyond.com/srd
---

# Duration

## Source text

<!-- SRD page 18 -->

Temporary Hit Points last until they’re depleted or\
you finish a Long Rest (see “Rules Glossary”).\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_duration ` |
| ` implementation ` | ` dnd2024:rules_damage_and_healing_duration ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` resource ` | ` temporary_hit_points ` |
| ` expires ` | ` ["depleted", "long_rest_finished"] ` |
