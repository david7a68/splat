---
splat_schema: 2
splat_id: ba1b2ff5-3b5f-5f57-8ee9-517931cc2356
splat_kind: creature
splat_key: dnd2024:creatures/vrock
splat_name: Vrock
splat_origin: dnd2024:reference/creature/vrock
splat_sources:
- SRD 5.2.1, pp. 336, https://www.dndbeyond.com/srd
---

# Vrock


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Fiend (Demon) |
| Alignment | Chaotic Evil |
| Armor Class | 15 |
| Hit Points | 152 (16d10 + 64) |
| Initiative | +2 (12) |
| Speed | 40 ft., Fly 60 ft. |
| Challenge | 6 (XP 2,300; PB +3) |
| Senses | Darkvision 120 ft.; Passive Perception 11 |
| Languages | Abyssal; telepathy 120 ft. |
| Resistances | Cold, Fire, Lightning |
| Immunities | Poison; Poisoned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 15 | +2 | +5 |
| Con | 18 | +4 | +4 |
| Int | 8 | -1 | -1 |
| Wis | 13 | +1 | +4 |
| Cha | 8 | -1 | +2 |


## Source text

<!-- SRD page 336 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Fiend (Demon), Chaotic Evil\
> AC 15 Initiative +2 (12)\
> HP 152 (16d10 + 64)\
> Speed 40 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 15 +2 +5 Con 18 +4 +4\
> Int 8 −1 −1 Wis 13 +1 +4 Cha 8 −1 +2\
> Resistances Cold, Fire, Lightning\
> Immunities Poison; Poisoned\
> Senses Darkvision 120 ft.; Passive Perception 11\
> Languages Abyssal; telepathy 120 ft.\
> CR 6 (XP 2,300; PB +3)


### Traits

Demonic Restoration. If the vrock dies outside the Abyss, its body dissolves into ichor, and it gains a new body instantly, reviving with all its Hit Points somewhere in the Abyss.

Magic Resistance. The vrock has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The vrock makes two Shred attacks.

Shred. Melee Attack Roll: +6, reach 5 ft. Hit: 10 (2d6 + 3) Piercing damage plus 10 (3d6) Poison damage.

Spores (Recharge 6). Constitution Saving Throw: DC 15, each creature in a 20-foot Emanation originating from the vrock. Failure: The target has the Poisoned condition and repeats the save at the end of each of its turns, ending the effect on itself on a success. While Poisoned, the target takes 5 (1d10) Poison damage at the start of each of its turns. Emptying a flask of Holy Water on the target ends the effect early.

Stunning Screech (1/Day). Constitution Saving Throw: DC 15, each creature in a 20-foot Emanation originating from the vrock (demons succeed automatically). Failure: 10 (3d6) Thunder damage, and the target has the Stunned condition until the end of the vrock’s next turn.

Original layout: [[../../srd-5.2.1/pages/336|page 336]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/vrock ` |
| ` implementation ` | ` dnd2024:creature_vrock ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold", "fire", "lightning"] ` |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["poisoned"] ` |

#### Features

##### Demonic restoration

<!-- splat:yaml path=splat_rule/parameters/features/demonic_restoration -->

```yaml
economy: trait
operation: passive_rule
trigger: self.dies
unless:
  plane: abyss
remove_old_body: true
new_body:
  delay_seconds: 0
  plane: abyss
  hp: maximum
```

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` saving_throw_advantage_against ` | ` ["spells", "magical_effects"] ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: shred
  count: 2
```

##### Shred

<!-- splat:values path=splat_rule/parameters/features/shred -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/shred/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 6
reach_ft: 5
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/shred/hit/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Average:
    path: average
    type: integer
  Roll count:
    path: roll/count
    type: integer
  Roll sides:
    path: roll/sides
    type: integer
  Roll modifier:
    path: roll/modifier
    type: integer
  Type:
    path: type
    type: symbol
-->

| Key | Average | Roll count | Roll sides | Roll modifier | Type |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 10 ` | ` 2 ` | ` 6 ` | ` 3 ` | ` Piercing ` |
| ` row_2 ` | ` 10 ` | ` 3 ` | ` 6 ` | ` 0 ` | ` Poison ` |

##### Spores

<!-- splat:yaml path=splat_rule/parameters/features/spores -->

```yaml
economy: action
resource:
  kind: recharge
  die: 6
  success_faces:
  - 6
  check: self.turn_start
  rest_recovery:
  - short
  - long
  capacity: 1
  initial_uses: 1
operation: saving_throw
ability: constitution
dc: 15
targets:
  kind: creature
  selection: each
  area:
    shape: emanation
    size_ft: 20
    origin: self
failure:
  conditions:
  - condition: poisoned
  repeat_save:
    event: target.turn_end
    success: end_effect
  while_poisoned:
    damage_each_target_turn_start:
    - roll:
        count: 1
        sides: 10
        modifier: 0
      type: poison
  ends_on:
    operation: empty_holy_water_flask_on_target
success:
  effects: []
```

##### Stunning screech

<!-- splat:yaml path=splat_rule/parameters/features/stunning_screech -->

```yaml
economy: action
resource:
  kind: daily
  uses: 1
  recovery: long_rest
operation: saving_throw
ability: constitution
dc: 15
targets:
  kind: creature
  selection: each
  area:
    shape: emanation
    size_ft: 20
    origin: self
failure:
  damage:
  - average: 10
    roll:
      count: 3
      sides: 6
      modifier: 0
    type: thunder
  conditions:
  - condition: stunned
    until: self.next_turn_end
automatic_success_tags:
- demon
success:
  effects: []
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 120 ` |
| ` passive_perception ` | ` 11 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- abyssal
understands: []
telepathy_range_ft: 120
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 60
> ```

