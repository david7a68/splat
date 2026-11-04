---
splat_schema: 2
splat_id: 3098f39d-b34d-58bd-a335-7d92cebdbfef
splat_kind: rule
splat_key: dnd2024:feats/general_feats_ability_score_improvement
splat_name: Ability Score Improvement
splat_origin: dnd2024:reference/pages/087
splat_sources:
- SRD 5.2.1, pp. 87, https://www.dndbeyond.com/srd
---

# Ability Score Improvement

## Source text

<!-- SRD page 87 -->

General Feat (Prerequisite: Level 4+)

Increase one ability score of your choice by 2, or increase two ability scores of your choice by 1. This feat can’t increase an ability score above 20.

Repeatable. You can take this feat more than once.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/general_feats_ability_score_improvement ` |
| ` implementation ` | ` dnd2024:feats_general_feats_ability_score_improvement ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feat_category ` | ` general ` |
| ` repeatable ` | ` true ` |

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` minimum_character_level ` | ` 4 ` |

#### Ability increase

<!-- splat:yaml path=splat_rule/parameters/ability_increase -->

```yaml
patterns:
- - 2
- - 1
  - 1
distinct_abilities: true
eligible_abilities: all
maximum_score: 20
```
