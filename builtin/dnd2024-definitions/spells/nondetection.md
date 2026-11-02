---
splat_schema: 1
splat_id: c6f8f1d2-2402-5e8f-b38a-f42e141b139f
splat_kind: spell
splat_key: dnd2024:spells/nondetection
splat_name: Nondetection
splat_origin:
  key: dnd2024:reference/spell/nondetection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 151
  url: https://www.dndbeyond.com/srd
---

# Nondetection

## Source text

<!-- SRD page 151 -->

Level 3 Abjuration (Bard, Ranger, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a pinch of diamond dust worth 25+ GP, which the spell consumes)\
Duration: 8 hours

For the duration, you hide a target that you touch from Divination spells. The target can be a willing creature, or it can be a place or an object no larger than 10 feet in any dimension. The target can’t be targeted by any Divination spell or perceived through magical scrying sensors.

Original layout: [[../../srd-5.2.1/pages/151|page 151]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["bard", "ranger", "wizard"] ` |
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
| ` description ` | ` a pinch of diamond dust worth 25+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/nondetection ` |
| ` implementation ` | ` dnd2024:spell_nondetection ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` touched ` | ` true ` |

##### Choices

<!-- splat:table
path: splat_rule/parameters/targets/choices
key: Key
order:
- row_1
- row_2
- row_3
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
  Maximum dimension feet:
    path: maximum_dimension_feet
    type: integer
    optional: true
-->

| Key | Kind | Willing | Maximum dimension feet |
| --- | --- | --- | --- |
| ` row_1 ` | ` Creature ` | true |  |
| ` row_2 ` | ` Place ` |  | ` 10 ` |
| ` row_3 ` | ` Object ` |  | ` 10 ` |

#### Protection

<!-- splat:values path=splat_rule/parameters/protection -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |
| ` cannot_be_targeted_by_spell_school ` | ` divination ` |
| ` cannot_be_perceived_through ` | ` magical_scrying_sensors ` |
