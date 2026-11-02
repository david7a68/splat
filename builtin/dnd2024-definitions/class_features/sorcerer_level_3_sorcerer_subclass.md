---
splat_schema: 1
splat_id: 7ef240a0-6697-5584-9b17-21cb393fa82f
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_3_sorcerer_subclass
splat_name: 'Level 3: Sorcerer Subclass'
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66
  url: https://www.dndbeyond.com/srd
---

# Level 3: Sorcerer Subclass

## Source text

<!-- SRD page 66 -->

You gain a Sorcerer subclass of your choice. The Draconic Sorcery subclass is detailed after this class’s description. A subclass is a specialization that grants you features at certain Sorcerer levels. For the rest of your career, you gain each of your subclass’s features that are of your Sorcerer level or lower.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/sorcerer_level_3_sorcerer_subclass ` |
| ` implementation ` | ` dnd2024:class_features_sorcerer_level_3_sorcerer_subclass ` |

### Parameters

#### Subclass choice

<!-- splat:values path=splat_rule/parameters/subclass_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` class ` | ` sorcerer ` |

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
| ` class ` | ` sorcerer ` |
| ` minimum_level ` | ` 3 ` |
