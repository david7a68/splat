---
splat_schema: 2
splat_id: 409feeac-c398-5165-908b-69d2c6eafe0a
splat_kind: rule
splat_key: dnd2024:glossary/blindsight
splat_name: Blindsight
splat_origin: dnd2024:reference/glossary/blindsight
splat_sources:
- SRD 5.2.1, pp. 177, https://www.dndbeyond.com/srd
---

# Blindsight

## Source text

<!-- SRD page 177 -->

If you have Blindsight, you can see within a specific range without relying on physical sight. Within that range, you can see anything that isn’t behind Total Cover even if you have the Blinded condition or are in Darkness. Moreover, in that range, you can see something that has the Invisible condition.

Original layout: [[../../srd-5.2.1/pages/177|page 177]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/blindsight ` |
| ` implementation ` | ` dnd2024:glossary_blindsight ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` range ` | ` feature_specified ` |
| ` physical_sight_required ` | ` false ` |
| ` sees_while ` | ` ["blinded", "darkness"] ` |
| ` sees_invisible ` | ` true ` |
| ` total_cover_blocks ` | ` true ` |
