---
splat_schema: 1
splat_id: a6b1254d-f8a5-55fb-a778-d8b0d1e04488
splat_kind: spell
splat_key: dnd2024:spells/mirage_arcane
splat_name: Mirage Arcane
splat_origin:
  key: dnd2024:reference/spell/mirage_arcane
splat_sources:
- title: SRD 5.2.1
  locator: pp. 149, 150
  url: https://www.dndbeyond.com/srd
---

# Mirage Arcane

## Source text

<!-- SRD page 149 -->

Level 7 Illusion (Bard, Druid, Wizard)\
Casting Time: 10 minutes\
Range: Sight\
Components: V, S\
Duration: 10 days

You make terrain in an area up to 1 mile square look, sound, smell, and even feel like some other sort of terrain. Open fields or a road could be made to resemble a swamp, hill, crevasse, or some other rough or impassable terrain. A pond can be made to seem like a grassy meadow, a precipice like a gentle slope, or a rock-strewn gully like a wide and smooth road.

Similarly, you can alter the appearance of structures or add them where none are present. The spell doesn’t disguise, conceal, or add creatures.

The illusion includes audible, visual, tactile, and olfactory elements, so it can turn clear ground into Difficult Terrain (or vice versa) or otherwise impede movement through the area. Any piece of the illusory terrain (such as a rock or stick) that is removed from the spell’s area disappears immediately.

<!-- SRD page 150 -->

Creatures with Truesight can see through the illusion to the terrain’s true form; however, all other elements of the illusion remain, so while the creature is aware of the illusion’s presence, the creature can still physically interact with the illusion.

Original layout: [[../../srd-5.2.1/pages/149|page 149]], [[../../srd-5.2.1/pages/150|page 150]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["bard", "druid", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 10 days ` |
| ` casting_time ` | ` 10 minutes ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` sight ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/mirage_arcane ` |
| ` implementation ` | ` dnd2024:spell_mirage_arcane ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 864000 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` maximum_square_miles ` | ` 1 ` |
| ` range ` | ` sight ` |

#### Terrain

<!-- splat:values path=splat_rule/parameters/terrain -->

| Field | Value |
| --- | --- |
| ` appearance_design ` | ` caster_choice ` |
| ` modalities ` | ` ["visual", "audible", "tactile", "olfactory"] ` |
| ` movement_effect_choices ` | ` ["create_difficult_terrain", "remove_difficult_terrain", "impede_movement"] ` |
| ` terrain_geometry ` | ` supplied_design ` |

#### Structures

<!-- splat:values path=splat_rule/parameters/structures -->

| Field | Value |
| --- | --- |
| ` alter_appearance ` | ` true ` |
| ` add_illusory_structures ` | ` true ` |

#### Creatures

<!-- splat:values path=splat_rule/parameters/creatures -->

| Field | Value |
| --- | --- |
| ` disguise ` | ` false ` |
| ` conceal ` | ` false ` |
| ` add ` | ` false ` |

#### Removed illusory piece

<!-- splat:values path=splat_rule/parameters/removed_illusory_piece -->

| Field | Value |
| --- | --- |
| ` event ` | ` object.left_area ` |
| ` effect ` | ` immediately_disappear ` |

#### Truesight

<!-- splat:values path=splat_rule/parameters/truesight -->

| Field | Value |
| --- | --- |
| ` see_true_terrain ` | ` true ` |
| ` aware_of_illusion ` | ` true ` |
| ` removes_other_modalities ` | ` false ` |
| ` physical_interaction_remains_possible ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
