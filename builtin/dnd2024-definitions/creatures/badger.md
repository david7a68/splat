---
splat_schema: 2
splat_id: 883a37d4-0528-5c57-a94c-cf5f8e56eb9d
splat_kind: creature
splat_key: dnd2024:creatures/badger
splat_name: Badger
splat_origin: dnd2024:reference/creature/badger
splat_sources:
- SRD 5.2.1, pp. 345, https://www.dndbeyond.com/srd
---

# Badger


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 5 (1d4 + 3) |
| Initiative | +0 (10) |
| Speed | 20 ft., Burrow 5 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Skills | Perception +3 |
| Senses | Darkvision 30 ft.; Passive Perception 13 |
| Languages | None |
| Resistances | Poison |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 10 | +0 | +0 |
| Dex | 11 | +0 | +0 |
| Con | 16 | +3 | +3 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 345 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Beast, Unaligned\
> AC 11  Initiative +0 (10)\
> HP 5 (1d4 + 3)\
> Speed 20 ft., Burrow 5 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 10 +0 +0 Dex 11 +0 +0 Con 16 +3 +3\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
> Skills Perception +3\
> Resistances Poison\
> Senses Darkvision 30 ft.; Passive Perception 13\
> Languages None\
> CR 0 (XP 10; PB +2)


### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing damage.

Original layout: [[../../srd-5.2.1/pages/345|page 345]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/badger ` |
| ` implementation ` | ` dnd2024:creature_badger ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["poison"] ` |

#### Features

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
  supplied_attack_total: 2
  reach_ft: 5
hit:
  damage:
  - average: 1
    roll:
      constant: 1
    type: piercing
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 30 ` |
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
> burrow_speed:
>   unit: ft
>   value: 5
> ```

