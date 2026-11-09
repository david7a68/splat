---
splat_schema: 2
splat_id: 90504663-a874-50e5-9d69-049c43447321
splat_kind: creature
splat_key: dnd2024:creatures/brass_dragon_wyrmling
splat_name: Brass Dragon Wyrmling
splat_origin: dnd2024:reference/creature/brass_dragon_wyrmling
splat_sources:
- SRD 5.2.1, pp. 268, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 54, Habitat
---

# Brass Dragon Wyrmling


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Dragon (Metallic) |
| Alignment | Chaotic Good |
| Armor Class | 15 |
| Hit Points | 22 (4d8 + 4) |
| Initiative | +2 (12) |
| Speed | 30 ft., Burrow 15 ft., Fly 60 ft. |
| Challenge | 1 (XP 200; PB +2) |
| Skills | Perception +4, Stealth +2 |
| Senses | Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 |
| Languages | Draconic |
| Immunities | Fire |
| Habitat | Desert |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +2 |
| Dex | 10 | +0 | +2 |
| Con | 13 | +1 | +1 |
| Int | 10 | +0 | +0 |
| Wis | 11 | +0 | +2 |
| Cha | 13 | +1 | +1 |


## Source text

<!-- SRD page 268 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Dragon (Metallic), Chaotic Good\
> AC 15 Initiative +2 (12)\
> HP 22 (4d8 + 4)\
> Speed 30 ft., Burrow 15 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 15 +2 +2 Dex 10 +0 +2 Con 13 +1 +1\
> Int 10 +0 +0 Wis 11 +0 +2 Cha 13 +1 +1\
> Skills Perception +4, Stealth +2\
> Immunities Fire\
> Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14\
> Languages Draconic\
> CR 1 (XP 200; PB +2)


### Actions

Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (1d10 + 2) Slashing damage.

Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 11, each creature in a 20-foot-long, 5-foot-wide Line. Failure: 14 (4d6) Fire damage. Success: Half damage.

Sleep Breath. Constitution Saving Throw: DC 11, each creature in a 15-foot Cone. Failure: The target has the Incapacitated condition until the end of its next turn, at which point it repeats the save. Second Failure: The target has the Unconscious condition for 1 minute. This effect ends for the target if it takes damage or a creature within 5 feet of it takes an action to wake it.

Original layout: [[../../srd-5.2.1/pages/268|page 268]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/brass_dragon_wyrmling ` |
| ` implementation ` | ` dnd2024:creature_brass_dragon_wyrmling ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |

#### Features

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
  supplied_attack_total: 4
  reach_ft: 5
hit:
  damage:
  - average: 7
    roll:
      count: 1
      sides: 10
      modifier: 2
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
dc: 11
targets:
  kind: creature
  selection: each
  area:
    shape: line
    size_ft: 20
    origin: self
    width_ft: 5
failure:
  damage:
  - average: 14
    roll:
      count: 4
      sides: 6
      modifier: 0
    type: fire
success:
  damage_multiplier: 0.5
  other_effects: false
```

##### Sleep breath

<!-- splat:yaml path=splat_rule/parameters/features/sleep_breath -->

```yaml
economy: action
operation: saving_throw
ability: constitution
dc: 11
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 15
    origin: self
failure:
  conditions:
  - condition: incapacitated
    until: target.next_turn_end
  repeat_save:
    event: target.next_turn_end
    failure:
      conditions:
      - condition: unconscious
        duration_seconds: 60
      ends_on:
      - event: target.damage_taken
      - event: creature.action
        operation: wake_target
        within_ft: 5
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
> burrow_speed:
>   unit: ft
>   value: 15
> ```

