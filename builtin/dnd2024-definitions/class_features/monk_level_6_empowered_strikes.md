---
splat_schema: 1
splat_id: 581fcd42-654d-51b2-9fa8-e5e5624a67cc
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_6_empowered_strikes
splat_name: 'Level 6: Empowered Strikes'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
---

# Level 6: Empowered Strikes

## Source text

<!-- SRD page 51 -->

Whenever you deal damage with your Unarmed Strike, it can deal your choice of Force damage or its normal damage type.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_6_empowered_strikes ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_6_empowered_strikes ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` unarmed_strike.damage ` |
| ` damage_type_choice ` | ` ["force", "normal"] ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` minimum_level ` | ` 6 ` |
