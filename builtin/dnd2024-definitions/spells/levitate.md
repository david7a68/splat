---
splat_schema: 2
splat_id: fa97a2c5-10f1-5117-95cd-2cdbb3d2e4a8
splat_kind: spell
splat_key: dnd2024:spells/levitate
splat_name: Levitate
splat_origin: dnd2024:reference/spell/levitate
splat_sources:
- SRD 5.2.1, pp. 144, https://www.dndbeyond.com/srd
---

# Levitate


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Transmutation |
| Classes | Sorcerer, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S, M (a metal spring) |
| Duration | Concentration, up to 10 minutes |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 144 -->

Level 2 Transmutation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a metal spring)\
Duration: Concentration, up to 10 minutes

One creature or loose object of your choice that you can see within range rises vertically up to 20 feet and remains suspended there for the duration. The spell can levitate an object that weighs up to 500 pounds. An unwilling creature that succeeds on a Constitution saving throw is unaffected.

The target can move only by pushing or pulling against a fixed object or surface within reach (such as a wall or a ceiling), which allows it to move as if it were climbing. You can change the target’s altitude by up to 20 feet in either direction on your turn. If you are the target, you can move up or down as part of your move. Otherwise, you can take a Magic action to move the target, which must remain within the spell’s range.

When the spell ends, the target floats gently to the ground if it is still aloft.

Original layout: [[../../srd-5.2.1/pages/144|page 144]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/levitate ` |
| ` implementation ` | ` dnd2024:spell_levitate ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 600 ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` choices ` | ` ["creature", "loose_object"] ` |
| ` visible ` | ` true ` |
| ` range_feet ` | ` 60 ` |
| ` object_weight_pounds_max ` | ` 500 ` |

#### Resistance

<!-- splat:yaml path=splat_rule/parameters/resistance -->

```yaml
only_if: unwilling_creature
save:
  ability: constitution
  dc: spell_save_dc
success: unaffected
```

#### Initial motion

<!-- splat:values path=splat_rule/parameters/initial_motion -->

| Field | Value |
| --- | --- |
| ` direction ` | ` up ` |
| ` distance_feet_max ` | ` 20 ` |
| ` suspended_after ` | ` true ` |

#### Target movement

<!-- splat:values path=splat_rule/parameters/target_movement -->

| Field | Value |
| --- | --- |
| ` requires ` | ` fixed_object_or_surface_in_reach ` |
| ` method ` | ` push_or_pull ` |
| ` movement_rule ` | ` climbing ` |

#### Caster altitude change

<!-- splat:yaml path=splat_rule/parameters/caster_altitude_change -->

```yaml
timing: caster_turn
distance_feet_max: 20
directions:
- up
- down
self_target:
  economy: part_of_movement
other_target:
  economy: magic_action
  target_must_remain_within_range_feet: 60
```

#### Spell end aloft

<!-- splat:values path=splat_rule/parameters/spell_end_aloft -->

| Field | Value |
| --- | --- |
| ` movement ` | ` float_gently_to_ground ` |

#### Target binding

<!-- splat:values path=splat_rule/parameters/target_binding -->

| Field | Value |
| --- | --- |
| ` exactly_one ` | ` ["target_creature", "target_object"] ` |
| ` must_match_selected_mode ` | ` true ` |

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

