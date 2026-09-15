---
splat_schema: 1
splat_id: 9c5c8008-8320-5d7f-8938-30a3f5e7036e
splat_kind: rule
splat_key: dnd2024:rules/proficiency_saving_throw_proficiencies
splat_name: Saving Throw Proficiencies
splat_origin:
  key: dnd2024:reference/pages/009
splat_sources:
- title: SRD 5.2.1
  locator: pp. 9
  url: https://www.dndbeyond.com/srd
---

# Saving Throw Proficiencies

## Source text

<!-- SRD page 9 -->

Proficiency in a saving throw lets a character add\
their Proficiency Bonus to saves that use a partic-\
ular ability. For example, proficiency in Wisdom\
saves lets you add your Proficiency Bonus to your\
Wisdom saves. Some monsters also have saving\
throw proficiencies, as noted in their stat blocks.\
Each class gives proficiency in at least two saving\
throws, representing that class’s training in evading\
or resisting certain threats. Wizards, for example,\
are proficient in Intelligence and Wisdom saves;\
they train to resist mental assault.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/proficiency_saving_throw_proficiencies ` |
| ` implementation ` | ` dnd2024:rule_proficiency_saving_throw_proficiencies ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` starting_class_save_proficiencies_minimum ` | ` 2 ` |
| ` monster_proficiencies ` | ` stat_block ` |

#### Saving throw bonus

<!-- splat:values path=splat_rule/parameters/saving_throw_bonus -->

| Field | Value |
| --- | --- |
| ` base ` | ` ability_modifier ` |
| ` proficient_add ` | ` proficiency_bonus ` |
