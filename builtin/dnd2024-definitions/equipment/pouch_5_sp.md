---
splat_schema: 1
splat_id: b1b73d18-7615-589b-9425-3e74a3ec54c8
splat_kind: item
splat_key: dnd2024:equipment/pouch_5_sp
splat_name: Pouch (5 SP)
splat_origin:
  key: dnd2024:reference/pages/099
splat_sources:
- title: SRD 5.2.1
  locator: pp. 99
  url: https://www.dndbeyond.com/srd
---

# Pouch (5 SP)

## Source text

<!-- SRD page 99 -->

A Pouch holds up to 6 pounds within one-fifth of a\
cubic foot.\

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
| ` value ` | ` 5 ` |
| ` unit ` | ` sp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/pouch_5_sp ` |
| ` implementation ` | ` dnd2024:equipment_pouch_5_sp ` |

### Parameters

#### Capacity

<!-- splat:values path=splat_rule/parameters/capacity -->

| Field | Value |
| --- | --- |
| ` mass_lb ` | ` 6 ` |
| ` volume_ft3 ` | ` 0.2 ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |
