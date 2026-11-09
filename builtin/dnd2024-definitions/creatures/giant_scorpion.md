---
splat_schema: 2
splat_id: e7f52e82-0918-5d23-8141-f95aaa5462ef
splat_kind: creature
splat_key: dnd2024:creatures/giant_scorpion
splat_name: Giant Scorpion
splat_origin: dnd2024:reference/creature/giant_scorpion
splat_sources:
- SRD 5.2.1, pp. 353, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 359, Habitat
---

# Giant Scorpion


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 15 |
| Hit Points | 52 (7d10 + 14) |
| Initiative | +1 (11) |
| Speed | 40 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Senses | Blindsight 60 ft.; Passive Perception 9 |
| Languages | None |
| Habitat | Desert |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 13 | +1 | +1 |
| Con | 15 | +2 | +2 |
| Int | 1 | -5 | -5 |
| Wis | 9 | -1 | -1 |
| Cha | 3 | -4 | -4 |


## Source text

<!-- SRD page 353 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 15  Initiative +1 (11)\
> HP 52 (7d10 + 14)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 13 +1 +1 Con 15 +2 +2\
> Int 1 −5 −5 Wis 9 −1 −1 Cha 3 −4 −4\
> Senses Blindsight 60 ft.; Passive Perception 9\
> Languages None\
> CR 3 (XP 700; PB +2)


### Actions

Multiattack. The scorpion makes two Claw attacks and one Sting attack.

Claw. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Bludgeoning damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 13) from one of two claws.

Sting. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Piercing damage plus 11 (2d10) Poison damage.

Original layout: [[../../srd-5.2.1/pages/353|page 353]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_scorpion ` |
| ` implementation ` | ` dnd2024:creature_giant_scorpion ` |

### Parameters

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: claw
  count: 2
- operation: attack_sequence
  feature: sting
  count: 1
```

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

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
  - average: 6
    roll:
      count: 1
      sides: 6
      modifier: 3
    type: bludgeoning
  grapple:
    condition: grappled
    source: self
    escape_dc: 13
    maximum_size: large
    appendage_pool:
      count: 2
      type: claws
      cost: 1
```

##### Sting

<!-- splat:values path=splat_rule/parameters/features/sting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/sting/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 5
reach_ft: 5
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/sting/hit/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Average:
    path: average
    type: integer
  Roll count:
    path: roll/count
    type: integer
  Roll sides:
    path: roll/sides
    type: integer
  Roll modifier:
    path: roll/modifier
    type: integer
  Type:
    path: type
    type: symbol
-->

| Key | Average | Roll count | Roll sides | Roll modifier | Type |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 7 ` | ` 1 ` | ` 8 ` | ` 3 ` | ` Piercing ` |
| ` row_2 ` | ` 11 ` | ` 2 ` | ` 10 ` | ` 0 ` | ` Poison ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 60 ` |
| ` passive_perception ` | ` 9 ` |

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
