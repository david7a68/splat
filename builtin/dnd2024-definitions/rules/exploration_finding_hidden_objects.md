---
splat_schema: 1
splat_id: 23405344-8b35-5e9e-9bd1-8bf11b6d21c2
splat_kind: rule
splat_key: dnd2024:rules/exploration_finding_hidden_objects
splat_name: Finding Hidden Objects
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
---

# Finding Hidden Objects

## Source text

<!-- SRD page 12 -->

When your character searches for hidden things,\
such as a secret door or a trap, the GM typically asks\
you to make a Wisdom (Perception) check, provided\
you describe the character searching in the hidden\
object’s vicinity. On a success, you find the object,\
other important details, or both.\
If you describe your character searching nowhere\
near a hidden object, a Wisdom (Perception) check\
won’t reveal the object, no matter the check’s total.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/exploration_finding_hidden_objects ` |
| ` implementation ` | ` dnd2024:rules_exploration_finding_hidden_objects ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` search_location_required ` | ` object_vicinity ` |
| ` success_options ` | ` ["object", "important_details", "object_and_important_details"] ` |

#### Check

<!-- splat:values path=splat_rule/parameters/check -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` skill ` | ` perception ` |
| ` dc_authority ` | ` gm ` |

#### Outside vicinity

<!-- splat:values path=splat_rule/parameters/outside_vicinity -->

| Field | Value |
| --- | --- |
| ` reveal_object ` | ` false ` |
| ` roll_irrelevant ` | ` true ` |
