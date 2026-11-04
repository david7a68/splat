---
splat_schema: 2
splat_id: 7383c4de-2cb9-53e1-a6ed-aac188f9b9cd
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_20_body_and_mind
splat_name: 'Level 20: Body and Mind'
splat_origin: dnd2024:reference/pages/052
splat_sources:
- SRD 5.2.1, pp. 52, https://www.dndbeyond.com/srd
---

# Level 20: Body and Mind

## Source text

<!-- SRD page 52 -->

You have developed your body and mind to new heights. Your Dexterity and Wisdom scores increase by 4, to a maximum of 25.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_20_body_and_mind ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_20_body_and_mind ` |

### Parameters

#### Ability increases

<!-- splat:table
path: splat_rule/parameters/ability_increases
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Ability:
    path: ability
    type: symbol
  Amount:
    path: amount
    type: integer
  Maximum:
    path: maximum
    type: integer
-->

| Key | Ability | Amount | Maximum |
| --- | --- | --- | --- |
| ` row_1 ` | ` Dexterity ` | ` 4 ` | ` 25 ` |
| ` row_2 ` | ` Wisdom ` | ` 4 ` | ` 25 ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` minimum_level ` | ` 20 ` |
