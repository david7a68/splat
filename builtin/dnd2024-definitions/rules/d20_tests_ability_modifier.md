---
splat_schema: 1
splat_id: 2dd56827-5b78-584b-85aa-63c9d4405d47
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_ability_modifier
splat_name: Ability Modifier
splat_origin:
  key: dnd2024:reference/pages/006
splat_sources:
- title: SRD 5.2.1
  locator: pp. 6
  url: https://www.dndbeyond.com/srd
---

# Ability Modifier

## Source text

<!-- SRD page 6 -->

An ability check is named for the ability modifier it\
uses: a Strength check, an Intelligence check, and so\
on. Different ability checks are called for in differ-\
ent situations, depending on which ability is most\
relevant. See the Ability Check Examples table for\
examples of each check’s use.\
Ability Check Examples\
Ability Make a Check To …\
Strength Lift, push, pull, or break something\
Dexterity Move nimbly, quickly, or quietly\
Constitution Push your body beyond normal limits\
Intelligence Reason or remember\
Wisdom Notice things in the environment or in\
creatures’ behavior\
Charisma Influence, entertain, or deceive\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_ability_modifier ` |
| ` implementation ` | ` dnd2024:rule_d20_tests_ability_modifier ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` check_modifier ` | ` selected_ability_modifier ` |

#### Ability selection

<!-- splat:values path=splat_rule/parameters/ability_selection -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` criterion ` | ` most_relevant_ability ` |

#### Ability examples

<!-- splat:values path=splat_rule/parameters/ability_examples -->

| Field | Value |
| --- | --- |
| ` strength ` | ` ["lift", "push", "pull", "break"] ` |
| ` dexterity ` | ` ["nimble_movement", "quick_movement", "quiet_movement"] ` |
| ` constitution ` | ` ["exceed_endurance"] ` |
| ` intelligence ` | ` ["reason", "remember"] ` |
| ` wisdom ` | ` ["notice_environment", "notice_behavior"] ` |
| ` charisma ` | ` ["influence", "entertain", "deceive"] ` |
