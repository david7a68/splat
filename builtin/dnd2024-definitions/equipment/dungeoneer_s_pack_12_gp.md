---
splat_schema: 1
splat_id: 295e1c4b-fe8f-5ea6-8b40-00531a7419fd
splat_kind: item
splat_key: dnd2024:equipment/dungeoneer_s_pack_12_gp
splat_name: Dungeoneer’s Pack (12 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
---

# Dungeoneer’s Pack (12 GP)

## Source text

<!-- SRD page 97 -->

A Dungeoneer’s Pack contains the following items:\
Backpack, Caltrops, Crowbar, 2 flasks of Oil, 10\
days of Rations, Rope, Tinderbox, 10 Torches, and\
Waterskin.\

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
| ` value ` | ` 12 ` |
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
| ` mechanic ` | ` dnd2024:equipment/dungeoneer_s_pack_12_gp ` |
| ` implementation ` | ` dnd2024:equipment_dungeoneer_s_pack_12_gp ` |

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
| ` 3 ` | ` 2 ` | ` purchase_unit ` |
| ` 4 ` | ` 10 ` | ` day ` |
| ` 5 ` | ` 1 ` | ` purchase_unit ` |
| ` 6 ` | ` 1 ` | ` purchase_unit ` |
| ` 7 ` | ` 10 ` | ` purchase_unit ` |
| ` 8 ` | ` 1 ` | ` purchase_unit ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/0/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/backpack_2_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/1/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/caltrops_1_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/2/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/crowbar_2_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/3/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/oil_1_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/4/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/rations_5_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/5/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/rope_1_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/6/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/tinderbox_5_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/7/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/torch_1_cp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/8/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/waterskin_2_sp ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 55.0 ` |
| ` unit ` | ` lb ` |
