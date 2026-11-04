---
splat_schema: 2
splat_id: 295e1c4b-fe8f-5ea6-8b40-00531a7419fd
splat_kind: item
splat_key: dnd2024:equipment/dungeoneer_s_pack_12_gp
splat_name: Dungeoneer’s Pack (12 GP)
splat_origin: dnd2024:reference/pages/097
splat_sources:
- SRD 5.2.1, pp. 97, https://www.dndbeyond.com/srd
---

# Dungeoneer’s Pack (12 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 97 -->

A Dungeoneer’s Pack contains the following items: Backpack, Caltrops, Crowbar, 2 flasks of Oil, 10 days of Rations, Rope, Tinderbox, 10 Torches, and Waterskin.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/dungeoneer_s_pack_12_gp ` |
| ` implementation ` | ` dnd2024:equipment_dungeoneer_s_pack_12_gp ` |

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
- caltrops_1_gp
- crowbar_2_gp
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
| [Caltrops (1 GP)](<caltrops_1_gp.md> "dnd2024:equipment/caltrops_1_gp") | ` 1 ` | ` Purchase unit ` |
| [Crowbar (2 GP)](<crowbar_2_gp.md> "dnd2024:equipment/crowbar_2_gp") | ` 1 ` | ` Purchase unit ` |
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 12
> mass:
>   unit: lb
>   value: 55.0
> ```

