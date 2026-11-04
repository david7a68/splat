---
splat_schema: 2
splat_id: 0756754f-1ab7-52fc-9e78-b5d5636dca96
splat_kind: creature
splat_key: dnd2024:creatures/blue_dragon_wyrmling
splat_name: Blue Dragon Wyrmling
splat_origin: dnd2024:reference/creature/blue_dragon_wyrmling
splat_sources:
- SRD 5.2.1, pp. 266, https://www.dndbeyond.com/srd
---

# Blue Dragon Wyrmling


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Dragon (Chromatic) |
| Alignment | Lawful Evil |
| Armor Class | 17 |
| Hit Points | 65 (10d8 + 20) |
| Initiative | +2 (12) |
| Speed | 30 ft., Burrow 15 ft., Fly 60 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Skills | Perception +4, Stealth +2 |
| Senses | Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 |
| Languages | Draconic |
| Immunities | Lightning |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 10 | +0 | +2 |
| Con | 15 | +2 | +2 |
| Int | 12 | +1 | +1 |
| Wis | 11 | +0 | +2 |
| Cha | 15 | +2 | +2 |


## Source text

<!-- SRD page 266 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Dragon (Chromatic), Lawful Evil\
> AC 17 Initiative +2 (12)\
> HP 65 (10d8 + 20)\
> Speed 30 ft., Burrow 15 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 10 +0 +2 Con 15 +2 +2\
> Int 12 +1 +1 Wis 11 +0 +2 Cha 15 +2 +2\
> Skills Perception +4, Stealth +2\
> Immunities Lightning\
> Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14\
> Languages Draconic\
> CR 3 (XP 700; PB +2)


### Actions

Multiattack. The dragon makes two Rend attacks.

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 8 (1d10 + 3) Slashing damage plus 3 (1d6) Lightning damage.

Lightning Breath (Recharge 5–6). Dexterity Saving Throw: DC 12, each creature in a 30-foot-long, 5-foot-wide Line. Failure: 21 (6d6) Lightning damage. Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/266|page 266]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/blue_dragon_wyrmling ` |
| ` implementation ` | ` dnd2024:creature_blue_dragon_wyrmling ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["lightning"] ` |

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: rend
  count: 2
```

##### Rend

<!-- splat:values path=splat_rule/parameters/features/rend -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/rend/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 5
reach_ft: 5
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/rend/hit/damage
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
| ` row_1 ` | ` 8 ` | ` 1 ` | ` 10 ` | ` 3 ` | ` Slashing ` |
| ` row_2 ` | ` 3 ` | ` 1 ` | ` 6 ` | ` 0 ` | ` Lightning ` |

##### Lightning breath

<!-- splat:yaml path=splat_rule/parameters/features/lightning_breath -->

```yaml
economy: action
resource:
  kind: recharge
  die: 6
  success_faces:
  - 5
  - 6
  check: self.turn_start
  rest_recovery:
  - short
  - long
  capacity: 1
  initial_uses: 1
operation: saving_throw
ability: dexterity
dc: 12
targets:
  kind: creature
  selection: each
  area:
    shape: line
    size_ft: 30
    origin: self
    width_ft: 5
failure:
  damage:
  - average: 21
    roll:
      count: 6
      sides: 6
      modifier: 0
    type: lightning
success:
  damage_multiplier: 0.5
  other_effects: false
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |
| ` stealth ` | ` 2 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 10 ` |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 14 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- draconic
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
> fly_speed:
>   unit: ft
>   value: 60
> burrow_speed:
>   unit: ft
>   value: 15
> ```

