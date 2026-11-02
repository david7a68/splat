---
splat_schema: 1
splat_id: 3d42e922-7a23-57de-82c0-8a9e62741a01
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_difficulty_class
splat_name: Difficulty Class
splat_origin:
  key: dnd2024:reference/pages/006
splat_sources:
- title: SRD 5.2.1
  locator: pp. 6
  url: https://www.dndbeyond.com/srd
---

# Difficulty Class

## Source text

<!-- SRD page 6 -->

The Difficulty Class of an ability check represents the task’s difficulty. The more difficult the task, the higher its DC. The rules provide DCs for certain checks, but the GM ultimately sets them. The Typical Difficulty Classes table presents a range of possible DCs for ability checks.

**Typical Difficulty Classes**

| Task Difficulty | DC | Task Difficulty | DC |
| --- | --- | --- | --- |
| Very easy | 5 | Very hard | 25 |
| Easy | 10 | Nearly impossible | 30 |
| Medium | 15 |  |  |
| Hard | 20 |  |  |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_difficulty_class ` |
| ` implementation ` | ` dnd2024:rules_d20_tests_difficulty_class ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` dc_authority ` | ` gm ` |
| ` specific_dc_priority ` | ` applicable_rule_subject_to_gm ` |

#### Suggested dc

<!-- splat:values path=splat_rule/parameters/suggested_dc -->

| Field | Value |
| --- | --- |
| ` very_easy ` | ` 5 ` |
| ` easy ` | ` 10 ` |
| ` medium ` | ` 15 ` |
| ` hard ` | ` 20 ` |
| ` very_hard ` | ` 25 ` |
| ` nearly_impossible ` | ` 30 ` |
