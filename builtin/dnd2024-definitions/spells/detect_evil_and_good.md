---
splat_schema: 1
splat_id: 7f2acf2e-b970-5d4d-a706-297e8c2e55a6
splat_kind: spell
splat_key: dnd2024:spells/detect_evil_and_good
splat_name: Detect Evil and Good
splat_origin:
  key: dnd2024:reference/spell/detect_evil_and_good
splat_sources:
- title: SRD 5.2.1
  locator: pp. 123
  url: https://www.dndbeyond.com/srd
---

# Detect Evil and Good

## Source text

<!-- SRD page 123 -->

Level 1 Divination (Cleric, Paladin)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 10 minutes\
For the duration, you sense the location of any Ab-\
erration, Celestial, Elemental, Fey, Fiend, or Undead\
within 30 feet of yourself. You also sense whether\
the Hallow spell is active there and, if so, where.\
The spell is blocked by 1 foot of stone, dirt, or\
wood; 1 inch of metal; or a thin sheet of lead.\

Original layout: [[../../srd-5.2.1/pages/123|page 123]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["cleric", "paladin"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

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

<!-- splat:rows path=splat_rule/parameters/barriers -->

| Row | materials | thickness_feet | thickness_inches | thickness |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` ["stone", "dirt", "wood"] ` | ` 1 ` |  |  |
| ` 1 ` | ` ["metal"] ` |  | ` 1 ` |  |
| ` 2 ` | ` ["lead"] ` |  |  | ` thin_sheet ` |
