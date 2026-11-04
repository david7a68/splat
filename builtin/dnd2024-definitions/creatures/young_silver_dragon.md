---
splat_schema: 2
splat_id: 461b291f-a76e-5371-97d7-09bcfdf5fbf5
splat_kind: creature
splat_key: dnd2024:creatures/young_silver_dragon
splat_name: Young Silver Dragon
splat_origin: dnd2024:reference/creature/young_silver_dragon
splat_sources:
- SRD 5.2.1, pp. 324, https://www.dndbeyond.com/srd
---

# Young Silver Dragon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Dragon (Metallic) |
| Alignment | Lawful Good |
| Armor Class | 18 |
| Hit Points | 168 (16d10 + 80) |
| Initiative | +4 (14) |
| Speed | 40 ft., Fly 80 ft. |
| Challenge | 9 (XP 5,000; PB +4) |
| Skills | History +6, Perception +8, Stealth +4 |
| Senses | Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 18 |
| Languages | Common, Draconic |
| Immunities | Cold |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 23 | +6 | +6 |
| Dex | 10 | +0 | +4 |
| Con | 21 | +5 | +5 |
| Int | 14 | +2 | +2 |
| Wis | 11 | +0 | +4 |
| Cha | 19 | +4 | +4 |


## Source text

<!-- SRD page 324 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Dragon (Metallic), Lawful Good\
> AC 18 Initiative +4 (14)\
> HP 168 (16d10 + 80)\
> Speed 40 ft., Fly 80 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 23 +6 +6 Dex 10 +0 +4 Con 21 +5 +5\
> Int 14 +2 +2 Wis 11 +0 +4 Cha 19 +4 +4\
> Skills History +6, Perception +8, Stealth +4\
> Immunities Cold\
> Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 18\
> Languages Common, Draconic\
> CR 9 (XP 5,000; PB +4)


### Actions

Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of Paralyzing Breath.

Rend. Melee Attack Roll: +10, reach 10 ft. Hit: 15 (2d8 + 6) Slashing damage.

Cold Breath (Recharge 5–6). Constitution Saving Throw: DC 17, each creature in a 30-foot Cone. Failure: 49 (11d8) Cold damage. Success: Half damage.

Paralyzing Breath. Constitution Saving Throw: DC 17, each creature in a 30-foot Cone. First Failure: The target has the Incapacitated condition until the end of its next turn, when it repeats the save. Second Failure: The target has the Paralyzed condition, and it repeats the save at the end of each of its turns, ending the effect on itself on a success. After 1 minute, it succeeds automatically.

Original layout: [[../../srd-5.2.1/pages/324|page 324]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/young_silver_dragon ` |
| ` implementation ` | ` dnd2024:creature_young_silver_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["cold"] ` |

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: rend
  count: 3
replacements:
- replace_count: 1
  maximum_uses: 1
  operation:
    operation: use_feature
    feature: paralyzing_breath
```

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 10
  reach_ft: 10
hit:
  damage:
  - average: 15
    roll:
      count: 2
      sides: 8
      modifier: 6
    type: slashing
```

##### Cold breath

<!-- splat:yaml path=splat_rule/parameters/features/cold_breath -->

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
ability: constitution
dc: 17
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 30
    origin: self
failure:
  damage:
  - average: 49
    roll:
      count: 11
      sides: 8
      modifier: 0
    type: cold
success:
  damage_multiplier: 0.5
  other_effects: false
```

##### Paralyzing breath

<!-- splat:yaml path=splat_rule/parameters/features/paralyzing_breath -->

```yaml
economy: action
operation: saving_throw
ability: constitution
dc: 17
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 30
    origin: self
failure:
  damage: []
  conditions:
  - condition: incapacitated
    until: target.next_turn_end
  repeat_save:
    event: target.next_turn_end
    failure:
      conditions:
      - condition: paralyzed
      repeat_save:
        event: target.turn_end
        success: end_effect
      automatic_success_after_seconds: 60
      automatic_success_clock_origin: initial_failed_save
    success: end_effect
success:
  effects: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` history ` | ` 6 ` |
| ` perception ` | ` 8 ` |
| ` stealth ` | ` 4 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 30 ` |
| ` darkvision ` | ` 120 ` |
| ` passive_perception ` | ` 18 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
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
>   value: 80
> ```

