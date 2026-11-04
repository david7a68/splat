---
splat_schema: 2
splat_id: f2af8a33-f692-53f2-b46a-7ff1e3a4fe3f
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_3_barbarian_subclass
splat_name: 'Level 3: Barbarian Subclass'
splat_origin: dnd2024:reference/pages/029
splat_sources:
- SRD 5.2.1, pp. 29, https://www.dndbeyond.com/srd
---

# Level 3: Barbarian Subclass

## Source text

<!-- SRD page 29 -->

You gain a Barbarian subclass of your choice. The Path of the Berserker subclass is detailed after this class’s description. A subclass is a specialization that grants you features at certain Barbarian levels. For the rest of your career, you gain each of your subclass’s features that are of your Barbarian level or lower.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_3_barbarian_subclass ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_3_barbarian_subclass ` |

### Parameters

#### Subclass choice

<!-- splat:yaml path=splat_rule/parameters/subclass_choice -->

```yaml
count: 1
class: barbarian
srd_options:
- key: dnd2024:subclasses/barbarian_path_of_the_berserker
```

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` from ` | ` selected_subclass ` |
| ` when_required_class_level_lte_current ` | ` true ` |
| ` reevaluate_on_class_level_gain ` | ` true ` |
