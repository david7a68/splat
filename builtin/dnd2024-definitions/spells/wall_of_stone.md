---
splat_schema: 2
splat_id: 2c639407-c344-5139-94ff-f9af21495524
splat_kind: spell
splat_key: dnd2024:spells/wall_of_stone
splat_name: Wall of Stone
splat_origin: dnd2024:reference/spell/wall_of_stone
splat_sources:
- SRD 5.2.1, pp. 173, https://www.dndbeyond.com/srd
---

# Wall of Stone


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Evocation |
| Classes | Druid, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S, M (a cube of granite) |
| Duration | Concentration, up to 10 minutes |


## Source text

<!-- SRD page 173 -->

Level 5 Evocation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a cube of granite)\
Duration: Concentration, up to 10 minutes

A nonmagical wall of solid stone springs into existence at a point you choose within range. The wall is 6 inches thick and is composed of ten 10-foot-by-10-foot panels. Each panel must be contiguous with another panel. Alternatively, you can create 10-foot-by-20-foot panels that are only 3 inches thick.

If the wall cuts through a creature’s space when it appears, the creature is pushed to one side of the wall (you choose which side). If a creature would be surrounded on all sides by the wall (or the wall and another solid surface), that creature can make a Dexterity saving throw. On a success, it can use its Reaction to move up to its Speed so that it is no longer enclosed by the wall.

The wall can have any shape you desire, though it can’t occupy the same space as a creature or object. The wall doesn’t need to be vertical or rest on a firm foundation. It must, however, merge with and be solidly supported by existing stone. Thus, you can use this spell to bridge a chasm or create a ramp.

If you create a span greater than 20 feet in length, you must halve the size of each panel to create supports. You can crudely shape the wall to create battlements and the like.

The wall is an object made of stone that can be damaged and thus breached. Each panel has AC 15 and 30 Hit Points per inch of thickness, and it has Immunity to Poison and Psychic damage. Reducing a panel to 0 Hit Points destroys it and might cause connected panels to collapse at the GM’s discretion.

If you maintain your Concentration on this spell for its full duration, the wall becomes permanent and can’t be dispelled. Otherwise, the wall disappears when the spell ends.

Original layout: [[../../srd-5.2.1/pages/173|page 173]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/wall_of_stone ` |
| ` implementation ` | ` dnd2024:spell_wall_of_stone ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 600 ` |

#### Wall

<!-- splat:values path=splat_rule/parameters/wall -->

| Field | Value |
| --- | --- |
| ` material ` | ` stone ` |
| ` magical ` | ` false ` |
| ` origin ` | ` chosen_point ` |
| ` range_feet ` | ` 120 ` |
| ` panel_count ` | ` 10 ` |
| ` every_panel_contiguous_with_another ` | ` true ` |

##### Panel choice

<!-- splat:table
path: splat_rule/parameters/wall/panel_choice
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Width feet:
    path: width_feet
    type: integer
  Height feet:
    path: height_feet
    type: integer
  Thickness inches:
    path: thickness_inches
    type: integer
-->

| Key | Width feet | Height feet | Thickness inches |
| --- | --- | --- | --- |
| ` row_1 ` | ` 10 ` | ` 10 ` | ` 6 ` |
| ` row_2 ` | ` 10 ` | ` 20 ` | ` 3 ` |

#### Appearance collision

<!-- splat:yaml path=splat_rule/parameters/appearance_collision -->

```yaml
creature_cut_by_wall:
  push_to_side: caster_choice
enclosed_creature:
  save:
    ability: dexterity
    dc: spell_save_dc
  success:
    optional: true
    economy: reaction
    movement_max: target_speed
    destination: no_longer_enclosed
```

#### Geometry

<!-- splat:yaml path=splat_rule/parameters/geometry -->

```yaml
shape: caster_choice
overlap_creature_or_object: false
vertical_required: false
firm_foundation_required: false
must_merge_with_existing_stone: true
solid_existing_stone_support_required: true
span_greater_than_feet:
  threshold: 20
  panel_size_scale:
    numerator: 1
    denominator: 2
  purpose: supports
crude_features_allowed: true
```

#### Panels

<!-- splat:yaml path=splat_rule/parameters/panels -->

```yaml
armor_class: 15
hit_points_per_thickness_inch: 30
damage_immunity:
- poison
- psychic
at_zero_hit_points: destroyed
connected_panel_collapse:
  chooser: gm
  selection: connected_panels
```

#### Duration

<!-- splat:yaml path=splat_rule/parameters/duration -->

```yaml
full_concentration_seconds: 600
full_concentration_result:
  permanent: true
  dispellable: false
earlier_spell_end: wall_disappears
```

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

