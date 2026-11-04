---
splat_schema: 2
splat_id: 433830ab-3fc4-5e39-84bb-882a97aa6945
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin: dnd2024:reference/pages/048
splat_sources:
- SRD 5.2.1, pp. 48, https://www.dndbeyond.com/srd
---

# Level 4: Ability Score Improvement

## Source text

<!-- SRD page 48 -->

You gain the Ability Score Improvement feat (see “Feats”) or another feat of your choice for which you qualify. You gain this feature again at Fighter levels 6, 8, 12, 14, and 16.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |
| ` implementation ` | ` dnd2024:class_features_fighter_level_4_ability_score_improvement ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` fighter ` |
| ` grant_at_class_levels ` | ` [4, 6, 8, 12, 14, 16] ` |

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
| ` class ` | ` fighter ` |
| ` minimum_level ` | ` 4 ` |
