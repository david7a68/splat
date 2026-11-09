---
splat_schema: 2
splat_id: 3fbef6af-bb6e-51d8-8bf2-9bd632abf451
splat_kind: creature
splat_key: dnd2024:creatures/axe_beak
splat_name: Axe Beak
splat_origin: dnd2024:reference/creature/axe_beak
splat_sources:
- SRD 5.2.1, pp. 260, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 24, Habitat
---

# Axe Beak


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 19 (3d10 + 3) |
| Initiative | +1 (11) |
| Speed | 50 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Senses | Passive Perception 10 |
| Languages | None |
| Habitat | Arctic, Grassland, Hill |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +2 |
| Dex | 12 | +1 | +1 |
| Con | 12 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 10 | +0 | +0 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 260 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Monstrosity, Unaligned\
> AC 11 Initiative +1 (11)\
> HP 19 (3d10 + 3)\
> Speed 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 14 +2 +2 Dex 12 +1 +1 Con 12 +1 +1\
> Int 2 −4 −4 Wis 10 +0 +0 Cha 5 −3 −3\
> Senses Passive Perception 10\
> Languages None\
> CR 1/4 (XP 50; PB +2)


### Actions

Beak. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/260|page 260]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/axe_beak ` |
| ` implementation ` | ` dnd2024:creature_axe_beak ` |

### Parameters

#### Features

##### Beak

<!-- splat:yaml path=splat_rule/parameters/features/beak -->

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
