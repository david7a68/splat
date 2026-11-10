---
splat_schema: 2
splat_id: 5ec64fe8-06de-53db-b4e8-36cfd3017b8c
splat_kind: creature
splat_key: dnd2024:creatures/silver_dragon_wyrmling
splat_name: Silver Dragon Wyrmling
splat_origin: dnd2024:reference/creature/silver_dragon_wyrmling
splat_sources:
- SRD 5.2.1, pp. 324, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 278, Habitat
---

# Silver Dragon Wyrmling


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Dragon (Metallic) |
| Alignment | Lawful Good |
| Armor Class | 17 |
| Hit Points | 45 (6d8 + 18) |
| Initiative | +2 (12) |
| Speed | 30 ft., Fly 60 ft. |
| Challenge | 2 (450 XP; PB +2) |
| Skills | Perception +4, Stealth +2 |
| Senses | Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 |
| Languages | Draconic |
| Immunities | Cold |
| Habitat | Mountain, Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +4 |
| Dex | 10 | +0 | +2 |
| Con | 17 | +3 | +3 |
| Int | 12 | +1 | +1 |
| Wis | 11 | +0 | +2 |
| Cha | 15 | +2 | +2 |


## Source text

<!-- SRD page 324 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Dragon (Metallic), Lawful Good\
> AC 17 Initiative +2 (12)\
> HP 45 (6d8 + 18)\
> Speed 30 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 19 +4 +4 Dex 10 +0 +2 Con 17 +3 +3\
> Int 12 +1 +1 Wis 11 +0 +2 Cha 15 +2 +2\
> Skills Perception +4, Stealth +2\
> Immunities Cold\
> Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14\
> Languages Draconic\
> CR 2 (450 XP; PB +2)


### Actions

Multiattack. The dragon makes two Rend attacks.

Rend. Melee Attack Roll: +6, reach 5 ft. Hit: 9 (1d10 + 4) Piercing damage.

Cold Breath (Recharge 5–6). Constitution Saving Throw: DC 13, each creature in a 15-foot Cone. Failure: 18 (4d8) Cold damage. Success: Half damage.

Paralyzing Breath. Constitution Saving Throw: DC 13, each creature in a 15-foot Cone. First Failure: The target has the Incapacitated condition until the end of its next turn, when it repeats the save. Second Failure: The target has the Paralyzed condition, and it repeats the save at the end of each of its turns, ending the effect on itself on a success. After 1 minute, it succeeds automatically.

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
| ` mechanic ` | ` dnd2024:creatures/silver_dragon_wyrmling ` |
| ` implementation ` | ` dnd2024:creature_silver_dragon_wyrmling ` |

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
  count: 2
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
  supplied_attack_total: 6
  reach_ft: 5
hit:
  damage:
  - average: 9
    roll:
      count: 1
      sides: 10
      modifier: 4
    type: piercing
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
dc: 13
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 15
    origin: self
failure:
  damage:
  - average: 18
    roll:
      count: 4
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
dc: 13
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 15
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
> ```

