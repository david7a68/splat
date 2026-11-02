---
splat_schema: 1
splat_id: 184d7087-e103-52ea-9a0a-026f7e09b6bb
splat_kind: item
splat_key: dnd2024:items/headband_of_intellect
splat_name: Headband of Intellect
splat_origin:
  key: dnd2024:reference/magic_item/headband_of_intellect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 225
  url: https://www.dndbeyond.com/srd
---

# Headband of Intellect

## Source text

<!-- SRD page 225 -->

Wondrous Item, Uncommon (Requires Attunement)

Your Intelligence is 19 while you wear this headband. It has no effect on you if your Intelligence is 19 or higher without it.

Original layout: [[../../srd-5.2.1/pages/225|page 225]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/headband_of_intellect ` |
| ` implementation ` | ` dnd2024:item_headband_of_intellect ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Ability floor

<!-- splat:values path=splat_rule/parameters/ability_floor -->

| Field | Value |
| --- | --- |
| ` ability ` | ` intelligence ` |
| ` score ` | ` 19 ` |
| ` requires_unmodified_score_below ` | ` 19 ` |
