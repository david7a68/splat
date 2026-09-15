---
splat_schema: 1
splat_id: a102cd30-ca0e-5c7e-8276-e374c171bf82
splat_kind: item
splat_key: dnd2024:equipment/barrel_2_gp
splat_name: Barrel (2 GP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
---

# Barrel (2 GP)

## Source text

<!-- SRD page 96 -->

A Barrel holds up to 40 gallons of liquid or up to 4\
cubic feet of dry goods.\

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
| ` value ` | ` 70.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/barrel_2_gp ` |
| ` implementation ` | ` dnd2024:equipment_barrel_2_gp ` |

### Parameters

#### Capacity

<!-- splat:yaml path=splat_rule/parameters/capacity -->

```yaml
alternative_capacities:
- liquid_gallons: 40
- dry_volume_ft3: 4
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 70.0 ` |
| ` unit ` | ` lb ` |
