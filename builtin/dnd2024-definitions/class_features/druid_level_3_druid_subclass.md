---
splat_schema: 2
splat_id: c8176565-b08a-54a0-83f7-e5b6e25b40b2
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_3_druid_subclass
splat_name: 'Level 3: Druid Subclass'
splat_origin: dnd2024:reference/pages/043
splat_sources:
- SRD 5.2.1, pp. 43, https://www.dndbeyond.com/srd
---

# Level 3: Druid Subclass

## Source text

<!-- SRD page 43 -->

You gain a Druid subclass of your choice. The Circle of the Land subclass is detailed after this class’s description. A subclass is a specialization that grants you features at certain Druid levels. For the rest of your career, you gain each of your subclass’s features that are of your Druid level or lower.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/druid_level_3_druid_subclass ` |
| ` implementation ` | ` dnd2024:class_features_druid_level_3_druid_subclass ` |

### Parameters

#### Subclass choice

<!-- splat:yaml path=splat_rule/parameters/subclass_choice -->

```yaml
count: 1
class: druid
srd_options:
- key: dnd2024:subclasses/druid_circle_of_the_land
```

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` from ` | ` selected_subclass ` |
| ` when_required_class_level_lte_current ` | ` true ` |
| ` reevaluate_on_class_level_gain ` | ` true ` |
