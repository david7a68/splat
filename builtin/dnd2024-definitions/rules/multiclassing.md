---
splat_schema: 1
splat_id: 823c2e47-2e1a-54e0-b527-5c297213e020
splat_kind: rule
splat_key: dnd2024:rules/multiclassing
splat_name: Multiclassing
splat_origin:
  key: dnd2024:reference/pages/024
splat_sources:
- title: SRD 5.2.1
  locator: pp. 24
  url: https://www.dndbeyond.com/srd
---

# Multiclassing

## Source text

<!-- SRD page 24 -->

Multiclassing allows you to gain levels in multiple classes. With this rule, you have the option of gaining a level in a new class whenever you advance in level instead of gaining a level in your current class. Doing so lets you mix the abilities of those classes to realize a character concept that might not be reflected in a single class.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/multiclassing ` |
| ` implementation ` | ` dnd2024:rules_multiclassing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` on_level_gain_choose ` | ` ["advance_existing_class", "gain_new_class"] ` |
| ` multiple_classes_allowed ` | ` true ` |
