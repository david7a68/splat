---
splat_schema: 1
splat_id: 0b509d42-d42f-5046-9d89-7acb9cc4312b
splat_kind: item
splat_key: dnd2024:items/ring_of_swimming
splat_name: Ring of Swimming
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_swimming
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239
  url: https://www.dndbeyond.com/srd
---

# Ring of Swimming

## Source text

<!-- SRD page 239 -->

Ring, Uncommon\
You have a Swim Speed of 40 feet while wearing this\
ring.\

Original layout: [[../../srd-5.2.1/pages/239|page 239]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` ring ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_swimming ` |
| ` implementation ` | ` dnd2024:item_ring_of_swimming ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn_on_finger_or_similar_digit ` |
| ` swim_speed_ft ` | ` 40 ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_rings ` |
