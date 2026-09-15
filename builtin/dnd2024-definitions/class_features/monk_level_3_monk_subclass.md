---
splat_schema: 1
splat_id: 00b433a4-b723-505f-a738-59011a9ef9ef
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_3_monk_subclass
splat_name: 'Level 3: Monk Subclass'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
---

# Level 3: Monk Subclass

## Source text

<!-- SRD page 51 -->

You gain a Monk subclass of your choice. The War-\
rior of the Open Hand subclass is detailed after this\
class’s description. A subclass is a specialization\
that grants you features at certain Monk levels. For\
the rest of your career, you gain each of your sub-\
class’s features that are of your Monk level or lower.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_3_monk_subclass ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_3_monk_subclass ` |

### Parameters

#### Subclass choice

<!-- splat:yaml path=splat_rule/parameters/subclass_choice -->

```yaml
count: 1
class: monk
srd_options:
- key: dnd2024:subclasses/monk_monk_subclass_warrior_of_the_open_hand
```

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` from ` | ` selected_subclass ` |
| ` when_required_class_level_lte_current ` | ` true ` |
| ` reevaluate_on_class_level_gain ` | ` true ` |
