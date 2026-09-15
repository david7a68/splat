---
splat_schema: 1
splat_id: 94b1c667-ab0d-515c-9f10-d7e2c9df23b2
splat_kind: spell
splat_key: dnd2024:spells/foresight
splat_name: Foresight
splat_origin:
  key: dnd2024:reference/spell/foresight
splat_sources:
- title: SRD 5.2.1
  locator: pp. 134
  url: https://www.dndbeyond.com/srd
---

# Foresight

## Source text

<!-- SRD page 134 -->

Level 9 Divination (Bard, Druid, Warlock, Wizard)\
Casting Time: 1 minute\
Range: Touch\
Components: V, S, M (a hummingbird feather)\
Duration: 8 hours\
You touch a willing creature and bestow a limited\
ability to see into the immediate future. For the du-\
ration, the target has Advantage on D20 Tests, and\
other creatures have Disadvantage on attack rolls\
against it. The spell ends early if you cast it again.\

Original layout: [[../../srd-5.2.1/pages/134|page 134]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 9 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["bard", "druid", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 8 hours ` |
| ` casting_time ` | ` 1 minute ` |
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
| ` description ` | ` a hummingbird feather ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/foresight ` |
| ` implementation ` | ` dnd2024:spell_foresight ` |

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

#### Modifiers

<!-- splat:table
path: splat_rule/parameters/modifiers
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Selector:
    path: selector
    type: symbol
  Advantage:
    path: advantage
    type: boolean
    optional: true
  Disadvantage:
    path: disadvantage
    type: boolean
    optional: true
-->

| Key | Selector | Advantage | Disadvantage |
| --- | --- | --- | --- |
| ` row_1 ` | ` Target d20 tests ` | true |  |
| ` row_2 ` | ` Other creatures attack rolls against target ` |  | true |

#### End early

<!-- splat:yaml path=splat_rule/parameters/end_early -->

```yaml
event: spell.cast_completed
subject: caster
spell:
  key: dnd2024:spells/foresight
```
