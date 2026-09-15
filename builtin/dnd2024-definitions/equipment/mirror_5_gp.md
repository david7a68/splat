---
splat_schema: 1
splat_id: f05f68f5-9772-5bed-b198-67bbbb36b06d
splat_kind: item
splat_key: dnd2024:equipment/mirror_5_gp
splat_name: Mirror (5 GP)
splat_origin:
  key: dnd2024:reference/pages/098
splat_sources:
- title: SRD 5.2.1
  locator: pp. 98
  url: https://www.dndbeyond.com/srd
---

# Mirror (5 GP)

## Source text

<!-- SRD page 98 -->

A handheld steel Mirror is useful for personal cos-\
metics but also for peeking around corners and re-\
flecting light as a signal.\

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
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.5 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/mirror_5_gp ` |
| ` implementation ` | ` dnd2024:equipment_mirror_5_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` material ` | ` steel ` |
| ` held ` | ` true ` |
| ` operations ` | ` ["personal_cosmetics", "peek_around_corner", "reflect_light_signal"] ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.5 ` |
| ` unit ` | ` lb ` |
