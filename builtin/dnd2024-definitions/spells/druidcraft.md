---
splat_schema: 1
splat_id: 980b488b-41db-5b44-a10f-103ef931e39e
splat_kind: spell
splat_key: dnd2024:spells/druidcraft
splat_name: Druidcraft
splat_origin:
  key: dnd2024:reference/spell/druidcraft
splat_sources:
- title: SRD 5.2.1
  locator: pp. 126
  url: https://www.dndbeyond.com/srd
---

# Druidcraft

## Source text

<!-- SRD page 126 -->

Transmutation Cantrip (Druid)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Instantaneous

Whispering to the spirits of nature, you create one of the following effects within range.

Weather Sensor. You create a Tiny, harmless sensory effect that predicts what the weather will be at your location for the next 24 hours. The effect might manifest as a golden orb for clear skies, a cloud for rain, falling snowflakes for snow, and so on. This effect persists for 1 round.

Bloom. You instantly make a flower blossom, a seed pod open, or a leaf bud bloom.

Sensory Effect. You create a harmless sensory effect, such as falling leaves, spectral dancing fairies, a gentle breeze, the sound of an animal, or the faint odor of skunk. The effect must fit in a 5-foot Cube.

Fire Play. You light or snuff out a candle, a torch, or a campfire.

Original layout: [[../../srd-5.2.1/pages/126|page 126]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["druid"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/druidcraft ` |
| ` implementation ` | ` dnd2024:spell_druidcraft ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Choice

<!-- splat:values path=splat_rule/parameters/choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` count ` | ` 1 ` |
| ` range_feet ` | ` 30 ` |

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
weather_sensor:
  size: tiny
  harmless: true
  forecast_location: caster_location
  forecast_hours: 24
  duration_rounds: 1
  sensory_manifestation:
    chooser: caster
bloom:
  instant: true
  target_choices:
  - flower
  - seed_pod
  - leaf_bud
  operations:
    flower: blossom
    seed_pod: open
    leaf_bud: bloom
sensory_effect:
  harmless: true
  maximum_cube_side_feet: 5
  sensory_design:
    chooser: caster
fire_play:
  operations:
  - light
  - snuff
  targets:
  - candle
  - torch
  - campfire
```
