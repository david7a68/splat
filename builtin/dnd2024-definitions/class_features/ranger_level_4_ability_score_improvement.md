---
splat_schema: 1
splat_id: 041d2982-f0cd-504e-85bd-9ad8aa43016e
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
---

# Level 4: Ability Score Improvement

## Source text

<!-- SRD page 59 -->

You gain the Ability Score Improvement feat (see “Feats”) or another feat of your choice for which you qualify. You gain this feature again at Ranger levels 8, 12, and 16.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/ranger_level_4_ability_score_improvement ` |
| ` implementation ` | ` dnd2024:class_features_ranger_level_4_ability_score_improvement ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |
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
