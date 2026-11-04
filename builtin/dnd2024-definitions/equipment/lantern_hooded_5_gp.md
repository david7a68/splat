---
splat_schema: 2
splat_id: ac8b78df-1670-5be3-a34d-104654c811ab
splat_kind: item
splat_key: dnd2024:equipment/lantern_hooded_5_gp
splat_name: Lantern, Hooded (5 GP)
splat_origin: dnd2024:reference/pages/098
splat_sources:
- SRD 5.2.1, pp. 98, https://www.dndbeyond.com/srd
---

# Lantern, Hooded (5 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 98 -->

A Hooded Lantern burns Oil as fuel to cast Bright Light in a 30-foot radius and Dim Light for an additional 30 feet. As a Bonus Action, you can lower the hood, reducing the light to Dim Light in a 5-foot radius, or raise it again.









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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 5
> mass:
>   unit: lb
>   value: 2.0
> ```

