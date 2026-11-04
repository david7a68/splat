---
splat_schema: 2
splat_id: ac8e87e8-da02-5b25-82a4-23f07e2140a1
splat_kind: item
splat_key: dnd2024:equipment/lantern_bullseye_10_gp
splat_name: Lantern, Bullseye (10 GP)
splat_origin: dnd2024:reference/pages/098
splat_sources:
- SRD 5.2.1, pp. 98, https://www.dndbeyond.com/srd
---

# Lantern, Bullseye (10 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 98 -->

A Bullseye Lantern burns Oil as fuel to cast Bright Light in a 60-foot Cone and Dim Light for an additional 60 feet.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/lantern_bullseye_10_gp ` |
| ` implementation ` | ` dnd2024:equipment_lantern_bullseye_10_gp ` |

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
| ` shape ` | ` cone ` |
| ` bright_ft ` | ` 60 ` |
| ` additional_dim_ft ` | ` 60 ` |

#### Fuel

<!-- splat:values path=splat_rule/parameters/fuel -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/oil_1_sp ` |

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
>   value: 10
> mass:
>   unit: lb
>   value: 2.0
> ```

