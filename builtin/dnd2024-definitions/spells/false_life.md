---
splat_schema: 1
splat_id: 039ce569-5324-5bfe-8b21-b1ae5f50ca85
splat_kind: spell
splat_key: dnd2024:spells/false_life
splat_name: False Life
splat_origin:
  key: dnd2024:reference/spell/false_life
splat_sources:
- title: SRD 5.2.1
  locator: pp. 129
  url: https://www.dndbeyond.com/srd
---

# False Life

## Source text

<!-- SRD page 129 -->

Level 1 Necromancy (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a drop of alcohol)\
Duration: Instantaneous

You gain 2d4 + 4 Temporary Hit Points.

Using a Higher-Level Spell Slot. You gain 5 additional Temporary Hit Points for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/129|page 129]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` necromancy ` |
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
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a drop of alcohol ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/false_life ` |
| ` implementation ` | ` dnd2024:spell_false_life ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |

#### Temporary hp

<!-- splat:yaml path=splat_rule/parameters/temporary_hp -->

```yaml
dice:
  count: 2
  sides: 4
  add:
    op: add
    args:
    - 4
    - op: multiply
      args:
      - 5
      - op: subtract
        args:
        - binding: cast_level
        - 1
```
