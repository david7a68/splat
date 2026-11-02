---
splat_schema: 1
splat_id: 54c22968-f68b-5337-aef0-496804aee395
splat_kind: spell
splat_key: dnd2024:spells/hallucinatory_terrain
splat_name: Hallucinatory Terrain
splat_origin:
  key: dnd2024:reference/spell/hallucinatory_terrain
splat_sources:
- title: SRD 5.2.1
  locator: pp. 139
  url: https://www.dndbeyond.com/srd
---

# Hallucinatory Terrain

## Source text

<!-- SRD page 139 -->

Level 4 Illusion (Bard, Druid, Warlock, Wizard)\
Casting Time: 10 minutes\
Range: 300 feet\
Components: V, S, M (a mushroom)\
Duration: 24 hours

You make natural terrain in a 150-foot Cube in range look, sound, and smell like another sort of natural terrain. Thus, open fields or a road can be made to resemble a swamp, hill, crevasse, or some other difficult or impassable terrain. A pond can be made to seem like a grassy meadow, a precipice like a gentle slope, or a rock-strewn gully like a wide and smooth road. Manufactured structures, equipment, and creatures within the area aren’t changed.

The tactile characteristics of the terrain are unchanged, so creatures entering the area are likely to notice the illusion. If the difference isn’t obvious by touch, a creature examining the illusion can take the Study action to make an Intelligence (Investigation) check against your spell save DC to disbelieve it. If a creature discerns that the terrain is illusory, the creature sees a vague image superimposed on the real terrain.

Original layout: [[../../srd-5.2.1/pages/139|page 139]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["bard", "druid", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 24 hours ` |
| ` casting_time ` | ` 10 minutes ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 300 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a mushroom ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/hallucinatory_terrain ` |
| ` implementation ` | ` dnd2024:spell_hallucinatory_terrain ` |

### Parameters

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cube ` |
| ` side_feet ` | ` 150 ` |
| ` target ` | ` natural_terrain ` |

#### Illusion

<!-- splat:values path=splat_rule/parameters/illusion -->

| Field | Value |
| --- | --- |
| ` replacement_choice ` | ` natural_terrain_kind ` |
| ` senses ` | ` ["sight", "hearing", "smell"] ` |
| ` tactile_unchanged ` | ` true ` |
| ` excluded ` | ` ["manufactured_structures", "equipment", "creatures"] ` |

#### Discern

<!-- splat:yaml path=splat_rule/parameters/discern -->

```yaml
touch_obvious_difference: automatic
otherwise:
  action: study
  ability: intelligence
  skill: investigation
  dc:
    binding: save_dc
  success: disbelieve
after_discern: vague_image_over_real_terrain
```
