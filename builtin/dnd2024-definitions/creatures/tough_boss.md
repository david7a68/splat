---
splat_schema: 2
splat_id: e046c72b-0474-5e7d-abb7-02703b4ac38c
splat_kind: creature
splat_key: dnd2024:creatures/tough_boss
splat_name: Tough Boss
splat_origin: dnd2024:reference/creature/tough_boss
splat_sources:
- SRD 5.2.1, pp. 332, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 307, Habitat
---

# Tough Boss


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Humanoid |
| Alignment | Neutral |
| Armor Class | 16 |
| Hit Points | 82 (11d8 + 33) |
| Initiative | +2 (12) |
| Speed | 30 ft. |
| Challenge | 4 (XP 1,100; PB +2) |
| Senses | Passive Perception 10 |
| Languages | Common plus one other language |
| Gear | Chain Mail, Heavy Crossbow, Warhammer |
| Habitat | Any |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +5 |
| Dex | 14 | +2 | +2 |
| Con | 16 | +3 | +5 |
| Int | 11 | +0 | +0 |
| Wis | 10 | +0 | +0 |
| Cha | 11 | +0 | +2 |


## Source text

<!-- SRD page 332 -->

Medium or Small Humanoid, Neutral\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 16  Initiative +2 (12)\
> HP 82 (11d8 + 33)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +5 Dex 14 +2 +2 Con 16 +3 +5\
> Int 11 +0 +0 Wis 10 +0 +0 Cha 11 +0 +2\
> Gear Chain Mail, Heavy Crossbow, Warhammer\
> Senses Passive Perception 10\
> Languages Common plus one other language\
> CR 4 (XP 1,100; PB +2)


### Traits

Pack Tactics. The tough has Advantage on an attack roll against a creature if at least one of the tough’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

### Actions

Multiattack. The tough makes two attacks, using Warhammer or Heavy Crossbow in any combination.

Warhammer. Melee Attack Roll: +5, reach 5 ft. Hit: 12 (2d8 + 3) Bludgeoning damage. If the target is a Large or smaller creature, the tough pushes the target up to 10 feet straight away from itself.

Heavy Crossbow. Ranged Attack Roll: +4, range 100/400 ft. Hit: 13 (2d10 + 2) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/332|page 332]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/tough_boss ` |
| ` implementation ` | ` dnd2024:creature_tough_boss ` |

### Parameters

#### Features

##### Pack tactics

<!-- splat:yaml path=splat_rule/parameters/features/pack_tactics -->

```yaml
economy: trait
operation: passive_rule
attack_advantage:
  target_kind: creature
  ally_within_target_ft: 5
  ally_excludes_conditions:
  - incapacitated
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_choice
  count: 2
  options:
  - warhammer
  - heavy_crossbow
```

##### Warhammer

<!-- splat:yaml path=splat_rule/parameters/features/warhammer -->

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
  - average: 12
    roll:
      count: 2
      sides: 8
      modifier: 3
    type: bludgeoning
  push:
    maximum_ft: 10
    direction: straight_away_from_self
    maximum_size: large
```

##### Heavy crossbow

<!-- splat:yaml path=splat_rule/parameters/features/heavy_crossbow -->

```yaml
economy: action
operation: attack
attack:
  mode: ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 4
  range_ft: 100
  long_range_ft: 400
hit:
  damage:
  - average: 13
    roll:
      count: 2
      sides: 10
      modifier: 2
    type: piercing
```

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
- row_3
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
| ` row_1 ` | ` Chain mail ` | ` 1 ` |
| ` row_2 ` | ` Heavy crossbow ` | ` 1 ` |
| ` row_3 ` | ` Warhammer ` | ` 1 ` |

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
additional_languages:
  count: 1
  chooser: creator
  can_speak: true
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
