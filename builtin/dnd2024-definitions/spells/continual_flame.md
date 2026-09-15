---
splat_schema: 1
splat_id: 9c0b220a-a3f4-5c12-b884-dc929fe32d70
splat_kind: spell
splat_key: dnd2024:spells/continual_flame
splat_name: Continual Flame
splat_origin:
  key: dnd2024:reference/spell/continual_flame
splat_sources:
- title: SRD 5.2.1
  locator: pp. 119
  url: https://www.dndbeyond.com/srd
---

# Continual Flame

## Source text

<!-- SRD page 119 -->

Level 2 Evocation (Cleric, Druid, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (ruby dust worth 50+ GP, which\
the spell consumes)\
Duration: Until dispelled\
A flame springs from an object that you touch. The\
effect casts Bright Light in a 20-foot radius and Dim\
Light for an additional 20 feet. It looks like a regular\
flame, but it creates no heat and consumes no fuel.\
The flame can be covered or hidden but not smoth-\
ered or quenched.\

Original layout: [[../../srd-5.2.1/pages/119|page 119]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["cleric", "druid", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Until dispelled ` |
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
| ` description ` | ` ruby dust worth 50+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/continual_flame ` |
| ` implementation ` | ` dnd2024:spell_continual_flame ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` object ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` touch ` | ` true ` |

#### Light

<!-- splat:values path=splat_rule/parameters/light -->

| Field | Value |
| --- | --- |
| ` bright_radius_feet ` | ` 20 ` |
| ` additional_dim_feet ` | ` 20 ` |

#### Flame

<!-- splat:values path=splat_rule/parameters/flame -->

| Field | Value |
| --- | --- |
| ` heat ` | ` false ` |
| ` fuel_consumption ` | ` false ` |
| ` coverable ` | ` true ` |
| ` hideable ` | ` true ` |
| ` smotherable ` | ` false ` |
| ` quenchable ` | ` false ` |
