---
splat_schema: 1
splat_id: eba460f4-6bf8-5344-8a11-bff968ccfecc
splat_kind: spell
splat_key: dnd2024:spells/magic_missile
splat_name: Magic Missile
splat_origin:
  key: dnd2024:reference/spell/magic_missile
splat_sources:
- title: SRD 5.2.1
  locator: pp. 146
  url: https://www.dndbeyond.com/srd
---

# Magic Missile

## Source text

<!-- SRD page 146 -->

Level 1 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Instantaneous\
You create three glowing darts of magical force.\
Each dart strikes a creature of your choice that you\
can see within range. A dart deals 1d4 + 1 Force\
damage to its target. The darts all strike simultane-\
ously, and you can direct them to hit one creature or\
several.\
Using a Higher-Level Spell Slot. The spell creates\
one more dart for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/146|page 146]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
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
| ` mechanic ` | ` dnd2024:spells/magic_missile ` |
| ` implementation ` | ` dnd2024:spell_magic_missile ` |

### Parameters

#### Darts

<!-- splat:yaml path=splat_rule/parameters/darts -->

```yaml
count:
  op: add
  args:
  - 3
  - op: multiply
    args:
    - 1
    - op: subtract
      args:
      - binding: cast_level
      - 1
target_per_dart:
  kind: creature
  maximum: 1
  within_spell_range: true
  visible: true
allow_same_target_multiple_darts: true
automatic_strike: true
simultaneous: true
damage_per_dart:
  dice:
    count: 1
    sides: 4
    add: 1
  type: force
```
