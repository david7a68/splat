---
splat_schema: 2
splat_id: 45736a8e-2b24-57d7-ba5a-3551a26cfee1
splat_kind: creature
splat_key: dnd2024:creatures/remorhaz
splat_name: Remorhaz
splat_origin: dnd2024:reference/creature/remorhaz
splat_sources:
- SRD 5.2.1, pp. 319, 320, https://www.dndbeyond.com/srd
---

# Remorhaz


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 17 |
| Hit Points | 195 (17d12 + 85) |
| Initiative | +5 (15) |
| Speed | 40 ft., Burrow 30 ft. |
| Challenge | 11 (XP 7,200; PB +4) |
| Senses | Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 10 |
| Languages | None |
| Immunities | Cold, Fire |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 24 | +7 | +7 |
| Dex | 13 | +1 | +1 |
| Con | 21 | +5 | +5 |
| Int | 4 | -3 | -3 |
| Wis | 10 | +0 | +0 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 319 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Monstrosity, Unaligned\
> AC 17 Initiative +5 (15)\
> HP 195 (17d12 + 85)\
> Speed 40 ft., Burrow 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 24 +7 +7 Dex 13 +1 +1 Con 21 +5 +5\
> Int 4 −3 −3 Wis 10 +0 +0 Cha 5 −3 −3\
> Immunities Cold, Fire\
> Senses Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 10\
> Languages None\
> CR 11 (XP 7,200; PB +4)


### Traits

Heat Aura. At the end of each of the remorhaz’s turns, each creature in a 5-foot Emanation originating from the remorhaz takes 16 (3d10) Fire damage.

### Actions

Bite. Melee Attack Roll: +11, reach 10 ft. Hit: 18 (2d10 + 7) Piercing damage plus 14 (4d6) Fire damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 17), and it has the Restrained condition until the grapple ends.

### Bonus Actions

Swallow. Strength Saving Throw: DC 19, one Large or smaller creature Grappled by the remorhaz (it can have up to two creatures swallowed at a time). Failure: The target is swallowed by the remorhaz, and the Grappled condition ends. A swallowed creature has the Blinded and Restrained conditions, it has Total Cover against attacks and other effects outside the remorhaz, and it takes 10 (3d6) Acid damage plus 10 (3d6) Fire damage at the start of each of the remorhaz’s turns.

<!-- SRD page 320 -->

If the remorhaz takes 30 damage or more on a single turn from a creature inside it, the remorhaz must succeed on a DC 15 Constitution saving throw at the end of that turn or regurgitate all swallowed creatures, each of which falls in a space within 5 feet of the remorhaz and has the Prone condition. If the remorhaz dies, any swallowed creature no longer has the Restrained condition and can escape from the corpse by using 15 feet of movement, exiting Prone.

Original layout: [[../../srd-5.2.1/pages/319|page 319]], [[../../srd-5.2.1/pages/320|page 320]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/remorhaz ` |
| ` implementation ` | ` dnd2024:creature_remorhaz ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["cold", "fire"] ` |

#### Features

##### Heat aura

<!-- splat:yaml path=splat_rule/parameters/features/heat_aura -->

```yaml
economy: trait
operation: passive_rule
trigger: self.turn_end
targets:
  kind: creature
  selection: each
  area:
    shape: emanation
    size_ft: 5
    origin: self
damage:
- roll:
    count: 3
    sides: 10
    modifier: 0
  type: fire
```

##### Bite

<!-- splat:values path=splat_rule/parameters/features/bite -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/bite/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 11
reach_ft: 10
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/bite/hit/damage
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
| ` row_1 ` | ` 18 ` | ` 2 ` | ` 10 ` | ` 7 ` | ` Piercing ` |
| ` row_2 ` | ` 14 ` | ` 4 ` | ` 6 ` | ` 0 ` | ` Fire ` |

###### Grapple

<!-- splat:yaml path=splat_rule/parameters/features/bite/hit/grapple -->

```yaml
condition: grappled
source: self
escape_dc: 17
maximum_size: large
while_grappled:
  conditions:
  - restrained
```

##### Swallow

<!-- splat:values path=splat_rule/parameters/features/swallow -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` operation ` | ` saving_throw ` |
| ` ability ` | ` strength ` |
| ` dc ` | ` 19 ` |
| ` capacity ` | ` 2 ` |

###### Failure

<!-- splat:values path=splat_rule/parameters/features/swallow/failure -->

| Field | Value |
| --- | --- |
| ` end_grapple ` | ` true ` |

###### Swallow

<!-- splat:values path=splat_rule/parameters/features/swallow/failure/swallow -->

| Field | Value |
| --- | --- |
| ` maximum_size ` | ` large ` |
| ` conditions ` | ` ["restrained", "blinded"] ` |
| ` cover_against_outside ` | ` total ` |

###### Periodic damage

<!-- splat:values path=splat_rule/parameters/features/swallow/failure/swallow/periodic_damage -->

| Field | Value |
| --- | --- |
| ` event ` | ` self.turn_start ` |

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/swallow/failure/swallow/periodic_damage/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
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

| Key | Roll count | Roll sides | Roll modifier | Type |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` 3 ` | ` 6 ` | ` 0 ` | ` Acid ` |
| ` row_2 ` | ` 3 ` | ` 6 ` | ` 0 ` | ` Fire ` |

###### Regurgitate

<!-- splat:yaml path=splat_rule/parameters/features/swallow/failure/swallow/regurgitate -->

```yaml
trigger: turn_end
if_damage_on_single_turn_from_one_inside_creature_at_least: 30
save:
  ability: constitution
  dc: 15
  failure:
    eject: all
    within_ft: 5
    conditions:
    - prone
```

###### On self death

<!-- splat:values path=splat_rule/parameters/features/swallow/failure/swallow/on_self_death -->

| Field | Value |
| --- | --- |
| ` remove_conditions ` | ` ["restrained"] ` |
| ` escape_movement_ft ` | ` 15 ` |
| ` exit_condition ` | ` prone ` |

###### Targets

<!-- splat:values path=splat_rule/parameters/features/swallow/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` grappled_by ` | ` self ` |
| ` maximum_size ` | ` large ` |

###### Success

<!-- splat:yaml path=splat_rule/parameters/features/swallow/success -->

```yaml
effects: []
```

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` tremorsense ` | ` 60 ` |
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
> burrow_speed:
>   unit: ft
>   value: 30
> ```

