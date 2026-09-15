---
splat_schema: 1
splat_id: cb7096af-bc9c-59a0-a8de-ec1b26e70cc4
splat_kind: item
splat_key: dnd2024:items/ring_of_protection
splat_name: Ring of Protection
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_protection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238
  url: https://www.dndbeyond.com/srd
---

# Ring of Protection

## Source text

<!-- SRD page 238 -->

Ring, Rare (Requires Attunement)\
You gain a +1 bonus to Armor Class and saving\
throws while wearing this ring.\

Original layout: [[../../srd-5.2.1/pages/238|page 238]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` ring ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_protection ` |
| ` implementation ` | ` dnd2024:item_ring_of_protection ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn_on_finger_or_similar_digit ` |
| ` ac_bonus ` | ` 1 ` |
| ` saving_throw_bonus ` | ` 1 ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_rings ` |
