---
splat_schema: 1
splat_id: 8d552b00-bc66-5976-a196-42d2616cf6cb
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_skills
splat_name: Skills
splat_origin:
  key: dnd2024:reference/pages/255
splat_sources:
- title: SRD 5.2.1
  locator: pp. 255
  url: https://www.dndbeyond.com/srd
---

# Skills

## Source text

<!-- SRD page 255 -->

The Skills entry specifies a monster’s skill proficiencies, if any. For example, a monster that is very perceptive and stealthy might have bonuses to Wisdom (Perception) and Dexterity (Stealth) checks. A skill bonus is the sum of a monster’s relevant ability modifier and its Proficiency Bonus. Other modifiers might apply.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_skills ` |
| ` implementation ` | ` dnd2024:rule_parts_of_a_stat_block_skills ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` listed_skills_are_proficiencies ` | ` true ` |

#### Skill bonus

<!-- splat:values path=splat_rule/parameters/skill_bonus -->

| Field | Value |
| --- | --- |
| ` op ` | ` add ` |

##### Args

<!-- splat:table
path: splat_rule/parameters/skill_bonus/args
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Variable:
    path: variable
    type: symbol
-->

| Key | Variable |
| --- | --- |
| ` row_1 ` | ` Ability modifier ` |
| ` row_2 ` | ` Proficiency bonus ` |
| ` row_3 ` | ` Other modifiers ` |
