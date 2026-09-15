---
splat_schema: 1
splat_id: ab7e6e89-a62a-5318-aa98-fe6a0ece3d0b
splat_kind: spell
splat_key: dnd2024:spells/thaumaturgy
splat_name: Thaumaturgy
splat_origin:
  key: dnd2024:reference/spell/thaumaturgy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 169
  url: https://www.dndbeyond.com/srd
---

# Thaumaturgy

## Source text

<!-- SRD page 169 -->

Transmutation Cantrip (Cleric)\
Casting Time: Action\
Range: 30 feet\
Components: V\
Duration: Up to 1 minute\
You manifest a minor wonder within range. You cre-\
ate one of the effects below within range. If you cast\
this spell multiple times, you can have up to three of\
its 1-minute effects active at a time.\
Altered Eyes. You alter the appearance of your\
eyes for 1 minute.\
Booming Voice. Your voice booms up to three\
times as loud as normal for 1 minute. For the dura-\
tion, you have Advantage on Charisma (Intimida-\
tion) checks.\
Fire Play. You cause flames to flicker, brighten,\
dim, or change color for 1 minute.\
Invisible Hand. You instantaneously cause an un-\
locked door or window to fly open or slam shut.\
Phantom Sound. You create an instantaneous\
sound that originates from a point of your choice\
within range, such as a rumble of thunder, the cry of\
a raven, or ominous whispers.\
Tremors. You cause harmless tremors in the\
ground for 1 minute.\

Original layout: [[../../srd-5.2.1/pages/169|page 169]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["cleric"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Up to 1 minute ` |
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
| ` mechanic ` | ` dnd2024:spells/thaumaturgy ` |
| ` implementation ` | ` dnd2024:spell_thaumaturgy ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` maximum_simultaneous_minute_effects ` | ` 3 ` |

#### Effect choice

<!-- splat:values path=splat_rule/parameters/effect_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` within_range ` | ` true ` |

#### Effects

<!-- splat:yaml path=splat_rule/parameters/effects -->

```yaml
altered_eyes:
  appearance_choice: caster
  minutes: 1
booming_voice:
  volume_multiplier_maximum: 3
  minutes: 1
  charisma_intimidation_checks: advantage
fire_play:
  choice:
  - flicker
  - brighten
  - dim
  - change_color
  minutes: 1
invisible_hand:
  target:
  - door
  - window
  unlocked_required: true
  choice:
  - open
  - slam_shut
  duration: instantaneous
phantom_sound:
  sound_choice: caster
  origin: chosen_point_in_range
  duration: instantaneous
tremors:
  target: ground
  harmless: true
  minutes: 1
```
