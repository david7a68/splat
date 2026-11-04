---
splat_schema: 2
splat_id: 08eddade-32e9-5fb4-b96b-b9c141eb5b8a
splat_kind: item
splat_key: dnd2024:equipment/tinderbox_5_sp
splat_name: Tinderbox (5 SP)
splat_origin: dnd2024:reference/pages/100
splat_sources:
- SRD 5.2.1, pp. 100, https://www.dndbeyond.com/srd
---

# Tinderbox (5 SP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 100 -->

A Tinderbox is a small container holding flint, fire steel, and tinder (usually dry cloth soaked in light oil) used to kindle a fire. Using it to light a Candle, Lamp, Lantern, or Torch—or anything else with exposed fuel—takes a Bonus Action. Lighting any other fire takes 1 minute.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/tinderbox_5_sp ` |
| ` implementation ` | ` dnd2024:equipment_tinderbox_5_sp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` components ` | ` ["flint", "fire_steel", "tinder"] ` |

#### Ignite

<!-- splat:yaml path=splat_rule/parameters/ignite -->

```yaml
exposed_fuel:
  economy: bonus_action
other_fire:
  duration:
    value: 1
    unit: minute
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: sp
>   value: 5
> mass:
>   unit: lb
>   value: 1.0
> ```

