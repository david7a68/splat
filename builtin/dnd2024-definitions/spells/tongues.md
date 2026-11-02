---
splat_schema: 1
splat_id: 1c5762b0-876d-57b7-bb92-c91e8614a4e7
splat_kind: spell
splat_key: dnd2024:spells/tongues
splat_name: Tongues
splat_origin:
  key: dnd2024:reference/spell/tongues
splat_sources:
- title: SRD 5.2.1
  locator: pp. 169, 170
  url: https://www.dndbeyond.com/srd
---

# Tongues

## Source text

<!-- SRD page 169 -->

Level 3 Divination (Bard, Cleric, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, M (a miniature ziggurat)\
Duration: 1 hour

This spell grants the creature you touch the ability to understand any spoken or signed language that it hears or sees. Moreover, when the target communicates by speaking or signing, any creature that knows at least one language can understand it if that creature can hear the speech or see the signing.

Original layout: [[../../srd-5.2.1/pages/169|page 169]], [[../../srd-5.2.1/pages/170|page 170]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["bard", "cleric", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
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
| ` description ` | ` a miniature ziggurat ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/tongues ` |
| ` implementation ` | ` dnd2024:spell_tongues ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` touch ` | ` true ` |

#### Understand

<!-- splat:values path=splat_rule/parameters/understand -->

| Field | Value |
| --- | --- |
| ` languages ` | ` any ` |
| ` spoken_if ` | ` heard ` |
| ` signed_if ` | ` seen ` |

#### Be understood

<!-- splat:values path=splat_rule/parameters/be_understood -->

| Field | Value |
| --- | --- |
| ` listener_knows_minimum_languages ` | ` 1 ` |
| ` speech_if ` | ` heard ` |
| ` signing_if ` | ` seen ` |
