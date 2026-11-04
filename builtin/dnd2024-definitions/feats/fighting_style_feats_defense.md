---
splat_schema: 2
splat_id: f591f989-571f-5ebb-af12-81da3f512204
splat_kind: rule
splat_key: dnd2024:feats/fighting_style_feats_defense
splat_name: Defense
splat_origin: dnd2024:reference/pages/088
splat_sources:
- SRD 5.2.1, pp. 88, https://www.dndbeyond.com/srd
---

# Defense

## Source text

<!-- SRD page 88 -->

Fighting Style Feat (Prerequisite: Fighting Style Feature)

While you’re wearing Light, Medium, or Heavy armor, you gain a +1 bonus to Armor Class.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/fighting_style_feats_defense ` |
| ` implementation ` | ` dnd2024:feats_fighting_style_feats_defense ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feat_category ` | ` fighting_style ` |
| ` prerequisite_feature ` | ` fighting_style ` |
| ` when_armor_category ` | ` ["light", "medium", "heavy"] ` |
| ` armor_class_bonus ` | ` 1 ` |
