---
splat_schema: 2
splat_id: 316c92fa-a021-535b-aff8-ac6e28364841
splat_kind: item
splat_key: dnd2024:equipment/candle_1_cp
splat_name: Candle (1 CP)
splat_origin: dnd2024:reference/pages/096
splat_sources:
- SRD 5.2.1, pp. 96, https://www.dndbeyond.com/srd
---

# Candle (1 CP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 96 -->

For 1 hour, a lit Candle sheds Bright Light in a 5-foot radius and Dim Light for an additional 5 feet.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/candle_1_cp ` |
| ` implementation ` | ` dnd2024:equipment_candle_1_cp ` |

### Parameters

#### Light

<!-- splat:values path=splat_rule/parameters/light -->

| Field | Value |
| --- | --- |
| ` requires_lit ` | ` true ` |
| ` shape ` | ` radius ` |
| ` bright_ft ` | ` 5 ` |
| ` additional_dim_ft ` | ` 5 ` |

#### Burn duration

<!-- splat:values path=splat_rule/parameters/burn_duration -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` hour ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.0 ` |
| ` unit ` | ` lb ` |
| ` negligible ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: cp
>   value: 1
> mass:
>   unit: lb
>   value: 0.0
> ```

