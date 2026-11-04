---
splat_schema: 2
splat_id: 3bd26cde-e502-5a5a-abf9-ffac810e55cb
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_3_ranger_subclass
splat_name: 'Level 3: Ranger Subclass'
splat_origin: dnd2024:reference/pages/059
splat_sources:
- SRD 5.2.1, pp. 59, https://www.dndbeyond.com/srd
---

# Level 3: Ranger Subclass

## Source text

<!-- SRD page 59 -->

You gain a Ranger subclass of your choice. The Hunter subclass is detailed after this class’s description. A subclass is a specialization that grants you features at certain Ranger levels. For the rest of your career, you gain each of your subclass’s features that are of your Ranger level or lower.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/ranger_level_3_ranger_subclass ` |
| ` implementation ` | ` dnd2024:class_features_ranger_level_3_ranger_subclass ` |

### Parameters

#### Subclass choice

<!-- splat:values path=splat_rule/parameters/subclass_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` class ` | ` ranger ` |

#### Feature grant policy

<!-- splat:values path=splat_rule/parameters/feature_grant_policy -->

| Field | Value |
| --- | --- |
| ` class_level_at_least_feature_level ` | ` true ` |
| ` events ` | ` ["subclass.selected", "character.level_gained"] ` |
| ` persistent ` | ` true ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |
| ` minimum_level ` | ` 3 ` |
