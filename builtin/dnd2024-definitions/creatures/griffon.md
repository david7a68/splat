---
splat_schema: 2
splat_id: f3b29089-44cc-5d81-b254-7e8304b98d9e
splat_kind: creature
splat_key: dnd2024:creatures/griffon
splat_name: Griffon
splat_origin: dnd2024:reference/creature/griffon
splat_sources:
- SRD 5.2.1, pp. 295, https://www.dndbeyond.com/srd
---

# Griffon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 59 (7d10 + 21) |
| Initiative | +2 (12) |
| Speed | 30 ft., Fly 80 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Perception +5 |
| Senses | Darkvision 60 ft.; Passive Perception 15 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 15 | +2 | +2 |
| Con | 16 | +3 | +3 |
| Int | 2 | -4 | -4 |
| Wis | 13 | +1 | +1 |
| Cha | 8 | -1 | -1 |


## Source text

<!-- SRD page 295 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Monstrosity, Unaligned\
> AC 12 Initiative +2 (12)\
> HP 59 (7d10 + 21)\
> Speed 30 ft., Fly 80 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3\
> Int 2 −4 −4 Wis 13 +1 +1 Cha 8 −1 −1\
> Skills Perception +5\
> Senses Darkvision 60 ft.; Passive Perception 15\
> Languages None\
> CR 2 (XP 450; PB +2)


### Actions

Multiattack. The griffon makes two Rend attacks.

Rend. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (1d8 + 4) Piercing damage. If the target is a Medium or smaller creature, it has the Grappled condition (escape DC 14) from both of the griffon’s front claws.

Original layout: [[../../srd-5.2.1/pages/295|page 295]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/griffon ` |
| ` implementation ` | ` dnd2024:creature_griffon ` |

### Parameters

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
  - average: 8
    roll:
      count: 1
      sides: 8
      modifier: 4
    type: piercing
  grapple:
    condition: grappled
    source: self
    escape_dc: 14
    maximum_size: medium
    appendage_pool:
      count: 2
      type: claws
      cost: 2
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
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
>   value: 80
> ```

