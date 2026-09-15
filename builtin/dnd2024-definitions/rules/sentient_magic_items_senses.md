---
splat_schema: 1
splat_id: 3d15486b-5227-541e-9b4f-8ccd165ba650
splat_kind: rule
splat_key: dnd2024:rules/sentient_magic_items_senses
splat_name: Senses
splat_origin:
  key: dnd2024:reference/pages/207
splat_sources:
- title: SRD 5.2.1
  locator: pp. 207
  url: https://www.dndbeyond.com/srd
---

# Senses

## Source text

<!-- SRD page 207 -->

A sentient item can perceive its surroundings out to\
a limited range. You can choose its senses or roll on\
the Sentient Item’s Senses table.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sentient_magic_items_senses ` |
| ` implementation ` | ` dnd2024:rule_sentient_magic_items_senses ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` generation_choice ` | ` ["gm_select", "random"] ` |

#### Roll

<!-- splat:yaml path=splat_rule/parameters/roll -->

```yaml
dice:
  count: 1
  sides: 4
```

#### Results

<!-- splat:table
path: splat_rule/parameters/results
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Roll:
    path: roll
    type: integer
  Hearing feet:
    path: hearing_feet
    type: integer
  Standard vision feet:
    path: standard_vision_feet
    type: integer
    optional: true
  Darkvision feet:
    path: darkvision_feet
    type: integer
    optional: true
-->

| Key | Roll | Hearing feet | Standard vision feet | Darkvision feet |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 30 ` | ` 30 ` |  |
| ` row_2 ` | ` 2 ` | ` 60 ` | ` 60 ` |  |
| ` row_3 ` | ` 3 ` | ` 120 ` | ` 120 ` |  |
| ` row_4 ` | ` 4 ` | ` 120 ` |  | ` 120 ` |
