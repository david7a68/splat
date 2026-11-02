---
splat_schema: 1
splat_id: 950eac6e-a446-58bb-b123-959d6cebb0f3
splat_kind: rule
splat_key: dnd2024:rules/combat_your_turn
splat_name: Your Turn
splat_origin:
  key: dnd2024:reference/pages/013
splat_sources:
- title: SRD 5.2.1
  locator: pp. 13, 14
  url: https://www.dndbeyond.com/srd
---

# Your Turn

## Source text

<!-- SRD page 13 -->

On your turn, you can move a distance up to your Speed and take one action. You decide whether to move first or take your action first.

The main actions you can take are listed in “Actions” earlier in “Playing the Game.” A character’s features and a monster’s stat block also provide action options. “Movement and Position” later in “Playing the Game” gives the rules for movement.

Communicating. You can communicate however you are able—through brief utterances and gestures—as you take your turn. Doing so uses neither your action nor your move.

Extended communication, such as a detailed explanation of something or an attempt to persuade a foe, requires an action. The Influence action is the main way you try to influence a monster.

Interacting with Things. You can interact with one object or feature of the environment for free, during either your move or action. For example, you could open a door during your move as you stride toward a foe.

If you want to interact with a second object, you need to take the Utilize action. Some magic items and other special objects always require an action to use, as stated in their descriptions.

### Playing On A Grid

If you play using a square grid and miniatures or other tokens, follow these rules.

Squares. Each square represents 5 feet.

Speed. Rather than moving foot by foot, move square by square on the grid, using your Speed in 5-foot segments. You can translate your Speed into squares by dividing it by 5. For example, a Speed of 30 feet translates into 6 squares. If you use a grid often, consider writing your Speed in squares on your character sheet.

Entering a Square. To enter a square, you must have enough movement left to pay for entering. It costs 1 square of movement to enter an unoccupied square that’s adjacent to your space (orthogonally or diagonally adjacent). A square of Difficult Terrain costs 2 squares to enter. Other effects might make a square cost even more.

Corners. Diagonal movement can’t cross the corner of a wall, a large tree, or another terrain feature that fills its space.

Ranges. To determine the range on a grid between two things—whether creatures or objects— count squares from a square adjacent to one of them and stop counting in the space of the other one. Count by the shortest route.

<!-- SRD page 14 -->

The GM might require you to use an action for any of these activities when it needs special care or when it presents an unusual obstacle. For instance, the GM might require you to take the Utilize action to open a stuck door or turn a crank to lower a drawbridge.

Doing Nothing on Your Turn. You can forgo moving, taking an action, or doing anything at all on your turn. If you can’t decide what to do, consider taking the defensive Dodge action or the Ready action to delay acting.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_your_turn ` |
| ` implementation ` | ` dnd2024:rule_combat_your_turn ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` movement_maximum ` | ` speed ` |
| ` actions ` | ` 1 ` |
| ` order ` | ` actor_choice ` |
| ` may_forgo ` | ` true ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` brief_utterances_or_gestures_cost ` | ` 0 ` |
| ` extended_cost_actions ` | ` 1 ` |
| ` persuasion_action ` | ` influence ` |

#### Object interactions

<!-- splat:yaml path=splat_rule/parameters/object_interactions -->

```yaml
free: 1
during:
- movement
- action
additional_action: utilize
special_item_cost: description
gm_require_utilize:
  when_any:
  - special_care
  - unusual_obstacle
```

#### Grid

<!-- splat:yaml path=splat_rule/parameters/grid -->

```yaml
square_feet: 5
speed_squares:
  op: floor
  args:
  - op: divide
    args:
    - variable: speed
    - 5
adjacent_entry_squares: 1
diagonal_entry_squares: 1
difficult_terrain_entry_squares: 2
entry_requires_sufficient_movement: true
diagonal_cross_solid_corner: false
range: shortest_adjacent_square_path_excluding_origin_including_destination
additional_costs_apply: true
```
