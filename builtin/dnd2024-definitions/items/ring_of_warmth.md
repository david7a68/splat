---
splat_schema: 1
splat_id: 93e59450-81e0-5b89-b4e4-50ba170b8500
splat_kind: item
splat_key: dnd2024:items/ring_of_warmth
splat_name: Ring of Warmth
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_warmth
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239
  url: https://www.dndbeyond.com/srd
---

# Ring of Warmth

## Source text

<!-- SRD page 239 -->

Ring, Uncommon (Requires Attunement)

If you take Cold damage while wearing this ring, the ring reduces the damage you take by 2d8.

In addition, while wearing this ring, you and everything you wear and carry are unharmed by temperatures of 0 degrees Fahrenheit or lower.

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
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_warmth ` |
| ` implementation ` | ` dnd2024:item_ring_of_warmth ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Damage reduction

<!-- splat:yaml path=splat_rule/parameters/damage_reduction -->

```yaml
type: cold
amount:
  roll:
    count: 2
    sides: 8
minimum_final_damage: 0
```

#### Temperature protection

<!-- splat:values path=splat_rule/parameters/temperature_protection -->

| Field | Value |
| --- | --- |
| ` fahrenheit_maximum ` | ` 0 ` |
| ` lower_limit ` | ` null ` |
| ` targets ` | ` ["wearer", "worn_items", "carried_items"] ` |
