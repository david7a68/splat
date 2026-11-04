---
splat_schema: 2
splat_id: 2f495087-e0fb-552c-9f6f-d587b25e9b14
splat_kind: item
splat_key: dnd2024:equipment/sack_1_cp
splat_name: Sack (1 CP)
splat_origin: dnd2024:reference/pages/099
splat_sources:
- SRD 5.2.1, pp. 99, https://www.dndbeyond.com/srd
---

# Sack (1 CP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 99 -->

A Sack holds up to 30 pounds within 1 cubic foot.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/sack_1_cp ` |
| ` implementation ` | ` dnd2024:equipment_sack_1_cp ` |

### Parameters

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
| ` value ` | ` 0.5 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: cp
>   value: 1
> mass:
>   unit: lb
>   value: 0.5
> ```

