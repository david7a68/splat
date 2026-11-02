---
splat_schema: 1
splat_id: 7ec5c6bc-1f82-5d91-a8ef-0125d960d85e
splat_kind: rule
splat_key: dnd2024:glossary/tremorsense
splat_name: Tremorsense
splat_origin:
  key: dnd2024:reference/glossary/tremorsense
splat_sources:
- title: SRD 5.2.1
  locator: pp. 190
  url: https://www.dndbeyond.com/srd
---

# Tremorsense

## Source text

<!-- SRD page 190 -->

A creature with Tremorsense can pinpoint the location of creatures and moving objects within a specific range, provided that the creature with Tremorsense and anything it is detecting are both in contact with the same surface (such as the ground, a wall, or a ceiling) or the same liquid.

Tremorsense can’t detect creatures or objects in the air, and it doesn’t count as a form of sight.

Original layout: [[../../srd-5.2.1/pages/190|page 190]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/tremorsense ` |
| ` implementation ` | ` dnd2024:glossary_tremorsense ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` range ` | ` feature_specified ` |
| ` detects ` | ` ["creatures", "moving_objects"] ` |
| ` shared_contact_required_any ` | ` ["same_surface", "same_liquid"] ` |
| ` surface_examples ` | ` ["ground", "wall", "ceiling"] ` |
| ` airborne_targets_detectable ` | ` false ` |
| ` counts_as_sight ` | ` false ` |
| ` result ` | ` pinpoint_location ` |
