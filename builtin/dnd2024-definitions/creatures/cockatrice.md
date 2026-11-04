---
splat_schema: 2
splat_id: 3f631fda-bb06-535a-8dd9-bb98f9c56127
splat_kind: creature
splat_key: dnd2024:creatures/cockatrice
splat_name: Cockatrice
splat_origin: dnd2024:reference/creature/cockatrice
splat_sources:
- SRD 5.2.1, pp. 275, https://www.dndbeyond.com/srd
---

# Cockatrice


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 22 (5d6 + 5) |
| Initiative | +1 (11) |
| Speed | 20 ft., Fly 40 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 11 |
| Languages | None |
| Immunities | Petrified |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 6 | -2 | -2 |
| Dex | 12 | +1 | +1 |
| Con | 12 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 13 | +1 | +1 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 275 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Monstrosity, Unaligned\
> AC 11 Initiative +1 (11)\
> HP 22 (5d6 + 5)\
> Speed 20 ft., Fly 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 6 −2 −2 Dex 12 +1 +1 Con 12 +1 +1\
> Int 2 −4 −4 Wis 13 +1 +1 Cha 5 −3 −3\
> Immunities Petrified\
> Senses Darkvision 60 ft.; Passive Perception 11\
> Languages None\
> CR 1/2 (XP 100; PB +2)


### Actions

Petrifying Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 3 (1d4 + 1) Piercing damage. If the target is a creature, it is subjected to the following effect. Constitution Saving Throw: DC 11. First Failure: The target has the Restrained condition. The target repeats the save at the end of its next turn if it is still Restrained, ending the effect on itself on a success. Second Failure: The target has the Petrified condition, instead of the Restrained condition, for 24 hours.

Original layout: [[../../srd-5.2.1/pages/275|page 275]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/cockatrice ` |
| ` implementation ` | ` dnd2024:creature_cockatrice ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition_immunities ` | ` ["petrified"] ` |

#### Features

##### Petrifying bite

<!-- splat:yaml path=splat_rule/parameters/features/petrifying_bite -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 3
  reach_ft: 5
hit:
  damage:
  - average: 3
    roll:
      count: 1
      sides: 4
      modifier: 1
    type: piercing
  save:
    operation: saving_throw
    ability: constitution
    dc: 11
    targets:
      kind: creature
    failure:
      conditions:
      - condition: restrained
      repeat_save:
        event: target.next_turn_end
        if_still_restrained: true
        success: end_effect
        failure:
          replace_condition:
            from: restrained
            to: petrified
          duration_seconds: 86400
```

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 11 ` |

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
> fly_speed:
>   unit: ft
>   value: 40
> ```

