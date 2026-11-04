---
splat_schema: 2
splat_id: 3b2cb6cd-1d2a-5659-ada8-195557ffd613
splat_kind: item
splat_key: dnd2024:equipment/ram_portable_4_gp
splat_name: Ram, Portable (4 GP)
splat_origin: dnd2024:reference/pages/099
splat_sources:
- SRD 5.2.1, pp. 99, https://www.dndbeyond.com/srd
---

# Ram, Portable (4 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 99 -->

You can use a Portable Ram to break down doors. When doing so, you gain a +4 bonus to the Strength check. One other character can help you use the ram, giving you Advantage on this check.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/ram_portable_4_gp ` |
| ` implementation ` | ` dnd2024:equipment_ram_portable_4_gp ` |

### Parameters

#### Break door

<!-- splat:values path=splat_rule/parameters/break_door -->

| Field | Value |
| --- | --- |
| ` ability ` | ` strength ` |
| ` check_bonus ` | ` 4 ` |
| ` helper_maximum ` | ` 1 ` |
| ` with_helper_mode ` | ` advantage ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 35.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 4
> mass:
>   unit: lb
>   value: 35.0
> ```

