---
splat_schema: 1
splat_id: a30dff02-a8b5-5c52-8344-5a0969e1d098
splat_kind: spell
splat_key: dnd2024:spells/forcecage
splat_name: Forcecage
splat_origin:
  key: dnd2024:reference/spell/forcecage
splat_sources:
- title: SRD 5.2.1
  locator: pp. 134
  url: https://www.dndbeyond.com/srd
---

# Forcecage

## Source text

<!-- SRD page 134 -->

Level 7 Evocation (Bard, Warlock, Wizard)\
Casting Time: Action\
Range: 100 feet\
Components: V, S, M (ruby dust worth 1,500+ GP, which the spell consumes)\
Duration: Concentration, up to 1 hour

An immobile, Invisible, Cube-shaped prison composed of magical force springs into existence around an area you choose within range. The prison can be a cage or a solid box, as you choose.

A prison in the shape of a cage can be up to 20 feet on a side and is made from 1/2-inch diameter bars spaced 1/2 inch apart. A prison in the shape of a box can be up to 10 feet on a side, creating a solid barrier that prevents any matter from passing through it and blocking any spells cast into or out from the area.

When you cast the spell, any creature that is completely inside the cage’s area is trapped. Creatures only partially within the area, or those too large to fit inside it, are pushed away from the center of the area until they are completely outside it.

A creature inside the cage can’t leave it by nonmagical means. If the creature tries to use teleportation or interplanar travel to leave, it must first make a Charisma saving throw. On a successful save, the creature can use that magic to exit the cage. On a failed save, the creature doesn’t exit the cage and wastes the spell or effect. The cage also extends into the Ethereal Plane, blocking ethereal travel.

This spell can’t be dispelled by Dispel Magic.

Original layout: [[../../srd-5.2.1/pages/134|page 134]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["bard", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 100 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` ruby dust worth 1,500+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/forcecage ` |
| ` implementation ` | ` dnd2024:spell_forcecage ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` dispel_magic_can_dispel ` | ` false ` |

#### Creation

<!-- splat:yaml path=splat_rule/parameters/creation -->

```yaml
shape: cube
material: magical_force
immobile: true
invisible: true
within_spell_range: true
duration: spell_duration
modes:
  cage:
    side_feet_maximum: 20
    bar_diameter_inches:
      numerator: 1
      denominator: 2
    bar_spacing_inches:
      numerator: 1
      denominator: 2
  box:
    side_feet_maximum: 10
    blocks:
    - all_matter
    - spells_cast_into_area
    - spells_cast_out_of_area
```

#### Initial creatures

<!-- splat:yaml path=splat_rule/parameters/initial_creatures -->

```yaml
fully_inside: trapped
partially_inside_or_too_large:
  push_direction: away_from_center
  until: fully_outside
```

#### Escape

<!-- splat:yaml path=splat_rule/parameters/escape -->

```yaml
nonmagical: prohibited
teleport_or_interplanar:
  prior_save:
    ability: charisma
    dc:
      binding: save_dc
    failure:
      exit: false
      spell_or_effect: wasted
    success:
      exit_allowed: true
ethereal: blocked
```
