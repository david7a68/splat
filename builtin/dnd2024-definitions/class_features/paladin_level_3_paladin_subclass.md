---
splat_schema: 1
splat_id: 39d77ca4-da27-593c-ac77-25bcf215fc20
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_3_paladin_subclass
splat_name: 'Level 3: Paladin Subclass'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
---

# Level 3: Paladin Subclass

## Source text

<!-- SRD page 55 -->

You gain a Paladin subclass of your choice. The Oath of Devotion subclass is detailed after this class’s description. A subclass is a specialization that grants you features at certain Paladin levels. For the rest of your career, you gain each of your subclass’s features that are of your Paladin level or lower.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/paladin_level_3_paladin_subclass ` |
| ` implementation ` | ` dnd2024:class_features_paladin_level_3_paladin_subclass ` |

### Parameters

#### Subclass choice

<!-- splat:yaml path=splat_rule/parameters/subclass_choice -->

```yaml
count: 1
class: paladin
srd_options:
- key: dnd2024:subclasses/paladin_paladin_subclass_oath_of_devotion
```

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` from ` | ` selected_subclass ` |
| ` when_required_class_level_lte_current ` | ` true ` |
| ` reevaluate_on_class_level_gain ` | ` true ` |
