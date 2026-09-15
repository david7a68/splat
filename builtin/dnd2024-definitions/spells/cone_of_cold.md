---
splat_schema: 1
splat_id: 9744acff-d657-5010-9742-e6daa4d540d9
splat_kind: spell
splat_key: dnd2024:spells/cone_of_cold
splat_name: Cone of Cold
splat_origin:
  key: dnd2024:reference/spell/cone_of_cold
splat_sources:
- title: SRD 5.2.1
  locator: pp. 117
  url: https://www.dndbeyond.com/srd
---

# Cone of Cold

## Source text

<!-- SRD page 117 -->

Level 5 Evocation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a small crystal or glass cone)\
Duration: Instantaneous\
You unleash a blast of cold air. Each creature in\
a 60-foot Cone originating from you makes a\
Constitution saving throw, taking 8d8 Cold damage\
on a failed save or half as much damage on a suc-\
cessful one. A creature killed by this spell becomes\
a frozen statue until it thaws.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 5.\

Original layout: [[../../srd-5.2.1/pages/117|page 117]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["druid", "sorcerer", "wizard"] ` |
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
| ` description ` | ` a small crystal or glass cone ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/cone_of_cold ` |
| ` implementation ` | ` dnd2024:spell_cone_of_cold ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures_in_area ` |
| ` shape ` | ` cone ` |
| ` length_feet ` | ` 60 ` |
| ` origin ` | ` caster ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
failure:
  damage:
    dice: 8d8
    type: cold
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Killed target

<!-- splat:values path=splat_rule/parameters/killed_target -->

| Field | Value |
| --- | --- |
| ` form ` | ` frozen_statue ` |
| ` until ` | ` thawed ` |

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 5 ` |
| ` damage_per_level ` | ` 1d8 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
