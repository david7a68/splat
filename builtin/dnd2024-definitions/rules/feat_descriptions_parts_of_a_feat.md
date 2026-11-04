---
splat_schema: 2
splat_id: 8872fe18-55a2-56be-9027-aa4ba24669e8
splat_kind: rule
splat_key: dnd2024:rules/feat_descriptions_parts_of_a_feat
splat_name: Parts of a Feat
splat_origin: dnd2024:reference/pages/087
splat_sources:
- SRD 5.2.1, pp. 87, https://www.dndbeyond.com/srd
---

# Parts of a Feat

## Source text

<!-- SRD page 87 -->

The description of a feat contains the following parts, which are presented after the feat’s name.

Category. A feat is a member of a category, which is noted in the feat. If you’re instructed to choose a feat from a specific category, such as the Origin category, that category must appear under the feat’s name. If you’re instructed to choose a feat and no category is specified, you can choose from any category.

Prerequisite. To take a feat, you must meet any prerequisite in its description unless a feature allows you to take the feat without the prerequisite. If a prerequisite includes a class, you must have at least 1 level in that class to take the feat.

Benefit. The benefits of a feat are detailed after any prerequisites are listed. If you have a feat, you gain its benefits.

Repeatable. A feat can be taken only once unless its description states otherwise in a “Repeatable” subsection.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/feat_descriptions_parts_of_a_feat ` |
| ` implementation ` | ` dnd2024:rules_feat_descriptions_parts_of_a_feat ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` owned_feat_grants_benefits ` | ` true ` |
| ` maximum_instances_default ` | ` 1 ` |
| ` repeatable_override ` | ` feat_repeatable_section ` |

#### Category constraint

<!-- splat:values path=splat_rule/parameters/category_constraint -->

| Field | Value |
| --- | --- |
| ` if_specified ` | ` must_match ` |
| ` otherwise ` | ` any_category ` |

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` must_satisfy_all ` | ` true ` |
| ` feature_can_waive ` | ` true ` |
| ` class_requirement_min_level ` | ` 1 ` |
