---
splat_schema: 1
splat_id: 2f495087-e0fb-552c-9f6f-d587b25e9b14
splat_kind: item
splat_key: dnd2024:equipment/sack_1_cp
splat_name: Sack (1 CP)
splat_origin:
  key: dnd2024:reference/pages/099
splat_sources:
- title: SRD 5.2.1
  locator: pp. 99
  url: https://www.dndbeyond.com/srd
---

# Sack (1 CP)

## Source text

<!-- SRD page 99 -->

A Sack holds up to 30 pounds within 1 cubic foot.

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
| ` value ` | ` 1 ` |
| ` unit ` | ` cp ` |

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
| ` mechanic ` | ` dnd2024:equipment/sack_1_cp ` |
| ` implementation ` | ` dnd2024:equipment_sack_1_cp ` |

### Parameters

#### Capacity

<!-- splat:values path=splat_rule/parameters/capacity -->

| Field | Value |
| --- | --- |
| ` mass_lb ` | ` 30 ` |
| ` volume_ft3 ` | ` 1 ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.5 ` |
| ` unit ` | ` lb ` |
