---
splat_schema: 1
splat_id: d39e9d55-9f59-51d3-af63-660d2d0a482f
splat_kind: spell
splat_key: dnd2024:spells/flaming_sphere
splat_name: Flaming Sphere
splat_origin:
  key: dnd2024:reference/spell/flaming_sphere
splat_sources:
- title: SRD 5.2.1
  locator: pp. 132
  url: https://www.dndbeyond.com/srd
---

# Flaming Sphere

## Source text

<!-- SRD page 132 -->

Level 2 Conjuration (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a ball of wax)\
Duration: Concentration, up to 1 minute

You create a 5-foot-diameter sphere of fire in an unoccupied space on the ground within range. It lasts for the duration. Any creature that ends its turn within 5 feet of the sphere makes a Dexterity saving throw, taking 2d6 Fire damage on a failed save or half as much damage on a successful one.

As a Bonus Action, you can move the sphere up to 30 feet, rolling it along the ground. If you move the sphere into a creature’s space, that creature makes the save against the sphere, and the sphere stops moving for the turn.

When you move the sphere, you can direct it over barriers up to 5 feet tall and jump it across pits up to 10 feet wide. Flammable objects that aren’t being worn or carried start burning if touched by the sphere, and it sheds Bright Light in a 20-foot radius and Dim Light for an additional 20 feet.

Using a Higher-Level Spell Slot. The damage increases by 1d6 for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/132|page 132]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["druid", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
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
| ` description ` | ` a ball of wax ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/flaming_sphere ` |
| ` implementation ` | ` dnd2024:spell_flaming_sphere ` |

### Parameters

#### Sphere

<!-- splat:values path=splat_rule/parameters/sphere -->

| Field | Value |
| --- | --- |
| ` diameter_feet ` | ` 5 ` |
| ` initial_space ` | ` unoccupied_ground ` |
| ` bright_radius_feet ` | ` 20 ` |
| ` additional_dim_feet ` | ` 20 ` |

#### Damage trigger

<!-- splat:yaml path=splat_rule/parameters/damage_trigger -->

```yaml
event: turn.end
distance_feet: 5
save:
  ability: dexterity
  dc:
    binding: save_dc
  failure:
    damage:
      amount:
        dice:
          count: 2
          sides: 6
          add: 0
      type: fire
  success:
    damage_fraction:
      numerator: 1
      denominator: 2
```

#### Move

<!-- splat:yaml path=splat_rule/parameters/move -->

```yaml
action: bonus_action
maximum_feet: 30
along_ground: true
barrier_max_height_feet: 5
pit_max_width_feet: 10
creature_space_entry:
  repeat_damage_save: true
  stop_movement_for_turn: true
```

#### Objects

<!-- splat:values path=splat_rule/parameters/objects -->

| Field | Value |
| --- | --- |
| ` on_touch ` | ` true ` |
| ` flammable_unworn_uncarried ` | ` burning ` |

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 2 ` |
| ` damage_dice ` | ` 1 ` |
| ` sides ` | ` 6 ` |
