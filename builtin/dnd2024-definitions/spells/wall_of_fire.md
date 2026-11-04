---
splat_schema: 2
splat_id: 7ef9958a-2469-5e21-ae1b-eb9291455538
splat_kind: spell
splat_key: dnd2024:spells/wall_of_fire
splat_name: Wall of Fire
splat_origin: dnd2024:reference/spell/wall_of_fire
splat_sources:
- SRD 5.2.1, pp. 172, https://www.dndbeyond.com/srd
---

# Wall of Fire


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Evocation |
| Classes | Druid, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S, M (a piece of charcoal) |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 172 -->

Level 4 Evocation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a piece of charcoal)\
Duration: Concentration, up to 1 minute

You create a wall of fire on a solid surface within range. You can make the wall up to 60 feet long, 20 feet high, and 1 foot thick, or a ringed wall up to 20 feet in diameter, 20 feet high, and 1 foot thick. The wall is opaque and lasts for the duration.

When the wall appears, each creature in its area makes a Dexterity saving throw, taking 5d8 Fire damage on a failed save or half as much damage on a successful one.

One side of the wall, selected by you when you cast this spell, deals 5d8 Fire damage to each creature that ends its turn within 10 feet of that side or inside the wall. A creature takes the same damage when it enters the wall for the first time on a turn or ends its turn there. The other side of the wall deals no damage.

Using a Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above 4.

Original layout: [[../../srd-5.2.1/pages/172|page 172]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/wall_of_fire ` |
| ` implementation ` | ` dnd2024:spell_wall_of_fire ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` surface ` | ` solid ` |
| ` opaque ` | ` true ` |
| ` cold_side_damage ` | ` 0 ` |

#### Geometry choice

<!-- splat:yaml path=splat_rule/parameters/geometry_choice -->

```yaml
wall:
  maximum_length_feet: 60
  maximum_height_feet: 20
  thickness_feet: 1
ring:
  maximum_diameter_feet: 20
  maximum_height_feet: 20
  thickness_feet: 1
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
          count: 5
          sides: 8
          add: 0
      type: fire
  success:
    damage_fraction:
      numerator: 1
      denominator: 2
```

#### Hot side

<!-- splat:yaml path=splat_rule/parameters/hot_side -->

```yaml
chooser: caster
chosen_at: cast
damage:
  amount:
    dice:
      count: 5
      sides: 8
      add: 0
  type: fire
event: turn.end
eligible:
- within_10_feet_of_hot_side
- inside_wall
```

#### Wall damage

<!-- splat:yaml path=splat_rule/parameters/wall_damage -->

```yaml
events:
- area.first_entry_on_turn
- turn.end
inside_wall: true
damage:
  amount:
    dice:
      count: 5
      sides: 8
      add: 0
  type: fire
save: false
same_end_turn_occurrence_not_duplicated: true
```

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 4 ` |
| ` damage_dice ` | ` 1 ` |
| ` sides ` | ` 8 ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

