---
splat_schema: 2
splat_id: c1e03a27-67a9-5276-bfba-615f7de246f6
splat_kind: item
splat_key: dnd2024:equipment/signal_whistle_5_cp
splat_name: Signal Whistle (5 CP)
splat_origin: dnd2024:reference/pages/099
splat_sources:
- SRD 5.2.1, pp. 99, https://www.dndbeyond.com/srd
---

# Signal Whistle (5 CP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 99 -->

When blown as a Utilize action, a Signal Whistle produces a sound that can be heard up to 600 feet away.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/signal_whistle_5_cp ` |
| ` implementation ` | ` dnd2024:equipment_signal_whistle_5_cp ` |

### Parameters

#### Use

<!-- splat:yaml path=splat_rule/parameters/use -->

```yaml
action: utilize
effect:
  sound_audible_range_ft: 600
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
>   unit: cp
>   value: 5
> mass:
>   unit: lb
>   value: 0.0
> ```

