---
splat_schema: 1
splat_id: cd9719c9-693a-55fd-9cc5-065882e41c16
splat_kind: spell
splat_key: dnd2024:spells/tiny_hut
splat_name: Tiny Hut
splat_origin:
  key: dnd2024:reference/spell/tiny_hut
splat_sources:
- title: SRD 5.2.1
  locator: pp. 169
  url: https://www.dndbeyond.com/srd
---

# Tiny Hut

## Source text

<!-- SRD page 169 -->

Level 3 Evocation (Bard, Wizard)\
Casting Time: 1 minute or Ritual\
Range: Self\
Components: V, S, M (a crystal bead)\
Duration: 8 hours\
A 10-foot Emanation springs into existence around\
you and remains stationary for the duration. The\
spell fails when you cast it if the Emanation isn’t big\
enough to fully encapsulate all creatures in its area.\
Creatures and objects within the Emanation\
when you cast the spell can move through it freely.\
All other creatures and objects are barred from\
passing through it. Spells of level 3 or lower can’t be\
cast through it, and the effects of such spells can’t\
extend into it.\
The atmosphere inside the Emanation is comfort-\
able and dry, regardless of the weather outside. Until\
the spell ends, you can command the interior to have\
Dim Light or Darkness (no action required). The Em-\
anation is opaque from the outside and of any color\
you choose, but it’s transparent from the inside.\
The spell ends early if you leave the Emanation or\
if you cast it again.\

Original layout: [[../../srd-5.2.1/pages/169|page 169]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["bard", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 8 hours ` |
| ` casting_time ` | ` 1 minute or Ritual ` |
| ` ritual ` | ` true ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a crystal bead ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/tiny_hut ` |
| ` implementation ` | ` dnd2024:spell_tiny_hut ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` early_end_conditions ` | ` ["caster_leaves_area", "caster_casts_tiny_hut_again"] ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 28800 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` emanation ` |
| ` radius_feet ` | ` 10 ` |
| ` origin ` | ` caster_at_cast ` |
| ` stationary ` | ` true ` |

#### Cast failure

<!-- splat:values path=splat_rule/parameters/cast_failure -->

| Field | Value |
| --- | --- |
| ` if ` | ` any_creature_in_area_not_fully_encapsulated ` |

#### Crossing

<!-- splat:values path=splat_rule/parameters/crossing -->

| Field | Value |
| --- | --- |
| ` creatures_and_objects_present_at_cast ` | ` freely_allowed ` |
| ` all_other_creatures_and_objects ` | ` barred ` |

#### Spell barrier

<!-- splat:values path=splat_rule/parameters/spell_barrier -->

| Field | Value |
| --- | --- |
| ` maximum_spell_level_blocked ` | ` 3 ` |
| ` cast_through ` | ` forbidden ` |
| ` effects_extend_inside ` | ` forbidden ` |

#### Interior

<!-- splat:yaml path=splat_rule/parameters/interior -->

```yaml
atmosphere: comfortable_and_dry
weather_independent: true
light_choice:
  chooser: caster
  economy: none
  options:
  - dim
  - darkness
```

#### Appearance

<!-- splat:values path=splat_rule/parameters/appearance -->

| Field | Value |
| --- | --- |
| ` outside ` | ` opaque ` |
| ` color ` | ` caster_choice ` |
| ` inside ` | ` transparent ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
