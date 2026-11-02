---
splat_schema: 1
splat_id: 836b656b-3b3d-56d0-88af-f9c9a42b32ed
splat_kind: item
splat_key: dnd2024:items/ring_of_invisibility
splat_name: Ring of Invisibility
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_invisibility
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238
  url: https://www.dndbeyond.com/srd
---

# Ring of Invisibility

## Source text

<!-- SRD page 238 -->

Ring, Legendary (Requires Attunement)

While wearing this ring, you can take a Magic action to give yourself the Invisible condition. You remain Invisible until the ring is removed or until you take a Bonus Action to become visible again.

Original layout: [[../../srd-5.2.1/pages/238|page 238]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` ring ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_invisibility ` |
| ` implementation ` | ` dnd2024:item_ring_of_invisibility ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn_on_finger_or_similar_digit ` |
| ` ends_on ` | ` ["ring_removed", "user_bonus_action_end"] ` |
| ` attack_or_cast_ends_effect ` | ` false ` |

#### Activate

<!-- splat:values path=splat_rule/parameters/activate -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` condition ` | ` invisible ` |
| ` target ` | ` self ` |

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
