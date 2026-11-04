---
splat_schema: 2
splat_id: 91923fd8-5c7e-565e-8d34-ce84dec1e42b
splat_kind: spell
splat_key: dnd2024:spells/commune_with_nature
splat_name: Commune with Nature
splat_origin: dnd2024:reference/spell/commune_with_nature
splat_sources:
- SRD 5.2.1, pp. 116, 117, https://www.dndbeyond.com/srd
---

# Commune with Nature


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Divination |
| Classes | Druid, Ranger |
| Casting Time | 1 minute or Ritual |
| Range |  |
| Components | V, S |
| Duration | Instantaneous |


## Source text

<!-- SRD page 116 -->

Level 5 Divination (Druid, Ranger)\
Casting Time: 1 minute or Ritual\
Range: Self\
Components: V, S\
Duration: Instantaneous

You commune with nature spirits and gain knowledge of the surrounding area. In the outdoors, the spell gives you knowledge of the area within 3 miles of you. In caves and other natural underground settings, the radius is limited to 300 feet. The spell doesn’t function where nature has been replaced by construction, such as in castles and settlements.

<!-- SRD page 117 -->

Choose three of the following facts; you learn those facts as they pertain to the spell’s area:

•  Locations of settlements

•  Locations of portals to other planes of existence

•  Location of one Challenge Rating 10+ creature (GM’s choice) that is a Celestial, an Elemental, a Fey, a Fiend, or an Undead

•  The most prevalent kind of plant, mineral, or Beast (you choose which to learn)

•  Locations of bodies of water

For example, you could determine the location of a powerful monster in the area, the locations of bodies of water, and the locations of any towns.

Original layout: [[../../srd-5.2.1/pages/116|page 116]], [[../../srd-5.2.1/pages/117|page 117]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/commune_with_nature ` |
| ` implementation ` | ` dnd2024:spell_commune_with_nature ` |

### Parameters

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` outdoors_radius_miles ` | ` 3 ` |
| ` natural_underground_radius_feet ` | ` 300 ` |
| ` fails_where_nature_replaced_by_construction ` | ` true ` |

#### Knowledge

<!-- splat:yaml path=splat_rule/parameters/knowledge -->

```yaml
choose_facts_count: 3
choices:
  settlements:
    output: locations
  planar_portals:
    output: locations
  powerful_creature:
    minimum_challenge_rating: 10
    creature_types:
    - celestial
    - elemental
    - fey
    - fiend
    - undead
    chooser: gm
    count: 1
    output: location
  most_prevalent_kind:
    caster_category_choice:
    - plant
    - mineral
    - beast
    output: kind
  bodies_of_water:
    output: locations
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```

