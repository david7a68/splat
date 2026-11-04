---
splat_schema: 2
splat_id: 712db94b-28bd-5dce-b8d7-ad4165453795
splat_kind: item
splat_key: dnd2024:equipment/shovel_2_gp
splat_name: Shovel (2 GP)
splat_origin: dnd2024:reference/pages/099
splat_sources:
- SRD 5.2.1, pp. 99, https://www.dndbeyond.com/srd
---

# Shovel (2 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 99 -->

Working for 1 hour, you can use a Shovel to dig a hole that is 5 feet on each side in soil or similar material.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/shovel_2_gp ` |
| ` implementation ` | ` dnd2024:equipment_shovel_2_gp ` |

### Parameters

#### Dig

<!-- splat:yaml path=splat_rule/parameters/dig -->

```yaml
duration:
  value: 1
  unit: hour
shape: cube
side_ft: 5
material_choices:
- soil
- similar_material
similar_material_authority: dm
```

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
> mass:
>   unit: lb
>   value: 5.0
> ```

