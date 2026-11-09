---
splat_schema: 2
splat_id: a3811a12-0bd8-5a41-9bc5-cd58e5d0bd3d
splat_kind: spell
splat_key: dnd2024:spells/darkvision
splat_name: Darkvision
splat_origin: dnd2024:reference/spell/darkvision
splat_sources:
- SRD 5.2.1, pp. 122, https://www.dndbeyond.com/srd
---

# Darkvision


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Transmutation |
| Classes | Druid, Ranger, Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a dried carrot) |
| Duration | 8 hours |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 122 -->

Level 2 Transmutation (Druid, Ranger, Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a dried carrot)\
Duration: 8 hours

For the duration, a willing creature you touch has Darkvision with a range of 150 feet.

Original layout: [[../../srd-5.2.1/pages/122|page 122]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/darkvision ` |
| ` implementation ` | ` dnd2024:spell_darkvision ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 28800 ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Sense

<!-- splat:values path=splat_rule/parameters/sense -->

| Field | Value |
| --- | --- |
| ` kind ` | ` darkvision ` |
| ` range_feet ` | ` 150 ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

