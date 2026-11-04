---
splat_schema: 2
splat_id: 523f2b01-7d7f-5e8a-b654-c6141dd94e91
splat_kind: spell
splat_key: dnd2024:spells/wall_of_force
splat_name: Wall of Force
splat_origin: dnd2024:reference/spell/wall_of_force
splat_sources:
- SRD 5.2.1, pp. 172, https://www.dndbeyond.com/srd
---

# Wall of Force


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Evocation |
| Classes | Wizard |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S, M (a shard of glass) |
| Duration | Concentration, up to 10 minutes |


## Source text

<!-- SRD page 172 -->

Level 5 Evocation (Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a shard of glass)\
Duration: Concentration, up to 10 minutes

An Invisible wall of force springs into existence at a point you choose within range. The wall appears in any orientation you choose, as a horizontal or vertical barrier or at an angle. It can be free floating or resting on a solid surface. You can form it into a hemispherical dome or a globe with a radius of up to 10 feet, or you can shape a flat surface made up of ten 10-foot-by-10-foot panels. Each panel must be contiguous with another panel. In any form, the wall is 1/4 inch thick and lasts for the duration. If the wall cuts through a creature’s space when it appears, the creature is pushed to one side of the wall (you choose which side).

Nothing can physically pass through the wall. It is immune to all damage and can’t be dispelled by Dispel Magic. A Disintegrate spell destroys the wall instantly, however. The wall also extends into the Ethereal Plane and blocks ethereal travel through the wall.

Original layout: [[../../srd-5.2.1/pages/172|page 172]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/wall_of_force ` |
| ` implementation ` | ` dnd2024:spell_wall_of_force ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` chosen_point_in_range ` |

#### Wall

<!-- splat:values path=splat_rule/parameters/wall -->

| Field | Value |
| --- | --- |
| ` invisible ` | ` true ` |
| ` orientation ` | ` caster_choice_any ` |
| ` support_choice ` | ` ["free_floating", "solid_surface"] ` |
| ` duration ` | ` spell_duration ` |

##### Shape choices

<!-- splat:table
path: splat_rule/parameters/wall/shape_choices
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Shape:
    path: shape
    type: symbol
  Radius max feet:
    path: radius_max_feet
    type: integer
    optional: true
  Count:
    path: count
    type: integer
    optional: true
  Width feet:
    path: width_feet
    type: integer
    optional: true
  Height feet:
    path: height_feet
    type: integer
    optional: true
  Each contiguous with another:
    path: each_contiguous_with_another
    type: boolean
    optional: true
-->

| Key | Shape | Radius max feet | Count | Width feet | Height feet | Each contiguous with another |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Hemispherical dome ` | ` 10 ` |  |  |  |  |
| ` row_2 ` | ` Globe ` | ` 10 ` |  |  |  |  |
| ` row_3 ` | ` Flat panels ` |  | ` 10 ` | ` 10 ` | ` 10 ` | true |

##### Thickness inches

<!-- splat:values path=splat_rule/parameters/wall/thickness_inches -->

| Field | Value |
| --- | --- |
| ` numerator ` | ` 1 ` |
| ` denominator ` | ` 4 ` |

#### Initial overlap

<!-- splat:values path=splat_rule/parameters/initial_overlap -->

| Field | Value |
| --- | --- |
| ` targets ` | ` creatures_cut_by_wall ` |
| ` push_to_side ` | ` caster_choice ` |

#### Barrier

<!-- splat:values path=splat_rule/parameters/barrier -->

| Field | Value |
| --- | --- |
| ` physical_passage ` | ` forbidden ` |
| ` damage_immunity ` | ` all ` |
| ` dispel_magic ` | ` ineffective ` |
| ` disintegrate ` | ` instant_destruction ` |
| ` extends_into ` | ` ethereal_plane ` |
| ` ethereal_passage ` | ` forbidden ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 10
  unit: minute
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

