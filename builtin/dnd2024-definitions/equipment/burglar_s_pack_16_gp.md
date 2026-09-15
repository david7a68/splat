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

A Burglar’s Pack contains the following items:\
Backpack, Ball Bearings, Bell, 10 Candles, Crowbar,\
Hooded Lantern, 7 flasks of Oil, 5 days of Rations,\
Rope, Tinderbox, and Waterskin.\

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

<!-- splat:rows path=splat_rule/parameters/contents -->

| Row | quantity | unit |
| --- | --- | --- |
| ` 0 ` | ` 1 ` | ` purchase_unit ` |
| ` 1 ` | ` 1 ` | ` purchase_unit ` |
| ` 2 ` | ` 1 ` | ` purchase_unit ` |
| ` 3 ` | ` 10 ` | ` purchase_unit ` |
| ` 4 ` | ` 1 ` | ` purchase_unit ` |
| ` 5 ` | ` 1 ` | ` purchase_unit ` |
| ` 6 ` | ` 7 ` | ` purchase_unit ` |
| ` 7 ` | ` 5 ` | ` day ` |
| ` 8 ` | ` 1 ` | ` purchase_unit ` |
| ` 9 ` | ` 1 ` | ` purchase_unit ` |
| ` 10 ` | ` 1 ` | ` purchase_unit ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/0/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/backpack_2_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/1/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/ball_bearings_1_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/2/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/bell_1_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/3/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/candle_1_cp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/4/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/crowbar_2_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/5/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/lantern_hooded_5_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/6/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/oil_1_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/7/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/rations_5_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/8/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/rope_1_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/9/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/tinderbox_5_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/10/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/waterskin_2_sp ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 42.0 ` |
| ` unit ` | ` lb ` |
