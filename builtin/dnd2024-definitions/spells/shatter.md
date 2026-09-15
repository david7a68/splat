---
splat_schema: 1
splat_id: 1299b231-a156-55d5-9887-9b1b51002cf4
splat_kind: spell
splat_key: dnd2024:spells/shatter
splat_name: Shatter
splat_origin:
  key: dnd2024:reference/spell/shatter
splat_sources:
- title: SRD 5.2.1
  locator: pp. 161
  url: https://www.dndbeyond.com/srd
---

# Shatter

## Source text

<!-- SRD page 161 -->

Level 2 Evocation (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a chip of mica)\
Duration: Instantaneous\
A loud noise erupts from a point of your choice\
within range. Each creature in a 10-foot-radius\
Sphere centered there makes a Constitution saving\
throw, taking 3d8 Thunder damage on a failed save\
or half as much damage on a successful one. A Con-\
struct has Disadvantage on the save.\
A nonmagical object that isn’t being worn or car-\
ried also takes the damage if it’s in the spell’s area.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/161|page 161]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["bard", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a chip of mica ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/shatter ` |
| ` implementation ` | ` dnd2024:spell_shatter ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` sound ` | ` loud_noise ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures_in_area ` |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 10 ` |
| ` origin ` | ` chosen_point_in_range ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
disadvantage_types:
- construct
failure:
  damage:
    dice: 3d8
    type: thunder
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Object damage

<!-- splat:yaml path=splat_rule/parameters/object_damage -->

```yaml
within_area: true
magical: false
worn: false
carried: false
damage:
  dice: 3d8
  type: thunder
save: none
```

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 2 ` |
| ` damage_per_level ` | ` 1d8 ` |
| ` applies_to ` | ` ["creatures", "objects"] ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
