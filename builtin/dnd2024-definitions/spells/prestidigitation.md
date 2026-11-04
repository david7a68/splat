---
splat_schema: 2
splat_id: dd7b2596-90d9-52e3-bd16-cf735492dfe2
splat_kind: spell
splat_key: dnd2024:spells/prestidigitation
splat_name: Prestidigitation
splat_origin: dnd2024:reference/spell/prestidigitation
splat_sources:
- SRD 5.2.1, pp. 154, https://www.dndbeyond.com/srd
---

# Prestidigitation


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Transmutation |
| Classes | Bard, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range | 10 feet |
| Components | V, S |
| Duration | Up to 1 hour |


## Source text

<!-- SRD page 154 -->

Transmutation Cantrip (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 10 feet\
Components: V, S\
Duration: Up to 1 hour

You create a magical effect within range. Choose the effect from the options below. If you cast this spell multiple times, you can have up to three of its non-instantaneous effects active at a time.

Sensory Effect. You create an instantaneous, harmless sensory effect, such as a shower of sparks, a puff of wind, faint musical notes, or an odd odor.

Fire Play. You instantaneously light or snuff out a candle, a torch, or a small campfire.

Clean or Soil. You instantaneously clean or soil an object no larger than 1 cubic foot.

Minor Sensation. You chill, warm, or flavor up to 1 cubic foot of nonliving material for 1 hour.

Magic Mark. You make a color, a small mark, or a symbol appear on an object or a surface for 1 hour.

Minor Creation. You create a nonmagical trinket or an illusory image that can fit in your hand. It lasts until the end of your next turn. A trinket can deal no damage and has no monetary worth.

Original layout: [[../../srd-5.2.1/pages/154|page 154]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/prestidigitation ` |
| ` implementation ` | ` dnd2024:spell_prestidigitation ` |

### Parameters

#### Selection

<!-- splat:values path=splat_rule/parameters/selection -->

| Field | Value |
| --- | --- |
| ` within_spell_range ` | ` true ` |
| ` effect_count ` | ` 1 ` |
| ` noninstantaneous_active_maximum_across_castings ` | ` 3 ` |

#### Modes

<!-- splat:yaml path=splat_rule/parameters/modes -->

```yaml
sensory_effect:
  duration: instantaneous
  harmless: true
  sensory_effect_choice: true
fire_play:
  duration: instantaneous
  choices:
  - light
  - snuff
  target_kinds:
  - candle
  - torch
  - small_campfire
clean_or_soil:
  duration: instantaneous
  choices:
  - clean
  - soil
  object_maximum_cubic_feet: 1
minor_sensation:
  choices:
  - chill
  - warm
  - flavor
  nonliving_material_maximum_cubic_feet: 1
  duration_hours: 1
magic_mark:
  choices:
  - color
  - small_mark
  - symbol
  target_kinds:
  - object
  - surface
  duration_hours: 1
minor_creation:
  choices:
  - nonmagical_trinket
  - illusory_image
  must_fit_in_caster_hand: true
  until: caster_next_turn_end
  trinket:
    damage_possible: false
    monetary_worth: 0
```
