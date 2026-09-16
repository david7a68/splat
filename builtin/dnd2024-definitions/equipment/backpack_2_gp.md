---
splat_schema: 1
splat_id: 6f92e855-bb92-5ee3-afd3-376606d4fd63
splat_kind: item
splat_key: dnd2024:equipment/backpack_2_gp
splat_name: Backpack (2 GP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
---

# Backpack (2 GP)

## Source text

<!-- SRD page 96 -->

A Backpack holds up to 30 pounds within 1 cubic\
foot. It can also serve as a saddlebag.\

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

### Containment

#### Capacity

<!-- splat:yaml path=splat_item/containment/capacity -->

```yaml
mass:
  value: 30
  unit: lb
volume:
  value: 1
  unit: ft3
```

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/backpack_2_gp ` |
| ` implementation ` | ` dnd2024:equipment_backpack_2_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` also_usable_as ` | ` saddlebag ` |

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
| ` value ` | ` 5.0 ` |
| ` unit ` | ` lb ` |
