---
splat_schema: 1
splat_id: 580f3471-1bd4-5fcc-b770-f3bf9d7b2dc9
splat_kind: item
splat_key: dnd2024:equipment/burglar_s_pack_16_gp
splat_name: Burglar’s Pack (16 GP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
---

# Burglar’s Pack (16 GP)

## Source text

<!-- SRD page 96 -->

A Burglar’s Pack contains the following items: Backpack, Ball Bearings, Bell, 10 Candles, Crowbar, Hooded Lantern, 7 flasks of Oil, 5 days of Rations, Rope, Tinderbox, and Waterskin.

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
| ` value ` | ` 16 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 42.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/burglar_s_pack_16_gp ` |
| ` implementation ` | ` dnd2024:equipment_burglar_s_pack_16_gp ` |

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
- ball_bearings_1_gp
- bell_1_gp
- candle_1_cp
- crowbar_2_gp
- lantern_hooded_5_gp
- oil_1_sp
- rations_5_sp
- rope_1_gp
- tinderbox_5_sp
- waterskin_2_sp
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
| [Ball Bearings (1 GP)](<ball_bearings_1_gp.md> "dnd2024:equipment/ball_bearings_1_gp") | ` 1 ` | ` Purchase unit ` |
| [Bell (1 GP)](<bell_1_gp.md> "dnd2024:equipment/bell_1_gp") | ` 1 ` | ` Purchase unit ` |
| [Candle (1 CP)](<candle_1_cp.md> "dnd2024:equipment/candle_1_cp") | ` 10 ` | ` Purchase unit ` |
| [Crowbar (2 GP)](<crowbar_2_gp.md> "dnd2024:equipment/crowbar_2_gp") | ` 1 ` | ` Purchase unit ` |
| [Lantern, Hooded (5 GP)](<lantern_hooded_5_gp.md> "dnd2024:equipment/lantern_hooded_5_gp") | ` 1 ` | ` Purchase unit ` |
| [Oil (1 SP)](<oil_1_sp.md> "dnd2024:equipment/oil_1_sp") | ` 7 ` | ` Purchase unit ` |
| [Rations (5 SP)](<rations_5_sp.md> "dnd2024:equipment/rations_5_sp") | ` 5 ` | ` Day ` |
| [Rope (1 GP)](<rope_1_gp.md> "dnd2024:equipment/rope_1_gp") | ` 1 ` | ` Purchase unit ` |
| [Tinderbox (5 SP)](<tinderbox_5_sp.md> "dnd2024:equipment/tinderbox_5_sp") | ` 1 ` | ` Purchase unit ` |
| [Waterskin (2 SP)](<waterskin_2_sp.md> "dnd2024:equipment/waterskin_2_sp") | ` 1 ` | ` Purchase unit ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 42.0 ` |
| ` unit ` | ` lb ` |
