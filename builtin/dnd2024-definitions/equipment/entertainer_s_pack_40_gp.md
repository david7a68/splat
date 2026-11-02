---
splat_schema: 1
splat_id: 61429588-f735-549e-9b5e-38558a098248
splat_kind: item
splat_key: dnd2024:equipment/entertainer_s_pack_40_gp
splat_name: Entertainer’s Pack (40 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
---

# Entertainer’s Pack (40 GP)

## Source text

<!-- SRD page 97 -->

An Entertainer’s Pack contains the following items: Backpack, Bedroll, Bell, Bullseye Lantern, 3 Costumes, Mirror, 8 flasks of Oil, 9 days of Rations, Tinderbox, and Waterskin.

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
| ` value ` | ` 58.5 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/entertainer_s_pack_40_gp ` |
| ` implementation ` | ` dnd2024:equipment_entertainer_s_pack_40_gp ` |

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
- bell_1_gp
- lantern_bullseye_10_gp
- costume_5_gp
- mirror_5_gp
- oil_1_sp
- rations_5_sp
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
| [Bedroll (1 GP)](<bedroll_1_gp.md> "dnd2024:equipment/bedroll_1_gp") | ` 1 ` | ` Purchase unit ` |
| [Bell (1 GP)](<bell_1_gp.md> "dnd2024:equipment/bell_1_gp") | ` 1 ` | ` Purchase unit ` |
| [Lantern, Bullseye (10 GP)](<lantern_bullseye_10_gp.md> "dnd2024:equipment/lantern_bullseye_10_gp") | ` 1 ` | ` Purchase unit ` |
| [Costume (5 GP)](<costume_5_gp.md> "dnd2024:equipment/costume_5_gp") | ` 3 ` | ` Purchase unit ` |
| [Mirror (5 GP)](<mirror_5_gp.md> "dnd2024:equipment/mirror_5_gp") | ` 1 ` | ` Purchase unit ` |
| [Oil (1 SP)](<oil_1_sp.md> "dnd2024:equipment/oil_1_sp") | ` 8 ` | ` Purchase unit ` |
| [Rations (5 SP)](<rations_5_sp.md> "dnd2024:equipment/rations_5_sp") | ` 9 ` | ` Day ` |
| [Tinderbox (5 SP)](<tinderbox_5_sp.md> "dnd2024:equipment/tinderbox_5_sp") | ` 1 ` | ` Purchase unit ` |
| [Waterskin (2 SP)](<waterskin_2_sp.md> "dnd2024:equipment/waterskin_2_sp") | ` 1 ` | ` Purchase unit ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 58.5 ` |
| ` unit ` | ` lb ` |
