---
splat_schema: 1
splat_id: 5c5a485f-c32d-5032-b88d-2e293d44ebcf
splat_kind: item
splat_key: dnd2024:items/ring_of_x_ray_vision
splat_name: Ring of X-ray Vision
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_x_ray_vision
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239, 240
  url: https://www.dndbeyond.com/srd
---

# Ring of X-ray Vision

## Source text

<!-- SRD page 239 -->

Ring, Rare (Requires Attunement)

While wearing this ring, you can take a Magic action to gain X-ray vision with a range of 30 feet for 1 minute. To you, solid objects within that radius appear transparent and don’t prevent light from passing through them. The vision can penetrate 1 foot of stone, 1 inch of common metal, or up to 3 feet of wood or dirt. Thicker substances or a thin sheet of lead block the vision.

<!-- SRD page 240 -->

Whenever you use the ring again before taking a Long Rest, you must succeed on a DC 15 Constitution saving throw or gain 1 Exhaustion level.

Original layout: [[../../srd-5.2.1/pages/239|page 239]], [[../../srd-5.2.1/pages/240|page 240]].

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
| ` mechanic ` | ` dnd2024:items/ring_of_x_ray_vision ` |
| ` implementation ` | ` dnd2024:item_ring_of_x_ray_vision ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn_on_finger_or_similar_digit ` |

#### Activate

<!-- splat:values path=splat_rule/parameters/activate -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` range_ft ` | ` 30 ` |
| ` objects_appear_transparent ` | ` true ` |
| ` light_passes_through_for_user_vision ` | ` true ` |
| ` blocked_by ` | ` ["thicker_substances", "any_sheet_of_lead"] ` |

##### Duration

<!-- splat:values path=splat_rule/parameters/activate/duration -->

| Field | Value |
| --- | --- |
| ` amount ` | ` 1 ` |
| ` unit ` | ` minute ` |

##### Penetration limits

<!-- splat:table
path: splat_rule/parameters/activate/penetration_limits
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Material:
    path: material
    type: symbol
  Thickness inches:
    path: thickness_inches
    type: integer
-->

| Key | Material | Thickness inches |
| --- | --- | --- |
| ` row_1 ` | ` Stone ` | ` 12 ` |
| ` row_2 ` | ` Common metal ` | ` 1 ` |
| ` row_3 ` | ` Wood ` | ` 36 ` |
| ` row_4 ` | ` Dirt ` | ` 36 ` |

#### Additional uses before long rest

<!-- splat:yaml path=splat_rule/parameters/additional_uses_before_long_rest -->

```yaml
save:
  ability: constitution
  dc: 15
  failure:
    exhaustion_increase: 1
  success:
    effect: none
```

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
