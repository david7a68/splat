---
splat_schema: 2
splat_id: 82451a28-9fdd-576c-a92f-0b4c706175ba
splat_kind: creature
splat_key: dnd2024:creatures/owl
splat_name: Owl
splat_origin: dnd2024:reference/creature/owl
splat_sources:
- SRD 5.2.1, pp. 358, https://www.dndbeyond.com/srd
---

# Owl


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 1 (1d4 - 1) |
| Initiative | +1 (11) |
| Speed | 5 ft., Fly 60 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Skills | Perception +5, Stealth +5 |
| Senses | Darkvision 120 ft.; Passive Perception 15 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 3 | -4 | -4 |
| Dex | 13 | +1 | +1 |
| Con | 8 | -1 | -1 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 358 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Beast, Unaligned\
> AC 11  Initiative +1 (11)\
> HP 1 (1d4 − 1)\
> Speed 5 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 3 −4 −4 Dex 13 +1 +1 Con 8 −1 −1\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
> Skills Perception +5, Stealth +5\
> Senses Darkvision 120 ft.; Passive Perception 15\
> Languages None\
> CR 0 (XP 10; PB +2)


### Traits

Flyby. The owl doesn’t provoke an Opportunity Attack when it flies out of an enemy’s reach.

### Actions

Talons. Melee Attack Roll: +3, reach 5 ft. Hit: 1 Slashing damage.

Original layout: [[../../srd-5.2.1/pages/358|page 358]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/owl ` |
| ` implementation ` | ` dnd2024:creature_owl ` |

### Parameters

#### Features

##### Flyby

<!-- splat:yaml path=splat_rule/parameters/features/flyby -->

```yaml
economy: trait
operation: passive_rule
opportunity_attacks: false
while:
  movement_mode: fly
```

##### Talons

<!-- splat:yaml path=splat_rule/parameters/features/talons -->

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
  - average: 1
    roll:
      constant: 1
    type: slashing
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |
| ` stealth ` | ` 5 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 120 ` |
| ` passive_perception ` | ` 15 ` |

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
>   value: 60
> ```

