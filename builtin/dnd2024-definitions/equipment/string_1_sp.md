---
splat_schema: 2
splat_id: fa51b46c-8b43-5174-b9b5-b535c228a2b3
splat_kind: item
splat_key: dnd2024:equipment/string_1_sp
splat_name: String (1 SP)
splat_origin: dnd2024:reference/pages/100
splat_sources:
- SRD 5.2.1, pp. 100, https://www.dndbeyond.com/srd
---

# String (1 SP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 100 -->

String is 10 feet long. You can tie a knot in it as a Utilize action.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/string_1_sp ` |
| ` implementation ` | ` dnd2024:equipment_string_1_sp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` length_ft ` | ` 10 ` |

#### Tie knot

<!-- splat:values path=splat_rule/parameters/tie_knot -->

| Field | Value |
| --- | --- |
| ` action ` | ` utilize ` |

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
>   unit: sp
>   value: 1
> mass:
>   unit: lb
>   value: 0.0
> ```

