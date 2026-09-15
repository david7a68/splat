---
splat_schema: 1
splat_id: 891bccc3-1366-53c0-b1a5-ce5494bda5e5
splat_kind: item
splat_key: dnd2024:equipment/robe_1_gp
splat_name: Robe (1 GP)
splat_origin:
  key: dnd2024:reference/pages/099
splat_sources:
- title: SRD 5.2.1
  locator: pp. 99
  url: https://www.dndbeyond.com/srd
---

# Robe (1 GP)

## Source text

<!-- SRD page 99 -->

A Robe has vocational or ceremonial significance.\
Some events and locations admit only people wear-\
ing a Robe bearing certain colors or symbols.\

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
| ` value ` | ` 4.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/robe_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_robe_1_gp ` |

### Parameters

#### Access requirement

<!-- splat:yaml path=splat_rule/parameters/access_requirement -->

```yaml
authority: dm
scope:
- event
- location
eligible_if_wearing: true
required_colors:
  type: choice
  authority: dm
required_symbols:
  type: choice
  authority: dm
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 4.0 ` |
| ` unit ` | ` lb ` |
