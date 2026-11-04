---
splat_schema: 2
splat_id: c9f591c5-9263-50d8-89c0-8fbe20db6c14
splat_kind: creature
splat_key: dnd2024:creatures/crocodile
splat_name: Crocodile
splat_origin: dnd2024:reference/creature/crocodile
splat_sources:
- SRD 5.2.1, pp. 347, https://www.dndbeyond.com/srd
---

# Crocodile


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 13 (2d10 + 2) |
| Initiative | +0 (10) |
| Speed | 20 ft., Swim 30 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Skills | Stealth +2 |
| Senses | Passive Perception 10 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +2 |
| Dex | 10 | +0 | +0 |
| Con | 13 | +1 | +3 |
| Int | 2 | -4 | -4 |
| Wis | 10 | +0 | +0 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 347 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 12  Initiative +0 (10)\
> HP 13 (2d10 + 2)\
> Speed 20 ft., Swim 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 15 +2 +2 Dex 10 +0 +0 Con 13 +1 +3\
> Int 2 −4 −4 Wis 10 +0 +0 Cha 5 −3 −3\
> Skills Stealth +2\
> Senses Passive Perception 10\
> Languages None\
> CR 1/2 (XP 100; PB +2)


### Traits

Hold Breath. The crocodile can hold its breath for 1 hour.

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Piercing damage. If the target is a Medium or smaller creature, it has the Grappled condition (escape DC 12). While Grappled, the target has the Restrained condition.

Original layout: [[../../srd-5.2.1/pages/347|page 347]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/crocodile ` |
| ` implementation ` | ` dnd2024:creature_crocodile ` |

### Parameters

#### Features

##### Hold breath

<!-- splat:values path=splat_rule/parameters/features/hold_breath -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` hold_breath_seconds ` | ` 3600 ` |

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
  supplied_attack_total: 4
  reach_ft: 5
hit:
  damage:
  - average: 6
    roll:
      count: 1
      sides: 8
      modifier: 2
    type: piercing
  grapple:
    condition: grappled
    source: self
    escape_dc: 12
    maximum_size: medium
    while_grappled:
      conditions:
      - restrained
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 2 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

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
>   value: 30
> ```

