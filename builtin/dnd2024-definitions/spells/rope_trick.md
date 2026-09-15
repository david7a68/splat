---
splat_schema: 1
splat_id: 9f77dd14-43cf-572f-a834-44b88653818b
splat_kind: spell
splat_key: dnd2024:spells/rope_trick
splat_name: Rope Trick
splat_origin:
  key: dnd2024:reference/spell/rope_trick
splat_sources:
- title: SRD 5.2.1
  locator: pp. 159
  url: https://www.dndbeyond.com/srd
---

# Rope Trick

## Source text

<!-- SRD page 159 -->

Level 2 Transmutation (Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a segment of rope)\
Duration: 1 hour\
You touch a rope. One end of it hovers upward until\
the rope hangs perpendicular to the ground or the\
rope reaches a ceiling. At the rope’s upper end, an\
Invisible 3-foot-by-5-foot portal opens to an extra-\
dimensional space that lasts until the spell ends.\
That space can be reached by climbing the rope,\
which can be pulled into or dropped out of it.\
The space can hold up to eight Medium or smaller\
creatures. Attacks, spells, and other effects can’t\
pass into or out of the space, but creatures inside\
it can see through the portal. Anything inside the\
space drops out when the spell ends.\

Original layout: [[../../srd-5.2.1/pages/159|page 159]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a segment of rope ` |
| ` consumed ` | ` false ` |

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
