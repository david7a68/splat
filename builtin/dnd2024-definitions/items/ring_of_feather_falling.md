---
splat_schema: 1
splat_id: 53b73aa5-d0ca-5d4d-a52f-4c0f65fe3cb7
splat_kind: item
splat_key: dnd2024:items/ring_of_feather_falling
splat_name: Ring of Feather Falling
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_feather_falling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238
  url: https://www.dndbeyond.com/srd
---

# Ring of Feather Falling

## Source text

<!-- SRD page 238 -->

Ring, Rare (Requires Attunement)

When you fall while wearing this ring, you descend 60 feet per round and take no damage from falling.

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
| ` mechanic ` | ` dnd2024:items/ring_of_feather_falling ` |
| ` implementation ` | ` dnd2024:item_ring_of_feather_falling ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` trigger ` | ` falling ` |
| ` descent_ft_per_round ` | ` 60 ` |
| ` fall_damage ` | ` 0 ` |
| ` attunement_required ` | ` true ` |
