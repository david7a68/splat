---
splat_schema: 1
splat_id: b27f6b06-d9d0-55c8-ac07-deb3ea70bf57
splat_kind: item
splat_key: dnd2024:equipment/bedroll_1_gp
splat_name: Bedroll (1 GP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
---

# Bedroll (1 GP)

## Source text

<!-- SRD page 96 -->

A Bedroll sleeps one Small or Medium creature. While in a Bedroll, you automatically succeed on saving throws against extreme cold (see “Gameplay Toolbox”).

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
| ` value ` | ` 7.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/bedroll_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_bedroll_1_gp ` |

### Parameters

#### Sleep capacity

<!-- splat:values path=splat_rule/parameters/sleep_capacity -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sizes ` | ` ["small", "medium"] ` |

#### While inside

<!-- splat:values path=splat_rule/parameters/while_inside -->

| Field | Value |
| --- | --- |
| ` saving_throw_scope ` | ` extreme_cold ` |
| ` result ` | ` automatic_success ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 7.0 ` |
| ` unit ` | ` lb ` |
