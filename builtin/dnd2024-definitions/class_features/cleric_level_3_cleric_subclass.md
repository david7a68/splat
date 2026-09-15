---
splat_schema: 1
splat_id: 924c1f25-428f-571d-a2af-14d189cc89fe
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_3_cleric_subclass
splat_name: 'Level 3: Cleric Subclass'
splat_origin:
  key: dnd2024:reference/pages/037
splat_sources:
- title: SRD 5.2.1
  locator: pp. 37
  url: https://www.dndbeyond.com/srd
---

# Level 3: Cleric Subclass

## Source text

<!-- SRD page 37 -->

You gain a Cleric subclass of your choice. The Life\
Domain subclass is detailed after this class’s de-\
scription. A subclass is a specialization that grants\
you features at certain Cleric levels. For the rest of\
your career, you gain each of your subclass’s fea-\
tures that are of your Cleric level or lower.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/cleric_level_3_cleric_subclass ` |
| ` implementation ` | ` dnd2024:class_features_cleric_level_3_cleric_subclass ` |

### Parameters

#### Subclass choice

<!-- splat:yaml path=splat_rule/parameters/subclass_choice -->

```yaml
count: 1
class: cleric
srd_options:
- key: dnd2024:subclasses/cleric_cleric_subclass_life_domain
```

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` from ` | ` selected_subclass ` |
| ` when_required_class_level_lte_current ` | ` true ` |
| ` reevaluate_on_class_level_gain ` | ` true ` |
