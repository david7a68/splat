---
splat_schema: 1
splat_id: 4793aa04-71bf-56c5-9573-07e54ab88961
splat_kind: spell
splat_key: dnd2024:spells/fire_bolt
splat_name: Fire Bolt
splat_origin:
  key: dnd2024:reference/spell/fire_bolt
splat_sources:
- title: SRD 5.2.1
  locator: pp. 132
  url: https://www.dndbeyond.com/srd
---

# Fire Bolt

## Source text

<!-- SRD page 132 -->

Evocation Cantrip (Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Instantaneous\
You hurl a mote of fire at a creature or an object\
within range. Make a ranged spell attack against the\
target. On a hit, the target takes 1d10 Fire damage.\
A flammable object hit by this spell starts burning if\
it isn’t being worn or carried.\
Cantrip Upgrade. The damage increases by 1d10\
when you reach levels 5 (2d10), 11 (3d10), and 17\
(4d10).\

Original layout: [[../../srd-5.2.1/pages/132|page 132]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/fire_bolt ` |
| ` implementation ` | ` dnd2024:spell_fire_bolt ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature_or_object ` |
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
      sides: 10
      add: 0
    type: fire
  ignite_if:
    target_flammable: true
    target_worn: false
    target_carried: false
```
