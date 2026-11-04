---
splat_schema: 2
splat_id: 8fe3fb9f-0729-5cca-8edc-1205f45e0457
splat_kind: rule
splat_key: dnd2024:rules/proficiency_the_bonus_doesn_t_stack
splat_name: The Bonus Doesn’t Stack
splat_origin: dnd2024:reference/pages/008
splat_sources:
- SRD 5.2.1, pp. 8, https://www.dndbeyond.com/srd
---

# The Bonus Doesn’t Stack

## Source text

<!-- SRD page 8 -->

Your Proficiency Bonus can’t be added to a die roll or another number more than once. For example, if a rule allows you to make a Charisma (Deception or Persuasion) check, you add your Proficiency Bonus if you’re proficient in either skill, but you don’t add it twice if you’re proficient in both skills.

Occasionally, a Proficiency Bonus might be multiplied or divided (doubled or halved, for example) before being added. For example, the Expertise feature (see “Rules Glossary”) doubles the Proficiency Bonus for certain ability checks. Whenever the bonus is used, it can be multiplied only once and divided only once.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/proficiency_the_bonus_doesn_t_stack ` |
| ` implementation ` | ` dnd2024:rules_proficiency_the_bonus_doesn_t_stack ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` proficiency_additions_max ` | ` 1 ` |
| ` proficiency_multiplications_max ` | ` 1 ` |
| ` proficiency_divisions_max ` | ` 1 ` |
| ` alternative_skill_proficiencies ` | ` any_qualifying_skill ` |
| ` multiple_qualifying_proficiencies ` | ` one_bonus ` |
