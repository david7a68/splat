---
splat_schema: 1
splat_id: d3530c90-dbc4-5def-bda2-896fc7190e55
splat_kind: spell
splat_key: dnd2024:spells/aura_of_life
splat_name: Aura of Life
splat_origin:
  key: dnd2024:reference/spell/aura_of_life
splat_sources:
- title: SRD 5.2.1
  locator: pp. 111
  url: https://www.dndbeyond.com/srd
---

# Aura of Life

## Source text

<!-- SRD page 111 -->

Level 4 Abjuration (Cleric, Paladin)\
Casting Time: Action\
Range: Self\
Components: V\
Duration: Concentration, up to 10 minutes\
An aura radiates from you in a 30-foot Emanation\
for the duration. While in the aura, you and your\
allies have Resistance to Necrotic damage, and your\
Hit Point maximums can’t be reduced. If an ally with\
0 Hit Points starts its turn in the aura, that ally re-\
gains 1 Hit Point.\

Original layout: [[../../srd-5.2.1/pages/111|page 111]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["cleric", "paladin"] ` |
| ` components ` | ` ["verbal"] ` |
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
| ` mechanic ` | ` dnd2024:spells/aura_of_life ` |
| ` implementation ` | ` dnd2024:spell_aura_of_life ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` beneficiaries ` | ` ["caster", "allies"] ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` emanation ` |
| ` radius_feet ` | ` 30 ` |
| ` origin ` | ` caster ` |

#### While inside

<!-- splat:values path=splat_rule/parameters/while_inside -->

| Field | Value |
| --- | --- |
| ` resistance ` | ` ["necrotic"] ` |
| ` prevent_hp_maximum_reduction ` | ` true ` |

#### Healing

<!-- splat:values path=splat_rule/parameters/healing -->

| Field | Value |
| --- | --- |
| ` event ` | ` turn.start ` |
| ` subject ` | ` ally ` |
| ` inside_aura ` | ` true ` |
| ` hp_equals ` | ` 0 ` |
| ` amount ` | ` 1 ` |
