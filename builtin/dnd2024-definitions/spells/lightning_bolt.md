---
splat_schema: 1
splat_id: 804e0928-5a2a-52de-965c-777d72385552
splat_kind: spell
splat_key: dnd2024:spells/lightning_bolt
splat_name: Lightning Bolt
splat_origin:
  key: dnd2024:reference/spell/lightning_bolt
splat_sources:
- title: SRD 5.2.1
  locator: pp. 144
  url: https://www.dndbeyond.com/srd
---

# Lightning Bolt

## Source text

<!-- SRD page 144 -->

Level 3 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a bit of fur and a crystal rod)\
Duration: Instantaneous\
A stroke of lightning forming a 100-foot-long,\
5-foot-wide Line blasts out from you in a direction\
you choose. Each creature in the Line makes a Dex-\
terity saving throw, taking 8d6 Lightning damage\
on a failed save or half as much damage on a suc-\
cessful one.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 3.\

Original layout: [[../../srd-5.2.1/pages/144|page 144]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
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
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a bit of fur and a crystal rod ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/lightning_bolt ` |
| ` implementation ` | ` dnd2024:spell_lightning_bolt ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures_in_area ` |
| ` shape ` | ` line ` |
| ` length_feet ` | ` 100 ` |
| ` width_feet ` | ` 5 ` |
| ` origin ` | ` caster ` |
| ` direction ` | ` caster_choice ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
failure:
  damage:
    dice: 8d6
    type: lightning
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 3 ` |
| ` damage_per_level ` | ` 1d6 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
