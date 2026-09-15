---
splat_schema: 1
splat_id: 9b83be13-c33e-586a-aeb4-2f3ba7817c6b
splat_kind: rule
splat_key: dnd2024:rules/crafting_nonmagical_items_tools
splat_name: Tools
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
---

# Tools

## Source text

<!-- SRD page 103 -->

The “Tools” section of “Equipment” lists which tools\
are required to make certain items. The GM assigns\
required tools for items not listed there.\
You must use the required tool to make an item\
and have proficiency with that tool. Anyone who\
helps you must also have proficiency with it.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/crafting_nonmagical_items_tools ` |
| ` implementation ` | ` dnd2024:rule_crafting_nonmagical_items_tools ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` required_tool_source ` | ` tool_craft_table ` |
| ` must_use_required_tool ` | ` true ` |
| ` proficiency_required_for ` | ` ["crafter", "every_assistant"] ` |

#### Unlisted required tool

<!-- splat:values path=splat_rule/parameters/unlisted_required_tool -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` choice ` | ` tool ` |
