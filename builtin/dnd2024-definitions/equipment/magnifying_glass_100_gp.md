---
splat_schema: 2
splat_id: 30968556-9039-54c6-b86f-d4fb6baadd28
splat_kind: item
splat_key: dnd2024:equipment/magnifying_glass_100_gp
splat_name: Magnifying Glass (100 GP)
splat_origin: dnd2024:reference/pages/098
splat_sources:
- SRD 5.2.1, pp. 98, https://www.dndbeyond.com/srd
---

# Magnifying Glass (100 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 98 -->

A Magnifying Glass grants Advantage on any ability check made to appraise or inspect a highly detailed item. Lighting a fire with a Magnifying Glass requires light as bright as sunlight to focus, tinder to ignite, and about 5 minutes for the fire to ignite.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/magnifying_glass_100_gp ` |
| ` implementation ` | ` dnd2024:equipment_magnifying_glass_100_gp ` |

### Parameters

#### Using

<!-- splat:values path=splat_rule/parameters/using -->

| Field | Value |
| --- | --- |
| ` roll ` | ` ability_check ` |
| ` mode ` | ` advantage ` |
| ` scope ` | ` appraise_or_inspect_highly_detailed_item ` |

#### Ignite

<!-- splat:yaml path=splat_rule/parameters/ignite -->

```yaml
minimum_light: sunlight_brightness
requires_tinder: true
duration:
  value: 5
  unit: minute
approximate: true
```

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
>   value: 100
> mass:
>   unit: lb
>   value: 0.0
> ```

