---
splat_schema: 2
splat_id: 6f92e855-bb92-5ee3-afd3-376606d4fd63
splat_kind: item
splat_key: dnd2024:equipment/backpack_2_gp
splat_name: Backpack (2 GP)
splat_origin: dnd2024:reference/pages/096
splat_sources:
- SRD 5.2.1, pp. 96, https://www.dndbeyond.com/srd
---

# Backpack (2 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 96 -->

A Backpack holds up to 30 pounds within 1 cubic foot. It can also serve as a saddlebag.









### Containment




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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 2
> containment:
>   capacity:
>     mass:
>       value: 30
>       unit: lb
>     volume:
>       value: 1
>       unit: ft3
> mass:
>   unit: lb
>   value: 5.0
> ```

