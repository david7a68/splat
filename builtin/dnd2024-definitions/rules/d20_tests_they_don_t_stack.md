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

If multiple situations affect a roll and they all grant\
Advantage on it, you still roll only two d20s. Simi-\
larly, if multiple situations impose Disadvantage on\
a roll, you roll only two d20s.\
If circumstances cause a roll to have both Advan-\
tage and Disadvantage, the roll has neither of them,\
and you roll one d20. This is true even if multiple\
circumstances impose Disadvantage and only one\
grants Advantage or vice versa. In such a situation,\
you have neither Advantage nor Disadvantage.\

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

<!-- splat:rows path=splat_rule/parameters/cases -->

| Row | advantage_sources_min | disadvantage_sources | dice | select | advantage_sources | disadvantage_sources_min | advantage | disadvantage |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 0 ` | ` 2 ` | ` highest ` |  |  |  |  |
| ` 1 ` |  |  | ` 2 ` | ` lowest ` | ` 0 ` | ` 1 ` |  |  |
| ` 2 ` | ` 1 ` |  | ` 1 ` |  |  | ` 1 ` | ` false ` | ` false ` |
| ` 3 ` |  | ` 0 ` | ` 1 ` |  | ` 0 ` |  |  |  |
