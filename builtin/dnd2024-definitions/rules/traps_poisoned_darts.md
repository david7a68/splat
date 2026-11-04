---
splat_schema: 2
splat_id: 549c3cfc-df28-5bad-89f1-b3cb2ceb2810
splat_kind: rule
splat_key: dnd2024:rules/traps_poisoned_darts
splat_name: Poisoned Darts
splat_origin: dnd2024:reference/pages/200
splat_sources:
- SRD 5.2.1, pp. 200, https://www.dndbeyond.com/srd
---

# Poisoned Darts

## Source text

<!-- SRD page 200 -->

Deadly Trap (Levels 1–4)

Trigger: A creature moves onto a pressure plate\
Duration: Instantaneous, and the trap resets at the start of the next turn if it has activated fewer than three times

When a creature moves onto this trap’s pressure plate for the first time on a turn, poisoned darts shoot from tubes embedded in the surrounding walls. The holes that house these tubes are obscured by dust and cobwebs or skillfully hidden amid bas-reliefs, murals, or frescoes.

Each creature in the darts’ path must succeed on a DC 13 Dexterity saving throw or be struck by 1d3 darts, taking 3 (1d6) Poison damage per dart.

Detect and Disarm. As a Search action, a creature can examine the walls and make a DC 15 Wisdom (Perception) check, detecting the holes on a successful check. Plugging all the holes with wax, cloth, or detritus prevents the darts from firing.

As a Search action, a creature can examine the section of floor that forms the pressure plate and make a DC 15 Wisdom (Perception) check, detecting the pressure plate on a successful check. Wedging an Iron Spike or a similar object under the pressure plate prevents the trap from triggering.

At Higher Levels. You can scale the trap for higher levels by increasing each dart’s Poison damage as follows: 7 (2d6) at levels 5–10, 14 (4d6) at levels 11–16, or 24 (7d6) at levels 17–20.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/traps_poisoned_darts ` |
| ` implementation ` | ` dnd2024:rules_traps_poisoned_darts ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` targets ` | ` all_creatures_in_dart_paths ` |

#### Trap

<!-- splat:values path=splat_rule/parameters/trap -->

| Field | Value |
| --- | --- |
| ` severity ` | ` deadly ` |
| ` levels ` | ` [1, 4] ` |
| ` trigger ` | ` creature_first_moves_onto_pressure_plate_on_turn ` |
| ` duration ` | ` instantaneous ` |
| ` maximum_activations ` | ` 3 ` |
| ` reset_event ` | ` next_turn.started ` |
| ` reset_if_activations_less_than ` | ` 3 ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` dc ` | ` 13 ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
dart_count:
  count: 1
  sides: 3
  add: 0
damage_per_dart:
  dice:
    count: 1
    sides: 6
    add: 0
  average: 3
  type: poison
```

#### Detection

<!-- splat:table
path: splat_rule/parameters/detection
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Location:
    path: location
    type: symbol
  Economy:
    path: economy
    type: symbol
  Check ability:
    path: check/ability
    type: symbol
  Check dc:
    path: check/dc
    type: integer
  Check skill:
    path: check/skill
    type: symbol
  Reveals:
    path: reveals
    type: symbol
-->

| Key | Location | Economy | Check ability | Check dc | Check skill | Reveals |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Walls ` | ` Search ` | ` Wisdom ` | ` 15 ` | ` Perception ` | ` Dart holes ` |
| ` row_2 ` | ` Floor pressure plate ` | ` Search ` | ` Wisdom ` | ` 15 ` | ` Perception ` | ` Pressure plate ` |

#### Disarm

<!-- splat:rows path=splat_rule/parameters/disarm -->

| Row | operation | materials | effect |
| --- | --- | --- | --- |
| ` 0 ` | ` plug_all_holes ` | ` ["wax", "cloth", "detritus"] ` | ` prevents_firing ` |
| ` 1 ` | ` wedge_pressure_plate ` | ` ["iron_spike", "similar_object"] ` | ` prevents_triggering ` |

#### Scaling

<!-- splat:table
path: splat_rule/parameters/scaling
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Levels:
    path: levels
    type: yaml
  Poison per dart dice:
    path: poison_per_dart/dice
    type: dice
  Poison per dart average:
    path: poison_per_dart/average
    type: integer
-->

| Key | Levels | Poison per dart dice | Poison per dart average |
| --- | --- | --- | --- |
| ` row_1 ` | ` [1, 4] ` | ` 1d6 ` | ` 3 ` |
| ` row_2 ` | ` [5, 10] ` | ` 2d6 ` | ` 7 ` |
| ` row_3 ` | ` [11, 16] ` | ` 4d6 ` | ` 14 ` |
| ` row_4 ` | ` [17, 20] ` | ` 7d6 ` | ` 24 ` |
