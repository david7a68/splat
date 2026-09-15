---
splat_schema: 1
splat_id: ff60f5c3-1473-525c-9dac-3aab5d4d9736
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_proficiency_bonus_3
splat_name: Proficiency Bonus
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
---

# Proficiency Bonus

## Source text

<!-- SRD page 7 -->

You add your Proficiency Bonus to your attack roll\
when you attack using a weapon you have profi-\
ciency with, as well as when you attack with a spell.\
See “Proficiency” later in “Playing the Game” for\
more information about weapon proficiencies.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_proficiency_bonus_3 ` |
| ` implementation ` | ` dnd2024:rule_d20_tests_proficiency_bonus_3 ` |

### Parameters

#### Add proficiency bonus when

<!-- splat:values path=splat_rule/parameters/add_proficiency_bonus_when -->

| Field | Value |
| --- | --- |
| ` any ` | ` ["proficient_with_used_weapon", "spell_attack"] ` |
