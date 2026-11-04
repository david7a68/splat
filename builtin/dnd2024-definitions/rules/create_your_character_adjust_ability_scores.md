---
splat_schema: 2
splat_id: 0f4a167b-d8b1-5a45-895b-1a23bae028e3
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_adjust_ability_scores
splat_name: Adjust Ability Scores
splat_origin: dnd2024:reference/pages/021
splat_sources:
- SRD 5.2.1, pp. 21, https://www.dndbeyond.com/srd
---

# Adjust Ability Scores

## Source text

<!-- SRD page 21 -->

After assigning your ability scores, adjust them according to your background. Your background lists three abilities; increase one of those scores by 2 and a different one by 1, or increase all three by 1. None of these increases can raise a score above 20.

Some players like to increase their class’s primary ability, while others prefer to increase a low score.

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

<!-- splat:table
path: splat_rule/parameters/options
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Increases:
    path: increases
    type: yaml
  Distinct abilities:
    path: distinct_abilities
    type: boolean
-->

| Key | Increases | Distinct abilities |
| --- | --- | --- |
| ` row_1 ` | ` [2, 1] ` | true |
| ` row_2 ` | ` [1, 1, 1] ` | true |
