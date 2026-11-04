---
splat_schema: 2
splat_id: e874d0e0-ba14-5ebe-a718-b0a315c60115
splat_kind: item
splat_key: dnd2024:equipment/bell_1_gp
splat_name: Bell (1 GP)
splat_origin: dnd2024:reference/pages/096
splat_sources:
- SRD 5.2.1, pp. 96, https://www.dndbeyond.com/srd
---

# Bell (1 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 96 -->

When rung as a Utilize action, a Bell produces a sound that can be heard up to 60 feet away.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/bell_1_gp ` |
| ` implementation ` | ` dnd2024:equipment_bell_1_gp ` |

### Parameters

#### Use

<!-- splat:yaml path=splat_rule/parameters/use -->

```yaml
action: utilize
effect:
  sound_audible_range_ft: 60
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
>   value: 1
> mass:
>   unit: lb
>   value: 0.0
> ```

