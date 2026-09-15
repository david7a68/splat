---
splat_schema: 1
splat_id: 0ac6271b-08dd-5592-acc1-10391ec2ffa3
splat_kind: item
splat_key: dnd2024:equipment/map_1_gp
splat_name: Map (1 GP)
splat_origin:
  key: dnd2024:reference/pages/098
splat_sources:
- title: SRD 5.2.1
  locator: pp. 98
  url: https://www.dndbeyond.com/srd
---

# Map (1 GP)

## Source text

<!-- SRD page 98 -->

If you consult an accurate Map, you gain a +5 bonus\
to Wisdom (Survival) checks you make to find your\
way in the place represented on it.\

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
| ` unit ` | ` gp ` |

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
| ` mechanic ` | ` dnd2024:equipment/map_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_map_1_gp ` |

### Parameters

#### Consult

<!-- splat:values path=splat_rule/parameters/consult -->

| Field | Value |
| --- | --- |
| ` accurate ` | ` true ` |
| ` scope ` | ` find_way_in_represented_place ` |
| ` check_bonus ` | ` 5 ` |
| ` ability ` | ` wisdom ` |
| ` skill ` | ` survival ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.0 ` |
| ` unit ` | ` lb ` |
| ` negligible ` | ` true ` |
