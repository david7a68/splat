---
splat_schema: 2
splat_id: cb18be95-6f20-5388-bc75-893a8f969930
splat_kind: rule
splat_key: dnd2024:rules/tools
splat_name: Tools
splat_origin: dnd2024:reference/pages/093
splat_sources:
- SRD 5.2.1, pp. 93, https://www.dndbeyond.com/srd
---

# Tools

## Source text

<!-- SRD page 93 -->

A tool helps you make specialized ability checks, craft certain items, or both. A tool’s description includes the tool’s cost and weight, as well as the following entries:

Ability. This entry lists the ability to use when making an ability check with the tool.

Utilize. This entry lists things you can do with the tool when you take the Utilize action. You can do one of those things each time you take the action. This entry also provides the DC for the action.

Craft. This entry lists what, if anything, you can craft with the tool. For crafting rules, see “Crafting Nonmagical Items,” “Brewing Potions of Healing,” and “Scribing Spell Scrolls” later in “Equipment.”

Variants. This entry appears if the tool has variants, which are listed. Each requires a separate proficiency.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/tools ` |
| ` implementation ` | ` dnd2024:rule_tools ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` tool_check_ability ` | ` tool_definition ` |
| ` craftable_items ` | ` tool_craft_entries ` |
| ` each_variant_requires_separate_proficiency ` | ` true ` |

#### Utilize

<!-- splat:values path=splat_rule/parameters/utilize -->

| Field | Value |
| --- | --- |
| ` options ` | ` tool_utilize_entries ` |
| ` options_per_action ` | ` 1 ` |
| ` dc ` | ` selected_entry_dc ` |
