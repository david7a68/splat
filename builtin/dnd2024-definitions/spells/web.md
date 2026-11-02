---
splat_schema: 1
splat_id: af3616c0-4b1f-5a0d-b2d9-1f0fff2fb376
splat_kind: spell
splat_key: dnd2024:spells/web
splat_name: Web
splat_origin:
  key: dnd2024:reference/spell/web
splat_sources:
- title: SRD 5.2.1
  locator: pp. 174
  url: https://www.dndbeyond.com/srd
---

# Web

## Source text

<!-- SRD page 174 -->

Level 2 Conjuration (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a bit of spiderweb)\
Duration: Concentration, up to 1 hour

You conjure a mass of sticky webbing at a point within range. The webs fill a 20-foot Cube there for the duration. The webs are Difficult Terrain, and the area within them is Lightly Obscured.

If the webs aren’t anchored between two solid masses (such as walls or trees) or layered across a floor, wall, or ceiling, the web collapses on itself, and the spell ends at the start of your next turn. Webs layered over a flat surface have a depth of 5 feet.

The first time a creature enters the webs on a turn or starts its turn there, it must succeed on a Dexterity saving throw or have the Restrained condition while in the webs or until it breaks free.

A creature Restrained by the webs can take an action to make a Strength (Athletics) check against your spell save DC. If it succeeds, it is no longer Restrained.

The webs are flammable. Any 5-foot Cube of webs exposed to fire burns away in 1 round, dealing 2d4 Fire damage to any creature that starts its turn in the fire.

Original layout: [[../../srd-5.2.1/pages/174|page 174]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a bit of spiderweb ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/web ` |
| ` implementation ` | ` dnd2024:spell_web ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` triggers ` | ` ["area.first_entry_on_turn", "turn.start"] ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cube ` |
| ` side_feet ` | ` 20 ` |
| ` difficult_terrain ` | ` true ` |
| ` lightly_obscured ` | ` true ` |

#### Support

<!-- splat:yaml path=splat_rule/parameters/support -->

```yaml
valid:
- anchored_between_two_solid_masses
- layered_floor_wall_or_ceiling
unsupported:
  collapse: true
  end_at: caster.next_turn.start
flat_surface_depth_feet: 5
```

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
dc:
  binding: save_dc
failure:
  condition: restrained
  until_any:
  - leaves_webs
  - breaks_free
success:
  effect: none
```

#### Escape

<!-- splat:yaml path=splat_rule/parameters/escape -->

```yaml
action: action
ability: strength
skill: athletics
dc:
  binding: save_dc
success: end_restrained
```

#### Fire

<!-- splat:yaml path=splat_rule/parameters/fire -->

```yaml
cube_side_feet: 5
burn_away_rounds: 1
damage_event: turn.start
target: creature_in_burning_webs
damage:
  amount:
    dice:
      count: 2
      sides: 4
      add: 0
  type: fire
```
