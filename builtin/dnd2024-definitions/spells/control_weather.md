---
splat_schema: 2
splat_id: 6f5c8e82-bab2-52a5-8227-2d0780779f67
splat_kind: spell
splat_key: dnd2024:spells/control_weather
splat_name: Control Weather
splat_origin: dnd2024:reference/spell/control_weather
splat_sources:
- SRD 5.2.1, pp. 120, https://www.dndbeyond.com/srd
---

# Control Weather


## Spell

| Field | Value |
| --- | --- |
| Level | 8 |
| School | Transmutation |
| Classes | Cleric, Druid, Wizard |
| Casting Time | 10 minutes |
| Range |  |
| Components | V, S, M (burning incense) |
| Duration | Concentration, up to 8 hours |


## Source text

<!-- SRD page 120 -->

Level 8 Transmutation (Cleric, Druid, Wizard)\
Casting Time: 10 minutes\
Range: Self\
Components: V, S, M (burning incense)\
Duration: Concentration, up to 8 hours

You take control of the weather within 5 miles of you for the duration. You must be outdoors to cast this spell, and it ends early if you go indoors.

When you cast the spell, you change the current weather conditions, which are determined by the GM. You can change precipitation, temperature, and wind. It takes 1d4 × 10 minutes for the new conditions to take effect. Once they do so, you can change the conditions again. When the spell ends, the weather gradually returns to normal.

When you change the weather conditions, find a current condition on the following tables and change its stage by one, up or down. When changing the wind, you can change its direction.

**Precipitation**

| Stage | Condition |
| --- | --- |
| 1 | Clear |
| 2 | Light clouds |
| 3 | Overcast or ground fog |
| 4 | Rain, hail, or snow |
| 5 | Torrential rain, driving hail, or blizzard |

**Temperature Wind**

| Stage | Condition | Stage | Condition |
| --- | --- | --- | --- |
| 1 | Heat wave | 1 | Calm |
| 2 | Hot | 2 | Moderate wind |
| 3 | Warm | 3 | Strong wind |
| 4 | Cool | 4 | Gale |
| 5 | Cold | 5 | Storm |
| 6 | Freezing |  |  |

Original layout: [[../../srd-5.2.1/pages/120|page 120]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/control_weather ` |
| ` implementation ` | ` dnd2024:spell_control_weather ` |

### Parameters

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` caster_outdoors ` | ` true ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` radius_miles ` | ` 5 ` |
| ` origin ` | ` caster ` |
| ` duration ` | ` spell_duration ` |

#### End when

<!-- splat:values path=splat_rule/parameters/end_when -->

| Field | Value |
| --- | --- |
| ` caster_goes_indoors ` | ` true ` |

#### Weather

<!-- splat:yaml path=splat_rule/parameters/weather -->

```yaml
initial_conditions_chooser: gm
change_each_stage_by_maximum: 1
change_directions:
- up
- down
change_delay_minutes:
  roll:
    dice:
      count: 1
      sides: 4
      add: 0
  multiply: 10
next_change_requires: previous_change_took_effect
wind_direction_change_allowed: true
on_spell_end:
  return_to_normal: gradual
  transition_duration:
    chooser: gm
    source_unspecified: true
precipitation_stages:
- - clear
- - light_clouds
- - overcast
  - ground_fog
- - rain
  - hail
  - snow
- - torrential_rain
  - driving_hail
  - blizzard
temperature_stages:
- heat_wave
- hot
- warm
- cool
- cold
- freezing
wind_stages:
- calm
- moderate
- strong
- gale
- storm
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```

