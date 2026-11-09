---
splat_schema: 2
splat_id: 7c7d3bae-a1a3-5189-a524-3966385eced8
splat_kind: creature
splat_key: dnd2024:creatures/shambling_mound
splat_name: Shambling Mound
splat_origin: dnd2024:reference/creature/shambling_mound
splat_sources:
- SRD 5.2.1, pp. 323, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 276, Habitat
---

# Shambling Mound


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Plant |
| Alignment | Unaligned |
| Armor Class | 15 |
| Hit Points | 110 (13d10 + 39) |
| Initiative | -1 (9) |
| Speed | 30 ft., Swim 20 ft. |
| Challenge | 5 (XP 1,800; PB +3) |
| Skills | Stealth +3 |
| Senses | Blindsight 60 ft.; Passive Perception 10 |
| Languages | None |
| Resistances | Cold, Fire |
| Immunities | Lightning; Deafened, Exhaustion |
| Habitat | Forest, Swamp |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 8 | -1 | -1 |
| Con | 16 | +3 | +3 |
| Int | 5 | -3 | -3 |
| Wis | 10 | +0 | +0 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 323 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Plant, Unaligned\
> AC 15  Initiative −1 (9)\
> HP 110 (13d10 + 39)\
> Speed 30 ft., Swim 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 8 −1 −1 Con 16 +3 +3\
> Int 5 −3 −3 Wis 10 +0 +0 Cha 5 −3 −3\
> Skills Stealth +3\
> Resistances Cold, Fire\
> Immunities Lightning; Deafened, Exhaustion\
> Senses Blindsight 60 ft.; Passive Perception 10\
> Languages None\
> CR 5 (XP 1,800; PB +3)


### Traits

Lightning Absorption. Whenever the shambling mound is subjected to Lightning damage, it regains a number of Hit Points equal to the Lightning damage dealt.

### Actions

Multiattack. The shambling mound makes three Charged Tendril attacks. It can replace one attack with a use of Engulf.

Charged Tendril. Melee Attack Roll: +7, reach 10 ft. Hit: 7 (1d6 + 4) Bludgeoning damage plus 5 (2d4) Lightning damage. If the target is a Medium or smaller creature, the shambling mound pulls the target 5 feet straight toward itself.

Engulf. Strength Saving Throw: DC 15, one Medium or smaller creature within 5 feet. Failure: The target is pulled into the shambling mound’s space and has the Grappled condition (escape DC 14). Until the grapple ends, the target has the Blinded and Restrained conditions, and it takes 10 (3d6) Lightning damage at the start of each of its turns. When the shambling mound moves, the Grappled target moves with it, costing it no extra movement. The shambling mound can have only one creature Grappled by this action at a time.

Original layout: [[../../srd-5.2.1/pages/323|page 323]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/shambling_mound ` |
| ` implementation ` | ` dnd2024:creature_shambling_mound ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold", "fire"] ` |
| ` damage_immunities ` | ` ["lightning"] ` |
| ` condition_immunities ` | ` ["deafened", "exhaustion"] ` |

#### Features

##### Lightning absorption

<!-- splat:yaml path=splat_rule/parameters/features/lightning_absorption -->

```yaml
economy: trait
operation: passive_rule
trigger: self.subjected_to_lightning_damage
heal:
  equal: lightning_damage_dealt
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: charged_tendril
  count: 3
replacements:
- replace_count: 1
  maximum_uses: 1
  operation:
    operation: use_feature
    feature: engulf
```

##### Charged tendril

<!-- splat:values path=splat_rule/parameters/features/charged_tendril -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/charged_tendril/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 7
reach_ft: 10
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/charged_tendril/hit/damage
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
| ` row_1 ` | ` 7 ` | ` 1 ` | ` 6 ` | ` 4 ` | ` Bludgeoning ` |
| ` row_2 ` | ` 5 ` | ` 2 ` | ` 4 ` | ` 0 ` | ` Lightning ` |

###### Pull

<!-- splat:values path=splat_rule/parameters/features/charged_tendril/hit/pull -->

| Field | Value |
| --- | --- |
| ` maximum_ft ` | ` 5 ` |
| ` direction ` | ` straight_toward_self ` |
| ` maximum_size ` | ` medium ` |

##### Engulf

<!-- splat:yaml path=splat_rule/parameters/features/engulf -->

```yaml
economy: action
operation: saving_throw
ability: strength
dc: 15
targets:
  kind: creature
  count: 1
  range_ft: 5
  maximum_size: medium
failure:
  move_target_to: self.space
  grapple:
    condition: grappled
    source: self
    escape_dc: 14
    while_grappled:
      conditions:
      - blinded
      - restrained
      damage_each_target_turn_start:
      - roll:
          count: 3
          sides: 6
          modifier: 0
        type: lightning
      moves_with_self: true
      movement_extra_cost: 0
capacity: 1
success:
  effects: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 3 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 60 ` |
| ` passive_perception ` | ` 10 ` |

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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> swim_speed:
>   unit: ft
>   value: 20
> ```

