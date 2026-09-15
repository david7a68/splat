---
splat_schema: 1
splat_id: 54d4c256-6dfc-5616-8d82-80502a91ce39
splat_kind: spell
splat_key: dnd2024:spells/beacon_of_hope
splat_name: Beacon of Hope
splat_origin:
  key: dnd2024:reference/spell/beacon_of_hope
splat_sources:
- title: SRD 5.2.1
  locator: pp. 112
  url: https://www.dndbeyond.com/srd
---

# Beacon of Hope

## Source text

<!-- SRD page 112 -->

Level 3 Abjuration (Cleric)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
Choose any number of creatures within range. For\
the duration, each target has Advantage on Wisdom\
saving throws and Death Saving Throws and re-\
gains the maximum number of Hit Points possible\
from any healing.\

Original layout: [[../../srd-5.2.1/pages/112|page 112]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["cleric"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/beacon_of_hope ` |
| ` implementation ` | ` dnd2024:spell_beacon_of_hope ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |
| ` advantage_on ` | ` ["wisdom_saving_throws", "death_saving_throws"] ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures ` |
| ` count ` | ` any ` |
| ` within_range ` | ` true ` |

#### Healing

<!-- splat:values path=splat_rule/parameters/healing -->

| Field | Value |
| --- | --- |
| ` amount ` | ` maximum_possible ` |
| ` sources ` | ` all ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: minute
```
