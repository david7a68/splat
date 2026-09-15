---
splat_schema: 1
splat_id: 8344fcc7-ab23-5c62-a184-db3913394a69
splat_kind: spell
splat_key: dnd2024:spells/grease
splat_name: Grease
splat_origin:
  key: dnd2024:reference/spell/grease
splat_sources:
- title: SRD 5.2.1
  locator: pp. 137
  url: https://www.dndbeyond.com/srd
---

# Grease

## Source text

<!-- SRD page 137 -->

Level 1 Conjuration (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a bit of pork rind or butter)\
Duration: 1 minute\
Nonflammable grease covers the ground in a 10-\
foot square centered on a point within range and\
turns it into Difficult Terrain for the duration.\
When the grease appears, each creature stand-\
ing in its area must succeed on a Dexterity saving\
throw or have the Prone condition. A creature that\
enters the area or ends its turn there must also suc-\
ceed on that save or fall Prone.\

Original layout: [[../../srd-5.2.1/pages/137|page 137]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a bit of pork rind or butter ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/grease ` |
| ` implementation ` | ` dnd2024:spell_grease ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 60 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` square ` |
| ` side_feet ` | ` 10 ` |
| ` surface ` | ` ground ` |
| ` origin ` | ` chosen_point ` |
| ` range_feet ` | ` 60 ` |

#### Properties

<!-- splat:values path=splat_rule/parameters/properties -->

| Field | Value |
| --- | --- |
| ` flammable ` | ` false ` |
| ` difficult_terrain ` | ` true ` |

#### Saving throw triggers

<!-- splat:rows path=splat_rule/parameters/saving_throw_triggers -->

| Row | event | subjects |
| --- | --- | --- |
| ` 0 ` | ` spell.cast_completed ` | ` creatures_standing_in_area ` |
| ` 1 ` | ` creature.entered_area ` |  |
| ` 2 ` | ` turn.end ` | ` creatures_in_area ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` dc ` | ` spell_save_dc ` |
| ` failure_condition ` | ` prone ` |
| ` success ` | ` unaffected ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
