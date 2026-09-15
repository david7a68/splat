---
splat_schema: 1
splat_id: a7433f8d-e2a1-5936-81d8-5bb4a482b18c
splat_kind: item
splat_key: dnd2024:equipment/blanket_5_sp
splat_name: Blanket (5 SP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
---

# Blanket (5 SP)

## Source text

<!-- SRD page 96 -->

While wrapped in a blanket, you have Advantage on\
saving throws against extreme cold (see “Gameplay\
Toolbox”).\

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
| ` value ` | ` 3.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/blanket_5_sp ` |
| ` implementation ` | ` dnd2024:equipment_blanket_5_sp ` |

### Parameters

#### While wrapped

<!-- splat:values path=splat_rule/parameters/while_wrapped -->

| Field | Value |
| --- | --- |
| ` roll ` | ` saving_throw ` |
| ` mode ` | ` advantage ` |
| ` scope ` | ` extreme_cold ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 3.0 ` |
| ` unit ` | ` lb ` |
