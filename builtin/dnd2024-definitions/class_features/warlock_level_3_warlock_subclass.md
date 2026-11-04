---
splat_schema: 2
splat_id: 4aa574cc-7fbf-59db-8e63-eac5762dd36d
splat_kind: rule
splat_key: dnd2024:class_features/warlock_level_3_warlock_subclass
splat_name: 'Level 3: Warlock Subclass'
splat_origin: dnd2024:reference/pages/072
splat_sources:
- SRD 5.2.1, pp. 72, https://www.dndbeyond.com/srd
---

# Level 3: Warlock Subclass

## Source text

<!-- SRD page 72 -->

You gain a Warlock subclass of your choice. The Fiend Patron subclass is detailed after this class’s description. A subclass is a specialization that grants you features at certain Warlock levels. For the rest of your career, you gain each of your subclass’s features that are of your Warlock level or lower.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/warlock_level_3_warlock_subclass ` |
| ` implementation ` | ` dnd2024:class_features_warlock_level_3_warlock_subclass ` |

### Parameters

#### Subclass choice

<!-- splat:values path=splat_rule/parameters/subclass_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` class ` | ` warlock ` |

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
| ` class ` | ` warlock ` |
| ` minimum_level ` | ` 3 ` |
