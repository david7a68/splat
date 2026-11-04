---
splat_schema: 2
splat_id: f05f68f5-9772-5bed-b198-67bbbb36b06d
splat_kind: item
splat_key: dnd2024:equipment/mirror_5_gp
splat_name: Mirror (5 GP)
splat_origin: dnd2024:reference/pages/098
splat_sources:
- SRD 5.2.1, pp. 98, https://www.dndbeyond.com/srd
---

# Mirror (5 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 98 -->

A handheld steel Mirror is useful for personal cosmetics but also for peeking around corners and reflecting light as a signal.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/mirror_5_gp ` |
| ` implementation ` | ` dnd2024:equipment_mirror_5_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` material ` | ` steel ` |
| ` held ` | ` true ` |
| ` operations ` | ` ["personal_cosmetics", "peek_around_corner", "reflect_light_signal"] ` |

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
>   unit: gp
>   value: 5
> mass:
>   unit: lb
>   value: 0.5
> ```

