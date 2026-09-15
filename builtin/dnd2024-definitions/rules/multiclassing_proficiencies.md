---
splat_schema: 1
splat_id: 261ca738-5b92-5fb3-9b25-4b24aa4bdbdf
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_proficiencies
splat_name: Proficiencies
splat_origin:
  key: dnd2024:reference/pages/025
splat_sources:
- title: SRD 5.2.1
  locator: pp. 25
  url: https://www.dndbeyond.com/srd
---

# Proficiencies

## Source text

<!-- SRD page 25 -->

When you gain your first level in a class other than\
your initial class, you gain only some of the new\
class’s starting proficiencies, as detailed in each\
class’s description in “Classes.”\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/multiclassing_proficiencies ` |
| ` implementation ` | ` dnd2024:rules_multiclassing_proficiencies ` |

### Parameters

#### New class first level

<!-- splat:values path=splat_rule/parameters/new_class_first_level -->

| Field | Value |
| --- | --- |
| ` grant ` | ` class_multiclass_proficiencies_only ` |
| ` exclude ` | ` other_starting_proficiencies ` |
| ` source ` | ` chosen_class_definition ` |
