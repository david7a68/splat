---
splat_schema: 2
splat_id: 55b31e22-129b-58eb-815a-62c84014c39c
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_proficiency_bonus
splat_name: Proficiency Bonus
splat_origin: dnd2024:reference/pages/025
splat_sources:
- SRD 5.2.1, pp. 25, https://www.dndbeyond.com/srd
---

# Proficiency Bonus

## Source text

<!-- SRD page 25 -->

Your Proficiency Bonus is based on your total character level, not your level in a particular class, as shown in the Character Advancement table. For example, if you are a level 3 Fighter / level 2 Rogue, you have the Proficiency Bonus of a level 5 character, which is +3.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/multiclassing_proficiency_bonus ` |
| ` implementation ` | ` dnd2024:rule_multiclassing_proficiency_bonus ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` proficiency_bonus_level_source ` | ` sum_all_class_levels ` |

#### Proficiency bonus

<!-- splat:yaml path=splat_rule/parameters/proficiency_bonus -->

```yaml
op: add
args:
- 2
- op: floor
  args:
  - op: divide
    args:
    - op: subtract
      args:
      - variable: total_level
      - 1
    - 4
```
