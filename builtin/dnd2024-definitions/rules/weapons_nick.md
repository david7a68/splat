---
splat_schema: 1
splat_id: 28f409b2-54e1-5730-b98f-9ee06070e38a
splat_kind: rule
splat_key: dnd2024:rules/weapons_nick
splat_name: Nick
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
---

# Nick

## Source text

<!-- SRD page 90 -->

When you make the extra attack of the Light prop-\
erty, you can make it as part of the Attack action in-\
stead of as a Bonus Action. You can make this extra\
attack only once per turn.\

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
