---
splat_schema: 1
splat_id: 97c986c3-b654-5430-ab0f-97c7ecf40a31
splat_kind: spell
splat_key: dnd2024:spells/true_seeing
splat_name: True Seeing
splat_origin:
  key: dnd2024:reference/spell/true_seeing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 171
  url: https://www.dndbeyond.com/srd
---

# True Seeing

## Source text

<!-- SRD page 171 -->

Level 6 Divination (Bard, Cleric, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (mushroom powder worth 25+\
GP, which the spell consumes)\
Duration: 1 hour\
For the duration, the willing creature you touch has\
Truesight with a range of 120 feet.\

Original layout: [[../../srd-5.2.1/pages/171|page 171]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["bard", "cleric", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
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
| ` description ` | ` mushroom powder worth 25+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/true_seeing ` |
| ` implementation ` | ` dnd2024:spell_true_seeing ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Sense

<!-- splat:values path=splat_rule/parameters/sense -->

| Field | Value |
| --- | --- |
| ` kind ` | ` truesight ` |
| ` range_feet ` | ` 120 ` |
| ` duration ` | ` spell_duration ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 1
  unit: hour
```
