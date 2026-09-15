---
splat_schema: 1
splat_id: 0f4a167b-d8b1-5a45-895b-1a23bae028e3
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_adjust_ability_scores
splat_name: Adjust Ability Scores
splat_origin:
  key: dnd2024:reference/pages/021
splat_sources:
- title: SRD 5.2.1
  locator: pp. 21
  url: https://www.dndbeyond.com/srd
---

# Adjust Ability Scores

## Source text

<!-- SRD page 21 -->

After assigning your ability scores, adjust them ac-\
cording to your background. Your background lists\
three abilities; increase one of those scores by 2 and\
a different one by 1, or increase all three by 1. None\
of these increases can raise a score above 20.\
Some players like to increase their class’s primary\
ability, while others prefer to increase a low score.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/create_your_character_adjust_ability_scores ` |
| ` implementation ` | ` dnd2024:rule_create_your_character_adjust_ability_scores ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ability_choices ` | ` background_three_abilities ` |
| ` score_maximum ` | ` 20 ` |
| ` timing ` | ` after_base_assignment ` |

#### Options

<!-- splat:rows path=splat_rule/parameters/options -->

| Row | increases | distinct_abilities |
| --- | --- | --- |
| ` 0 ` | ` [2, 1] ` | ` true ` |
| ` 1 ` | ` [1, 1, 1] ` | ` true ` |
