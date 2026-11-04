---
splat_schema: 2
splat_id: 7bd71c86-f783-5366-b46c-79f87f366625
splat_kind: creature
splat_key: dnd2024:creatures/killer_whale
splat_name: Killer Whale
splat_origin: dnd2024:reference/creature/killer_whale
splat_sources:
- SRD 5.2.1, pp. 356, https://www.dndbeyond.com/srd
---

# Killer Whale


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 90 (12d12 + 12) |
| Initiative | +2 (12) |
| Speed | 5 ft., Swim 60 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Skills | Perception +3, Stealth +4 |
| Senses | Blindsight 120 ft.; Passive Perception 13 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +4 |
| Dex | 14 | +2 | +2 |
| Con | 13 | +1 | +1 |
| Int | 3 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 356 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Beast, Unaligned\
> AC 12  Initiative +2 (12)\
> HP 90 (12d12 + 12)\
> Speed 5 ft., Swim 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 19 +4 +4 Dex 14 +2 +2 Con 13 +1 +1\
> Int 3 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
> Skills Perception +3, Stealth +4\
> Senses Blindsight 120 ft.; Passive Perception 13\
> Languages None\
> CR 3 (XP 700; PB +2)


### Traits

Hold Breath. The whale can hold its breath for 30 minutes.

### Actions

Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 21 (5d6 + 4) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/356|page 356]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/killer_whale ` |
| ` implementation ` | ` dnd2024:creature_killer_whale ` |

### Parameters

#### Features

##### Hold breath

<!-- splat:values path=splat_rule/parameters/features/hold_breath -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` hold_breath_seconds ` | ` 1800 ` |

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

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
  - average: 21
    roll:
      count: 5
      sides: 6
      modifier: 4
    type: piercing
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |
| ` stealth ` | ` 4 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 120 ` |
| ` passive_perception ` | ` 13 ` |

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
>   value: 60
> ```

