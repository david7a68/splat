---
splat_schema: 2
splat_id: 2067f60b-da8d-51ff-a1cb-2a7483cb44bd
splat_kind: rule
splat_key: dnd2024:class_features/champion_level_3_improved_critical
splat_name: 'Level 3: Improved Critical'
splat_origin: dnd2024:reference/pages/049
splat_sources:
- SRD 5.2.1, pp. 49, https://www.dndbeyond.com/srd
---

# Level 3: Improved Critical

## Source text

<!-- SRD page 49 -->

Your attack rolls with weapons and Unarmed Strikes can score a Critical Hit on a roll of 19 or 20 on the d20.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/champion_level_3_improved_critical ` |
| ` implementation ` | ` dnd2024:class_features_champion_level_3_improved_critical ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_sources ` | ` ["weapon", "unarmed_strike"] ` |
| ` critical_d20_faces ` | ` [19, 20] ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` fighter ` |
| ` minimum_level ` | ` 3 ` |
