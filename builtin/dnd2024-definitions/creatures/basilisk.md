---
splat_schema: 2
splat_id: 1b37be75-c5c9-5acc-9039-e032862c647f
splat_kind: creature
splat_key: dnd2024:creatures/basilisk
splat_name: Basilisk
splat_origin: dnd2024:reference/creature/basilisk
splat_sources:
- SRD 5.2.1, pp. 262, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 32, Habitat
---

# Basilisk


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 15 |
| Hit Points | 52 (8d8 + 16) |
| Initiative | -1 (9) |
| Speed | 20 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 9 |
| Languages | None |
| Habitat | Mountain, Underdark |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 8 | -1 | -1 |
| Con | 15 | +2 | +2 |
| Int | 2 | -4 | -4 |
| Wis | 8 | -1 | -1 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 262 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Monstrosity, Unaligned\
> AC 15 Initiative −1 (9)\
> HP 52 (8d8 + 16)\
> Speed 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 8 −1 −1 Con 15 +2 +2\
> Int 2 −4 −4 Wis 8 −1 −1 Cha 7 −2 −2\
> Senses Darkvision 60 ft.; Passive Perception 9\
> Languages None\
> CR 3 (XP 700; PB +2)


### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Piercing damage plus 7 (2d6) Poison damage.

### Bonus Actions

Petrifying Gaze (Recharge 4–6). Constitution Saving Throw: DC 12, each creature in a 30-foot Cone. If the basilisk sees its reflection in the Cone, the basilisk must make this save. First Failure: The target has the Restrained condition and repeats the save at the end of its next turn if it is still Restrained, ending the effect on itself on a success. Second Failure: The target has the Petrified condition instead of the Restrained condition.

Original layout: [[../../srd-5.2.1/pages/262|page 262]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/basilisk ` |
| ` implementation ` | ` dnd2024:creature_basilisk ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 9 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

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
attack_bonus: 5
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 6
      add: 3
    average: 10
  - type: poison
    amount:
      dice:
        count: 2
        sides: 6
      add: 0
    average: 7
```

##### Petrifying gaze

<!-- splat:yaml path=splat_rule/parameters/features/petrifying_gaze -->

```yaml
economy: bonus_action
op: saving_throw
ability: constitution
dc: 12
targets:
  kind: creature
  count: all
  area:
    shape: cone
    origin: actor
    size_ft: 30
failure:
- op: condition.apply
  condition: restrained
- op: saving_throw.repeat
  event: target.next_turn_end
  only_if_condition: restrained
  ability: constitution
  dc: 12
  success:
  - op: effect.end
  failure:
  - op: condition.remove
    condition: restrained
  - op: condition.apply
    condition: petrified
success: []
include_actor_if_sees_own_reflection_in_area: true
recharge:
  die_sides: 6
  success_min: 4
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
```

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
