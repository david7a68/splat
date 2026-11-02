---
splat_schema: 1
splat_id: d7877970-e1f1-5007-9751-73a71408b6b9
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_senses
splat_name: Senses
splat_origin:
  key: dnd2024:reference/pages/256
splat_sources:
- title: SRD 5.2.1
  locator: pp. 256
  url: https://www.dndbeyond.com/srd
---

# Senses

## Source text

<!-- SRD page 256 -->

The Senses entry specifies a monster’s Passive Perception score, as well as any special senses the monster possesses. Passive Perception and special senses are described in “Rules Glossary.”

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_senses ` |
| ` implementation ` | ` dnd2024:rule_parts_of_a_stat_block_senses ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` value_source ` | ` stat_block_senses ` |
| ` grant_only_listed_senses ` | ` true ` |

#### Apply definitions

<!-- splat:rows path=splat_rule/parameters/apply_definitions -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:glossary/passive_perception ` |
| ` 1 ` | ` dnd2024:glossary/blindsight ` |
| ` 2 ` | ` dnd2024:glossary/darkvision ` |
| ` 3 ` | ` dnd2024:glossary/tremorsense ` |
| ` 4 ` | ` dnd2024:glossary/truesight ` |
