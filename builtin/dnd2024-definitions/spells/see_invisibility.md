---
splat_schema: 1
splat_id: b62a1a1e-7eec-54a5-83f8-3de063fb1e61
splat_kind: spell
splat_key: dnd2024:spells/see_invisibility
splat_name: See Invisibility
splat_origin:
  key: dnd2024:reference/spell/see_invisibility
splat_sources:
- title: SRD 5.2.1
  locator: pp. 160
  url: https://www.dndbeyond.com/srd
---

# See Invisibility

## Source text

<!-- SRD page 160 -->

Level 2 Divination (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a pinch of talc)\
Duration: 1 hour

For the duration, you see creatures and objects that have the Invisible condition as if they were visible, and you can see into the Ethereal Plane. Creatures and objects there appear ghostly.

Original layout: [[../../srd-5.2.1/pages/160|page 160]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["bard", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a pinch of talc ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/see_invisibility ` |
| ` implementation ` | ` dnd2024:spell_see_invisibility ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |

#### Vision

<!-- splat:values path=splat_rule/parameters/vision -->

| Field | Value |
| --- | --- |
| ` invisible_creatures_and_objects ` | ` as_visible ` |
| ` ethereal_plane ` | ` true ` |
| ` ethereal_appearance ` | ` ghostly ` |
