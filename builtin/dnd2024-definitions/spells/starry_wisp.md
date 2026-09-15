---
splat_schema: 1
splat_id: d533b3c2-46d0-5370-b015-b543d0fa3e9e
splat_kind: spell
splat_key: dnd2024:spells/starry_wisp
splat_name: Starry Wisp
splat_origin:
  key: dnd2024:reference/spell/starry_wisp
splat_sources:
- title: SRD 5.2.1
  locator: pp. 165
  url: https://www.dndbeyond.com/srd
---

# Starry Wisp

## Source text

<!-- SRD page 165 -->

Evocation Cantrip (Bard, Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
You launch a mote of light at one creature or object\
within range. Make a ranged spell attack against the\
target. On a hit, the target takes 1d8 Radiant dam-\
age, and until the end of your next turn, it emits Dim\
Light in a 10-foot radius and can’t benefit from the\
Invisible condition.\
Cantrip Upgrade. The damage increases by 1d8\
when you reach levels 5 (2d8), 11 (3d8), and 17\
(4d8).\

Original layout: [[../../srd-5.2.1/pages/165|page 165]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["bard", "druid"] ` |
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
| ` mechanic ` | ` dnd2024:spells/starry_wisp ` |
| ` implementation ` | ` dnd2024:spell_starry_wisp ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature_or_object ` |
| ` count ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
kind: ranged_spell
hit:
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
  light:
    level: dim
    radius_feet: 10
  cannot_benefit_from_condition: invisible
  until: caster_next_turn_end
```
