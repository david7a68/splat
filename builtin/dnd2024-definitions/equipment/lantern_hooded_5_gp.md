---
splat_schema: 1
splat_id: ac8b78df-1670-5be3-a34d-104654c811ab
splat_kind: item
splat_key: dnd2024:equipment/lantern_hooded_5_gp
splat_name: Lantern, Hooded (5 GP)
splat_origin:
  key: dnd2024:reference/pages/098
splat_sources:
- title: SRD 5.2.1
  locator: pp. 98
  url: https://www.dndbeyond.com/srd
---

# Lantern, Hooded (5 GP)

## Source text

<!-- SRD page 98 -->

A Hooded Lantern burns Oil as fuel to cast Bright\
Light in a 30-foot radius and Dim Light for an ad-\
ditional 30 feet. As a Bonus Action, you can lower\
the hood, reducing the light to Dim Light in a 5-foot\
radius, or raise it again.\

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
| ` value ` | ` 2.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/lantern_hooded_5_gp ` |
| ` implementation ` | ` dnd2024:equipment_lantern_hooded_5_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` hours_per_flask ` | ` 6 ` |

#### Light

<!-- splat:values path=splat_rule/parameters/light -->

| Field | Value |
| --- | --- |
| ` requires_lit ` | ` true ` |
| ` shape ` | ` radius ` |
| ` bright_ft ` | ` 30 ` |
| ` additional_dim_ft ` | ` 30 ` |

#### Fuel

<!-- splat:values path=splat_rule/parameters/fuel -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/oil_1_sp ` |

#### Hood

<!-- splat:yaml path=splat_rule/parameters/hood -->

```yaml
economy: bonus_action
operations:
- lower
- raise
lowered_light:
  shape: radius
  bright_ft: 0
  dim_ft: 5
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2.0 ` |
| ` unit ` | ` lb ` |
