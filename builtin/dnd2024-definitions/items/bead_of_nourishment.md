---
splat_schema: 1
splat_id: 834fece5-d33d-57c5-950b-87fe340a339f
splat_kind: item
splat_key: dnd2024:items/bead_of_nourishment
splat_name: Bead of Nourishment
splat_origin:
  key: dnd2024:reference/magic_item/bead_of_nourishment
splat_sources:
- title: SRD 5.2.1
  locator: pp. 213
  url: https://www.dndbeyond.com/srd
---

# Bead of Nourishment

## Source text

<!-- SRD page 213 -->

Wondrous Item, Common

This flavorless, gelatinous bead dissolves on your tongue and provides as much nourishment as 1 day of Rations.

Original layout: [[../../srd-5.2.1/pages/213|page 213]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` common ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/bead_of_nourishment ` |
| ` implementation ` | ` dnd2024:item_bead_of_nourishment ` |

### Parameters

#### Consumption

<!-- splat:values path=splat_rule/parameters/consumption -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` dissolve_on_tongue ` |
| ` consume_item ` | ` true ` |
| ` nourishment_days_of_rations ` | ` 1 ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
