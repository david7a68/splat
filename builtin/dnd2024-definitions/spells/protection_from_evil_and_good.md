---
splat_schema: 1
splat_id: e59c5b60-ee4d-5fd7-87bd-063b867cf8f7
splat_kind: spell
splat_key: dnd2024:spells/protection_from_evil_and_good
splat_name: Protection from Evil and Good
splat_origin:
  key: dnd2024:reference/spell/protection_from_evil_and_good
splat_sources:
- title: SRD 5.2.1
  locator: pp. 157
  url: https://www.dndbeyond.com/srd
---

# Protection from Evil and Good

## Source text

<!-- SRD page 157 -->

Level 1 Abjuration (Cleric, Druid, Paladin, Warlock,\
Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a flask of Holy Water worth 25+\
GP, which the spell consumes)\
Duration: Concentration up to 10 minutes\
Until the spell ends, one willing creature you touch\
is protected against creatures that are Aberrations,\
Celestials, Elementals, Fey, Fiends, or Undead.\
The protection grants several benefits. Creatures\
of those types have Disadvantage on attack rolls\
against the target. The target also can’t be pos-\
sessed by or gain the Charmed or Frightened condi-\
tions from them. If the target is already possessed,\
Charmed, or Frightened by such a creature, the tar-\
get has Advantage on any new saving throw against\
the relevant effect.\

Original layout: [[../../srd-5.2.1/pages/157|page 157]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["cleric", "druid", "paladin", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration up to 10 minutes ` |
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
| ` description ` | ` a flask of Holy Water worth 25+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/protection_from_evil_and_good ` |
| ` implementation ` | ` dnd2024:spell_protection_from_evil_and_good ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` willing ` | ` true ` |
| ` touched ` | ` true ` |

#### Protection

<!-- splat:yaml path=splat_rule/parameters/protection -->

```yaml
duration: spell_duration
against_types:
- aberration
- celestial
- elemental
- fey
- fiend
- undead
attacks_against_target: disadvantage
prevent:
- possession
- charmed_condition
- frightened_condition
already_affected:
  advantage_on_new_saves_against:
  - possession
  - charmed
  - frightened
  ends_existing_effect: false
```
