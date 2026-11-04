---
splat_schema: 2
splat_id: 86f03b35-d50e-5368-bcab-f83001970d18
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_20_primal_champion
splat_name: 'Level 20: Primal Champion'
splat_origin: dnd2024:reference/pages/030
splat_sources:
- SRD 5.2.1, pp. 30, https://www.dndbeyond.com/srd
---

# Level 20: Primal Champion

## Source text

<!-- SRD page 30 -->

You embody primal power. Your Strength and Constitution scores increase by 4, to a maximum of 25.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_20_primal_champion ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_20_primal_champion ` |

### Parameters

#### Ability score increases

<!-- splat:table
path: splat_rule/parameters/ability_score_increases
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
| ` row_1 ` | ` Strength ` | ` 4 ` | ` 25 ` |
| ` row_2 ` | ` Constitution ` | ` 4 ` | ` 25 ` |
