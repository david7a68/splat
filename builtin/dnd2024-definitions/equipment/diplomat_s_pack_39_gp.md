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

A Diplomat’s Pack contains the following items:\
Chest, Fine Clothes, Ink, 5 Ink Pens, Lamp, 2 Map\
or Scroll Cases, 4 flasks of Oil, 5 sheets of Paper, 5\
sheets of Parchment, Perfume, and Tinderbox.\

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

<!-- splat:rows path=splat_rule/parameters/contents -->

| Row | quantity | unit |
| --- | --- | --- |
| ` 0 ` | ` 1 ` | ` purchase_unit ` |
| ` 1 ` | ` 1 ` | ` purchase_unit ` |
| ` 2 ` | ` 1 ` | ` purchase_unit ` |
| ` 3 ` | ` 5 ` | ` purchase_unit ` |
| ` 4 ` | ` 1 ` | ` purchase_unit ` |
| ` 5 ` | ` 2 ` | ` purchase_unit ` |
| ` 6 ` | ` 4 ` | ` purchase_unit ` |
| ` 7 ` | ` 5 ` | ` purchase_unit ` |
| ` 8 ` | ` 5 ` | ` purchase_unit ` |
| ` 9 ` | ` 1 ` | ` purchase_unit ` |
| ` 10 ` | ` 1 ` | ` purchase_unit ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/0/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/chest_5_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/1/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/clothes_fine_15_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/2/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/ink_10_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/3/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/ink_pen_2_cp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/4/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/lamp_5_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/5/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/case_map_or_scroll_1_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/6/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/oil_1_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/7/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/paper_2_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/8/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/parchment_1_sp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/9/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/perfume_5_gp ` |

##### Item

<!-- splat:values path=splat_rule/parameters/contents/10/item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/tinderbox_5_sp ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 39.0 ` |
| ` unit ` | ` lb ` |
