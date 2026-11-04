---
splat_schema: 2
splat_id: bf865e9a-0312-53ab-80bb-d9b12ffc9d80
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_proficiency_bonus
splat_name: Proficiency Bonus
splat_origin: dnd2024:reference/pages/006
splat_sources:
- SRD 5.2.1, pp. 6, https://www.dndbeyond.com/srd
---

# Proficiency Bonus

## Source text

<!-- SRD page 6 -->

Add your Proficiency Bonus to an ability check when the GM determines that a skill or tool proficiency is relevant to the check and you have that proficiency. For example, if a rule refers to a Strength (Acrobatics or Athletics) check, you can add your Proficiency Bonus to the check if you have proficiency in the Acrobatics or Athletics skill. See “Proficiency” later in “Playing the Game” for more information about skill and tool proficiencies.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_proficiency_bonus ` |
| ` implementation ` | ` dnd2024:rules_d20_tests_proficiency_bonus ` |

### Parameters

#### Proficiency bonus

<!-- splat:yaml path=splat_rule/parameters/proficiency_bonus -->

```yaml
applies_when:
  all:
  - gm_relevant_skill_or_tool
  - actor_has_proficiency
any_alternative_skill_suffices: true
times: 1
```
