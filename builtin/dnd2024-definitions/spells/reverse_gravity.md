---
splat_schema: 2
splat_id: ce55eb66-205d-561f-b66d-52bc60dae093
splat_kind: spell
splat_key: dnd2024:spells/reverse_gravity
splat_name: Reverse Gravity
splat_origin: dnd2024:reference/spell/reverse_gravity
splat_sources:
- SRD 5.2.1, pp. 159, https://www.dndbeyond.com/srd
---

# Reverse Gravity


## Spell

| Field | Value |
| --- | --- |
| Level | 7 |
| School | Transmutation |
| Classes | Druid, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 100 feet |
| Components | V, S, M (a lodestone and iron filings) |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 159 -->

Level 7 Transmutation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 100 feet\
Components: V, S, M (a lodestone and iron filings)\
Duration: Concentration, up to 1 minute

This spell reverses gravity in a 50-foot-radius, 100-foot high Cylinder centered on a point within range. All creatures and objects in that area that aren’t anchored to the ground fall upward and reach the top of the Cylinder. A creature can make a Dexterity saving throw to grab a fixed object it can reach, thus avoiding the fall upward.

If a ceiling or an anchored object is encountered in this upward fall, creatures and objects strike it just as they would during a downward fall. If an affected creature or object reaches the Cylinder’s top without striking anything, it hovers there for the duration. When the spell ends, affected objects and creatures fall downward.

Original layout: [[../../srd-5.2.1/pages/159|page 159]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/reverse_gravity ` |
| ` implementation ` | ` dnd2024:spell_reverse_gravity ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 60 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cylinder ` |
| ` radius_feet ` | ` 50 ` |
| ` height_feet ` | ` 100 ` |
| ` origin ` | ` chosen_point ` |
| ` range_feet ` | ` 100 ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kinds ` | ` ["creature", "object"] ` |
| ` in_area ` | ` true ` |
| ` anchored_to_ground ` | ` false ` |

#### Avoid fall

<!-- splat:yaml path=splat_rule/parameters/avoid_fall -->

```yaml
target_kind: creature
requires: fixed_object_in_reach
save:
  ability: dexterity
  dc: spell_save_dc
success: grab_fixed_object_and_avoid_upward_fall
```

#### Upward fall

<!-- splat:values path=splat_rule/parameters/upward_fall -->

| Field | Value |
| --- | --- |
| ` destination ` | ` cylinder_top ` |
| ` collision_with ` | ` ["ceiling", "anchored_object"] ` |
| ` collision_rule ` | ` normal_downward_fall ` |
| ` no_collision_at_top ` | ` hover_for_duration ` |

#### Spell end

<!-- splat:values path=splat_rule/parameters/spell_end -->

| Field | Value |
| --- | --- |
| ` affected_creatures_and_objects ` | ` fall_downward ` |

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

