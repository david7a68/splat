---
splat_schema: 2
splat_id: 05638e36-0859-546d-99bc-47ed8f8331e2
splat_kind: spell
splat_key: dnd2024:spells/locate_animals_or_plants
splat_name: Locate Animals or Plants
splat_origin: dnd2024:reference/spell/locate_animals_or_plants
splat_sources:
- SRD 5.2.1, pp. 144, https://www.dndbeyond.com/srd
---

# Locate Animals or Plants


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Divination |
| Classes | Bard, Druid, Ranger |
| Casting Time | Action or Ritual |
| Range |  |
| Components | V, S, M (fur from a bloodhound) |
| Duration | Instantaneous |


## Source text

<!-- SRD page 144 -->

Level 2 Divination (Bard, Druid, Ranger)\
Casting Time: Action or Ritual\
Range: Self\
Components: V, S, M (fur from a bloodhound)\
Duration: Instantaneous

Describe or name a specific kind of Beast, Plant creature, or nonmagical plant. You learn the direction and distance to the closest creature or plant of that kind within 5 miles, if any are present.

Original layout: [[../../srd-5.2.1/pages/144|page 144]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/locate_animals_or_plants ` |
| ` implementation ` | ` dnd2024:spell_locate_animals_or_plants ` |

### Parameters

#### Query

<!-- splat:values path=splat_rule/parameters/query -->

| Field | Value |
| --- | --- |
| ` selection ` | ` named_or_described_kind ` |
| ` categories ` | ` ["beast", "plant_creature", "nonmagical_plant"] ` |
| ` radius_miles ` | ` 5 ` |
| ` match ` | ` closest ` |
| ` output ` | ` ["direction", "distance"] ` |
| ` requires_match_present ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```

