---
splat_schema: 2
splat_id: d0494afe-effc-54d5-8b6b-4d9ef95ba0df
splat_kind: creature
splat_key: dnd2024:creatures/satyr
splat_name: Satyr
splat_origin: dnd2024:reference/creature/satyr
splat_sources:
- SRD 5.2.1, pp. 321, 322, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 268, Habitat
---

# Satyr


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Fey |
| Alignment | Chaotic Neutral |
| Armor Class | 13 |
| Hit Points | 31 (7d8) |
| Initiative | +3 (13) |
| Speed | 40 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Skills | Perception +2, Performance +6, Stealth +5 |
| Senses | Passive Perception 12 |
| Languages | Common, Elvish, Sylvan |
| Habitat | Forest, Planar (Feywild) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 12 | +1 | +1 |
| Dex | 16 | +3 | +3 |
| Con | 11 | +0 | +0 |
| Int | 12 | +1 | +1 |
| Wis | 10 | +0 | +0 |
| Cha | 14 | +2 | +2 |


## Source text

<!-- SRD page 321 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Fey, Chaotic Neutral\
> AC 13 Initiative +3 (13)\
> HP 31 (7d8)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 12 +1 +1 Dex 16 +3 +3 Con 11 +0 +0\
> Int 12 +1 +1 Wis 10 +0 +0 Cha 14 +2 +2\
> Skills Perception +2, Performance +6, Stealth +5\
> Senses Passive Perception 12\
> Languages Common, Elvish, Sylvan\
> CR 1/2 (XP 100; PB +2)


<!-- SRD page 322 -->

### Traits

Magic Resistance. The satyr has Advantage on saving throws against spells and other magical effects.

### Actions

Hooves. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3) Bludgeoning damage. If the target is a Medium or smaller creature, the satyr pushes the target up to 10 feet straight away from itself.

Mockery. Wisdom Saving Throw: DC 12, one creature the satyr can see within 90 feet. Failure: 5 (1d6 + 2) Psychic damage.

Original layout: [[../../srd-5.2.1/pages/321|page 321]], [[../../srd-5.2.1/pages/322|page 322]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/satyr ` |
| ` implementation ` | ` dnd2024:creature_satyr ` |

### Parameters

#### Features

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` saving_throw_advantage_against ` | ` ["spells", "magical_effects"] ` |

##### Hooves

<!-- splat:yaml path=splat_rule/parameters/features/hooves -->

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
  - average: 5
    roll:
      count: 1
      sides: 4
      modifier: 3
    type: bludgeoning
  push:
    maximum_ft: 10
    direction: straight_away_from_self
    maximum_size: medium
```

##### Mockery

<!-- splat:yaml path=splat_rule/parameters/features/mockery -->

```yaml
economy: action
operation: saving_throw
ability: wisdom
dc: 12
targets:
  kind: creature
  count: 1
  range_ft: 90
  visible: true
failure:
  damage:
  - average: 5
    roll:
      count: 1
      sides: 6
      modifier: 2
    type: psychic
success:
  effects: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 2 ` |
| ` performance ` | ` 6 ` |
| ` stealth ` | ` 5 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
- elvish
- sylvan
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
