---
splat_schema: 1
splat_id: d4964a32-fc12-548d-94b6-bf58ce7dd303
splat_kind: rule
splat_key: dnd2024:rules/character_backgrounds_ability_scores
splat_name: Ability Scores
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
---

# Ability Scores

## Source text

<!-- SRD page 83 -->

A background lists three of your character’s ability\
scores. Increase one by 2 and another one by 1, or\
increase all three by 1. None of these increases can\
raise a score above 20.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/character_backgrounds_ability_scores ` |
| ` implementation ` | ` dnd2024:rules_character_backgrounds_ability_scores ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` eligible_abilities ` | ` background_three_abilities ` |
| ` permutation_allowed ` | ` true ` |
| ` ability_score_max_after_increase ` | ` 20 ` |

#### Allocation choices

<!-- splat:yaml path=splat_rule/parameters/allocation_choices -->

```yaml
- - 2
  - 1
  - 0
- - 1
  - 1
  - 1
```
