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

<!-- splat:table
path: splat_rule/parameters/contents
key: Item
order:
- backpack_2_gp
- book_25_gp
- ink_10_gp
- ink_pen_2_cp
- lamp_5_sp
- oil_1_sp
- parchment_1_sp
- tinderbox_5_sp
columns:
  Item:
    path: item
    type: reference
  Quantity:
    path: quantity
    type: integer
  Unit:
    path: unit
    type: symbol
-->

| Item | Quantity | Unit |
| --- | --- | --- |
| [Backpack (2 GP)](<backpack_2_gp.md> "dnd2024:equipment/backpack_2_gp") | ` 1 ` | ` Purchase unit ` |
| [Book (25 GP)](<book_25_gp.md> "dnd2024:equipment/book_25_gp") | ` 1 ` | ` Purchase unit ` |
| [Ink (10 GP)](<ink_10_gp.md> "dnd2024:equipment/ink_10_gp") | ` 1 ` | ` Purchase unit ` |
| [Ink Pen (2 CP)](<ink_pen_2_cp.md> "dnd2024:equipment/ink_pen_2_cp") | ` 1 ` | ` Purchase unit ` |
| [Lamp (5 SP)](<lamp_5_sp.md> "dnd2024:equipment/lamp_5_sp") | ` 1 ` | ` Purchase unit ` |
| [Oil (1 SP)](<oil_1_sp.md> "dnd2024:equipment/oil_1_sp") | ` 10 ` | ` Purchase unit ` |
| [Parchment (1 SP)](<parchment_1_sp.md> "dnd2024:equipment/parchment_1_sp") | ` 10 ` | ` Purchase unit ` |
| [Tinderbox (5 SP)](<tinderbox_5_sp.md> "dnd2024:equipment/tinderbox_5_sp") | ` 1 ` | ` Purchase unit ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 22.0 ` |
| ` unit ` | ` lb ` |
