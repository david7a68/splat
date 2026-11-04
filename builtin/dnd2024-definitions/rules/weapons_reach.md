---
splat_schema: 2
splat_id: 2a3dabab-9c38-55dc-9bf9-412bc5eaef1a
splat_kind: rule
splat_key: dnd2024:rules/weapons_reach
splat_name: Reach
splat_origin: dnd2024:reference/pages/090
splat_sources:
- SRD 5.2.1, pp. 90, https://www.dndbeyond.com/srd
---

# Reach

## Source text

<!-- SRD page 90 -->

A Reach weapon adds 5 feet to your reach when you attack with it, as well as when determining your reach for Opportunity Attacks with it.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_reach ` |
| ` implementation ` | ` dnd2024:rule_weapons_reach ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` reach_bonus_feet ` | ` 5 ` |
| ` scope ` | ` ["attacks_with_weapon", "opportunity_attack_reach_with_weapon"] ` |
