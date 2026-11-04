---
splat_schema: 2
splat_id: 4080e354-b8a9-5ebf-9da8-b9b1499a1427
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin: dnd2024:reference/pages/037
splat_sources:
- SRD 5.2.1, pp. 37, https://www.dndbeyond.com/srd
---

# Level 4: Ability Score Improvement

## Source text

<!-- SRD page 37 -->

You gain the Ability Score Improvement feat (see “Feats”) or another feat of your choice for which you qualify. You gain this feature again at Cleric levels 8, 12, and 16.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/cleric_level_4_ability_score_improvement ` |
| ` implementation ` | ` dnd2024:class_features_cleric_level_4_ability_score_improvement ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` cleric ` |
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
| ` class ` | ` cleric ` |
| ` minimum_level ` | ` 4 ` |
