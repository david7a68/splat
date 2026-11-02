---
splat_schema: 1
splat_id: 0beda0ed-f8cd-515d-9e57-9416c350b41b
splat_kind: spell
splat_key: dnd2024:spells/inflict_wounds
splat_name: Inflict Wounds
splat_origin:
  key: dnd2024:reference/spell/inflict_wounds
splat_sources:
- title: SRD 5.2.1
  locator: pp. 143
  url: https://www.dndbeyond.com/srd
---

# Inflict Wounds

## Source text

<!-- SRD page 143 -->

Level 1 Necromancy (Cleric)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous

A creature you touch makes a Constitution saving throw, taking 2d10 Necrotic damage on a failed save or half as much damage on a successful one.

Using a Higher-Level Spell Slot. The damage increases by 1d10 for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/143|page 143]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` necromancy ` |
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
| ` mode ` | ` touch ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/inflict_wounds ` |
| ` implementation ` | ` dnd2024:spell_inflict_wounds ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` touched ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc:
  binding: save_dc
failure:
  damage:
    dice:
      count:
        op: add
        args:
        - 2
        - op: multiply
          args:
          - 1
          - op: subtract
            args:
            - binding: cast_level
            - 1
      sides: 10
      add: 0
    type: necrotic
success:
  damage:
    dice:
      count:
        op: add
        args:
        - 2
        - op: multiply
          args:
          - 1
          - op: subtract
            args:
            - binding: cast_level
            - 1
      sides: 10
      add: 0
    type: necrotic
  damage_fraction:
    numerator: 1
    denominator: 2
```
