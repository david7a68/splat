---
splat_schema: 1
splat_id: 43d65482-fac6-5e5f-8b01-9080c296908a
splat_kind: item
splat_key: dnd2024:equipment/diplomat_s_pack_39_gp
splat_name: Diplomat’s Pack (39 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
---

# Diplomat’s Pack (39 GP)

## Source text

<!-- SRD page 97 -->

A Diplomat’s Pack contains the following items: Chest, Fine Clothes, Ink, 5 Ink Pens, Lamp, 2 Map or Scroll Cases, 4 flasks of Oil, 5 sheets of Paper, 5 sheets of Parchment, Perfume, and Tinderbox.

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
| ` value ` | ` 39 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 39.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/diplomat_s_pack_39_gp ` |
| ` implementation ` | ` dnd2024:equipment_diplomat_s_pack_39_gp ` |

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
- chest_5_gp
- clothes_fine_15_gp
- ink_10_gp
- ink_pen_2_cp
- lamp_5_sp
- case_map_or_scroll_1_gp
- oil_1_sp
- paper_2_sp
- parchment_1_sp
- perfume_5_gp
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
| [Chest (5 GP)](<chest_5_gp.md> "dnd2024:equipment/chest_5_gp") | ` 1 ` | ` Purchase unit ` |
| [Clothes, Fine (15 GP)](<clothes_fine_15_gp.md> "dnd2024:equipment/clothes_fine_15_gp") | ` 1 ` | ` Purchase unit ` |
| [Ink (10 GP)](<ink_10_gp.md> "dnd2024:equipment/ink_10_gp") | ` 1 ` | ` Purchase unit ` |
| [Ink Pen (2 CP)](<ink_pen_2_cp.md> "dnd2024:equipment/ink_pen_2_cp") | ` 5 ` | ` Purchase unit ` |
| [Lamp (5 SP)](<lamp_5_sp.md> "dnd2024:equipment/lamp_5_sp") | ` 1 ` | ` Purchase unit ` |
| [Case, Map or Scroll (1 GP)](<case_map_or_scroll_1_gp.md> "dnd2024:equipment/case_map_or_scroll_1_gp") | ` 2 ` | ` Purchase unit ` |
| [Oil (1 SP)](<oil_1_sp.md> "dnd2024:equipment/oil_1_sp") | ` 4 ` | ` Purchase unit ` |
| [Paper (2 SP)](<paper_2_sp.md> "dnd2024:equipment/paper_2_sp") | ` 5 ` | ` Purchase unit ` |
| [Parchment (1 SP)](<parchment_1_sp.md> "dnd2024:equipment/parchment_1_sp") | ` 5 ` | ` Purchase unit ` |
| [Perfume (5 GP)](<perfume_5_gp.md> "dnd2024:equipment/perfume_5_gp") | ` 1 ` | ` Purchase unit ` |
| [Tinderbox (5 SP)](<tinderbox_5_sp.md> "dnd2024:equipment/tinderbox_5_sp") | ` 1 ` | ` Purchase unit ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 39.0 ` |
| ` unit ` | ` lb ` |
