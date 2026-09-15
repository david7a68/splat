---
splat_schema: 1
splat_id: e32900e0-05e5-50fd-a5e7-88b8373756fb
splat_kind: rule
splat_key: dnd2024:rules/the_six_abilities_ability_scores
splat_name: Ability Scores
splat_origin:
  key: dnd2024:reference/pages/005
splat_sources:
- title: SRD 5.2.1
  locator: pp. 5
  url: https://www.dndbeyond.com/srd
---

# Ability Scores

## Source text

<!-- SRD page 5 -->

Each ability has a score from 1 to 20, although some\
monsters have a score as high as 30. The score\
represents the magnitude of an ability. The Ability\
Scores table summarizes what the scores mean.\
Ability Scores\
Score Meaning\
1 This is the lowest a score can normally go.\
If an effect reduces a score to 0, that effect\
explains what happens.\
2–9 This represents a weak capability.\
10–11 This represents the human average.\
12–19 This represents a strong capability.\
20 This is the highest an adventurer’s score can\
go unless a feature says otherwise.\
21–29 This represents an extraordinary capability.\
30 This is the highest a score can go.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/the_six_abilities_ability_scores ` |
| ` implementation ` | ` dnd2024:rules_the_six_abilities_ability_scores ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` normal_min ` | ` 1 ` |
| ` adventurer_max ` | ` 20 ` |
| ` absolute_max ` | ` 30 ` |
| ` feature_can_override_adventurer_max ` | ` true ` |
| ` score_zero_consequence ` | ` reducing_effect_definition ` |

#### Score bands

<!-- splat:table
path: splat_rule/parameters/score_bands
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
columns:
  Key:
    type: key
  Roll:
    path: ''
    type: range
  Category:
    path: category
    type: symbol
-->

| Key | Roll | Category |
| --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` Normal minimum ` |
| ` row_2 ` | ` 2–9 ` | ` Weak ` |
| ` row_3 ` | ` 10–11 ` | ` Human average ` |
| ` row_4 ` | ` 12–19 ` | ` Strong ` |
| ` row_5 ` | ` 20 ` | ` Adventurer maximum ` |
| ` row_6 ` | ` 21–29 ` | ` Extraordinary ` |
| ` row_7 ` | ` 30 ` | ` Absolute maximum ` |
