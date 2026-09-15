---
splat_schema: 1
splat_id: d53fd494-856f-5144-b5a3-f8dc0a4a977c
splat_kind: spell
splat_key: dnd2024:spells/detect_poison_and_disease
splat_name: Detect Poison and Disease
splat_origin:
  key: dnd2024:reference/spell/detect_poison_and_disease
splat_sources:
- title: SRD 5.2.1
  locator: pp. 123
  url: https://www.dndbeyond.com/srd
---

# Detect Poison and Disease

## Source text

<!-- SRD page 123 -->

Level 1 Divination (Cleric, Druid, Paladin, Ranger)\
Casting Time: Action or Ritual\
Range: Self\
Components: V, S, M (a yew leaf)\
Duration: Concentration, up to 10 minutes\
For the duration, you sense the location of poisons,\
poisonous or venomous creatures, and magical\
contagions within 30 feet of yourself. You sense the\
kind of poison, creature, or contagion in each case.\
The spell is blocked by 1 foot of stone, dirt, or\
wood; 1 inch of metal; or a thin sheet of lead.\

Original layout: [[../../srd-5.2.1/pages/123|page 123]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["cleric", "druid", "paladin", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
| ` casting_time ` | ` Action or Ritual ` |
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
| ` description ` | ` a yew leaf ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/detect_poison_and_disease ` |
| ` implementation ` | ` dnd2024:spell_detect_poison_and_disease ` |

### Parameters

#### Sense

<!-- splat:yaml path=splat_rule/parameters/sense -->

```yaml
radius_feet: 30
origin: caster
duration: spell_duration
categories:
- poison
- poisonous_creature
- venomous_creature
- magical_contagion
outputs:
- location
- kind
barrier_blocks:
- materials:
  - stone
  - dirt
  - wood
  minimum_thickness_feet: 1
- material: metal
  minimum_thickness_inches: 1
- material: lead
  any_thin_sheet: true
```
