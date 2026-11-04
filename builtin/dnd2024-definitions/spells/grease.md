---
splat_schema: 2
splat_id: 8344fcc7-ab23-5c62-a184-db3913394a69
splat_kind: spell
splat_key: dnd2024:spells/grease
splat_name: Grease
splat_origin: dnd2024:reference/spell/grease
splat_sources:
- SRD 5.2.1, pp. 137, https://www.dndbeyond.com/srd
---

# Grease


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Conjuration |
| Classes | Sorcerer, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S, M (a bit of pork rind or butter) |
| Duration | 1 minute |


## Source text

<!-- SRD page 137 -->

Level 1 Conjuration (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a bit of pork rind or butter)\
Duration: 1 minute

Nonflammable grease covers the ground in a 10-foot square centered on a point within range and turns it into Difficult Terrain for the duration.

When the grease appears, each creature standing in its area must succeed on a Dexterity saving throw or have the Prone condition. A creature that enters the area or ends its turn there must also succeed on that save or fall Prone.

Original layout: [[../../srd-5.2.1/pages/137|page 137]].







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

<!-- splat:table
path: splat_rule/parameters/saving_throw_triggers
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Event:
    path: event
    type: string
  Subjects:
    path: subjects
    type: symbol
    optional: true
-->

| Key | Event | Subjects |
| --- | --- | --- |
| ` row_1 ` | ` spell.cast_completed ` | ` Creatures standing in area ` |
| ` row_2 ` | ` creature.entered_area ` |  |
| ` row_3 ` | ` turn.end ` | ` Creatures in area ` |

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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

