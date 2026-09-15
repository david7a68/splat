---
splat_schema: 1
splat_id: 4437228f-449d-529b-9a14-a144e4cc5cd3
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_ability_modifier_2
splat_name: Ability Modifier
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
---

# Ability Modifier

## Source text

<!-- SRD page 7 -->

Saving throws are named for the ability modifiers\
they use: a Constitution saving throw, a Wisdom\
saving throw, and so on. Different saving throws are\
used to resist different kinds of effects, as shown on\
the Saving Throw Examples table.\
Saving Throw Examples\
Ability Make a Save To …\
Strength Physically resist direct force\
Dexterity Dodge out of harm’s way\
Constitution Endure a toxic hazard\
Intelligence Recognize an illusion as fake\
Wisdom Resist a mental assault\
Charisma Assert your identity\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_ability_modifier_2 ` |
| ` implementation ` | ` dnd2024:rules_d20_tests_ability_modifier_2 ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` saving_throw_modifier ` | ` named_ability_modifier ` |

#### Example abilities

<!-- splat:values path=splat_rule/parameters/example_abilities -->

| Field | Value |
| --- | --- |
| ` direct_force ` | ` strength ` |
| ` dodge ` | ` dexterity ` |
| ` toxic_hazard ` | ` constitution ` |
| ` illusion ` | ` intelligence ` |
| ` mental_assault ` | ` wisdom ` |
| ` identity ` | ` charisma ` |
