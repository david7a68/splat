---
splat_schema: 1
splat_id: 36c9d75d-8e19-5ba5-81e0-57c2ffc09e87
splat_kind: spell
splat_key: dnd2024:spells/goodberry
splat_name: Goodberry
splat_origin:
  key: dnd2024:reference/spell/goodberry
splat_sources:
- title: SRD 5.2.1
  locator: pp. 137
  url: https://www.dndbeyond.com/srd
---

# Goodberry

## Source text

<!-- SRD page 137 -->

Level 1 Conjuration (Druid, Ranger)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a sprig of mistletoe)\
Duration: 24 hours\
Ten berries appear in your hand and are infused\
with magic for the duration. A creature can take a\
Bonus Action to eat one berry. Eating a berry re-\
stores 1 Hit Point, and the berry provides enough\
nourishment to sustain a creature for one day.\
Uneaten berries disappear when the spell ends.\

Original layout: [[../../srd-5.2.1/pages/137|page 137]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["druid", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 24 hours ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a sprig of mistletoe ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/goodberry ` |
| ` implementation ` | ` dnd2024:spell_goodberry ` |

### Parameters

#### Creation

<!-- splat:values path=splat_rule/parameters/creation -->

| Field | Value |
| --- | --- |
| ` kind ` | ` berry ` |
| ` count ` | ` 10 ` |
| ` location ` | ` caster_hand ` |
| ` duration ` | ` spell_duration ` |
| ` uneaten_disappear_on_end ` | ` true ` |

#### Eat

<!-- splat:values path=splat_rule/parameters/eat -->

| Field | Value |
| --- | --- |
| ` cost ` | ` bonus_action ` |
| ` berries_consumed ` | ` 1 ` |
| ` healing_hp ` | ` 1 ` |
| ` nourishment_creature_days ` | ` 1 ` |
