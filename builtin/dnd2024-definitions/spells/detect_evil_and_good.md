---
splat_schema: 2
splat_id: 7f2acf2e-b970-5d4d-a706-297e8c2e55a6
splat_kind: spell
splat_key: dnd2024:spells/detect_evil_and_good
splat_name: Detect Evil and Good
splat_origin: dnd2024:reference/spell/detect_evil_and_good
splat_sources:
- SRD 5.2.1, pp. 123, https://www.dndbeyond.com/srd
---

# Detect Evil and Good


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Divination |
| Classes | Cleric, Paladin |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Concentration, up to 10 minutes |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 123 -->

Level 1 Divination (Cleric, Paladin)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 10 minutes

For the duration, you sense the location of any Aberration, Celestial, Elemental, Fey, Fiend, or Undead within 30 feet of yourself. You also sense whether the Hallow spell is active there and, if so, where.

The spell is blocked by 1 foot of stone, dirt, or wood; 1 inch of metal; or a thin sheet of lead.

Original layout: [[../../srd-5.2.1/pages/123|page 123]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/detect_evil_and_good ` |
| ` implementation ` | ` dnd2024:spell_detect_evil_and_good ` |

### Parameters

#### Detection

<!-- splat:values path=splat_rule/parameters/detection -->

| Field | Value |
| --- | --- |
| ` radius_feet ` | ` 30 ` |
| ` origin ` | ` caster ` |
| ` creature_types ` | ` ["aberration", "celestial", "elemental", "fey", "fiend", "undead"] ` |
| ` outputs ` | ` ["locations", "hallow_presence", "hallow_locations"] ` |

#### Barriers

<!-- splat:table
path: splat_rule/parameters/barriers
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
  Thickness feet:
    path: thickness_feet
    type: integer
    optional: true
  Thickness inches:
    path: thickness_inches
    type: integer
    optional: true
  Thickness:
    path: thickness
    type: symbol
    optional: true
-->

| Key | Materials | Thickness feet | Thickness inches | Thickness |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` ["stone", "dirt", "wood"] ` | ` 1 ` |  |  |
| ` row_2 ` | ` ["metal"] ` |  | ` 1 ` |  |
| ` row_3 ` | ` ["lead"] ` |  |  | ` Thin sheet ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```

