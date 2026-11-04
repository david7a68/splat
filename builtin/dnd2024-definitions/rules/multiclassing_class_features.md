---
splat_schema: 2
splat_id: 1459cd0d-5d5e-5fc9-bdcc-531985823a54
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_class_features
splat_name: Class Features
splat_origin: dnd2024:reference/pages/025
splat_sources:
- SRD 5.2.1, pp. 25, https://www.dndbeyond.com/srd
---

# Class Features

## Source text

<!-- SRD page 25 -->

When you gain a new level in a class, you get its features for that level. A few features have additional rules when you’re multiclassing. Check the information about multiclassing included in each of your classes’ descriptions.

Special rules apply to Extra Attack, Spellcasting, and features (such as Unarmored Defense) that give you alternative ways to calculate your Armor Class.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/multiclassing_class_features ` |
| ` implementation ` | ` dnd2024:rules_multiclassing_class_features ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` gain_features_for ` | ` new_class_level ` |
| ` apply_class_multiclass_rules ` | ` true ` |
| ` special_handling ` | ` ["extra_attack", "spellcasting", "alternative_ac_calculations"] ` |

#### Special handling rules

<!-- splat:rows path=splat_rule/parameters/special_handling_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/multiclassing_extra_attack ` |
| ` 1 ` | ` dnd2024:rules/multiclassing_spellcasting ` |
| ` 2 ` | ` dnd2024:rules/multiclassing_armor_class ` |
