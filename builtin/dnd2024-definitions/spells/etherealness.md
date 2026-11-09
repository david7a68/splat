---
splat_schema: 2
splat_id: c13d9fe6-2fcf-51a6-a8a0-c3a3e669942b
splat_kind: spell
splat_key: dnd2024:spells/etherealness
splat_name: Etherealness
splat_origin: dnd2024:reference/spell/etherealness
splat_sources:
- SRD 5.2.1, pp. 128, https://www.dndbeyond.com/srd
---

# Etherealness


## Spell

| Field | Value |
| --- | --- |
| Level | 7 |
| School | Conjuration |
| Classes | Bard, Cleric, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Up to 8 hours |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 128 -->

Level 7 Conjuration (Bard, Cleric, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Up to 8 hours

You step into the border regions of the Ethereal Plane, where it overlaps with your current plane. You remain in the Border Ethereal for the duration. During this time, you can move in any direction. If you move up or down, every foot of movement costs an extra foot. You can perceive the plane you left, which looks gray, and you can’t see anything there more than 60 feet away.

While on the Ethereal Plane, you can affect and be affected only by creatures, objects, and effects on that plane. Creatures that aren’t on the Ethereal Plane can’t perceive or interact with you unless a feature gives them the ability to do so.

When the spell ends, you return to the plane you left in the spot that corresponds to your space in the Border Ethereal. If you appear in an occupied space, you are shunted to the nearest unoccupied space and take Force damage equal to twice the number of feet you are moved.

This spell ends instantly if you cast it while you are on the Ethereal Plane or a plane that doesn’t border it, such as one of the Outer Planes.

Using a Higher-Level Spell Slot. You can target up to three willing creatures (including yourself) for each spell slot level above 7. The creatures must be within 10 feet of you when you cast the spell.

Original layout: [[../../srd-5.2.1/pages/128|page 128]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/etherealness ` |
| ` implementation ` | ` dnd2024:spell_etherealness ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |

#### Plane

<!-- splat:values path=splat_rule/parameters/plane -->

| Field | Value |
| --- | --- |
| ` destination ` | ` border_ethereal ` |
| ` must_border_origin ` | ` true ` |
| ` instant_end_if_origin ` | ` ["ethereal", "nonbordering_plane"] ` |

#### Movement

<!-- splat:values path=splat_rule/parameters/movement -->

| Field | Value |
| --- | --- |
| ` any_direction ` | ` true ` |
| ` vertical_additional_cost_per_foot ` | ` 1 ` |

#### Perception

<!-- splat:values path=splat_rule/parameters/perception -->

| Field | Value |
| --- | --- |
| ` origin_plane ` | ` true ` |
| ` origin_plane_max_feet ` | ` 60 ` |
| ` origin_plane_color ` | ` gray ` |

#### Interaction

<!-- splat:values path=splat_rule/parameters/interaction -->

| Field | Value |
| --- | --- |
| ` can_affect_and_be_affected_by ` | ` ethereal_entities_only ` |
| ` nonethereal_perception_and_interaction ` | ` requires_feature ` |

#### End

<!-- splat:values path=splat_rule/parameters/end -->

| Field | Value |
| --- | --- |
| ` return ` | ` corresponding_origin_plane_space ` |
| ` occupied ` | ` shunt_nearest_unoccupied ` |
| ` force_damage_per_shunted_foot ` | ` 2 ` |

#### Upcast targets

<!-- splat:values path=splat_rule/parameters/upcast_targets -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 7 ` |
| ` maximum_per_level ` | ` 3 ` |
| ` includes_caster ` | ` true ` |
| ` willing ` | ` true ` |
| ` within_caster_feet ` | ` 10 ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```

