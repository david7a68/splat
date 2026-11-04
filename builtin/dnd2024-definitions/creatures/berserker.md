---
splat_schema: 2
splat_id: 7cf98ec7-cf46-57a1-95ea-89be0fe74537
splat_kind: creature
splat_key: dnd2024:creatures/berserker
splat_name: Berserker
splat_origin: dnd2024:reference/creature/berserker
splat_sources:
- SRD 5.2.1, pp. 263, https://www.dndbeyond.com/srd
---

# Berserker


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Humanoid |
| Alignment | Neutral |
| Armor Class | 13 |
| Hit Points | 67 (9d8 + 27) |
| Initiative | +1 (11) |
| Speed | 30 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Senses | Passive Perception 10 |
| Languages | Common |
| Gear | Greataxe, Hide Armor |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 12 | +1 | +1 |
| Con | 17 | +3 | +3 |
| Int | 9 | -1 | -1 |
| Wis | 11 | +0 | +0 |
| Cha | 9 | -1 | -1 |


## Source text

<!-- SRD page 263 -->

Medium or Small Humanoid, Neutral\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 13 Initiative +1 (11)\
> HP 67 (9d8 + 27)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 12 +1 +1 Con 17 +3 +3\
> Int 9 −1 −1 Wis 11 +0 +0 Cha 9 −1 −1\
> Gear Greataxe, Hide Armor\
> Senses Passive Perception 10\
> Languages Common\
> CR 2 (XP 450; PB +2)


### Traits

Bloodied Frenzy. While Bloodied, the berserker has Advantage on attack rolls and saving throws.

### Actions

Greataxe. Melee Attack Roll: +5, reach 5 ft. Hit: 9 (1d12 + 3) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/263|page 263]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/berserker ` |
| ` implementation ` | ` dnd2024:creature_berserker ` |

### Parameters

#### Features

##### Bloodied frenzy

<!-- splat:yaml path=splat_rule/parameters/features/bloodied_frenzy -->

```yaml
economy: trait
operation: passive_rule
while:
  self_bloodied: true
advantage_on:
- attack_rolls
- saving_throws
```

##### Greataxe

<!-- splat:yaml path=splat_rule/parameters/features/greataxe -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  reach_ft: 5
hit:
  damage:
  - average: 9
    roll:
      count: 1
      sides: 12
      modifier: 3
    type: slashing
```

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Item:
    path: item
    type: symbol
  Quantity:
    path: quantity
    type: integer
-->

| Key | Item | Quantity |
| --- | --- | --- |
| ` row_1 ` | ` Greataxe ` | ` 1 ` |
| ` row_2 ` | ` Hide armor ` | ` 1 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
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
