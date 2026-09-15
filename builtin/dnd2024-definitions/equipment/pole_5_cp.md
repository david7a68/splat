---
splat_schema: 1
splat_id: 0d82864d-775d-549a-aece-1e16c16a7233
splat_kind: item
splat_key: dnd2024:equipment/pole_5_cp
splat_name: Pole (5 CP)
splat_origin:
  key: dnd2024:reference/pages/099
splat_sources:
- title: SRD 5.2.1
  locator: pp. 99
  url: https://www.dndbeyond.com/srd
---

# Pole (5 CP)

## Source text

<!-- SRD page 99 -->

A Pole is 10 feet long. You can use it to touch\
something up to 10 feet away. If you must make a\
Strength (Athletics) check as part of a High or Long\
Jump, you can use the Pole to vault, giving yourself\
Advantage on the check.\

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
| ` unit ` | ` cp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 7.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/pole_5_cp ` |
| ` implementation ` | ` dnd2024:equipment_pole_5_cp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` length_ft ` | ` 10 ` |
| ` touch_range_ft ` | ` 10 ` |

#### Vault

<!-- splat:values path=splat_rule/parameters/vault -->

| Field | Value |
| --- | --- |
| ` roll ` | ` ability_check ` |
| ` mode ` | ` advantage ` |
| ` ability ` | ` strength ` |
| ` skill ` | ` athletics ` |
| ` scope ` | ` required_high_or_long_jump_check ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 7.0 ` |
| ` unit ` | ` lb ` |
