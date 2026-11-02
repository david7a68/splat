---
splat_schema: 1
splat_id: cb02a936-1e4b-53d6-9cb1-a396c3205902
splat_kind: item
splat_key: dnd2024:items/eyes_of_minute_seeing
splat_name: Eyes of Minute Seeing
splat_origin:
  key: dnd2024:reference/magic_item/eyes_of_minute_seeing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 221
  url: https://www.dndbeyond.com/srd
---

# Eyes of Minute Seeing

## Source text

<!-- SRD page 221 -->

Wondrous Item, Uncommon

These crystal lenses fit over the eyes. While wearing them, your vision improves significantly out to a range of 1 foot, granting you Darkvision within that range and Advantage on Intelligence (Investigation) checks made to examine something within that range.

Original layout: [[../../srd-5.2.1/pages/221|page 221]].

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
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/eyes_of_minute_seeing ` |
| ` implementation ` | ` dnd2024:item_eyes_of_minute_seeing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` darkvision_ft ` | ` 1 ` |

#### Investigation

<!-- splat:values path=splat_rule/parameters/investigation -->

| Field | Value |
| --- | --- |
| ` advantage ` | ` true ` |
| ` target_distance_ft ` | ` 1 ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
