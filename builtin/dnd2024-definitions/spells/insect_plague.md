---
splat_schema: 2
splat_id: b10f57ec-1516-5549-8079-74b97a695911
splat_kind: spell
splat_key: dnd2024:spells/insect_plague
splat_name: Insect Plague
splat_origin: dnd2024:reference/spell/insect_plague
splat_sources:
- SRD 5.2.1, pp. 143, https://www.dndbeyond.com/srd
---

# Insect Plague


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Conjuration |
| Classes | Cleric, Druid, Sorcerer |
| Casting Time | Action |
| Range | 300 feet |
| Components | V, S, M (a locust) |
| Duration | Concentration, up to 10 minutes |


## Source text

<!-- SRD page 143 -->

Level 5 Conjuration (Cleric, Druid, Sorcerer)\
Casting Time: Action\
Range: 300 feet\
Components: V, S, M (a locust)\
Duration: Concentration, up to 10 minutes

Swarming locusts fill a 20-foot-radius Sphere centered on a point you choose within range. The Sphere remains for the duration, and its area is Lightly Obscured and Difficult Terrain.

When the swarm appears, each creature in it makes a Constitution saving throw, taking 4d10 Piercing damage on a failed save or half as much damage on a successful one. A creature also makes this save when it enters the spell’s area for the first time on a turn or ends its turn there. A creature makes this save only once per turn.

Using a Higher-Level Spell Slot. The damage increases by 1d10 for each spell slot level above 5.

Original layout: [[../../srd-5.2.1/pages/143|page 143]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/insect_plague ` |
| ` implementation ` | ` dnd2024:spell_insect_plague ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 600 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 20 ` |
| ` origin ` | ` chosen_point ` |
| ` range_feet ` | ` 300 ` |
| ` stationary ` | ` true ` |

#### Properties

<!-- splat:values path=splat_rule/parameters/properties -->

| Field | Value |
| --- | --- |
| ` obscurement ` | ` light ` |
| ` difficult_terrain ` | ` true ` |

#### Triggers

<!-- splat:table
path: splat_rule/parameters/triggers
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Event:
    path: event
    type: string
  Subjects:
    path: subjects
    type: symbol
    optional: true
  Occurrence per turn:
    path: occurrence_per_turn
    type: symbol
    optional: true
-->

| Key | Event | Subjects | Occurrence per turn |
| --- | --- | --- | --- |
| ` row_1 ` | ` spell.cast_completed ` | ` Creatures in area ` |  |
| ` row_2 ` | ` creature.entered_area ` |  | ` First ` |
| ` row_3 ` | ` turn.end ` | ` Creatures in area ` |  |

#### Save limit

<!-- splat:values path=splat_rule/parameters/save_limit -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` per ` | ` turn ` |
| ` scope ` | ` target ` |

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
  base_count: 4
  sides: 10
  extra_per_slot: 1
  base_slot: 5
type: piercing
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

