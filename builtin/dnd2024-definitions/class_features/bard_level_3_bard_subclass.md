---
splat_schema: 1
splat_id: eea6db6d-0573-53f9-8b0f-50a625d5f69c
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_3_bard_subclass
splat_name: 'Level 3: Bard Subclass'
splat_origin:
  key: dnd2024:reference/pages/032
splat_sources:
- title: SRD 5.2.1
  locator: pp. 32
  url: https://www.dndbeyond.com/srd
splat_reading:
  title: Bard Subclass
  summary: 'Choose a Bard subclass. Your subclass grants additional features as you gain Bard levels.


    You gain all features from your chosen subclass whose required level is equal to or below your Bard
    level.'
  related_heading: Available in this package
  related:
    dnd2024:subclasses/bard_college_of_lore: College of Lore
---

# Level 3: Bard Subclass

## Source text

<!-- SRD page 32 -->

You gain a Bard subclass of your choice. The College of Lore subclass is detailed after this class’s description. A subclass is a specialization that grants you features at certain Bard levels. For the rest of your career, you gain each of your subclass’s features that are of your Bard level or lower.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/bard_level_3_bard_subclass ` |
| ` implementation ` | ` dnd2024:class_features_bard_level_3_bard_subclass ` |

### Parameters

#### Subclass choice

<!-- splat:values path=splat_rule/parameters/subclass_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` class ` | ` bard ` |

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
| ` class ` | ` bard ` |
| ` minimum_level ` | ` 3 ` |
