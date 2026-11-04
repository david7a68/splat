---
splat_schema: 2
splat_id: b95c4da6-7550-576c-a3f2-5b00548dd0cf
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_generate_your_scores
splat_name: Generate Your Scores
splat_origin: dnd2024:reference/pages/021
splat_sources:
- SRD 5.2.1, pp. 21, https://www.dndbeyond.com/srd
---

# Generate Your Scores

## Source text

<!-- SRD page 21 -->

Determine your ability scores by using one of the following three methods. Your GM might prefer you to use a particular one.

Standard Array. Use the following six scores for your abilities: 15, 14, 13, 12, 10, 8.

Random Generation. Roll four d6s and record the total of the highest three dice. Do this five more times, so you have six numbers.

Point Cost. You have 27 points to spend on your ability scores. The cost of each score is shown on the Ability Score Point Costs table. For example, a score of 14 costs 7 of your 27 points.

**Ability Score Point Costs**

| Score | Cost | Score | Cost |
| --- | --- | --- | --- |
| 8 | 0 | 12 | 4 |
| 9 | 1 | 13 | 5 |
| 10 | 2 | 14 | 7 |
| 11 | 3 | 15 | 9 |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/create_your_character_generate_your_scores ` |
| ` implementation ` | ` dnd2024:rules_create_your_character_generate_your_scores ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` standard_array ` | ` [15, 14, 13, 12, 10, 8] ` |

#### Method choice

<!-- splat:values path=splat_rule/parameters/method_choice -->

| Field | Value |
| --- | --- |
| ` authority ` | ` gm_or_player ` |
| ` options ` | ` ["standard_array", "random_generation", "point_cost"] ` |

#### Random generation

<!-- splat:yaml path=splat_rule/parameters/random_generation -->

```yaml
repeat: 6
roll:
  count: 4
  sides: 6
  add: 0
keep_highest: 3
aggregate: sum
```

#### Point cost

<!-- splat:yaml path=splat_rule/parameters/point_cost -->

```yaml
budget: 27
score_costs:
  '8': 0
  '9': 1
  '10': 2
  '11': 3
  '12': 4
  '13': 5
  '14': 7
  '15': 9
score_count: 6
```
