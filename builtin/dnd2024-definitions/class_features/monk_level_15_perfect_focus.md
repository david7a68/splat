---
splat_schema: 1
splat_id: e04152e0-b024-5dc6-9893-240fbb60edcc
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_15_perfect_focus
splat_name: 'Level 15: Perfect Focus'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
---

# Level 15: Perfect Focus

## Source text

<!-- SRD page 52 -->

When you roll Initiative and don’t use Uncanny Metabolism, you regain expended Focus Points until you have 4 if you have 3 or fewer.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_15_perfect_focus ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_15_perfect_focus ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` when_uncanny_metabolism_used ` | ` false ` |
| ` resource ` | ` focus_points ` |
| ` recover_to_minimum ` | ` 4 ` |
