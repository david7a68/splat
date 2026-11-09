---
splat_schema: 2
splat_id: c58ffaf5-abf4-5105-bae0-03f347059b4c
splat_kind: spell
splat_key: dnd2024:spells/find_the_path
splat_name: Find the Path
splat_origin: dnd2024:reference/spell/find_the_path
splat_sources:
- SRD 5.2.1, pp. 131, https://www.dndbeyond.com/srd
---

# Find the Path


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Divination |
| Classes | Bard, Cleric, Druid |
| Casting Time | 1 minute |
| Range |  |
| Components | V, S, M (a set of divination tools—such as cards or runes—worth 100+ GP) |
| Duration | Concentration, up to 1 day |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 131 -->

Level 6 Divination (Bard, Cleric, Druid)\
Casting Time: 1 minute\
Range: Self\
Components: V, S, M (a set of divination tools—such as cards or runes—worth 100+ GP)\
Duration: Concentration, up to 1 day

You magically sense the most direct physical route to a location you name. You must be familiar with the location, and the spell fails if you name a destination on another plane of existence, a moving destination (such as a mobile fortress), or an unspecific destination (such as “a green dragon’s lair”).

For the duration, as long as you are on the same plane of existence as the destination, you know how far it is and in what direction it lies. Whenever you face a choice of paths along the way there, you know which path is the most direct.

Original layout: [[../../srd-5.2.1/pages/131|page 131]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/find_the_path ` |
| ` implementation ` | ` dnd2024:spell_find_the_path ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` output ` | ` most_direct_physical_route ` |
| ` supplied_world_facts ` | ` ["physical_route_network", "destination_familiarity", "destination_motion", "destination_plane"] ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 86400 ` |

#### Destination

<!-- splat:values path=splat_rule/parameters/destination -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` named ` | ` true ` |
| ` familiar ` | ` true ` |
| ` must_be_specific ` | ` true ` |
| ` must_be_stationary ` | ` true ` |
| ` same_plane_at_cast ` | ` true ` |
| ` violation ` | ` spell_fails ` |

#### Ongoing information

<!-- splat:values path=splat_rule/parameters/ongoing_information -->

| Field | Value |
| --- | --- |
| ` requires ` | ` same_plane_as_destination ` |
| ` outputs ` | ` ["distance", "direction"] ` |
| ` at_path_choice ` | ` most_direct_path ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```

