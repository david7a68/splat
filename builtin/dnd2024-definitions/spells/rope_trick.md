---
splat_schema: 2
splat_id: 9f77dd14-43cf-572f-a834-44b88653818b
splat_kind: spell
splat_key: dnd2024:spells/rope_trick
splat_name: Rope Trick
splat_origin: dnd2024:reference/spell/rope_trick
splat_sources:
- SRD 5.2.1, pp. 159, https://www.dndbeyond.com/srd
---

# Rope Trick


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Transmutation |
| Classes | Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a segment of rope) |
| Duration | 1 hour |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 159 -->

Level 2 Transmutation (Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a segment of rope)\
Duration: 1 hour

You touch a rope. One end of it hovers upward until the rope hangs perpendicular to the ground or the rope reaches a ceiling. At the rope’s upper end, an Invisible 3-foot-by-5-foot portal opens to an extradimensional space that lasts until the spell ends. That space can be reached by climbing the rope, which can be pulled into or dropped out of it.

The space can hold up to eight Medium or smaller creatures. Attacks, spells, and other effects can’t pass into or out of the space, but creatures inside it can see through the portal. Anything inside the space drops out when the spell ends.

Original layout: [[../../srd-5.2.1/pages/159|page 159]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/rope_trick ` |
| ` implementation ` | ` dnd2024:spell_rope_trick ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` rope ` |
| ` touch ` | ` true ` |

#### Rope

<!-- splat:values path=splat_rule/parameters/rope -->

| Field | Value |
| --- | --- |
| ` upper_end_hovers_until ` | ` perpendicular_or_ceiling_reached ` |
| ` can_pull_inside ` | ` true ` |
| ` can_drop_outside ` | ` true ` |

#### Portal

<!-- splat:values path=splat_rule/parameters/portal -->

| Field | Value |
| --- | --- |
| ` location ` | ` rope_upper_end ` |
| ` invisible ` | ` true ` |
| ` width_feet ` | ` 3 ` |
| ` height_feet ` | ` 5 ` |
| ` destination ` | ` extradimensional_space ` |
| ` access ` | ` climb_rope ` |
| ` duration ` | ` spell_duration ` |

#### Space

<!-- splat:values path=splat_rule/parameters/space -->

| Field | Value |
| --- | --- |
| ` capacity_creatures ` | ` 8 ` |
| ` maximum_creature_size ` | ` medium ` |
| ` blocked_crossing ` | ` ["attacks", "spells", "other_effects"] ` |
| ` vision_from_inside_through_portal ` | ` true ` |
| ` spell_end ` | ` contents_drop_out ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 1
  unit: hour
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

