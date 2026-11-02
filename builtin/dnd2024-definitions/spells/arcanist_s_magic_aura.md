---
splat_schema: 1
splat_id: 82c626da-f5c6-5dc8-939d-560e4030f7e3
splat_kind: spell
splat_key: dnd2024:spells/arcanist_s_magic_aura
splat_name: Arcanist’s Magic Aura
splat_origin:
  key: dnd2024:reference/spell/arcanist_s_magic_aura
splat_sources:
- title: SRD 5.2.1
  locator: pp. 110, 111
  url: https://www.dndbeyond.com/srd
---

# Arcanist’s Magic Aura

## Source text

<!-- SRD page 110 -->

Level 2 Illusion (Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a small square of silk)\
Duration: 24 hours

<!-- SRD page 111 -->

With a touch, you place an illusion on a willing creature or an object that isn’t being worn or carried. A creature gains the Mask effect below, and an object gains the False Aura effect below. The effect lasts for the duration. If you cast the spell on the same target every day for 30 days, the illusion lasts until dispelled.

Mask (Creature). Choose a creature type other than the target’s actual type. Spells and other magical effects treat the target as if it were a creature of the chosen type.

False Aura (Object). You change the way the target appears to spells and magical effects that detect magical auras, such as Detect Magic. You can make a nonmagical object appear magical, make a magic item appear nonmagical, or change the object’s aura so that it appears to belong to a school of magic you choose.

Original layout: [[../../srd-5.2.1/pages/110|page 110]], [[../../srd-5.2.1/pages/111|page 111]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 24 hours ` |
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
| ` description ` | ` a small square of silk ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/arcanist_s_magic_aura ` |
| ` implementation ` | ` dnd2024:spell_arcanist_s_magic_aura ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` touch ` | ` true ` |

##### Options

<!-- splat:table
path: splat_rule/parameters/targeting/options
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Kind:
    path: kind
    type: symbol
  Willing:
    path: willing
    type: boolean
    optional: true
  Mode:
    path: mode
    type: symbol
  Worn:
    path: worn
    type: boolean
    optional: true
  Carried:
    path: carried
    type: boolean
    optional: true
-->

| Key | Kind | Willing | Mode | Worn | Carried |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Creature ` | true | ` Mask ` |  |  |
| ` row_2 ` | ` Object ` |  | ` False aura ` | false | false |

#### Mask

<!-- splat:values path=splat_rule/parameters/mask -->

| Field | Value |
| --- | --- |
| ` choice ` | ` creature_type ` |
| ` excluded ` | ` target_actual_type ` |
| ` affected_systems ` | ` ["spells", "magical_effects"] ` |
| ` treated_type ` | ` chosen_type ` |

#### False aura

<!-- splat:values path=splat_rule/parameters/false_aura -->

| Field | Value |
| --- | --- |
| ` affected_systems ` | ` ["magic_aura_detection_spells", "magic_aura_detection_effects"] ` |
| ` choices ` | ` ["nonmagical_appears_magical", "magical_appears_nonmagical", "chosen_school_aura"] ` |

#### Persistence

<!-- splat:values path=splat_rule/parameters/persistence -->

| Field | Value |
| --- | --- |
| ` same_target ` | ` true ` |
| ` frequency ` | ` daily ` |
| ` consecutive_days ` | ` 30 ` |
| ` resulting_duration ` | ` until_dispelled ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 24
  unit: hour
```
