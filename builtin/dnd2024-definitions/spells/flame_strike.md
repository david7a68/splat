---
splat_schema: 1
splat_id: 095ce437-b53b-536e-a720-5fa6a0397580
splat_kind: spell
splat_key: dnd2024:spells/flame_strike
splat_name: Flame Strike
splat_origin:
  key: dnd2024:reference/spell/flame_strike
splat_sources:
- title: SRD 5.2.1
  locator: pp. 132
  url: https://www.dndbeyond.com/srd
---

# Flame Strike

## Source text

<!-- SRD page 132 -->

Level 5 Evocation (Cleric)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a pinch of sulfur)\
Duration: Instantaneous\
A vertical column of brilliant fire roars down from\
above. Each creature in a 10-foot-radius, 40-foot-\
high Cylinder centered on a point within range\
makes a Dexterity saving throw, taking 5d6 Fire\
damage and 5d6 Radiant damage on a failed save or\
half as much damage on a successful one.\
Using a Higher-Level Spell Slot. The Fire damage\
and the Radiant damage increase by 1d6 for each\
spell slot level above 5.\

Original layout: [[../../srd-5.2.1/pages/132|page 132]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["cleric"] ` |
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
| ` description ` | ` a pinch of sulfur ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/flame_strike ` |
| ` implementation ` | ` dnd2024:spell_flame_strike ` |

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
| ` shape ` | ` cylinder ` |
| ` radius_feet ` | ` 10 ` |
| ` height_feet ` | ` 40 ` |
| ` origin ` | ` chosen_point ` |
| ` range_feet ` | ` 60 ` |

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
| ` ability ` | ` dexterity ` |
| ` dc ` | ` spell_save_dc ` |

#### Damage

<!-- splat:rows path=splat_rule/parameters/damage -->

| Row | type |
| --- | --- |
| ` 0 ` | ` fire ` |
| ` 1 ` | ` radiant ` |

##### Dice

<!-- splat:values path=splat_rule/parameters/damage/0/dice -->

| Field | Value |
| --- | --- |
| ` base_count ` | ` 5 ` |
| ` sides ` | ` 6 ` |
| ` extra_per_slot ` | ` 1 ` |
| ` base_slot ` | ` 5 ` |

##### Dice

<!-- splat:values path=splat_rule/parameters/damage/1/dice -->

| Field | Value |
| --- | --- |
| ` base_count ` | ` 5 ` |
| ` sides ` | ` 6 ` |
| ` extra_per_slot ` | ` 1 ` |
| ` base_slot ` | ` 5 ` |

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
