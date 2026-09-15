---
splat_schema: 1
splat_id: a86bb862-32fd-5f90-8086-40ae456a5d04
splat_kind: item
splat_key: dnd2024:equipment/tent_2_gp
splat_name: Tent (2 GP)
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
---

# Tent (2 GP)

## Source text

<!-- SRD page 100 -->

A Tent sleeps up to two Small or Medium creatures.\

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
| ` value ` | ` 2 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 20.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/tent_2_gp ` |
| ` implementation ` | ` dnd2024:equipment_tent_2_gp ` |

### Parameters

#### Sleep capacity

<!-- splat:values path=splat_rule/parameters/sleep_capacity -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` sizes ` | ` ["small", "medium"] ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 20.0 ` |
| ` unit ` | ` lb ` |
