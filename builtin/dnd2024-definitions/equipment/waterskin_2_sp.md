---
splat_schema: 1
splat_id: 23793d47-d030-5aa6-8ced-c834314af839
splat_kind: item
splat_key: dnd2024:equipment/waterskin_2_sp
splat_name: Waterskin (2 SP)
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
---

# Waterskin (2 SP)

## Source text

<!-- SRD page 100 -->

A Waterskin holds up to 4 pints. If you don’t drink sufficient water, you risk dehydration (see “Rules Glossary”).

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
| ` unit ` | ` sp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/waterskin_2_sp ` |
| ` implementation ` | ` dnd2024:equipment_waterskin_2_sp ` |

### Parameters

#### Capacity

<!-- splat:values path=splat_rule/parameters/capacity -->

| Field | Value |
| --- | --- |
| ` liquid_pints ` | ` 4 ` |

#### Nutrition rule

<!-- splat:values path=splat_rule/parameters/nutrition_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/dehydration_hazard ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5.0 ` |
| ` unit ` | ` lb ` |
| ` when ` | ` full ` |
