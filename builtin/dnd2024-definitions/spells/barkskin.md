---
splat_schema: 1
splat_id: 0c3a1349-5ee0-5340-a9ab-d57829167f14
splat_kind: spell
splat_key: dnd2024:spells/barkskin
splat_name: Barkskin
splat_origin:
  key: dnd2024:reference/spell/barkskin
splat_sources:
- title: SRD 5.2.1
  locator: pp. 112
  url: https://www.dndbeyond.com/srd
---

# Barkskin

## Source text

<!-- SRD page 112 -->

Level 2 Transmutation (Druid, Ranger)\
Casting Time: Bonus Action\
Range: Touch\
Component: V, S, M (a handful of bark)\
Duration: 1 hour

You touch a willing creature. Until the spell ends, the target’s skin assumes a bark-like appearance, and the target has an Armor Class of 17 if its AC is lower than that.

Original layout: [[../../srd-5.2.1/pages/112|page 112]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["druid", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
| ` casting_time ` | ` Bonus Action ` |
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
| ` description ` | ` a handful of bark ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/barkskin ` |
| ` implementation ` | ` dnd2024:spell_barkskin ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` appearance ` | ` bark_skin ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Armor class

<!-- splat:values path=splat_rule/parameters/armor_class -->

| Field | Value |
| --- | --- |
| ` minimum ` | ` 17 ` |
