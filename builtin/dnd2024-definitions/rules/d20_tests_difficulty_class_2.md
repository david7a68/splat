---
splat_schema: 1
splat_id: 61e01774-725e-58c2-ba42-f192091a2fdf
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_difficulty_class_2
splat_name: Difficulty Class
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
---

# Difficulty Class

## Source text

<!-- SRD page 7 -->

The Difficulty Class for a saving throw is deter-\
mined by the effect that causes it or by the GM. For\
example, if a spell forces you to make a save, the DC\
is determined by the caster’s spellcasting ability\
and Proficiency Bonus. Monster abilities that call\
for saves specify the DC.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_difficulty_class_2 ` |
| ` implementation ` | ` dnd2024:rule_d20_tests_difficulty_class_2 ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` save_dc_source_priority ` | ` ["effect", "gm_choice"] ` |
| ` monster_ability_dc ` | ` stat_block_value ` |

#### Spell dc

<!-- splat:yaml path=splat_rule/parameters/spell_dc -->

```yaml
op: add
args:
- 8
- variable: spellcasting_ability_modifier
- variable: proficiency_bonus
```
