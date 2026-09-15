---
splat_schema: 1
splat_id: a3811a12-0bd8-5a41-9bc5-cd58e5d0bd3d
splat_kind: spell
splat_key: dnd2024:spells/darkvision
splat_name: Darkvision
splat_origin:
  key: dnd2024:reference/spell/darkvision
splat_sources:
- title: SRD 5.2.1
  locator: pp. 122
  url: https://www.dndbeyond.com/srd
---

# Darkvision

## Source text

<!-- SRD page 122 -->

Level 2 Transmutation (Druid, Ranger, Sorcerer,\
Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a dried carrot)\
Duration: 8 hours\
For the duration, a willing creature you touch has\
Darkvision with a range of 150 feet.\

Original layout: [[../../srd-5.2.1/pages/122|page 122]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["druid", "ranger", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 8 hours ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a dried carrot ` |
| ` consumed ` | ` false ` |

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
