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

An Entertainer’s Pack contains the following items:\
Backpack, Bedroll, Bell, Bullseye Lantern, 3 Cos-\
tumes, Mirror, 8 flasks of Oil, 9 days of Rations, Tin-\
derbox, and Waterskin.\

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

<!-- splat:rows path=splat_rule/parameters/contents -->

| Row | quantity | unit |
| --- | --- | --- |
| ` 0 ` | ` 1 ` | ` purchase_unit ` |
| ` 1 ` | ` 1 ` | ` purchase_unit ` |
| ` 2 ` | ` 1 ` | ` purchase_unit ` |
| ` 3 ` | ` 1 ` | ` purchase_unit ` |
| ` 4 ` | ` 3 ` | ` purchase_unit ` |
| ` 5 ` | ` 1 ` | ` purchase_unit ` |
| ` 6 ` | ` 8 ` | ` purchase_unit ` |
| ` 7 ` | ` 9 ` | ` day ` |
| ` 8 ` | ` 1 ` | ` purchase_unit ` |
| ` 9 ` | ` 1 ` | ` purchase_unit ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/0/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/backpack_2_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/1/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/bedroll_1_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/2/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/bell_1_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/3/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/lantern_bullseye_10_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/4/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/costume_5_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/5/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/mirror_5_gp ` |

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
| ` key ` | ` dnd2024:equipment/tinderbox_5_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/9/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/waterskin_2_sp ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 58.5 ` |
| ` unit ` | ` lb ` |
