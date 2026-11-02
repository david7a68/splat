---
splat_schema: 1
splat_id: 725d861d-fce4-55f8-b9c5-b5416518d9a2
splat_kind: item
splat_key: dnd2024:equipment/ink_pen_2_cp
splat_name: Ink Pen (2 CP)
splat_origin:
  key: dnd2024:reference/pages/098
splat_sources:
- title: SRD 5.2.1
  locator: pp. 98
  url: https://www.dndbeyond.com/srd
---

# Ink Pen (2 CP)

## Source text

<!-- SRD page 98 -->

Using Ink, an Ink Pen is used to write or draw.

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
| ` unit ` | ` cp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/ink_pen_2_cp ` |
| ` implementation ` | ` dnd2024:equipment_ink_pen_2_cp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` operations ` | ` ["write", "draw"] ` |

#### Requires item

<!-- splat:values path=splat_rule/parameters/requires_item -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/ink_10_gp ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.0 ` |
| ` unit ` | ` lb ` |
| ` negligible ` | ` true ` |
