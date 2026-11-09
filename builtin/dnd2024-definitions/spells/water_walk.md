---
splat_schema: 2
splat_id: ae4dc889-df0f-5d43-b166-eff1b6d007a8
splat_kind: spell
splat_key: dnd2024:spells/water_walk
splat_name: Water Walk
splat_origin: dnd2024:reference/spell/water_walk
splat_sources:
- SRD 5.2.1, pp. 173, 174, https://www.dndbeyond.com/srd
---

# Water Walk


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Transmutation |
| Classes | Cleric, Druid, Ranger, Sorcerer |
| Casting Time | Action or Ritual |
| Range | 30 feet |
| Components | V, S, M (a piece of cork) |
| Duration | 1 hour |
| Ritual | Yes |
| Concentration | No |


## Source text

<!-- SRD page 173 -->

Level 3 Transmutation (Cleric, Druid, Ranger, Sorcerer)\
Casting Time: Action or Ritual\
Range: 30 feet\
Components: V, S, M (a piece of cork)\
Duration: 1 hour

<!-- SRD page 174 -->

This spell grants the ability to move across any liquid surface—such as water, acid, mud, snow, quicksand, or lava—as if it were harmless solid ground (creatures crossing molten lava can still take damage from the heat). Up to ten willing creatures of your choice within range gain this ability for the duration.

An affected target must take a Bonus Action to pass from the liquid’s surface into the liquid itself and vice versa, but if the target falls into the liquid, the target passes through the surface into the liquid below.

Original layout: [[../../srd-5.2.1/pages/173|page 173]], [[../../srd-5.2.1/pages/174|page 174]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/water_walk ` |
| ` implementation ` | ` dnd2024:spell_water_walk ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 3600 ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` willing ` | ` true ` |
| ` chooser ` | ` caster ` |
| ` maximum_count ` | ` 10 ` |
| ` range_feet ` | ` 30 ` |

#### Movement

<!-- splat:values path=splat_rule/parameters/movement -->

| Field | Value |
| --- | --- |
| ` surface_types ` | ` ["any_liquid", "water", "acid", "mud", "snow", "quicksand", "lava"] ` |
| ` treat_surface_as ` | ` harmless_solid_ground ` |
| ` lava_heat_damage ` | ` still_possible ` |

#### Cross surface

<!-- splat:values path=splat_rule/parameters/cross_surface -->

| Field | Value |
| --- | --- |
| ` directions ` | ` ["surface_to_liquid", "liquid_to_surface"] ` |
| ` economy ` | ` bonus_action ` |

#### Falling into liquid

<!-- splat:values path=splat_rule/parameters/falling_into_liquid -->

| Field | Value |
| --- | --- |
| ` passes_through_surface ` | ` true ` |
| ` enters_liquid ` | ` true ` |
| ` bonus_action_required ` | ` false ` |

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

