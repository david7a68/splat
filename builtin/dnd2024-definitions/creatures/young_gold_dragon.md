---
splat_schema: 2
splat_id: a3525aeb-3b3c-53c4-91a3-5d690a0171a4
splat_kind: creature
splat_key: dnd2024:creatures/young_gold_dragon
splat_name: Young Gold Dragon
splat_origin: dnd2024:reference/creature/young_gold_dragon
splat_sources:
- SRD 5.2.1, pp. 291, https://www.dndbeyond.com/srd
---

# Young Gold Dragon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Dragon (Metallic) |
| Alignment | Lawful Good |
| Armor Class | 18 |
| Hit Points | 178 (17d10 + 85) |
| Initiative | +6 (16) |
| Speed | 40 ft., Fly 80 ft., Swim 40 ft. |
| Challenge | 10 (XP 5,900; PB +4) |
| Skills | Insight +5, Perception +9, Persuasion +9, Stealth +6 |
| Senses | Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 19 |
| Languages | Common, Draconic |
| Immunities | Fire |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 23 | +6 | +6 |
| Dex | 14 | +2 | +6 |
| Con | 21 | +5 | +5 |
| Int | 16 | +3 | +3 |
| Wis | 13 | +1 | +5 |
| Cha | 20 | +5 | +5 |


## Source text

<!-- SRD page 291 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Dragon (Metallic), Lawful Good\
> AC 18 Initiative +6 (16)\
> HP 178 (17d10 + 85)\
> Speed 40 ft., Fly 80 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 23 +6 +6 Dex 14 +2 +6 Con 21 +5 +5\
> Int 16 +3 +3 Wis 13 +1 +5 Cha 20 +5 +5\
> Skills Insight +5, Perception +9, Persuasion +9, Stealth +6\
> Immunities Fire\
> Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 19\
> Languages Common, Draconic\
> CR 10 (XP 5,900; PB +4)


### Traits

Amphibious. The dragon can breathe air and water.

### Actions

Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of Weakening Breath.

Rend. Melee Attack Roll: +10, reach 10 ft. Hit: 17 (2d10 + 6) Slashing damage.

Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 17, each creature in a 30-foot Cone. Failure: 55 (10d10) Fire damage. Success: Half damage.

Weakening Breath. Strength Saving Throw: DC 17, each creature that isn’t currently affected by this breath in a 30-foot Cone. Failure: The target has Disadvantage on Strength-based D20 Tests and subtracts 3 (1d6) from its damage rolls. It repeats the save at the end of each of its turns, ending the effect on itself on a success. After 1 minute, it succeeds automatically.

Original layout: [[../../srd-5.2.1/pages/291|page 291]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/young_gold_dragon ` |
| ` implementation ` | ` dnd2024:creature_young_gold_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` breathing ` | ` ["air", "water"] ` |

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
    feature: weakening_breath
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
  - average: 17
    roll:
      count: 2
      sides: 10
      modifier: 6
    type: slashing
```

##### Fire breath

<!-- splat:yaml path=splat_rule/parameters/features/fire_breath -->

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
  - average: 55
    roll:
      count: 10
      sides: 10
      modifier: 0
    type: fire
success:
  damage_multiplier: 0.5
  other_effects: false
```

##### Weakening breath

<!-- splat:yaml path=splat_rule/parameters/features/weakening_breath -->

```yaml
economy: action
operation: saving_throw
ability: strength
dc: 17
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 30
    origin: self
  exclude_already_affected_by: this_breath
failure:
  disadvantage_on:
    roll: d20_test
    ability: strength
  damage_roll_modifier:
    subtract:
      count: 1
      sides: 6
      modifier: 0
  repeat_save:
    event: target.turn_end
    success: end_effect
  automatic_success_after_seconds: 60
success:
  effects: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` insight ` | ` 5 ` |
| ` perception ` | ` 9 ` |
| ` persuasion ` | ` 9 ` |
| ` stealth ` | ` 6 ` |

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
| ` passive_perception ` | ` 19 ` |

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
> swim_speed:
>   unit: ft
>   value: 40
> ```

