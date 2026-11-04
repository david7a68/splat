---
splat_schema: 2
splat_id: 28f409b2-54e1-5730-b98f-9ee06070e38a
splat_kind: rule
splat_key: dnd2024:rules/weapons_nick
splat_name: Nick
splat_origin: dnd2024:reference/pages/090
splat_sources:
- SRD 5.2.1, pp. 90, https://www.dndbeyond.com/srd
---

# Nick

## Source text

<!-- SRD page 90 -->

When you make the extra attack of the Light property, you can make it as part of the Attack action instead of as a Bonus Action. You can make this extra attack only once per turn.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_nick ` |
| ` implementation ` | ` dnd2024:rules_weapons_nick ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` extra_light_attack_economy_choice ` | ` ["part_of_attack_action", "bonus_action"] ` |
| ` extra_light_attack_per_turn_limit ` | ` 1 ` |
