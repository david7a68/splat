---
splat_schema: 2
splat_id: a7433f8d-e2a1-5936-81d8-5bb4a482b18c
splat_kind: item
splat_key: dnd2024:equipment/blanket_5_sp
splat_name: Blanket (5 SP)
splat_origin: dnd2024:reference/pages/096
splat_sources:
- SRD 5.2.1, pp. 96, https://www.dndbeyond.com/srd
---

# Blanket (5 SP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 96 -->

While wrapped in a blanket, you have Advantage on saving throws against extreme cold (see “Gameplay Toolbox”).









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/blanket_5_sp ` |
| ` implementation ` | ` dnd2024:equipment_blanket_5_sp ` |

### Parameters

#### While wrapped

<!-- splat:values path=splat_rule/parameters/while_wrapped -->

| Field | Value |
| --- | --- |
| ` roll ` | ` saving_throw ` |
| ` mode ` | ` advantage ` |
| ` scope ` | ` extreme_cold ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 3.0 ` |
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
>   value: 3.0
> ```

