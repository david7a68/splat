---
splat_schema: 1
splat_id: fa71bdec-f747-5cbb-a32b-e357056938af
splat_kind: spell
splat_key: dnd2024:spells/sacred_flame
splat_name: Sacred Flame
splat_origin:
  key: dnd2024:reference/spell/sacred_flame
splat_sources:
- title: SRD 5.2.1
  locator: pp. 159
  url: https://www.dndbeyond.com/srd
---

# Sacred Flame

## Source text

<!-- SRD page 159 -->

Evocation Cantrip (Cleric)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
Flame-like radiance descends on a creature that you\
can see within range. The target must succeed on\
a Dexterity saving throw or take 1d8 Radiant dam-\
age. The target gains no benefit from Half Cover or\
Three-Quarters Cover for this save.\
Cantrip Upgrade. The damage increases by 1d8\
when you reach levels 5 (2d8), 11 (3d8), and 17\
(4d8).\

Original layout: [[../../srd-5.2.1/pages/159|page 159]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["cleric"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/sacred_flame ` |
| ` implementation ` | ` dnd2024:spell_sacred_flame ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ignore_cover_for_save ` | ` ["half", "three_quarters"] ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` visible ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
dc:
  binding: save_dc
failure:
  damage:
    dice:
      count:
        by_caster_level:
        - minimum: 1
          count: 1
        - minimum: 5
          count: 2
        - minimum: 11
          count: 3
        - minimum: 17
          count: 4
      sides: 8
      add: 0
    type: radiant
success:
  effect: none
```
