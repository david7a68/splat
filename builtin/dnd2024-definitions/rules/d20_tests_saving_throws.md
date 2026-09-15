---
splat_schema: 1
splat_id: f9acbe43-d563-53b5-a159-5e8a7268b7fc
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_saving_throws
splat_name: Saving Throws
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
---

# Saving Throws

## Source text

<!-- SRD page 7 -->

A saving throw—also called a save—represents an\
attempt to evade or resist a threat, such as a fiery\
explosion, a blast of poisonous gas, or a spell trying\
to invade your mind. You don’t normally choose to\
make a save; you must make one because your char-\
acter or a monster (if you’re the GM) is at risk. A\
save’s result is detailed in the effect that caused it.\
If you don’t want to resist the effect, you can\
choose to fail the save without rolling.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_saving_throws ` |
| ` implementation ` | ` dnd2024:rule_d20_tests_saving_throws ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` threat_requires_save ` |
| ` result_source ` | ` triggering_effect ` |

#### Voluntary failure

<!-- splat:values path=splat_rule/parameters/voluntary_failure -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |
| ` roll ` | ` false ` |
