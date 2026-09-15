---
splat_schema: 1
splat_id: e49dd73b-6cc4-5e8c-9ab2-6abfa3cfdd17
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_interactions_with_rerolls
splat_name: Interactions with Rerolls
splat_origin:
  key: dnd2024:reference/pages/008
splat_sources:
- title: SRD 5.2.1
  locator: pp. 8
  url: https://www.dndbeyond.com/srd
---

# Interactions with Rerolls

## Source text

<!-- SRD page 8 -->

When you have Advantage or Disadvantage and\
something in the game lets you reroll or replace the\
d20, you can reroll or replace only one die, not both.\
You choose which one.\
For example, if you have Heroic Inspiration (see\
the sidebar below) and roll a 3 and an 18 on an abil-\
ity check that has Advantage or Disadvantage, you\
could expend your Heroic Inspiration to reroll one\
of those dice, not both of them.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_interactions_with_rerolls ` |
| ` implementation ` | ` dnd2024:rule_d20_tests_interactions_with_rerolls ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` when ` | ` ["advantage_or_disadvantage", "reroll_or_replace_d20"] ` |
| ` maximum_affected_dice ` | ` 1 ` |
| ` die_selection ` | ` roller_choice ` |
