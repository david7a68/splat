---
splat_schema: 2
splat_id: 167d0da6-5679-5113-811a-486cf5968c01
splat_kind: item
splat_key: dnd2024:equipment/costume_5_gp
splat_name: Costume (5 GP)
splat_origin: dnd2024:reference/pages/097
splat_sources:
- SRD 5.2.1, pp. 97, https://www.dndbeyond.com/srd
---

# Costume (5 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 97 -->

While wearing a Costume, you have Advantage on any ability check you make to impersonate the person or type of person it represents.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/costume_5_gp ` |
| ` implementation ` | ` dnd2024:equipment_costume_5_gp ` |

### Parameters

#### While worn

<!-- splat:values path=splat_rule/parameters/while_worn -->

| Field | Value |
| --- | --- |
| ` roll ` | ` ability_check ` |
| ` mode ` | ` advantage ` |
| ` scope ` | ` impersonate_represented_person_or_person_type ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 4.0 ` |
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
>   value: 4.0
> ```

