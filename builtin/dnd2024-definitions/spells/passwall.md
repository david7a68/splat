---
splat_schema: 1
splat_id: fa9efad8-6342-54dc-b7ed-b9c16ff919a0
splat_kind: spell
splat_key: dnd2024:spells/passwall
splat_name: Passwall
splat_origin:
  key: dnd2024:reference/spell/passwall
splat_sources:
- title: SRD 5.2.1
  locator: pp. 151
  url: https://www.dndbeyond.com/srd
---

# Passwall

## Source text

<!-- SRD page 151 -->

Level 5 Transmutation (Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a pinch of sesame seeds)\
Duration: 1 hour\
A passage appears at a point that you can see on a\
wooden, plaster, or stone surface (such as a wall,\
ceiling, or floor) within range and lasts for the dura-\
tion. You choose the opening’s dimensions: up to 5\
feet wide, 8 feet tall, and 20 feet deep. The passage\
creates no instability in a structure surrounding it.\
When the opening disappears, any creatures or\
objects still in the passage created by the spell are\
safely ejected to an unoccupied space nearest to the\
surface on which you cast the spell.\

Original layout: [[../../srd-5.2.1/pages/151|page 151]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a pinch of sesame seeds ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/passwall ` |
| ` implementation ` | ` dnd2024:spell_passwall ` |

### Parameters

#### Surface

<!-- splat:values path=splat_rule/parameters/surface -->

| Field | Value |
| --- | --- |
| ` materials ` | ` ["wood", "plaster", "stone"] ` |
| ` visible ` | ` true ` |

#### Passage

<!-- splat:values path=splat_rule/parameters/passage -->

| Field | Value |
| --- | --- |
| ` dimensions_chooser ` | ` caster ` |
| ` maximum_width_feet ` | ` 5 ` |
| ` maximum_height_feet ` | ` 8 ` |
| ` maximum_depth_feet ` | ` 20 ` |
| ` structural_instability ` | ` false ` |

#### End

<!-- splat:values path=splat_rule/parameters/end -->

| Field | Value |
| --- | --- |
| ` subjects ` | ` creatures_and_objects_in_passage ` |
| ` eject_safely ` | ` true ` |
| ` destination ` | ` unoccupied_nearest_cast_surface ` |
