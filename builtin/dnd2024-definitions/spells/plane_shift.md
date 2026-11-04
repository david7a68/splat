---
splat_schema: 2
splat_id: 3e7549e0-4480-5966-9405-1d87d9f3e672
splat_kind: spell
splat_key: dnd2024:spells/plane_shift
splat_name: Plane Shift
splat_origin: dnd2024:reference/spell/plane_shift
splat_sources:
- SRD 5.2.1, pp. 153, https://www.dndbeyond.com/srd
---

# Plane Shift


## Spell

| Field | Value |
| --- | --- |
| Level | 7 |
| School | Conjuration |
| Classes | Cleric, Druid, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a forked, metal rod worth 250+ GP and attuned to a plane of existence) |
| Duration | Instantaneous |


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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

