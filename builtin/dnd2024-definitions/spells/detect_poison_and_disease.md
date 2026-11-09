---
splat_schema: 2
splat_id: d53fd494-856f-5144-b5a3-f8dc0a4a977c
splat_kind: spell
splat_key: dnd2024:spells/detect_poison_and_disease
splat_name: Detect Poison and Disease
splat_origin: dnd2024:reference/spell/detect_poison_and_disease
splat_sources:
- SRD 5.2.1, pp. 123, https://www.dndbeyond.com/srd
---

# Detect Poison and Disease


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Divination |
| Classes | Cleric, Druid, Paladin, Ranger |
| Casting Time | Action or Ritual |
| Range |  |
| Components | V, S, M (a yew leaf) |
| Duration | Concentration, up to 10 minutes |
| Ritual | Yes |
| Concentration | Yes |


## Source text

<!-- SRD page 123 -->

Level 1 Divination (Cleric, Druid, Paladin, Ranger)\
Casting Time: Action or Ritual\
Range: Self\
Components: V, S, M (a yew leaf)\
Duration: Concentration, up to 10 minutes

For the duration, you sense the location of poisons, poisonous or venomous creatures, and magical contagions within 30 feet of yourself. You sense the kind of poison, creature, or contagion in each case.

The spell is blocked by 1 foot of stone, dirt, or wood; 1 inch of metal; or a thin sheet of lead.

Original layout: [[../../srd-5.2.1/pages/123|page 123]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/detect_poison_and_disease ` |
| ` implementation ` | ` dnd2024:spell_detect_poison_and_disease ` |

### Parameters

#### Sense

<!-- splat:values path=splat_rule/parameters/sense -->

| Field | Value |
| --- | --- |
| ` radius_feet ` | ` 30 ` |
| ` origin ` | ` caster ` |
| ` duration ` | ` spell_duration ` |
| ` categories ` | ` ["poison", "poisonous_creature", "venomous_creature", "magical_contagion"] ` |
| ` outputs ` | ` ["location", "kind"] ` |

##### Barrier blocks

<!-- splat:table
path: splat_rule/parameters/sense/barrier_blocks
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Materials:
    path: materials
    type: yaml
    optional: true
  Minimum thickness feet:
    path: minimum_thickness_feet
    type: integer
    optional: true
  Material:
    path: material
    type: symbol
    optional: true
  Minimum thickness inches:
    path: minimum_thickness_inches
    type: integer
    optional: true
  Any thin sheet:
    path: any_thin_sheet
    type: boolean
    optional: true
-->

| Key | Materials | Minimum thickness feet | Material | Minimum thickness inches | Any thin sheet |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` ["stone", "dirt", "wood"] ` | ` 1 ` |  |  |  |
| ` row_2 ` |  |  | ` Metal ` | ` 1 ` |  |
| ` row_3 ` |  |  | ` Lead ` |  | true |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```

