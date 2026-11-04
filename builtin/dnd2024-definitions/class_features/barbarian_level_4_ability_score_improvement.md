---
splat_schema: 2
splat_id: e742b115-ac2a-5f2d-b2b4-77ebb417f155
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin: dnd2024:reference/pages/029
splat_sources:
- SRD 5.2.1, pp. 29, https://www.dndbeyond.com/srd
---

# Level 4: Ability Score Improvement

## Source text

<!-- SRD page 29 -->

You gain the Ability Score Improvement feat (see “Feats”) or another feat of your choice for which you qualify. You gain this feature again at Barbarian levels 8, 12, and 16.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_4_ability_score_improvement ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_4_ability_score_improvement ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` barbarian ` |
| ` grant_at_levels ` | ` [4, 8, 12, 16] ` |

#### Feat choice

<!-- splat:yaml path=splat_rule/parameters/feat_choice -->

```yaml
count_per_milestone: 1
category: any
require_prerequisites: true
default:
  key: dnd2024:feats/general_feats_ability_score_improvement
```
