---
splat_schema: 1
splat_id: 3a6f1cf5-22be-59da-82ea-0c3bdcb50da5
splat_kind: spell
splat_key: dnd2024:spells/aid
splat_name: Aid
splat_origin:
  key: dnd2024:reference/spell/aid
splat_sources:
- title: SRD 5.2.1
  locator: pp. 107
  url: https://www.dndbeyond.com/srd
---

# Aid

## Source text

<!-- SRD page 107 -->

Level 2 Abjuration (Bard, Cleric, Druid, Paladin, Ranger)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a strip of white cloth)\
Duration: 8 hours

Choose up to three creatures within range. Each target’s Hit Point maximum and current Hit Points increase by 5 for the duration.

Using a Higher-Level Spell Slot. Each target’s Hit Points increase by 5 for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/107|page 107]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["bard", "cleric", "druid", "paladin", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 8 hours ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a strip of white cloth ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/aid ` |
| ` implementation ` | ` dnd2024:spell_aid ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 3 ` |
| ` within_spell_range ` | ` true ` |

#### Hit points

<!-- splat:yaml path=splat_rule/parameters/hit_points -->

```yaml
current_increase:
  op: add
  args:
  - 5
  - op: multiply
    args:
    - 5
    - op: subtract
      args:
      - binding: cast_level
      - 2
maximum_increase:
  op: add
  args:
  - 5
  - op: multiply
    args:
    - 5
    - op: subtract
      args:
      - binding: cast_level
      - 2
duration: spell_duration
```
