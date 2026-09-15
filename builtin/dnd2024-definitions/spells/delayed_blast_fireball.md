---
splat_schema: 1
splat_id: 429c8b57-3f89-57f7-a475-502b156c6617
splat_kind: spell
splat_key: dnd2024:spells/delayed_blast_fireball
splat_name: Delayed Blast Fireball
splat_origin:
  key: dnd2024:reference/spell/delayed_blast_fireball
splat_sources:
- title: SRD 5.2.1
  locator: pp. 122
  url: https://www.dndbeyond.com/srd
---

# Delayed Blast Fireball

## Source text

<!-- SRD page 122 -->

Level 7 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (a ball of bat guano and sulfur)\
Duration: Concentration, up to 1 minute\
A beam of yellow light flashes from you, then con-\
denses at a chosen point within range as a glowing\
bead for the duration. When the spell ends, the bead\
explodes, and each creature in a 20-foot-radius\
Sphere centered on that point makes a Dexterity\
saving throw. A creature takes Fire damage equal\
to the total accumulated damage on a failed save or\
half as much damage on a successful one.\
The spell’s base damage is 12d6, and the damage\
increases by 1d6 whenever your turn ends and the\
spell hasn’t ended.\
If a creature touches the glowing bead before the\
spell ends, that creature makes a Dexterity saving\
throw. On a failed save, the spell ends, causing the\
bead to explode. On a successful save, the creature\
can throw the bead up to 40 feet. If the thrown bead\
enters a creature’s space or collides with a solid ob-\
ject, the spell ends, and the bead explodes.\
When the bead explodes, flammable objects in the\
explosion that aren’t being worn or carried start\
burning.\
Using a Higher-Level Spell Slot. The base damage\
increases by 1d6 for each spell slot level above 7.\

Original layout: [[../../srd-5.2.1/pages/122|page 122]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 150 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a ball of bat guano and sulfur ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/delayed_blast_fireball ` |
| ` implementation ` | ` dnd2024:spell_delayed_blast_fireball ` |

### Parameters

#### Bead

<!-- splat:yaml path=splat_rule/parameters/bead -->

```yaml
initial_location: chosen_point_within_range
duration: spell_duration
base_damage:
  dice:
    count:
      op: add
      args:
      - 12
      - op: multiply
        args:
        - 1
        - op: subtract
          args:
          - binding: cast_level
          - 7
    sides: 6
    add: 0
  type: fire
accumulate:
  event: caster_turn_end
  requires: spell_not_ended
  additional_dice:
    dice:
      count: 1
      sides: 6
      add: 0
touch:
  save:
    ability: dexterity
    dc:
      binding: save_dc
    failure:
      end_spell: true
    success:
      throw_optional: true
      throw_maximum_feet: 40
thrown_collision_end_spell:
- creature_space
- solid_object
```

#### Explosion

<!-- splat:yaml path=splat_rule/parameters/explosion -->

```yaml
trigger: spell_ends
origin: current_bead_location
shape: sphere
radius_feet: 20
targets: each_creature_in_area
save:
  ability: dexterity
  dc:
    binding: save_dc
  failure:
    damage: accumulated
  success:
    damage: accumulated
    damage_fraction:
      numerator: 1
      denominator: 2
ignite:
  kind: flammable_object
  worn: false
  carried: false
  in_area: true
```
