---
splat_schema: 2
splat_id: 0887b5a4-499c-5a15-85a4-c2c34f6e7b8d
splat_kind: item
splat_key: dnd2024:equipment/lamp_5_sp
splat_name: Lamp (5 SP)
splat_origin: dnd2024:reference/pages/098
splat_sources:
- SRD 5.2.1, pp. 98, https://www.dndbeyond.com/srd
---

# Lamp (5 SP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 98 -->

A Lamp burns Oil as fuel to cast Bright Light in a 15-foot radius and Dim Light for an additional 30 feet.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/lamp_5_sp ` |
| ` implementation ` | ` dnd2024:equipment_lamp_5_sp ` |

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
| ` bright_ft ` | ` 15 ` |
| ` additional_dim_ft ` | ` 30 ` |

#### Fuel

<!-- splat:values path=splat_rule/parameters/fuel -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/oil_1_sp ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: sp
>   value: 5
> mass:
>   unit: lb
>   value: 1.0
> ```

