---
splat_schema: 1
splat_id: c3617baa-f4ba-5cd0-949f-5afb9c1525d8
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_3_rogue_subclass
splat_name: 'Level 3: Rogue Subclass'
splat_origin:
  key: dnd2024:reference/pages/062
splat_sources:
- title: SRD 5.2.1
  locator: pp. 62
  url: https://www.dndbeyond.com/srd
---

# Level 3: Rogue Subclass

## Source text

<!-- SRD page 62 -->

You gain a Rogue subclass of your choice. The Thief\
subclass is detailed after this class’s description. A\
subclass is a specialization that grants you features\
at certain Rogue levels. For the rest of your career,\
you gain each of your subclass’s features that are of\
your Rogue level or lower.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/rogue_level_3_rogue_subclass ` |
| ` implementation ` | ` dnd2024:class_features_rogue_level_3_rogue_subclass ` |

### Parameters

#### Subclass choice

<!-- splat:values path=splat_rule/parameters/subclass_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` class ` | ` rogue ` |

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
| ` class ` | ` rogue ` |
| ` minimum_level ` | ` 3 ` |
