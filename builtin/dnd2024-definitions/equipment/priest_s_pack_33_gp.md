---
splat_schema: 2
splat_id: b42094da-a7f0-5ca4-9f72-a1d9357cd344
splat_kind: item
splat_key: dnd2024:equipment/priest_s_pack_33_gp
splat_name: Priest’s Pack (33 GP)
splat_origin: dnd2024:reference/pages/099
splat_sources:
- SRD 5.2.1, pp. 99, https://www.dndbeyond.com/srd
---

# Priest’s Pack (33 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 99 -->

A Priest’s Pack contains the following items: Backpack, Blanket, Holy Water, Lamp, 7 days of Rations, Robe, and Tinderbox.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/priest_s_pack_33_gp ` |
| ` implementation ` | ` dnd2024:equipment_priest_s_pack_33_gp ` |

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
- blanket_5_sp
- holy_water_25_gp
- lamp_5_sp
- rations_5_sp
- robe_1_gp
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
| [Blanket (5 SP)](<blanket_5_sp.md> "dnd2024:equipment/blanket_5_sp") | ` 1 ` | ` Purchase unit ` |
| [Holy Water (25 GP)](<holy_water_25_gp.md> "dnd2024:equipment/holy_water_25_gp") | ` 1 ` | ` Purchase unit ` |
| [Lamp (5 SP)](<lamp_5_sp.md> "dnd2024:equipment/lamp_5_sp") | ` 1 ` | ` Purchase unit ` |
| [Rations (5 SP)](<rations_5_sp.md> "dnd2024:equipment/rations_5_sp") | ` 7 ` | ` Day ` |
| [Robe (1 GP)](<robe_1_gp.md> "dnd2024:equipment/robe_1_gp") | ` 1 ` | ` Purchase unit ` |
| [Tinderbox (5 SP)](<tinderbox_5_sp.md> "dnd2024:equipment/tinderbox_5_sp") | ` 1 ` | ` Purchase unit ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 29.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 33
> mass:
>   unit: lb
>   value: 29.0
> ```

