---
splat_schema: 1
splat_id: 5da0359a-5e88-508c-a5a6-e35a5944b18b
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_ability_checks
splat_name: Ability Checks
splat_origin:
  key: dnd2024:reference/pages/006
splat_sources:
- title: SRD 5.2.1
  locator: pp. 6
  url: https://www.dndbeyond.com/srd
---

# Ability Checks

## Source text

<!-- SRD page 6 -->

An ability check represents a creature using talent\
and training to try to overcome a challenge, such as\
forcing open a stuck door, picking a lock, entertain-\
ing a crowd, or deciphering a cipher. The GM and the\
rules often call for an ability check when a creature\
attempts something other than an attack that has a\
chance of meaningful failure. When the outcome is\
uncertain and narratively interesting, the dice de-\
termine the result.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_ability_checks ` |
| ` implementation ` | ` dnd2024:rules_d20_tests_ability_checks ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` check ` | ` ability_check ` |
| ` call_authority ` | ` ["gm", "applicable_rule"] ` |
| ` ability_and_dc ` | ` gm_or_applicable_rule ` |

#### Eligibility

<!-- splat:values path=splat_rule/parameters/eligibility -->

| Field | Value |
| --- | --- |
| ` attack ` | ` false ` |
| ` meaningful_failure_possible ` | ` true ` |
| ` outcome_uncertain ` | ` true ` |
| ` narrative_interest ` | ` true ` |
