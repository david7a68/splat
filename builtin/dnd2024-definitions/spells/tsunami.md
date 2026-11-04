---
splat_schema: 2
splat_id: e1dd804f-df5d-58a0-ab02-c00e841c8ce3
splat_kind: spell
splat_key: dnd2024:spells/tsunami
splat_name: Tsunami
splat_origin: dnd2024:reference/spell/tsunami
splat_sources:
- SRD 5.2.1, pp. 171, https://www.dndbeyond.com/srd
---

# Tsunami


## Spell

| Field | Value |
| --- | --- |
| Level | 8 |
| School | Conjuration |
| Classes | Druid |
| Casting Time | 1 minute |
| Range |  |
| Components | V, S |
| Duration | Concentration, up to 6 rounds |


## Source text

<!-- SRD page 171 -->

Level 8 Conjuration (Druid)\
Casting Time: 1 minute\
Range: 1 mile\
Components: V, S\
Duration: Concentration, up to 6 rounds

A wall of water springs into existence at a point you choose within range. You can make the wall up to 300 feet long, 300 feet high, and 50 feet thick. The wall lasts for the duration.

When the wall appears, each creature in its area makes a Strength saving throw, taking 6d10 Bludgeoning damage on a failed save or half as much damage on a successful one.

At the start of each of your turns after the wall appears, the wall, along with any creatures in it, moves 50 feet away from you. Any Huge or smaller creature inside the wall or whose space the wall enters when it moves must succeed on a Strength saving throw or take 5d10 Bludgeoning damage. A creature can take this damage only once per round. At the end of the turn, the wall’s height is reduced by 50 feet, and the damage the wall deals on later rounds is reduced by 1d10. When the wall reaches 0 feet in height, the spell ends.

A creature caught in the wall can move by swimming. Because of the wave’s force, though, the creature must succeed on a Strength (Athletics) check against your spell save DC to move at all. If it fails the check, it can’t move. A creature that moves out of the wall falls to the ground.

Original layout: [[../../srd-5.2.1/pages/171|page 171]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/tsunami ` |
| ` implementation ` | ` dnd2024:spell_tsunami ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_rounds ` | ` 6 ` |

#### Wall

<!-- splat:values path=splat_rule/parameters/wall -->

| Field | Value |
| --- | --- |
| ` origin ` | ` chosen_point ` |
| ` range_miles ` | ` 1 ` |
| ` maximum_length_feet ` | ` 300 ` |
| ` maximum_height_feet ` | ` 300 ` |
| ` maximum_duration_rounds ` | ` 6 ` |
| ` maximum_thickness_feet ` | ` 50 ` |

#### Initial

<!-- splat:yaml path=splat_rule/parameters/initial -->

```yaml
targets: all_creatures_in_area
save:
  ability: strength
  dc: spell_save_dc
failure_damage:
  dice: 6d10
  type: bludgeoning
success_damage_scale:
  numerator: 1
  denominator: 2
```

#### Movement

<!-- splat:values path=splat_rule/parameters/movement -->

| Field | Value |
| --- | --- |
| ` event ` | ` turn.start ` |
| ` subject ` | ` caster ` |
| ` only_turns_after_wall_appeared ` | ` true ` |
| ` distance_feet ` | ` 50 ` |
| ` direction ` | ` away_from_caster ` |
| ` carry ` | ` creatures_inside ` |

#### Moving wall damage

<!-- splat:yaml path=splat_rule/parameters/moving_wall_damage -->

```yaml
targets:
  kinds: creature
  maximum_size: huge
  positions:
  - inside_wall
  - space_entered_by_wall
save:
  ability: strength
  dc: spell_save_dc
  success: no_damage
failure_damage:
  initial_dice: 5d10
  type: bludgeoning
  later_round_dice_decrement: 1
damage_limit:
  count: 1
  per: round
  scope: creature
```

#### Decay

<!-- splat:values path=splat_rule/parameters/decay -->

| Field | Value |
| --- | --- |
| ` event ` | ` turn.end ` |
| ` subject ` | ` caster ` |
| ` sequence ` | ` after_wall_movement_turn ` |
| ` height_decrease_feet ` | ` 50 ` |
| ` future_damage_decrease_d10 ` | ` 1 ` |
| ` height_zero ` | ` end_spell ` |

#### Trapped movement

<!-- splat:yaml path=splat_rule/parameters/trapped_movement -->

```yaml
method: swimming
check:
  ability: strength
  skill: athletics
  dc: spell_save_dc
success: may_move
failure: cannot_move
exit_wall: fall_to_ground
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: 1 mile
> ```

