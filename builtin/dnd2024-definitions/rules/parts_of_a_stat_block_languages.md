---
splat_schema: 2
splat_id: 5d2d566f-ec0b-5790-82d5-550176a81d5f
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_languages
splat_name: Languages
splat_origin: dnd2024:reference/pages/256
splat_sources:
- SRD 5.2.1, pp. 256, https://www.dndbeyond.com/srd
---

# Languages

## Source text

<!-- SRD page 256 -->

This entry lists languages that the monster can use to communicate. Sometimes a monster can understand a language but can’t communicate with it, which is noted in its entry. “None” indicates that a creature doesn’t comprehend any language.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_languages ` |
| ` implementation ` | ` dnd2024:rules_parts_of_a_stat_block_languages ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` language_entry_modes ` | ` ["communicate", "understand_only", "none"] ` |

#### None

<!-- splat:values path=splat_rule/parameters/none -->

| Field | Value |
| --- | --- |
| ` understands_any_language ` | ` false ` |

#### Understand only

<!-- splat:values path=splat_rule/parameters/understand_only -->

| Field | Value |
| --- | --- |
| ` can_communicate_in_language ` | ` false ` |
