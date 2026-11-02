---
splat_schema: 1
splat_id: 3e7549e0-4480-5966-9405-1d87d9f3e672
splat_kind: spell
splat_key: dnd2024:spells/plane_shift
splat_name: Plane Shift
splat_origin:
  key: dnd2024:reference/spell/plane_shift
splat_sources:
- title: SRD 5.2.1
  locator: pp. 153
  url: https://www.dndbeyond.com/srd
---

# Plane Shift

## Source text

<!-- SRD page 153 -->

Level 7 Conjuration (Cleric, Druid, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a forked, metal rod worth 250+ GP and attuned to a plane of existence)\
Duration: Instantaneous

You and up to eight willing creatures who link hands in a circle are transported to a different plane of existence. You can specify a target destination in general terms, such as a specific city on the Elemental Plane of Fire or palace on the second level of the Nine Hells, and you appear in or near that destination, as determined by the GM.

Alternatively, if you know the sigil sequence of a teleportation circle on another plane of existence, this spell can take you to that circle. If the teleportation circle is too small to hold all the creatures you transported, they appear in the closest unoccupied spaces next to the circle.

Original layout: [[../../srd-5.2.1/pages/153|page 153]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["cleric", "druid", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
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
| ` description ` | ` a forked, metal rod worth 250+ GP and attuned to a plane of existence ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/plane_shift ` |
| ` implementation ` | ` dnd2024:spell_plane_shift ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` caster ` | ` true ` |
| ` additional_creatures_maximum ` | ` 8 ` |
| ` willing ` | ` true ` |
| ` link_hands_in_circle ` | ` true ` |

#### Transport

<!-- splat:yaml path=splat_rule/parameters/transport -->

```yaml
different_plane: true
plane_matches_attuned_material: true
destination_modes:
  general:
    caster_specifies: general_terms
    arrival_chooser: gm
    arrival_constraint: in_or_near_destination
  teleportation_circle:
    requires_known_sigil_sequence: true
    destination: circle_on_other_plane
    overflow: closest_unoccupied_spaces_next_to_circle
```
