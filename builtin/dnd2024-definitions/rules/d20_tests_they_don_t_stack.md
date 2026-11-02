---
splat_schema: 1
splat_id: 668748bb-2334-5900-958a-e36aa5809c31
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_they_don_t_stack
splat_name: They Don’t Stack
splat_origin:
  key: dnd2024:reference/pages/008
splat_sources:
- title: SRD 5.2.1
  locator: pp. 8
  url: https://www.dndbeyond.com/srd
---

# They Don’t Stack

## Source text

<!-- SRD page 8 -->

If multiple situations affect a roll and they all grant Advantage on it, you still roll only two d20s. Similarly, if multiple situations impose Disadvantage on a roll, you roll only two d20s.

If circumstances cause a roll to have both Advantage and Disadvantage, the roll has neither of them, and you roll one d20. This is true even if multiple circumstances impose Disadvantage and only one grants Advantage or vice versa. In such a situation, you have neither Advantage nor Disadvantage.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_they_don_t_stack ` |
| ` implementation ` | ` dnd2024:rules_d20_tests_they_don_t_stack ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` roll ` | ` d20 ` |

#### Cases

<!-- splat:table
path: splat_rule/parameters/cases
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Advantage sources min:
    path: advantage_sources_min
    type: integer
    optional: true
  Disadvantage sources:
    path: disadvantage_sources
    type: integer
    optional: true
  Dice:
    path: dice
    type: integer
  Select:
    path: select
    type: symbol
    optional: true
  Advantage sources:
    path: advantage_sources
    type: integer
    optional: true
  Disadvantage sources min:
    path: disadvantage_sources_min
    type: integer
    optional: true
  Advantage:
    path: advantage
    type: boolean
    optional: true
  Disadvantage:
    path: disadvantage
    type: boolean
    optional: true
-->

| Key | Advantage sources min | Disadvantage sources | Dice | Select | Advantage sources | Disadvantage sources min | Advantage | Disadvantage |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 0 ` | ` 2 ` | ` Highest ` |  |  |  |  |
| ` row_2 ` |  |  | ` 2 ` | ` Lowest ` | ` 0 ` | ` 1 ` |  |  |
| ` row_3 ` | ` 1 ` |  | ` 1 ` |  |  | ` 1 ` | false | false |
| ` row_4 ` |  | ` 0 ` | ` 1 ` |  | ` 0 ` |  |  |  |
