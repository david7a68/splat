---
splat_schema: 1
splat_id: fa3e0b6e-00cb-5252-aa37-be08cf4e2434
splat_kind: spell
splat_key: dnd2024:spells/longstrider
splat_name: Longstrider
splat_origin:
  key: dnd2024:reference/spell/longstrider
splat_sources:
- title: SRD 5.2.1
  locator: pp. 145
  url: https://www.dndbeyond.com/srd
---

# Longstrider

## Source text

<!-- SRD page 145 -->

Level 1 Transmutation (Bard, Druid, Ranger, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a pinch of dirt)\
Duration: 1 hour\
You touch a creature. The target’s Speed increases\
by 10 feet until the spell ends.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/145|page 145]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["bard", "druid", "ranger", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a pinch of dirt ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/longstrider ` |
| ` implementation ` | ` dnd2024:spell_longstrider ` |

### Parameters

#### Targeting

<!-- splat:yaml path=splat_rule/parameters/targeting -->

```yaml
kind: creatures
touch: true
count:
  base: 1
  per_slot_above: 1
  slot_base: 1
```

#### Effect

<!-- splat:values path=splat_rule/parameters/effect -->

| Field | Value |
| --- | --- |
| ` speed_bonus_feet ` | ` 10 ` |
| ` duration ` | ` spell_duration ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 1
  unit: hour
```
