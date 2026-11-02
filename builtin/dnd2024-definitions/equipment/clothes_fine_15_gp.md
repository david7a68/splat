---
splat_schema: 1
splat_id: 64766bd6-c8ff-50e0-8e32-cd9b03056446
splat_kind: item
splat_key: dnd2024:equipment/clothes_fine_15_gp
splat_name: Clothes, Fine (15 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
---

# Clothes, Fine (15 GP)

## Source text

<!-- SRD page 97 -->

Fine Clothes are made of expensive fabrics and adorned with expertly crafted details. Some events and locations admit only people wearing these clothes.

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
| ` value ` | ` 15 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 6.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/clothes_fine_15_gp ` |
| ` implementation ` | ` dnd2024:equipment_clothes_fine_15_gp ` |

### Parameters

#### Access requirement

<!-- splat:values path=splat_rule/parameters/access_requirement -->

| Field | Value |
| --- | --- |
| ` authority ` | ` dm ` |
| ` scope ` | ` ["event", "location"] ` |
| ` eligible_if_wearing ` | ` true ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 6.0 ` |
| ` unit ` | ` lb ` |
