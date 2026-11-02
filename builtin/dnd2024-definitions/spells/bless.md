---
splat_schema: 1
splat_id: 20599c22-b59a-5f0f-935c-d7703c8a4d69
splat_kind: spell
splat_key: dnd2024:spells/bless
splat_name: Bless
splat_origin:
  key: dnd2024:reference/spell/bless
splat_sources:
- title: SRD 5.2.1
  locator: pp. 113
  url: https://www.dndbeyond.com/srd
---

# Bless

## Source text

<!-- SRD page 113 -->

Level 1 Enchantment (Cleric, Paladin)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a Holy Symbol worth 5+ GP)\
Duration: Concentration, up to 1 minute

You bless up to three creatures within range. Whenever a target makes an attack roll or a saving throw before the spell ends, the target adds 1d4 to the attack roll or save.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/113|page 113]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["cleric", "paladin"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
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
| ` description ` | ` a Holy Symbol worth 5+ GP ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/bless ` |
| ` implementation ` | ` dnd2024:spell_bless ` |

### Parameters

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
kind: creature
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
      - 1
within_spell_range: true
```

#### Roll modifier

<!-- splat:yaml path=splat_rule/parameters/roll_modifier -->

```yaml
rolls:
- attack
- save
amount:
  dice:
    count: 1
    sides: 4
    add: 0
operation: add
duration: spell_duration
```
