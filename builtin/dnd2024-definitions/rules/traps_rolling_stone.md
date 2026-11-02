---
splat_schema: 1
splat_id: a367499e-87dd-54dd-bdb3-c16d1fee56f7
splat_kind: rule
splat_key: dnd2024:rules/traps_rolling_stone
splat_name: Rolling Stone
splat_origin:
  key: dnd2024:reference/pages/201
splat_sources:
- title: SRD 5.2.1
  locator: pp. 201
  url: https://www.dndbeyond.com/srd
---

# Rolling Stone

## Source text

<!-- SRD page 201 -->

Deadly Trap (Levels 11–16) or Nuisance Trap (Levels 17–20)

Trigger: A creature moves onto a pressure plate\
Duration: Until the stone stops rolling

When a creature moves onto a hidden pressure plate, a 5-foot-radius orb of solid stone is released from a secret compartment and begins to roll. The stone and all creatures nearby roll Initiative; the stone gets a +8 bonus on its Initiative roll.

On its turn, the stone moves 60 feet in one direction, changing course if redirected by an obstacle. The stone can move through creatures’ spaces, and creatures can move through the stone’s space, treating it as Difficult Terrain. Whenever the stone enters a creature’s space for the first time on a turn or a creature enters the stone’s space while the stone is rolling, that creature must succeed on a DC 15 Dexterity saving throw or take 55 (10d10) Bludgeoning damage and have the Prone condition.

The stone stops when it hits a wall or similar barrier. It can’t go around corners, but creative dungeon builders incorporate curving turns into nearby passages that allow the stone to keep moving.

Detect and Disarm. As a Study action, a creature can examine the section of floor that forms the pressure plate and make a DC 15 Intelligence (Investigation) check, deducing the pressure plate’s function on a successful check. Wedging an Iron Spike or a similar object under the pressure plate prevents the trap from triggering.

Destroy the Stone. The stone is a Large object that has AC 17, HP 100, a Damage Threshold of 10, and Immunity to Poison and Psychic damage.

Slow the Stone. As an action, a creature can try to slow down the stone with a DC 20 Strength (Athletics) check. If the check is successful, the distance the stone moves on its turn is reduced by 15 feet. If that distance drops to 0, it stops moving and is no longer a threat.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/traps_rolling_stone ` |
| ` implementation ` | ` dnd2024:rule_traps_rolling_stone ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` creature_enters_pressure_plate ` |
| ` duration ` | ` until_stone_stops ` |
| ` initiative_participants ` | ` stone_and_nearby_creatures ` |

#### Severity by level

<!-- splat:table
path: splat_rule/parameters/severity_by_level
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Levels:
    path: levels
    type: yaml
  Severity:
    path: severity
    type: symbol
-->

| Key | Levels | Severity |
| --- | --- | --- |
| ` row_1 ` | ` [11, 16] ` | ` Deadly ` |
| ` row_2 ` | ` [17, 20] ` | ` Nuisance ` |

#### Stone

<!-- splat:values path=splat_rule/parameters/stone -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 5 ` |
| ` size ` | ` large ` |
| ` material ` | ` solid_stone ` |
| ` ac ` | ` 17 ` |
| ` hit_points ` | ` 100 ` |
| ` damage_threshold ` | ` 10 ` |
| ` damage_immunities ` | ` ["poison", "psychic"] ` |
| ` initiative_bonus ` | ` 8 ` |
| ` movement_feet_per_turn ` | ` 60 ` |

#### Movement

<!-- splat:values path=splat_rule/parameters/movement -->

| Field | Value |
| --- | --- |
| ` direction ` | ` single_direction ` |
| ` obstacle_can_redirect ` | ` true ` |
| ` can_pass_creature_spaces ` | ` true ` |
| ` creatures_can_pass_stone_space ` | ` true ` |
| ` stone_space ` | ` difficult_terrain ` |
| ` stops_at ` | ` wall_or_similar_barrier ` |
| ` turns_corners ` | ` false ` |
| ` curving_passages_allow_redirection ` | ` true ` |

#### Collision

<!-- splat:yaml path=splat_rule/parameters/collision -->

```yaml
triggers:
- stone_enters_creature_space_first_time_on_turn
- creature_enters_rolling_stone_space
save:
  ability: dexterity
  dc: 15
  failure:
    damage:
      dice:
        count: 10
        sides: 10
    average: 55
    type: bludgeoning
    condition: prone
  success: no_effect
```

#### Detect

<!-- splat:values path=splat_rule/parameters/detect -->

| Field | Value |
| --- | --- |
| ` action ` | ` study ` |
| ` ability ` | ` intelligence ` |
| ` skill ` | ` investigation ` |
| ` dc ` | ` 15 ` |
| ` success ` | ` deduce_pressure_plate_function ` |

#### Disarm

<!-- splat:values path=splat_rule/parameters/disarm -->

| Field | Value |
| --- | --- |
| ` wedge_under_plate ` | ` iron_spike_or_similar ` |
| ` effect ` | ` prevents_trigger ` |

#### Slow

<!-- splat:yaml path=splat_rule/parameters/slow -->

```yaml
action_cost: 1
ability: strength
skill: athletics
dc: 20
success:
  movement_feet_per_turn_subtract: 15
  at_zero: stone_stops
```
