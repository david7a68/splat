---
splat_schema: 1
splat_id: 30b7c9ab-a9f5-5560-a14b-b94d63fb9429
splat_kind: item
splat_key: dnd2024:equipment/crowbar_2_gp
splat_name: Crowbar (2 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
---

# Crowbar (2 GP)

## Source text

<!-- SRD page 97 -->

Using a Crowbar gives you Advantage on Strength\
checks where the Crowbar’s leverage can be\
applied.\

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
| ` value ` | ` 5.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/crowbar_2_gp ` |
| ` implementation ` | ` dnd2024:equipment_crowbar_2_gp ` |

### Parameters

#### Using

<!-- splat:values path=splat_rule/parameters/using -->

| Field | Value |
| --- | --- |
| ` roll ` | ` ability_check ` |
| ` mode ` | ` advantage ` |
| ` ability ` | ` strength ` |
| ` requires ` | ` leverage_applicable ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5.0 ` |
| ` unit ` | ` lb ` |
