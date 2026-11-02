---
splat_schema: 1
splat_id: 0711c1c0-69f4-58aa-af31-bcc55926bdf2
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_3_fighter_subclass
splat_name: 'Level 3: Fighter Subclass'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
---

# Level 3: Fighter Subclass

## Source text

<!-- SRD page 48 -->

You gain a Fighter subclass of your choice. The Champion subclass is detailed after this class’s description. A subclass is a specialization that grants you features at certain Fighter levels. For the rest of your career, you gain each of your subclass’s features that are of your Fighter level or lower.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fighter_level_3_fighter_subclass ` |
| ` implementation ` | ` dnd2024:class_features_fighter_level_3_fighter_subclass ` |

### Parameters

#### Subclass choice

<!-- splat:yaml path=splat_rule/parameters/subclass_choice -->

```yaml
count: 1
class: fighter
srd_options:
- key: dnd2024:subclasses/fighter_fighter_subclass_champion
```

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` from ` | ` selected_subclass ` |
| ` when_required_class_level_lte_current ` | ` true ` |
| ` reevaluate_on_class_level_gain ` | ` true ` |
