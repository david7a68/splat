---
splat_schema: 1
splat_id: 6d81fcb1-cee3-5b8e-8192-2a12d5c40bd8
splat_kind: spell
splat_key: dnd2024:spells/circle_of_death
splat_name: Circle of Death
splat_origin:
  key: dnd2024:reference/spell/circle_of_death
splat_sources:
- title: SRD 5.2.1
  locator: pp. 115
  url: https://www.dndbeyond.com/srd
---

# Circle of Death

## Source text

<!-- SRD page 115 -->

Level 6 Necromancy (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (the powder of a crushed black\
pearl worth 500+ GP)\
Duration: Instantaneous\
Negative energy ripples out in a 60-foot-radius\
Sphere from a point you choose within range. Each\
creature in that area makes a Constitution saving\
throw, taking 8d8 Necrotic damage on a failed save\
or half as much damage on a successful one.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 2d8 for each spell slot level above 6.\

Original layout: [[../../srd-5.2.1/pages/115|page 115]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` necromancy ` |
| ` classes ` | ` ["sorcerer", "warlock", "wizard"] ` |
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
| ` description ` | ` the powder of a crushed black pearl worth 500+ GP ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/circle_of_death ` |
| ` implementation ` | ` dnd2024:spell_circle_of_death ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 60 ` |
| ` origin ` | ` chosen_point ` |
| ` range_feet ` | ` 150 ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` selection ` | ` all_in_area ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` spell_save_dc ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
dice:
  base_count: 8
  sides: 8
  extra_per_slot: 2
  base_slot: 6
type: necrotic
```

#### Success damage scale

<!-- splat:values path=splat_rule/parameters/success_damage_scale -->

| Field | Value |
| --- | --- |
| ` numerator ` | ` 1 ` |
| ` denominator ` | ` 2 ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
