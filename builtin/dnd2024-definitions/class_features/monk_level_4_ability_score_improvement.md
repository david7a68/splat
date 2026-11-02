---
splat_schema: 1
splat_id: aa86c9ec-b987-5ad4-bc95-7b76041a0846
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
---

# Level 4: Ability Score Improvement

## Source text

<!-- SRD page 51 -->

You gain the Ability Score Improvement feat (see “Feats”) or another feat of your choice for which you qualify. You gain this feature again at Monk levels 8, 12, and 16.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_4_ability_score_improvement ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_4_ability_score_improvement ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` grant_at_class_levels ` | ` [4, 8, 12, 16] ` |

#### Feat choice

<!-- splat:yaml path=splat_rule/parameters/feat_choice -->

```yaml
count_each: 1
default:
  key: dnd2024:feats/general_feats_ability_score_improvement
allowed_categories: any
must_meet_prerequisites: true
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` minimum_level ` | ` 4 ` |
