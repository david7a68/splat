---
splat_schema: 2
splat_id: 1247b5c7-49a9-5a6a-ae89-f338091b04ea
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_immunities
splat_name: Immunities
splat_origin: dnd2024:reference/pages/255
splat_sources:
- SRD 5.2.1, pp. 255, https://www.dndbeyond.com/srd
---

# Immunities

## Source text

<!-- SRD page 255 -->

This entry lists the monster’s Immunities, if any. If the monster has damage and condition Immunities, the damage types are listed before the conditions. See “Playing the Game” for details.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_immunities ` |
| ` implementation ` | ` dnd2024:rules_parts_of_a_stat_block_immunities ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` immunity_kinds ` | ` ["damage_type", "condition"] ` |
| ` entry_order ` | ` ["damage_types", "conditions"] ` |
| ` damage_immunity ` | ` prevent_matching_damage ` |
| ` condition_immunity ` | ` prevent_matching_condition ` |
