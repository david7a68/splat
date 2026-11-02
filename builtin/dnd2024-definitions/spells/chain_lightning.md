---
splat_schema: 1
splat_id: f2b3e04b-6619-5597-a715-8a7219204e35
splat_kind: spell
splat_key: dnd2024:spells/chain_lightning
splat_name: Chain Lightning
splat_origin:
  key: dnd2024:reference/spell/chain_lightning
splat_sources:
- title: SRD 5.2.1
  locator: pp. 114
  url: https://www.dndbeyond.com/srd
---

# Chain Lightning

## Source text

<!-- SRD page 114 -->

Level 6 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (three silver pins)\
Duration: Instantaneous

You launch a lightning bolt toward a target you can see within range. Three bolts then leap from that target to as many as three other targets of your choice, each of which must be within 30 feet of the first target. A target can be a creature or an object and can be targeted by only one of the bolts.

Each target makes a Dexterity saving throw, taking 10d8 Lightning damage on a failed save or half as much damage on a successful one.

Using a Higher-Level Spell Slot. One additional bolt leaps from the first target to another target for each spell slot level above 6.

Original layout: [[../../srd-5.2.1/pages/114|page 114]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 150 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` three silver pins ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/chain_lightning ` |
| ` implementation ` | ` dnd2024:spell_chain_lightning ` |

### Parameters

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
primary:
  kind: creature_or_object
  visible: true
  within_spell_range: true
secondary:
  kind: creature_or_object
  maximum:
    op: add
    args:
    - 3
    - op: multiply
      args:
      - 1
      - op: subtract
        args:
        - binding: cast_level
        - 6
  distance_from_primary_maximum_feet: 30
  chooser: caster
maximum_bolts_per_target: 1
```

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
dc:
  binding: save_dc
failure:
  damage:
    dice:
      count: 10
      sides: 8
      add: 0
    type: lightning
success:
  damage:
    dice:
      count: 10
      sides: 8
      add: 0
    type: lightning
  damage_fraction:
    numerator: 1
    denominator: 2
```
