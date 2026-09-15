---
splat_schema: 1
splat_id: e453db69-a71f-5b63-87bc-c0c86214f934
splat_kind: spell
splat_key: dnd2024:spells/meteor_swarm
splat_name: Meteor Swarm
splat_origin:
  key: dnd2024:reference/spell/meteor_swarm
splat_sources:
- title: SRD 5.2.1
  locator: pp. 149
  url: https://www.dndbeyond.com/srd
---

# Meteor Swarm

## Source text

<!-- SRD page 149 -->

Level 9 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 1 mile\
Components: V, S\
Duration: Instantaneous\
Blazing orbs of fire plummet to the ground at four\
different points you can see within range. Each\
creature in a 40-foot-radius Sphere centered on\
each of those points makes a Dexterity saving\
throw. A creature takes 20d6 Fire damage and 20d6\
Bludgeoning damage on a failed save or half as\
much damage on a successful one. A creature in the\
area of more than one fiery Sphere is affected only\
once.\
A nonmagical object that isn’t being worn or car-\
ried also takes the damage if it’s in the spell’s area,\
and the object starts burning if it’s flammable.\

Original layout: [[../../srd-5.2.1/pages/149|page 149]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 9 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` 1 mile ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/meteor_swarm ` |
| ` implementation ` | ` dnd2024:spell_meteor_swarm ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Areas

<!-- splat:values path=splat_rule/parameters/areas -->

| Field | Value |
| --- | --- |
| ` count ` | ` 4 ` |
| ` different_points ` | ` true ` |
| ` point_visible ` | ` true ` |
| ` range_miles ` | ` 1 ` |
| ` impact ` | ` ground ` |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 40 ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` in_any_area ` | ` true ` |
| ` overlapping_areas_affect_count ` | ` 1 ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` dc ` | ` spell_save_dc ` |

#### Damage

<!-- splat:table
path: splat_rule/parameters/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Dice:
    path: dice
    type: string
  Type:
    path: type
    type: symbol
-->

| Key | Dice | Type |
| --- | --- | --- |
| ` row_1 ` | ` 20d6 ` | ` Fire ` |
| ` row_2 ` | ` 20d6 ` | ` Bludgeoning ` |

#### Success damage scale

<!-- splat:values path=splat_rule/parameters/success_damage_scale -->

| Field | Value |
| --- | --- |
| ` numerator ` | ` 1 ` |
| ` denominator ` | ` 2 ` |

#### Objects

<!-- splat:values path=splat_rule/parameters/objects -->

| Field | Value |
| --- | --- |
| ` magical ` | ` false ` |
| ` worn ` | ` false ` |
| ` carried ` | ` false ` |
| ` in_area ` | ` true ` |
| ` take_damage ` | ` true ` |
| ` flammable_effect ` | ` start_burning ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
