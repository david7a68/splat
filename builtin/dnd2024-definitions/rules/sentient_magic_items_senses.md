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

<!-- splat:rows path=splat_rule/parameters/results -->

| Row | roll | hearing_feet | standard_vision_feet | darkvision_feet |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 30 ` | ` 30 ` |  |
| ` 1 ` | ` 2 ` | ` 60 ` | ` 60 ` |  |
| ` 2 ` | ` 3 ` | ` 120 ` | ` 120 ` |  |
| ` 3 ` | ` 4 ` | ` 120 ` |  | ` 120 ` |
