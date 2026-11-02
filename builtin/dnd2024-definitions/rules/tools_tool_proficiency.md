---
splat_schema: 1
splat_id: ac696a4c-7355-59a2-a72d-a4bb01a883a9
splat_kind: rule
splat_key: dnd2024:rules/tools_tool_proficiency
splat_name: Tool Proficiency
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
---

# Tool Proficiency

## Source text

<!-- SRD page 93 -->

If you have proficiency with a tool, add your Proficiency Bonus to any ability check you make that uses the tool. If you have proficiency in a skill that’s used with that check, you have Advantage on the check too.

Your features might give you proficiency with a tool. A monster has proficiency with any tool in its stat block.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/tools_tool_proficiency ` |
| ` implementation ` | ` dnd2024:rules_tools_tool_proficiency ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` tool_check_proficiency_bonus ` | ` true ` |
| ` also_proficient_in_used_skill ` | ` advantage ` |
| ` monster_proficiency ` | ` all_stat_block_tools ` |
