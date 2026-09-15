---
splat_schema: 1
splat_id: ac6d8d9a-faa7-563d-b2ad-29ad961d51d5
splat_kind: item
splat_key: dnd2024:equipment/explorer_s_pack_10_gp
splat_name: Explorer’s Pack (10 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
---

# Explorer’s Pack (10 GP)

## Source text

<!-- SRD page 97 -->

An Explorer’s Pack contains the following items:\
Backpack, Bedroll, 2 flasks of Oil, 10 days of Ra-\
tions, Rope, Tinderbox, 10 Torches, and Waterskin.\

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
| ` value ` | ` 10 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 55.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/explorer_s_pack_10_gp ` |
| ` implementation ` | ` dnd2024:equipment_explorer_s_pack_10_gp ` |

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
- bedroll_1_gp
- oil_1_sp
- rations_5_sp
- rope_1_gp
- tinderbox_5_sp
- torch_1_cp
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
| [Bedroll (1 GP)](<bedroll_1_gp.md> "dnd2024:equipment/bedroll_1_gp") | ` 1 ` | ` Purchase unit ` |
| [Oil (1 SP)](<oil_1_sp.md> "dnd2024:equipment/oil_1_sp") | ` 2 ` | ` Purchase unit ` |
| [Rations (5 SP)](<rations_5_sp.md> "dnd2024:equipment/rations_5_sp") | ` 10 ` | ` Day ` |
| [Rope (1 GP)](<rope_1_gp.md> "dnd2024:equipment/rope_1_gp") | ` 1 ` | ` Purchase unit ` |
| [Tinderbox (5 SP)](<tinderbox_5_sp.md> "dnd2024:equipment/tinderbox_5_sp") | ` 1 ` | ` Purchase unit ` |
| [Torch (1 CP)](<torch_1_cp.md> "dnd2024:equipment/torch_1_cp") | ` 10 ` | ` Purchase unit ` |
| [Waterskin (2 SP)](<waterskin_2_sp.md> "dnd2024:equipment/waterskin_2_sp") | ` 1 ` | ` Purchase unit ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 55.0 ` |
| ` unit ` | ` lb ` |
