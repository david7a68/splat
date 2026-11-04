---
splat_schema: 2
splat_id: 8c8df744-4409-56d5-8c63-b2b4ac3eed11
splat_kind: spell
splat_key: dnd2024:spells/hallow
splat_name: Hallow
splat_origin: dnd2024:reference/spell/hallow
splat_sources:
- SRD 5.2.1, pp. 138, 139, https://www.dndbeyond.com/srd
---

# Hallow


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Abjuration |
| Classes | Cleric |
| Casting Time | 24 hours |
| Range |  |
| Components | V, S, M (incense worth 1,000+ GP, which the spell consumes) |
| Duration | Until dispelled |


## Source text

<!-- SRD page 138 -->

Level 5 Abjuration (Cleric)\
Casting Time: 24 hours\
Range: Touch\
Components: V, S, M (incense worth 1,000+ GP, which the spell consumes)\
Duration: Until dispelled

You touch a point and infuse an area around it with holy or unholy power. The area can have a radius up to 60 feet, and the spell fails if the radius includes an area already under the effect of Hallow. The affected area has the following effects.

Hallowed Ward. Choose any of these creature types: Aberration, Celestial, Elemental, Fey, Fiend, or Undead. Creatures of the chosen types can’t willingly enter the area, and any creature that is possessed by or that has the Charmed or Frightened condition from such creatures isn’t possessed, Charmed, or Frightened by them while in the area.

Extra Effect. You bind an extra effect to the area from the list below:

Courage. Creatures of any types you choose can’t gain the Frightened condition while in the area.

Darkness. Darkness fills the area. Normal light, as well as magical light created by spells of a level lower than this spell, can’t illuminate the area.

<!-- SRD page 139 -->

Daylight. Bright light fills the area. Magical Darkness created by spells of a level lower than this spell can’t extinguish the light.

Peaceful Rest. Dead bodies interred in the area can’t be turned into Undead.

Extradimensional Interference. Creatures of any types you choose can’t enter or exit the area using teleportation or interplanar travel.

Fear. Creatures of any types you choose have the Frightened condition while in the area.

Resistance. Creatures of any types you choose have Resistance to one damage type of your choice while in the area.

Silence. No sound can emanate from within the area, and no sound can reach into it.

Tongues. Creatures of any types you choose can communicate with any other creature in the area even if they don’t share a common language.

Vulnerability. Creatures of any types you choose have Vulnerability to one damage type of your choice while in the area.

Original layout: [[../../srd-5.2.1/pages/138|page 138]], [[../../srd-5.2.1/pages/139|page 139]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/hallow ` |
| ` implementation ` | ` dnd2024:spell_hallow ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` until ` | ` dispelled_or_source_specific_end ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` origin ` | ` touched_point ` |
| ` radius_feet_max ` | ` 60 ` |
| ` overlap_existing_hallow ` | ` spell_fails ` |

#### Ward

<!-- splat:yaml path=splat_rule/parameters/ward -->

```yaml
chooser: caster
type_selection: any_subset
type_options:
- aberration
- celestial
- elemental
- fey
- fiend
- undead
selected_types:
  willing_entry: forbidden
occupants:
  suppress_effects_from_selected_types:
  - possession
  - charmed
  - frightened
  suppression_scope: while_in_area
```

#### Extra effect

<!-- splat:values path=splat_rule/parameters/extra_effect -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` count ` | ` 1 ` |

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
courage:
  creature_types: caster_choice
  while_in_area:
    gain_frightened: forbidden
darkness:
  light_level: darkness
  normal_light_illuminates: false
  magical_light_illuminates:
    spell_level_less_than_cast_level: false
daylight:
  light_level: bright
  magical_darkness_extinguishes:
    spell_level_less_than_cast_level: false
peaceful_rest:
  subjects: dead_bodies_interred_in_area
  transform_into_undead: forbidden
extradimensional_interference:
  creature_types: caster_choice
  forbidden_travel:
  - teleport_enter
  - teleport_exit
  - interplanar_enter
  - interplanar_exit
fear:
  creature_types: caster_choice
  while_in_area:
    condition: frightened
resistance:
  creature_types: caster_choice
  damage_type:
    chooser: caster
    count: 1
  while_in_area:
    resistance: chosen_damage_type
silence:
  sound_from_inside: blocked
  sound_entering: blocked
tongues:
  creature_types: caster_choice
  communication_with: any_creature_in_area
  shared_language_required: false
vulnerability:
  creature_types: caster_choice
  damage_type:
    chooser: caster
    count: 1
  while_in_area:
    vulnerability: chosen_damage_type
```

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
>   consumed: true
> range:
>   mode: touch
> ```

