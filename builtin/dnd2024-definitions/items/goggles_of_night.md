---
splat_schema: 1
splat_id: dbda84de-126f-5a7e-bc4d-3220e15d1b56
splat_kind: item
splat_key: dnd2024:items/goggles_of_night
splat_name: Goggles of Night
splat_origin:
  key: dnd2024:reference/magic_item/goggles_of_night
splat_sources:
- title: SRD 5.2.1
  locator: pp. 224
  url: https://www.dndbeyond.com/srd
---

# Goggles of Night

## Source text

<!-- SRD page 224 -->

Wondrous Item, Uncommon

While wearing these dark lenses, you have Darkvision out to 60 feet. If you already have Darkvision, wearing the goggles increases its range by 60 feet.

Original layout: [[../../srd-5.2.1/pages/224|page 224]].

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
| ` mechanic ` | ` dnd2024:items/goggles_of_night ` |
| ` implementation ` | ` dnd2024:item_goggles_of_night ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Darkvision

<!-- splat:values path=splat_rule/parameters/darkvision -->

| Field | Value |
| --- | --- |
| ` without_existing_ft ` | ` 60 ` |
| ` existing_range_bonus_ft ` | ` 60 ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
