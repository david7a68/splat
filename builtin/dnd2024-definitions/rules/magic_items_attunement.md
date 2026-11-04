---
splat_schema: 2
splat_id: 07122e38-0ba9-58cb-b12f-52525bac2cf0
splat_kind: rule
splat_key: dnd2024:rules/magic_items_attunement
splat_name: Attunement
splat_origin: dnd2024:reference/pages/102
splat_sources:
- SRD 5.2.1, pp. 102, https://www.dndbeyond.com/srd
---

# Attunement

## Source text

<!-- SRD page 102 -->

Some magic items require a creature to form a bond—called Attunement—with them before the creature can use an item’s magical properties. Without becoming attuned to an item that requires Attunement, you gain only its nonmagical benefits unless its description states otherwise. For example, a magic Shield that requires Attunement provides the benefits of a normal Shield if you aren’t attuned to it, but none of its magical properties.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_items_attunement ` |
| ` implementation ` | ` dnd2024:rules_magic_items_attunement ` |

### Parameters

#### Requires attunement item

<!-- splat:values path=splat_rule/parameters/requires_attunement_item -->

| Field | Value |
| --- | --- |
| ` magical_benefits_require_attunement ` | ` true ` |
| ` nonmagical_benefits_require_attunement ` | ` false ` |
| ` item_specific_exception_allowed ` | ` true ` |
