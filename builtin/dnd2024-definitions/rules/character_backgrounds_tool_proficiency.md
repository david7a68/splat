---
splat_schema: 1
splat_id: 9e0f5823-c8b9-5dad-b8f5-fc2d481dd646
splat_kind: rule
splat_key: dnd2024:rules/character_backgrounds_tool_proficiency
splat_name: Tool Proficiency
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
---

# Tool Proficiency

## Source text

<!-- SRD page 83 -->

Each background gives a character proficiency with one tool—either a specific tool or one chosen from the Artisan’s Tools category. Tools are detailed in “Equipment.”

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/character_backgrounds_tool_proficiency ` |
| ` implementation ` | ` dnd2024:rules_character_backgrounds_tool_proficiency ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` grant_count ` | ` 1 ` |
| ` source ` | ` background_definition ` |
| ` selection ` | ` ["fixed_tool", "choose_artisans_tool"] ` |
