---
splat_schema: 1
splat_id: 2442f30a-e4d3-5d0b-b6ce-04eabd8ebed5
splat_kind: item
splat_key: dnd2024:equipment/scholar_s_pack_40_gp
splat_name: Scholar’s Pack (40 GP)
splat_origin:
  key: dnd2024:reference/pages/099
splat_sources:
- title: SRD 5.2.1
  locator: pp. 99
  url: https://www.dndbeyond.com/srd
---

# Scholar’s Pack (40 GP)

## Source text

<!-- SRD page 99 -->

A Scholar’s Pack contains the following items: Back-\
pack, Book, Ink, Ink Pen, Lamp, 10 flasks of Oil, 10\
sheets of Parchment, and Tinderbox.\

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` adventuring_gear ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 40 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 22.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/scholar_s_pack_40_gp ` |
| ` implementation ` | ` dnd2024:equipment_scholar_s_pack_40_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` grant_contents_on_acquisition ` | ` true ` |
| ` grant_idempotency ` | ` once_per_acquired_pack_instance ` |

#### Contents

<!-- splat:rows path=splat_rule/parameters/contents -->

| Row | quantity | unit |
| --- | --- | --- |
| ` 0 ` | ` 1 ` | ` purchase_unit ` |
| ` 1 ` | ` 1 ` | ` purchase_unit ` |
| ` 2 ` | ` 1 ` | ` purchase_unit ` |
| ` 3 ` | ` 1 ` | ` purchase_unit ` |
| ` 4 ` | ` 1 ` | ` purchase_unit ` |
| ` 5 ` | ` 10 ` | ` purchase_unit ` |
| ` 6 ` | ` 10 ` | ` purchase_unit ` |
| ` 7 ` | ` 1 ` | ` purchase_unit ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/0/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/backpack_2_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/1/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/book_25_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/2/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/ink_10_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/3/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/ink_pen_2_cp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/4/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/lamp_5_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/5/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/oil_1_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/6/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/parchment_1_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/7/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/tinderbox_5_sp ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 22.0 ` |
| ` unit ` | ` lb ` |
