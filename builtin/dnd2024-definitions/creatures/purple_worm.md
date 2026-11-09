---
splat_schema: 2
splat_id: b0392038-ffb8-5c6d-9fe9-99bc4eda61ee
splat_kind: creature
splat_key: dnd2024:creatures/purple_worm
splat_name: Purple Worm
splat_origin: dnd2024:reference/creature/purple_worm
splat_sources:
- SRD 5.2.1, pp. 316, 317, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 250, Habitat
---

# Purple Worm


## Stat block

| Stat | Value |
| --- | --- |
| Size | Gargantuan |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 18 |
| Hit Points | 247 (15d20 + 90) |
| Initiative | +3 (13) |
| Speed | 50 ft., Burrow 50 ft. |
| Challenge | 15 (XP 13,000; PB +5) |
| Senses | Blindsight 30 ft., Tremorsense 60 ft.; Passive Perception 9 |
| Languages | None |
| Habitat | Desert, Underdark |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 28 | +9 | +9 |
| Dex | 7 | -2 | -2 |
| Con | 22 | +6 | +11 |
| Int | 1 | -5 | -5 |
| Wis | 8 | -1 | +4 |
| Cha | 4 | -3 | -3 |


## Source text

<!-- SRD page 316 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Gargantuan Monstrosity, Unaligned\
> AC 18 Initiative +3 (13)\
> HP 247 (15d20 + 90)\
> Speed 50 ft., Burrow 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 28 +9 +9 Dex 7 −2 −2 Con 22 +6 +11\
> Int 1 −5 −5 Wis 8 −1 +4 Cha 4 −3 −3\
> Senses Blindsight 30 ft., Tremorsense 60 ft.; Passive Perception 9\
> Languages None\
> CR 15 (XP 13,000; PB +5)


<!-- SRD page 317 -->

### Traits

Tunneler. The worm can burrow through solid rock at half its Burrow Speed and leaves a 10-foot-diameter tunnel in its wake.

### Actions

Multiattack. The worm makes one Bite attack and one Tail Stinger attack.

Bite. Melee Attack Roll: +14, reach 10 ft. Hit: 22 (3d8 + 9) Piercing damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 19), and it has the Restrained condition until the grapple ends.

Tail Stinger. Melee Attack Roll: +14, reach 10 ft. Hit: 16 (2d6 + 9) Piercing damage plus 35 (10d6) Poison damage.

### Bonus Actions

Swallow. Strength Saving Throw: DC 19, one Large or smaller creature Grappled by the worm (it can have up to three creatures swallowed at a time). Failure: The target is swallowed by the worm, and the Grappled condition ends. A swallowed creature has the Blinded and Restrained conditions, has Total Cover against attacks and other effects outside the worm, and takes 17 (5d6) Acid damage at the start of each of the worm’s turns.

If the worm takes 30 damage or more on a single turn from a creature inside it, the worm must succeed on a DC 21 Constitution saving throw at the end of that turn or regurgitate all swallowed creatures, each of which falls in a space within 5 feet of the worm and has the Prone condition. If the worm dies, any swallowed creature no longer has the Restrained condition and can escape from the corpse using 20 feet of movement, exiting Prone.

Original layout: [[../../srd-5.2.1/pages/316|page 316]], [[../../srd-5.2.1/pages/317|page 317]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/purple_worm ` |
| ` implementation ` | ` dnd2024:creature_purple_worm ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 9 ` |

#### Senses

<!-- splat:table
path: splat_rule/parameters/senses
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Sense:
    path: sense
    type: symbol
  Range ft:
    path: range_ft
    type: integer
-->

| Key | Sense | Range ft |
| --- | --- | --- |
| ` row_1 ` | ` Blindsight ` | ` 30 ` |
| ` row_2 ` | ` Tremorsense ` | ` 60 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 14
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 3
        sides: 8
      add: 9
    average: 22
- op: grapple
  escape_dc: 19
  when:
    target_kind: creature
    target_maximum_size: large
  while_grappled:
  - op: condition.apply
    condition: restrained
```

##### Tail stinger

<!-- splat:yaml path=splat_rule/parameters/features/tail_stinger -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 14
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 6
      add: 9
    average: 16
  - type: poison
    amount:
      dice:
        count: 10
        sides: 6
      add: 0
    average: 35
```

##### Tunneler

<!-- splat:values path=splat_rule/parameters/features/tunneler -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` burrow.solid_rock ` |
| ` speed_multiplier ` | ` 0.5 ` |
| ` leaves_tunnel_diameter_ft ` | ` 10 ` |

##### Swallow

<!-- splat:yaml path=splat_rule/parameters/features/swallow -->

```yaml
economy: bonus_action
op: saving_throw
ability: strength
dc: 19
targets:
  kind: creature
  count: 1
  maximum_size: large
  grappled_by: actor
failure:
- op: swallow
  end_grapple: true
  conditions:
  - blinded
  - restrained
  total_cover_against: outside_actor
  periodic_damage:
    event: actor.turn_start
    damage:
    - type: acid
      amount:
        dice:
          count: 5
          sides: 6
        add: 0
      average: 17
success: []
swallowed_capacity: 3
regurgitation:
  trigger: turn.end
  ability: constitution
  dc: 21
  failure:
    targets: all_swallowed_creatures
    exit_within_ft: 5
    condition: prone
  when:
    damage_to_actor_from_one_swallowed_creature_this_turn_at_least: 30
actor_death:
  remove_condition: restrained
  escape_movement_cost_ft: 20
  exit_condition: prone
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` multiattack ` |

###### Sequence

<!-- splat:table
path: splat_rule/parameters/features/multiattack/sequence
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Count:
    path: count
    type: integer
  Feature:
    path: feature
    type: symbol
-->

| Key | Count | Feature |
| --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` Bite ` |
| ` row_2 ` | ` 1 ` | ` Tail stinger ` |

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> burrow_speed:
>   unit: ft
>   value: 50
> ```

