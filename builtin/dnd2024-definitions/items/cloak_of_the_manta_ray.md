---
splat_schema: 1
splat_id: c02fe88c-e434-5de9-aa45-c00a1c9b376d
splat_kind: item
splat_key: dnd2024:items/cloak_of_the_manta_ray
splat_name: Cloak of the Manta Ray
splat_origin:
  key: dnd2024:reference/magic_item/cloak_of_the_manta_ray
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
---

# Cloak of the Manta Ray

## Source text

<!-- SRD page 216 -->

Wondrous Item, Uncommon (Requires Attunement)\
While wearing this cloak, you can breathe underwa-\
ter, and you have a Swim Speed of 60 feet.\

Original layout: [[../../srd-5.2.1/pages/216|page 216]].

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
| ` mechanic ` | ` dnd2024:items/cloak_of_the_manta_ray ` |
| ` implementation ` | ` dnd2024:item_cloak_of_the_manta_ray ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` breathing ` | ` ["underwater"] ` |
| ` swim_speed_ft ` | ` 60 ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
