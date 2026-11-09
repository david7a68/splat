---
splat_schema: 2
splat_id: 095ce437-b53b-536e-a720-5fa6a0397580
splat_kind: spell
splat_key: dnd2024:spells/flame_strike
splat_name: Flame Strike
splat_origin: dnd2024:reference/spell/flame_strike
splat_sources:
- SRD 5.2.1, pp. 132, https://www.dndbeyond.com/srd
---

# Flame Strike


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Evocation |
| Classes | Cleric |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S, M (a pinch of sulfur) |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 132 -->

Level 5 Evocation (Cleric)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a pinch of sulfur)\
Duration: Instantaneous

A vertical column of brilliant fire roars down from above. Each creature in a 10-foot-radius, 40-foot-high Cylinder centered on a point within range makes a Dexterity saving throw, taking 5d6 Fire damage and 5d6 Radiant damage on a failed save or half as much damage on a successful one.

Using a Higher-Level Spell Slot. The Fire damage and the Radiant damage increase by 1d6 for each spell slot level above 5.

Original layout: [[../../srd-5.2.1/pages/132|page 132]].







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

<!-- splat:table
path: splat_rule/parameters/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Dice base count:
    path: dice/base_count
    type: integer
  Dice sides:
    path: dice/sides
    type: integer
  Dice extra per slot:
    path: dice/extra_per_slot
    type: integer
  Dice base slot:
    path: dice/base_slot
    type: integer
  Type:
    path: type
    type: symbol
-->

| Key | Dice base count | Dice sides | Dice extra per slot | Dice base slot | Type |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 5 ` | ` 6 ` | ` 1 ` | ` 5 ` | ` Fire ` |
| ` row_2 ` | ` 5 ` | ` 6 ` | ` 1 ` | ` 5 ` | ` Radiant ` |

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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

