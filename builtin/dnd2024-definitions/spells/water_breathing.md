---
splat_schema: 1
splat_id: 7e9bfc16-0d2f-55b1-87a6-d440c7271add
splat_kind: spell
splat_key: dnd2024:spells/water_breathing
splat_name: Water Breathing
splat_origin:
  key: dnd2024:reference/spell/water_breathing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 173
  url: https://www.dndbeyond.com/srd
---

# Water Breathing

## Source text

<!-- SRD page 173 -->

Level 3 Transmutation (Druid, Ranger, Sorcerer, Wizard)\
Casting Time: Action or Ritual\
Range: 30 feet\
Components: V, S, M (a short reed)\
Duration: 24 hours

This spell grants up to ten willing creatures of your choice within range the ability to breathe underwater until the spell ends. Affected creatures also retain their normal mode of respiration.

Original layout: [[../../srd-5.2.1/pages/173|page 173]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["druid", "ranger", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 24 hours ` |
| ` casting_time ` | ` Action or Ritual ` |
| ` ritual ` | ` true ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a short reed ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/water_breathing ` |
| ` implementation ` | ` dnd2024:spell_water_breathing ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 10 ` |
| ` within_spell_range ` | ` true ` |
| ` willing ` | ` true ` |

#### Respiration

<!-- splat:values path=splat_rule/parameters/respiration -->

| Field | Value |
| --- | --- |
| ` underwater ` | ` true ` |
| ` retain_normal_respiration ` | ` true ` |
| ` duration ` | ` spell_duration ` |
