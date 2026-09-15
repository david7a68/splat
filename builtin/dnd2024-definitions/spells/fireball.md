---
splat_schema: 1
splat_id: a0ee8b56-876b-5b45-8c3f-57d58f6f1b45
splat_kind: spell
splat_key: dnd2024:spells/fireball
splat_name: Fireball
splat_origin:
  key: dnd2024:reference/spell/fireball
splat_sources:
- title: SRD 5.2.1
  locator: pp. 131
  url: https://www.dndbeyond.com/srd
---

# Fireball

## Source text

<!-- SRD page 131 -->

Level 3 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (a ball of bat guano and sulfur)\
Duration: Instantaneous\
A bright streak flashes from you to a point you\
choose within range and then blossoms with a\
low roar into a fiery explosion. Each creature in a\
20-foot-radius Sphere centered on that point makes\
a Dexterity saving throw, taking 8d6 Fire damage\
on a failed save or half as much damage on a suc-\
cessful one.\
Flammable objects in the area that aren’t being\
worn or carried start burning.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 3.\

Original layout: [[../../srd-5.2.1/pages/131|page 131]].

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
| ` feet ` | ` 150 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a ball of bat guano and sulfur ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/fireball ` |
| ` implementation ` | ` dnd2024:spell_fireball ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures_in_area ` |
| ` origin ` | ` chosen_point_in_range ` |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 20 ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
failure:
  damage:
    dice: 8d6
    type: fire
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Objects

<!-- splat:values path=splat_rule/parameters/objects -->

| Field | Value |
| --- | --- |
| ` within_area ` | ` true ` |
| ` flammable ` | ` true ` |
| ` worn ` | ` false ` |
| ` carried ` | ` false ` |
| ` effect ` | ` burning ` |

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
