---
splat_schema: 1
splat_id: f4efbbe1-2fb0-5d26-9b2b-cd250764ceac
splat_kind: item
splat_key: dnd2024:equipment/rations_5_sp
splat_name: Rations (5 SP)
splat_origin:
  key: dnd2024:reference/pages/099
splat_sources:
- title: SRD 5.2.1
  locator: pp. 99
  url: https://www.dndbeyond.com/srd
---

# Rations (5 SP)

## Source text

<!-- SRD page 99 -->

Rations consist of travel-ready food, including jerky, dried fruit, hardtack, and nuts. See “Malnutrition” in “Rules Glossary” for the risks of not eating.

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
| ` value ` | ` 2.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/rations_5_sp ` |
| ` implementation ` | ` dnd2024:equipment_rations_5_sp ` |

### Parameters

#### Food

<!-- splat:values path=splat_rule/parameters/food -->

| Field | Value |
| --- | --- |
| ` travel_ready ` | ` true ` |
| ` components ` | ` ["jerky", "dried_fruit", "hardtack", "nuts"] ` |
| ` mass_lb ` | ` 2 ` |

#### Nutrition rule

<!-- splat:values path=splat_rule/parameters/nutrition_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/malnutrition_hazard ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2.0 ` |
| ` unit ` | ` lb ` |
