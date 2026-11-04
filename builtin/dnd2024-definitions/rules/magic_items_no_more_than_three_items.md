---
splat_schema: 2
splat_id: 4ff66ee4-42dc-5665-b6da-a29b38a97302
splat_kind: rule
splat_key: dnd2024:rules/magic_items_no_more_than_three_items
splat_name: No More Than Three Items
splat_origin: dnd2024:reference/pages/102
splat_sources:
- SRD 5.2.1, pp. 102, https://www.dndbeyond.com/srd
---

# No More Than Three Items

## Source text

<!-- SRD page 102 -->

You can be attuned to no more than three magic items at a time. Any attempt to attune to a fourth item fails; you must end your Attunement to an item first. Additionally, you can’t attune to more than one copy of an item. For example, you can’t attune to more than one Ring of Protection at a time.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_items_no_more_than_three_items ` |
| ` implementation ` | ` dnd2024:rule_magic_items_no_more_than_three_items ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attuned_items_maximum ` | ` 3 ` |
| ` fourth_attempt ` | ` fails ` |
| ` same_definition_copies_attuned_maximum ` | ` 1 ` |
| ` must_end_existing_before_new_if_at_limit ` | ` true ` |
