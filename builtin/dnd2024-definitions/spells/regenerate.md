---
splat_schema: 1
splat_id: 4cac4ccd-a035-56ef-accb-deae22998694
splat_kind: spell
splat_key: dnd2024:spells/regenerate
splat_name: Regenerate
splat_origin:
  key: dnd2024:reference/spell/regenerate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158
  url: https://www.dndbeyond.com/srd
---

# Regenerate

## Source text

<!-- SRD page 158 -->

Level 7 Transmutation (Bard, Cleric, Druid)\
Casting Time: 1 minute\
Range: Touch\
Components: V, S, M (a prayer wheel)\
Duration: 1 hour\
A creature you touch regains 4d8 + 15 Hit Points.\
For the duration, the target regains 1 Hit Point at\
the start of each of its turns, and any severed body\
parts regrow after 2 minutes.\

Original layout: [[../../srd-5.2.1/pages/158|page 158]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["bard", "cleric", "druid"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
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
| ` description ` | ` a prayer wheel ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/regenerate ` |
| ` implementation ` | ` dnd2024:spell_regenerate ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` touch ` | ` true ` |

#### Initial healing

<!-- splat:values path=splat_rule/parameters/initial_healing -->

| Field | Value |
| --- | --- |
| ` dice ` | ` 4d8 ` |
| ` add ` | ` 15 ` |

#### Ongoing

<!-- splat:yaml path=splat_rule/parameters/ongoing -->

```yaml
duration: spell_duration
healing:
  event: target.turn_started
  amount: 1
severed_part_regrowth_minutes: 2
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 1
  unit: hour
```
