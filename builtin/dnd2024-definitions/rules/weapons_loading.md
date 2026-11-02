---
splat_schema: 1
splat_id: 840c7c4e-fc58-5081-8e5b-851090e4a0a4
splat_kind: rule
splat_key: dnd2024:rules/weapons_loading
splat_name: Loading
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
---

# Loading

## Source text

<!-- SRD page 90 -->

You can fire only one piece of ammunition from a Loading weapon when you use an action, a Bonus Action, or a Reaction to fire it, regardless of the number of attacks you can normally make.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_loading ` |
| ` implementation ` | ` dnd2024:rules_weapons_loading ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ammunition_fired_limit ` | ` 1 ` |
| ` limit_scope ` | ` each_action_bonus_action_or_reaction ` |
| ` extra_attacks_override_limit ` | ` false ` |
