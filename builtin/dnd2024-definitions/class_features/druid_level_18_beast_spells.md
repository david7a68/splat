---
splat_schema: 1
splat_id: 0616e899-1014-5aea-9796-b3832feee43f
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_18_beast_spells
splat_name: 'Level 18: Beast Spells'
splat_origin:
  key: dnd2024:reference/pages/043
splat_sources:
- title: SRD 5.2.1
  locator: pp. 43
  url: https://www.dndbeyond.com/srd
---

# Level 18: Beast Spells

## Source text

<!-- SRD page 43 -->

While using Wild Shape, you can cast spells in Beast form, except for any spell that has a Material component with a cost specified or that consumes its Material component.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/druid_level_18_beast_spells ` |
| ` implementation ` | ` dnd2024:class_features_druid_level_18_beast_spells ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` while ` | ` wild_shape.beast_form ` |
| ` allow_spellcasting ` | ` true ` |

#### Exclude spells

<!-- splat:values path=splat_rule/parameters/exclude_spells -->

| Field | Value |
| --- | --- |
| ` material_component_cost_specified ` | ` true ` |
| ` or_material_component_consumed ` | ` true ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` druid ` |
| ` minimum_level ` | ` 18 ` |
