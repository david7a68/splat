---
splat_schema: 1
splat_id: 3fc3bcca-9101-5850-aec5-f2ee508380ba
splat_kind: spell
splat_key: dnd2024:spells/elementalism
splat_name: Elementalism
splat_origin:
  key: dnd2024:reference/spell/elementalism
splat_sources:
- title: SRD 5.2.1
  locator: pp. 127
  url: https://www.dndbeyond.com/srd
---

# Elementalism

## Source text

<!-- SRD page 127 -->

Transmutation Cantrip (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Instantaneous\
You exert control over the elements, creating one of\
the following effects within range.\
Beckon Air. You create a breeze strong enough to\
ripple cloth, stir dust, rustle leaves, and close open\
doors and shutters, all in a 5-foot Cube. Doors and\
shutters being held open by someone or something\
aren’t affected.\
Beckon Earth. You create a thin shroud of dust or\
sand that covers surfaces in a 5-foot-square area, or\
you cause a single word to appear in your handwrit-\
ing in a patch of dirt or sand.\
Beckon Fire. You create a thin cloud of harmless\
embers and colored, scented smoke in a 5-foot Cube.\
You choose the color and scent, and the embers can\
light candles, torches, or lamps in that area. The\
smoke’s scent lingers for 1 minute.\
Beckon Water. You create a spray of cool mist that\
lightly dampens creatures and objects in a 5-foot\
Cube. Alternatively, you create 1 cup of clean water\
either in an open container or on a surface, and the\
water evaporates in 1 minute.\
Sculpt Element. You cause dirt, sand, fire, smoke,\
mist, or water that can fit in a 1-foot Cube to assume\
a crude shape (such as that of a creature) for 1 hour.\

Original layout: [[../../srd-5.2.1/pages/127|page 127]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["druid", "sorcerer", "wizard"] ` |
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
| ` mechanic ` | ` dnd2024:spells/elementalism ` |
| ` implementation ` | ` dnd2024:spell_elementalism ` |

### Parameters

#### Selection

<!-- splat:values path=splat_rule/parameters/selection -->

| Field | Value |
| --- | --- |
| ` effect_count ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |

#### Modes

<!-- splat:yaml path=splat_rule/parameters/modes -->

```yaml
beckon_air:
  shape: cube
  side_feet: 5
  effects:
  - ripple_cloth
  - stir_dust
  - rustle_leaves
  - close_doors
  - close_shutters
  doors_shutters_held_open_immune: true
beckon_earth:
  choices:
  - cover: thin_dust_or_sand
    shape: square
    side_feet: 5
  - write_words_maximum: 1
    medium:
    - dirt
    - sand
    handwriting: caster
beckon_fire:
  shape: cube
  side_feet: 5
  embers_harmless: true
  smoke_color_choice: true
  smoke_scent_choice: true
  scent_duration_minutes: 1
  can_light:
  - candles
  - torches
  - lamps
beckon_water:
  choices:
  - shape: cube
    side_feet: 5
    effect: lightly_dampen_creatures_and_objects
    temperature: cool
  - clean_water_cups: 1
    destination:
    - open_container
    - surface
    evaporates_after_minutes: 1
sculpt_element:
  materials:
  - dirt
  - sand
  - fire
  - smoke
  - mist
  - water
  maximum_cube_side_feet: 1
  shape_choice: crude
  duration_hours: 1
```
