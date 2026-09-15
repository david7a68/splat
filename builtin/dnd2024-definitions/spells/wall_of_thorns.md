---
splat_schema: 1
splat_id: f189d34d-37cc-5a3a-8c99-504e524dbfd3
splat_kind: spell
splat_key: dnd2024:spells/wall_of_thorns
splat_name: Wall of Thorns
splat_origin:
  key: dnd2024:reference/spell/wall_of_thorns
splat_sources:
- title: SRD 5.2.1
  locator: pp. 173
  url: https://www.dndbeyond.com/srd
---

# Wall of Thorns

## Source text

<!-- SRD page 173 -->

Level 6 Conjuration (Druid)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a handful of thorns)\
Duration: Concentration, up to 10 minutes\
You create a wall of tangled brush bristling with\
needle-sharp thorns. The wall appears within range\
on a solid surface and lasts for the duration. You\
choose to make the wall up to 60 feet long, 10 feet\
high, and 5 feet thick or a circle that has a 20-foot\
diameter and is up to 20 feet high and 5 feet thick.\
The wall blocks line of sight.\
When the wall appears, each creature in its area\
makes a Dexterity saving throw, taking 7d8 Pierc-\
ing damage on a failed save or half as much damage\
on a successful one.\
A creature can move through the wall, albeit\
slowly and painfully. For every 1 foot a creature\
moves through the wall, it must spend 4 feet of\
movement. Furthermore, the first time a creature\
enters a space in the wall on a turn or ends its turn\
there, the creature makes a Dexterity saving throw,\
taking 7d8 Slashing damage on a failed save or half\
as much damage on a successful one. A creature\
makes this save only once per turn.\
Using a Higher-Level Spell Slot. Both types of\
damage increase by 1d8 for each spell slot level\
above 6.\

Original layout: [[../../srd-5.2.1/pages/173|page 173]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["druid"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a handful of thorns ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/wall_of_thorns ` |
| ` implementation ` | ` dnd2024:spell_wall_of_thorns ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` surface ` | ` solid ` |
| ` blocks_line_of_sight ` | ` true ` |

#### Geometry choice

<!-- splat:yaml path=splat_rule/parameters/geometry_choice -->

```yaml
wall:
  maximum_length_feet: 60
  maximum_height_feet: 10
  maximum_thickness_feet: 5
circle:
  diameter_feet: 20
  maximum_height_feet: 20
  maximum_thickness_feet: 5
```

#### Appearance effect

<!-- splat:yaml path=splat_rule/parameters/appearance_effect -->

```yaml
event: spell.cast_completed
targets: creatures_in_wall
save:
  ability: dexterity
  dc:
    binding: save_dc
  failure:
    damage:
      amount:
        dice:
          count: 7
          sides: 8
          add: 0
      type: piercing
  success:
    damage_fraction:
      numerator: 1
      denominator: 2
```

#### Movement

<!-- splat:values path=splat_rule/parameters/movement -->

| Field | Value |
| --- | --- |
| ` per_foot_cost_feet ` | ` 4 ` |

#### Later effect

<!-- splat:yaml path=splat_rule/parameters/later_effect -->

```yaml
triggers:
- area.first_entry_on_turn
- turn.end
save:
  ability: dexterity
  dc:
    binding: save_dc
  failure:
    damage:
      amount:
        dice:
          count: 7
          sides: 8
          add: 0
      type: slashing
  success:
    damage_fraction:
      numerator: 1
      denominator: 2
maximum_saves_per_creature_per_turn: 1
```

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 6 ` |
| ` piercing_dice ` | ` 1 ` |
| ` slashing_dice ` | ` 1 ` |
| ` sides ` | ` 8 ` |
