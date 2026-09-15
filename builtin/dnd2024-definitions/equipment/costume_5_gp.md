---
splat_schema: 1
splat_id: 167d0da6-5679-5113-811a-486cf5968c01
splat_kind: item
splat_key: dnd2024:equipment/costume_5_gp
splat_name: Costume (5 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
---

# Costume (5 GP)

## Source text

<!-- SRD page 97 -->

While wearing a Costume, you have Advantage on\
any ability check you make to impersonate the per-\
son or type of person it represents.\

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
| ` value ` | ` 4.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/costume_5_gp ` |
| ` implementation ` | ` dnd2024:equipment_costume_5_gp ` |

### Parameters

#### While worn

<!-- splat:values path=splat_rule/parameters/while_worn -->

| Field | Value |
| --- | --- |
| ` roll ` | ` ability_check ` |
| ` mode ` | ` advantage ` |
| ` scope ` | ` impersonate_represented_person_or_person_type ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 4.0 ` |
| ` unit ` | ` lb ` |
