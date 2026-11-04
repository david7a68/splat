---
splat_schema: 2
splat_id: 2cf078a5-c631-5ea9-b405-61451c723b20
splat_kind: spell
splat_key: dnd2024:spells/floating_disk
splat_name: Floating Disk
splat_origin: dnd2024:reference/spell/floating_disk
splat_sources:
- SRD 5.2.1, pp. 133, https://www.dndbeyond.com/srd
---

# Floating Disk


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Conjuration |
| Classes | Wizard |
| Casting Time | Action or Ritual |
| Range | 30 feet |
| Components | V, S, M (a drop of mercury) |
| Duration | 1 hour |


## Source text

<!-- SRD page 133 -->

Level 1 Conjuration (Wizard)\
Casting Time: Action or Ritual\
Range: 30 feet\
Components: V, S, M (a drop of mercury)\
Duration: 1 hour

This spell creates a circular, horizontal plane of force, 3 feet in diameter and 1 inch thick, that floats 3 feet above the ground in an unoccupied space of your choice that you can see within range. The disk remains for the duration and can hold up to 500 pounds. If more weight is placed on it, the spell ends, and everything on the disk falls to the ground.

The disk is immobile while you are within 20 feet of it. If you move more than 20 feet away from it, the disk follows you so that it remains within 20 feet of you. It can move across uneven terrain, up or down stairs, slopes and the like, but it can’t cross an elevation change of 10 feet or more. For example, the disk can’t move across a 10-foot-deep pit, nor could it leave such a pit if it was created at the bottom.

If you move more than 100 feet from the disk (typically because it can’t move around an obstacle to follow you), the spell ends.

Original layout: [[../../srd-5.2.1/pages/133|page 133]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/floating_disk ` |
| ` implementation ` | ` dnd2024:spell_floating_disk ` |

### Parameters

#### Creation

<!-- splat:yaml path=splat_rule/parameters/creation -->

```yaml
shape: horizontal_circular_plane
material: force
diameter_feet: 3
thickness_inches: 1
height_above_ground_feet: 3
space:
  unoccupied: true
  visible: true
  within_spell_range: true
capacity_pounds: 500
duration: spell_duration
```

#### Movement

<!-- splat:values path=splat_rule/parameters/movement -->

| Field | Value |
| --- | --- |
| ` immobile_if_caster_within_feet ` | ` 20 ` |
| ` follows_if_caster_farther_than_feet ` | ` 20 ` |
| ` follow_distance_maximum_feet ` | ` 20 ` |
| ` allowed_terrain ` | ` ["uneven", "stairs", "slopes"] ` |
| ` cannot_cross_elevation_change_feet_minimum ` | ` 10 ` |

#### End if

<!-- splat:values path=splat_rule/parameters/end_if -->

| Field | Value |
| --- | --- |
| ` load_pounds_greater_than ` | ` 500 ` |
| ` caster_distance_feet_greater_than ` | ` 100 ` |

#### End cleanup

<!-- splat:values path=splat_rule/parameters/end_cleanup -->

| Field | Value |
| --- | --- |
| ` contents ` | ` fall_to_ground ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

