---
splat_schema: 1
splat_id: 67b895f2-bf1b-5105-b40c-2ed954a893da
splat_kind: creature
splat_key: dnd2024:creatures/gelatinous_cube
splat_name: Gelatinous Cube
splat_origin:
  key: dnd2024:reference/creature/gelatinous_cube
splat_sources:
- title: SRD 5.2.1
  locator: pp. 286, 287
  url: https://www.dndbeyond.com/srd
---

# Gelatinous Cube

## Source text

<!-- SRD page 286 -->

Large Ooze, Unaligned\
AC 6 Initiative −4 (6)\
HP 63 (6d10 + 30)\
Speed 15 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 3 −4 −4 Con 20 +5 +5\
Int 1 −5 −5 Wis 6 −2 −2 Cha 1 −5 −5\
Immunities Acid; Blinded, Charmed, Deafened, Exhaustion, Frightened, Prone\
Senses Blindsight 60 ft.; Passive Perception 8\
Languages None\
CR 2 (XP 450; PB +2)

<!-- SRD page 287 -->

### Traits

Ooze Cube. The cube fills its entire space and is transparent. Other creatures can enter that space, but a creature that does so is subjected to the cube’s Engulf and has Disadvantage on the saving throw.

Creatures inside the cube have Total Cover, and the cube can hold one Large creature or up to four Medium or Small creatures inside itself at a time.

As an action, a creature within 5 feet of the cube can pull a creature or an object out of the cube by succeeding on a DC 12 Strength (Athletics) check, and the puller takes 10 (3d6) Acid damage.

Transparent. Even when the cube is in plain sight, a creature must succeed on a DC 15 Wisdom (Perception) check to notice the cube if the creature hasn’t witnessed the cube move or otherwise act.

### Actions

Pseudopod. Melee Attack Roll: +4, reach 5 ft. Hit: 12 (3d6 + 2) Acid damage.

Engulf. The cube moves up to its Speed without provoking Opportunity Attacks. The cube can move through the spaces of Large or smaller creatures if it has room inside itself to contain them (see the Ooze Cube trait). Dexterity Saving Throw: DC 12, each creature whose space the cube enters for the first time during this move. Failure: 10 (3d6) Acid damage, and the target is engulfed. An engulfed target is suffocating, can’t cast spells with a Verbal component, has the Restrained condition, and takes 10 (3d6) Acid damage at the start of each of the cube’s turns. When the cube moves, the engulfed target moves with it. An engulfed target can try to escape by taking an action to make a DC 12 Strength (Athletics) check. On a successful check, the target escapes and enters the nearest unoccupied space. Success: Half damage, and the target moves to an unoccupied space within 5 feet of the cube. If there is no unoccupied space, the target fails the save instead.

Original layout: [[../../srd-5.2.1/pages/286|page 286]], [[../../srd-5.2.1/pages/287|page 287]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` ooze ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 15 ft. ` |
| ` immunities_text ` | ` Acid; Blinded, Charmed, Deafened, Exhaustion, Frightened, Prone ` |
| ` senses_text ` | ` Blindsight 60 ft.; Passive Perception 8 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 63 (6d10 + 30) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 14 ` | ` 2 ` |
| ` Dexterity ` | ` 3 ` | ` -4 ` |
| ` Constitution ` | ` 20 ` | ` 5 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 6 ` | ` -2 ` |
| ` Charisma ` | ` 1 ` | ` -5 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 6 ` |  |
| ` max_hp ` | ` 63 ` |  |
| ` initiative ` | ` -4 ` |  |
| ` speed ` | ` 15 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/gelatinous_cube ` |
| ` implementation ` | ` dnd2024:creature_gelatinous_cube ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["acid"] ` |
| ` condition_immunities ` | ` ["blinded", "charmed", "deafened", "exhaustion", "frightened", "prone"] ` |

#### Features

##### Ooze cube

<!-- splat:values path=splat_rule/parameters/features/ooze_cube -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` fills_space ` | ` true ` |
| ` transparent ` | ` true ` |
| ` other_creatures_can_enter ` | ` true ` |
| ` inside_cover ` | ` total ` |

###### On creature entry

<!-- splat:values path=splat_rule/parameters/features/ooze_cube/on_creature_entry -->

| Field | Value |
| --- | --- |
| ` use ` | ` engulf ` |
| ` save_disadvantage ` | ` true ` |

###### Capacity

###### Choose one

<!-- splat:table
path: splat_rule/parameters/features/ooze_cube/capacity/choose_one
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Size:
    path: size
    type: symbol
    optional: true
  Count:
    path: count
    type: integer
  Sizes:
    path: sizes
    type: yaml
    optional: true
-->

| Key | Size | Count | Sizes |
| --- | --- | --- | --- |
| ` row_1 ` | ` Large ` | ` 1 ` |  |
| ` row_2 ` |  | ` 4 ` | ` ["medium", "small"] ` |

###### Pull out

<!-- splat:yaml path=splat_rule/parameters/features/ooze_cube/pull_out -->

```yaml
economy: action
actor_within_ft: 5
targets:
- creature
- object
check: strength_athletics
dc: 12
actor_damage:
- roll:
    count: 3
    sides: 6
    modifier: 0
  type: acid
```

##### Transparent

<!-- splat:yaml path=splat_rule/parameters/features/transparent -->

```yaml
economy: trait
operation: passive_rule
notice_check:
  ability: wisdom
  skill: perception
  dc: 15
  required_even_plain_sight: true
  unless: observer_has_seen_cube_move_or_act
```

##### Pseudopod

<!-- splat:yaml path=splat_rule/parameters/features/pseudopod -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 4
  reach_ft: 5
hit:
  damage:
  - average: 12
    roll:
      count: 3
      sides: 6
      modifier: 2
    type: acid
```

##### Engulf

<!-- splat:yaml path=splat_rule/parameters/features/engulf -->

```yaml
economy: action
operation: saving_throw
ability: dexterity
dc: 12
targets:
  kind: creature
  trigger: self.enters_target_space
  limit: first_time_during_this_move
failure:
  damage:
  - roll:
      count: 3
      sides: 6
      modifier: 0
    type: acid
  engulfed:
    suffocating: true
    can_cast_verbal_spells: false
    conditions:
    - restrained
    damage_each_self_turn_start:
    - roll:
        count: 3
        sides: 6
        modifier: 0
      type: acid
    moves_with_self: true
    escape:
      economy: action
      check: strength_athletics
      dc: 12
      destination: nearest_unoccupied_space
success:
  damage_multiplier: 0.5
  move_target:
    unoccupied: true
    within_self_ft: 5
  if_no_unoccupied_space: failure_instead
before_save:
  move:
    operation: move
    maximum_ft:
      multiply:
      - stat: speed
      - 1
    provoke_opportunity_attacks: false
  pass_through:
    maximum_size: large
    requires_capacity: true
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 60 ` |
| ` passive_perception ` | ` 8 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands: []
```

#### Resolution conventions

<!-- splat:yaml path=splat_rule/parameters/resolution_conventions -->

```yaml
distance_unit: foot
duration_unit: second
damage_roll_mode:
  choice:
  - listed_average
  - roll_dice
  chooser: gm
supplied_totals: before_additional_effect_modifiers
multiattack_nested_economy_cost: 0
nested_feature_resource_cost: normal
spell_level_unless_specified: lowest_possible
spell_upcast_without_permission: false
spell_casting_time_at_least_one_minute: normal_casting_procedure
geometry:
  input: adjudication
  assumptions:
  - range_and_visibility_verified
  - area_membership_verified
  - path_and_destination_legal
unlisted_success_branch: no_effect
unlisted_duration: until_removed_by_applicable_rule
```
