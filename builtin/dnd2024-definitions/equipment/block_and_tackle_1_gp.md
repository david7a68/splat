---
splat_schema: 1
splat_id: a8e95bdd-9833-5cd5-8a85-1656c48edf56
splat_kind: item
splat_key: dnd2024:equipment/block_and_tackle_1_gp
splat_name: Block and Tackle (1 GP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
---

# Block and Tackle (1 GP)

## Source text

<!-- SRD page 96 -->

A Block and Tackle allows you to hoist up to four times the weight you can normally lift.

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
| ` value ` | ` 5.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/block_and_tackle_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_block_and_tackle_1_gp ` |

### Parameters

#### Hoist capacity

<!-- splat:yaml path=splat_rule/parameters/hoist_capacity -->

```yaml
op: multiply
operands:
- 4
- stat: normal_lift_capacity
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5.0 ` |
| ` unit ` | ` lb ` |
