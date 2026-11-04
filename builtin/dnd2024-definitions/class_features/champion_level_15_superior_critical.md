---
splat_schema: 2
splat_id: dba9cc10-b4d9-5a2e-b9fb-2b1549c3f37f
splat_kind: rule
splat_key: dnd2024:class_features/champion_level_15_superior_critical
splat_name: 'Level 15: Superior Critical'
splat_origin: dnd2024:reference/pages/049
splat_sources:
- SRD 5.2.1, pp. 49, https://www.dndbeyond.com/srd
---

# Level 15: Superior Critical

## Source text

<!-- SRD page 49 -->

Your attack rolls with weapons and Unarmed Strikes can now score a Critical Hit on a roll of 18–20 on the d20.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/champion_level_15_superior_critical ` |
| ` implementation ` | ` dnd2024:class_features_champion_level_15_superior_critical ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_sources ` | ` ["weapon", "unarmed_strike"] ` |
| ` critical_d20_faces ` | ` [18, 19, 20] ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` fighter ` |
| ` minimum_level ` | ` 15 ` |
