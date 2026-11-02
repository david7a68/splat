---
splat_schema: 1
splat_id: 481e80c6-2f2e-5a32-b487-836f6b1955e4
splat_kind: spell
splat_key: dnd2024:spells/blur
splat_name: Blur
splat_origin:
  key: dnd2024:reference/spell/blur
splat_sources:
- title: SRD 5.2.1
  locator: pp. 114
  url: https://www.dndbeyond.com/srd
---

# Blur

## Source text

<!-- SRD page 114 -->

Level 2 Illusion (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V\
Duration: Concentration, up to 1 minute

Your body becomes blurred. For the duration, any creature has Disadvantage on attack rolls against you. An attacker is immune to this effect if it perceives you with Blindsight or Truesight.

Original layout: [[../../srd-5.2.1/pages/114|page 114]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
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
| ` mechanic ` | ` dnd2024:spells/blur ` |
| ` implementation ` | ` dnd2024:spell_blur ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |

#### Incoming attacks

<!-- splat:values path=splat_rule/parameters/incoming_attacks -->

| Field | Value |
| --- | --- |
| ` disadvantage ` | ` true ` |
| ` duration ` | ` spell_duration ` |
| ` immune_if_attacker_perceives_caster_using ` | ` ["blindsight", "truesight"] ` |
