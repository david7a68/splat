---
splat_schema: 2
splat_id: 0ecbedb2-9cb8-518b-927c-f760d9df468f
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_size
splat_name: Size
splat_origin: dnd2024:reference/pages/254
splat_sources:
- SRD 5.2.1, pp. 254, https://www.dndbeyond.com/srd
---

# Size

## Source text

<!-- SRD page 254 -->

A monster is Tiny, Small, Medium, Large, Huge, or Gargantuan. If size options are presented, you choose the creature’s size from those options. See “Playing the Game” for information on size.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_size ` |
| ` implementation ` | ` dnd2024:rule_parts_of_a_stat_block_size ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` size_options ` | ` ["tiny", "small", "medium", "large", "huge", "gargantuan"] ` |

#### Multiple stat block sizes

<!-- splat:values path=splat_rule/parameters/multiple_stat_block_sizes -->

| Field | Value |
| --- | --- |
| ` selector ` | ` gm ` |
| ` constraint ` | ` listed_options ` |

#### Size mechanics

<!-- splat:values path=splat_rule/parameters/size_mechanics -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/size ` |
