---
splat_schema: 1
splat_id: 6eb7670c-062c-5c8f-a699-68fd07a69273
splat_kind: rule
splat_key: dnd2024:rules/exploration_what_is_an_object
splat_name: What Is an Object?
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
---

# What Is an Object?

## Source text

<!-- SRD page 12 -->

For the purpose of the rules, an object is a discrete, inanimate item like a window, door, sword, book, table, chair, or stone. It isn’t a building or a vehicle, which are composed of many objects.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/exploration_what_is_an_object ` |
| ` implementation ` | ` dnd2024:rules_exploration_what_is_an_object ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` composite_excluded ` | ` ["building", "vehicle"] ` |
| ` composite_parts_may_be_objects ` | ` true ` |

#### Object

<!-- splat:values path=splat_rule/parameters/object -->

| Field | Value |
| --- | --- |
| ` discrete ` | ` true ` |
| ` inanimate ` | ` true ` |
