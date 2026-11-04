---
splat_schema: 2
splat_id: 7be357ec-e71b-54e7-974a-3d8acad27110
splat_kind: creature
splat_key: dnd2024:creatures/animated_flying_sword
splat_name: Animated Flying Sword
splat_origin: dnd2024:reference/creature/animated_flying_sword
splat_sources:
- SRD 5.2.1, pp. 259, https://www.dndbeyond.com/srd
---

# Animated Flying Sword


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Construct |
| Alignment | Unaligned |
| Armor Class | 17 |
| Hit Points | 14 (4d6) |
| Initiative | +4 (14) |
| Speed | 5 ft., Fly 50 ft. (hover) |
| Challenge | 1/4 (XP 50; PB +2) |
| Senses | Blindsight 60 ft.; Passive Perception 7 |
| Languages | None |
| Immunities | Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 12 | +1 | +1 |
| Dex | 15 | +2 | +4 |
| Con | 11 | +0 | +0 |
| Int | 1 | -5 | -5 |
| Wis | 5 | -3 | -3 |
| Cha | 1 | -5 | -5 |


## Source text

<!-- SRD page 259 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Construct, Unaligned\
> AC 17 Initiative +4 (14)\
> HP 14 (4d6)\
> Speed 5 ft., Fly 50 ft. (hover)\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 12 +1 +1 Dex 15 +2 +4 Con 11 +0 +0\
> Int 1 −5 −5 Wis 5 −3 −3 Cha 1 −5 −5\
> Immunities Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned\
> Senses Blindsight 60 ft.; Passive Perception 7\
> Languages None\
> CR 1/4 (XP 50; PB +2)


### Actions

Slash. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/259|page 259]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/animated_flying_sword ` |
| ` implementation ` | ` dnd2024:creature_animated_flying_sword ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison", "psychic"] ` |
| ` condition_immunities ` | ` ["charmed", "deafened", "exhaustion", "frightened", "paralyzed", "petrified", "poisoned"] ` |

#### Features

##### Slash

<!-- splat:yaml path=splat_rule/parameters/features/slash -->

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
  - average: 6
    roll:
      count: 1
      sides: 8
      modifier: 2
    type: slashing
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 60 ` |
| ` passive_perception ` | ` 7 ` |

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
>   value: 50
> ```

