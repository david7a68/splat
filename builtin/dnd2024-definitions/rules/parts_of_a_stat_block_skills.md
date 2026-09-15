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

The Skills entry specifies a monster’s skill profi-\
ciencies, if any. For example, a monster that is very\
perceptive and stealthy might have bonuses to Wis-\
dom (Perception) and Dexterity (Stealth) checks. A\
skill bonus is the sum of a monster’s relevant ability\
modifier and its Proficiency Bonus. Other modifiers\
might apply.\

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

<!-- splat:yaml path=splat_rule/parameters/skill_bonus -->

```yaml
op: add
args:
- variable: ability_modifier
- variable: proficiency_bonus
- variable: other_modifiers
```
