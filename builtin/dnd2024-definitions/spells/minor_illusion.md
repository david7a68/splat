---
splat_schema: 2
splat_id: 1fdd3059-d5ea-5f9b-b97a-4262ccdc3324
splat_kind: spell
splat_key: dnd2024:spells/minor_illusion
splat_name: Minor Illusion
splat_origin: dnd2024:reference/spell/minor_illusion
splat_sources:
- SRD 5.2.1, pp. 149, https://www.dndbeyond.com/srd
---

# Minor Illusion


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Illusion |
| Classes | Bard, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range | 30 feet |
| Components | S, M (a bit of fleece) |
| Duration | 1 minute |


## Source text

<!-- SRD page 149 -->

Illusion Cantrip (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: S, M (a bit of fleece)\
Duration: 1 minute

You create a sound or an image of an object within range that lasts for the duration. See the descriptions below for the effects of each. The illusion ends if you cast this spell again.

If a creature takes a Study action to examine the sound or image, the creature can determine that it is an illusion with a successful Intelligence (Investigation) check against your spell save DC. If a creature discerns the illusion for what it is, the illusion becomes faint to the creature.

Sound. If you create a sound, its volume can range from a whisper to a scream. It can be your voice, someone else’s voice, a lion’s roar, a beating of drums, or any other sound you choose. The sound continues unabated throughout the duration, or you can make discrete sounds at different times before the spell ends.

Image. If you create an image of an object—such as a chair, muddy footprints, or a small chest—it must be no larger than a 5-foot Cube. The image can’t create sound, light, smell, or any other sensory effect. Physical interaction with the image reveals it to be an illusion, since things can pass through it.

Original layout: [[../../srd-5.2.1/pages/149|page 149]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/minor_illusion ` |
| ` implementation ` | ` dnd2024:spell_minor_illusion ` |

### Parameters

#### Illusion

<!-- splat:values path=splat_rule/parameters/illusion -->

| Field | Value |
| --- | --- |
| ` within_spell_range ` | ` true ` |
| ` duration ` | ` spell_duration ` |
| ` end_on_recast ` | ` true ` |
| ` choose_modes ` | ` 1 ` |

#### Modes

<!-- splat:yaml path=splat_rule/parameters/modes -->

```yaml
sound:
  volume_minimum: whisper
  volume_maximum: scream
  sound_choice: any
  timing_choices:
  - continuous
  - discrete_over_duration
image:
  depiction: object
  maximum_cube_side_feet: 5
  prohibit_sensory_effects:
  - sound
  - light
  - smell
  - all_nonvisual
  physical_interaction:
    reveals_illusion: true
    things_pass_through: true
```

#### Discern

<!-- splat:yaml path=splat_rule/parameters/discern -->

```yaml
cost: study_action
check:
  ability: intelligence
  skill: investigation
  dc:
    binding: save_dc
success: recognize_illusion
recognized_illusion_appearance: faint
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

