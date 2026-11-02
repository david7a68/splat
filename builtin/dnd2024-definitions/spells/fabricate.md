---
splat_schema: 1
splat_id: 160877bd-5c87-5b3a-b2f2-867c566a400e
splat_kind: spell
splat_key: dnd2024:spells/fabricate
splat_name: Fabricate
splat_origin:
  key: dnd2024:reference/spell/fabricate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 129
  url: https://www.dndbeyond.com/srd
---

# Fabricate

## Source text

<!-- SRD page 129 -->

Level 4 Transmutation (Wizard)\
Casting Time: 10 minutes\
Range: 120 feet\
Components: V, S\
Duration: Instantaneous

You convert raw materials into products of the same material. For example, you can fabricate a wooden bridge from a clump of trees, a rope from a patch of hemp, or clothes from flax or wool.

Choose raw materials that you can see within range. You can fabricate a Large or smaller object (contained within a 10-foot Cube or eight connected 5-foot Cubes) given a sufficient quantity of material. If you’re working with metal, stone, or another mineral substance, however, the fabricated object can be no larger than Medium (contained within a 5-foot Cube). The quality of any fabricated objects is based on the quality of the raw materials.

Creatures and magic items can’t be created by this spell. You also can’t use it to create items that require a high degree of skill—such as weapons and armor—unless you have proficiency with the type of Artisan’s Tools used to craft such objects.

Original layout: [[../../srd-5.2.1/pages/129|page 129]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` 10 minutes ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/fabricate ` |
| ` implementation ` | ` dnd2024:spell_fabricate ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` forbidden_output ` | ` ["creature", "magic_item"] ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Inputs

<!-- splat:values path=splat_rule/parameters/inputs -->

| Field | Value |
| --- | --- |
| ` kind ` | ` raw_materials ` |
| ` visible ` | ` true ` |
| ` range_feet ` | ` 120 ` |
| ` sufficient_quantity_required ` | ` true ` |

#### Output

<!-- splat:values path=splat_rule/parameters/output -->

| Field | Value |
| --- | --- |
| ` kind ` | ` object ` |
| ` material ` | ` same_as_inputs ` |
| ` quality ` | ` based_on_input_quality ` |
| ` maximum_size ` | ` large ` |

##### Geometry choice

<!-- splat:table
path: splat_rule/parameters/output/geometry_choice
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Cube side feet:
    path: cube_side_feet
    type: integer
  Count:
    path: count
    type: integer
  Connected:
    path: connected
    type: boolean
    optional: true
-->

| Key | Cube side feet | Count | Connected |
| --- | --- | --- | --- |
| ` row_1 ` | ` 10 ` | ` 1 ` |  |
| ` row_2 ` | ` 5 ` | ` 8 ` | true |

#### Mineral exception

<!-- splat:values path=splat_rule/parameters/mineral_exception -->

| Field | Value |
| --- | --- |
| ` materials ` | ` ["metal", "stone", "other_mineral"] ` |
| ` maximum_size ` | ` medium ` |
| ` maximum_cube_side_feet ` | ` 5 ` |

#### Skilled craft

<!-- splat:values path=splat_rule/parameters/skilled_craft -->

| Field | Value |
| --- | --- |
| ` examples ` | ` ["weapon", "armor"] ` |
| ` requires_proficiency ` | ` corresponding_artisans_tools ` |

#### Supplied facts

<!-- splat:values path=splat_rule/parameters/supplied_facts -->

| Field | Value |
| --- | --- |
| ` material_sufficiency ` | ` boolean ` |
| ` input_quality ` | ` choice ` |
| ` requires_high_skill ` | ` boolean ` |
| ` corresponding_tools ` | ` definition ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
