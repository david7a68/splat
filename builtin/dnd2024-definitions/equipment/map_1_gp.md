---
splat_schema: 2
splat_id: 0ac6271b-08dd-5592-acc1-10391ec2ffa3
splat_kind: item
splat_key: dnd2024:equipment/map_1_gp
splat_name: Map (1 GP)
splat_origin: dnd2024:reference/pages/098
splat_sources:
- SRD 5.2.1, pp. 98, https://www.dndbeyond.com/srd
---

# Map (1 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 98 -->

If you consult an accurate Map, you gain a +5 bonus to Wisdom (Survival) checks you make to find your way in the place represented on it.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/map_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_map_1_gp ` |

### Parameters

#### Consult

<!-- splat:values path=splat_rule/parameters/consult -->

| Field | Value |
| --- | --- |
| ` accurate ` | ` true ` |
| ` scope ` | ` find_way_in_represented_place ` |
| ` check_bonus ` | ` 5 ` |
| ` ability ` | ` wisdom ` |
| ` skill ` | ` survival ` |

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
>   unit: gp
>   value: 1
> mass:
>   unit: lb
>   value: 0.0
> ```

