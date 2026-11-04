---
splat_schema: 2
splat_id: e96d0346-2e14-54db-8a74-6b9a0e43bbac
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_proficiency_bonus_2
splat_name: Proficiency Bonus
splat_origin: dnd2024:reference/pages/007
splat_sources:
- SRD 5.2.1, pp. 7, https://www.dndbeyond.com/srd
---

# Proficiency Bonus

## Source text

<!-- SRD page 7 -->

You add your Proficiency Bonus to your saving throw if you have proficiency in that kind of save. See “Proficiency” later in “Playing the Game.”

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_proficiency_bonus_2 ` |
| ` implementation ` | ` dnd2024:rules_d20_tests_proficiency_bonus_2 ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` test ` | ` saving_throw ` |

#### Bonus

<!-- splat:yaml path=splat_rule/parameters/bonus -->

```yaml
when: proficient_in_save_ability
value:
  var: subject.proficiency_bonus
```
