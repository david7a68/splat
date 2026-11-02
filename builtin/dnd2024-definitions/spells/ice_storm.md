---
splat_schema: 1
splat_id: fbc7531b-6863-52d4-a864-6056ce2201b1
splat_kind: spell
splat_key: dnd2024:spells/ice_storm
splat_name: Ice Storm
splat_origin:
  key: dnd2024:reference/spell/ice_storm
splat_sources:
- title: SRD 5.2.1
  locator: pp. 141, 142
  url: https://www.dndbeyond.com/srd
---

# Ice Storm

## Source text

<!-- SRD page 141 -->

Level 4 Evocation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 300 feet\
Components: V, S, M (a mitten)\
Duration: Instantaneous

Hail falls in a 20-foot-radius, 40-foot-high Cylinder centered on a point within range. Each creature in the Cylinder makes a Dexterity saving throw. A creature takes 2d10 Bludgeoning damage and 4d6 Cold damage on a failed save or half as much damage on a successful one.

Hailstones turn ground in the Cylinder into Difficult Terrain until the end of your next turn.

<!-- SRD page 142 -->

Using a Higher-Level Spell Slot. The Bludgeoning damage increases by 1d10 for each spell slot level above 4.

Original layout: [[../../srd-5.2.1/pages/141|page 141]], [[../../srd-5.2.1/pages/142|page 142]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
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
| ` feet ` | ` 300 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a mitten ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/ice_storm ` |
| ` implementation ` | ` dnd2024:spell_ice_storm ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures_in_area ` |
| ` shape ` | ` cylinder ` |
| ` radius_feet ` | ` 20 ` |
| ` height_feet ` | ` 40 ` |
| ` origin ` | ` chosen_point_in_range ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |

##### Failure

###### Damage

<!-- splat:table
path: splat_rule/parameters/save/failure/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Dice:
    path: dice
    type: string
  Type:
    path: type
    type: symbol
-->

| Key | Dice | Type |
| --- | --- | --- |
| ` row_1 ` | ` 2d10 ` | ` Bludgeoning ` |
| ` row_2 ` | ` 4d6 ` | ` Cold ` |

##### Success

<!-- splat:yaml path=splat_rule/parameters/save/success -->

```yaml
damage_fraction:
  numerator: 1
  denominator: 2
```

#### Terrain

<!-- splat:values path=splat_rule/parameters/terrain -->

| Field | Value |
| --- | --- |
| ` area ` | ` ground_in_cylinder ` |
| ` difficult_terrain ` | ` true ` |
| ` until ` | ` caster_next_turn_end ` |

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 4 ` |
| ` bludgeoning_damage_per_level ` | ` 1d10 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
