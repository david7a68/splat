---
splat_schema: 2
splat_id: 5bc78e7c-531a-5d0e-8cb9-d4be2e2b6056
splat_kind: creature
splat_key: dnd2024:creatures/giant_constrictor_snake
splat_name: Giant Constrictor Snake
splat_origin: dnd2024:reference/creature/giant_constrictor_snake
splat_sources:
- SRD 5.2.1, pp. 350, https://www.dndbeyond.com/srd
---

# Giant Constrictor Snake


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 60 (8d12 + 8) |
| Initiative | +2 (12) |
| Speed | 30 ft., Swim 30 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Perception +2 |
| Senses | Blindsight 10 ft.; Passive Perception 12 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +4 |
| Dex | 14 | +2 | +2 |
| Con | 12 | +1 | +1 |
| Int | 1 | -5 | -5 |
| Wis | 10 | +0 | +0 |
| Cha | 3 | -4 | -4 |


## Source text

<!-- SRD page 350 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Beast, Unaligned\
> AC 12  Initiative +2 (12)\
> HP 60 (8d12 + 8)\
> Speed 30 ft., Swim 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 19 +4 +4 Dex 14 +2 +2 Con 12 +1 +1\
> Int 1 −5 −5 Wis 10 +0 +0 Cha 3 −4 −4\
> Skills Perception +2\
> Senses Blindsight 10 ft.; Passive Perception 12\
> Languages None\
> CR 2 (XP 450; PB +2)


### Actions

Multiattack. The snake makes one Bite attack and uses Constrict.

Bite. Melee Attack Roll: +6, reach 10 ft. Hit: 11 (2d6 + 4) Piercing damage.

Constrict. Strength Saving Throw: DC 14, one Large or smaller creature the snake can see within 10 feet. Failure: 13 (2d8 + 4) Bludgeoning damage, and the target has the Grappled condition (escape DC 14).

Original layout: [[../../srd-5.2.1/pages/350|page 350]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_constrictor_snake ` |
| ` implementation ` | ` dnd2024:creature_giant_constrictor_snake ` |

### Parameters

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: bite
  count: 1
- operation: use_feature
  feature: constrict
```

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
  supplied_attack_total: 6
  reach_ft: 10
hit:
  damage:
  - average: 11
    roll:
      count: 2
      sides: 6
      modifier: 4
    type: piercing
```

##### Constrict

<!-- splat:yaml path=splat_rule/parameters/features/constrict -->

```yaml
economy: action
operation: saving_throw
ability: strength
dc: 14
targets:
  kind: creature
  count: 1
  range_ft: 10
  visible: true
  maximum_size: large
failure:
  damage:
  - average: 13
    roll:
      count: 2
      sides: 8
      modifier: 4
    type: bludgeoning
  grapple:
    condition: grappled
    source: self
    escape_dc: 14
success:
  effects: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 2 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 10 ` |
| ` passive_perception ` | ` 12 ` |

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

