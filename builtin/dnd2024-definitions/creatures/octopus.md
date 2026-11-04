---
splat_schema: 2
splat_id: e63a0a68-c054-5130-8733-4d0df87a3799
splat_kind: creature
splat_key: dnd2024:creatures/octopus
splat_name: Octopus
splat_origin: dnd2024:reference/creature/octopus
splat_sources:
- SRD 5.2.1, pp. 357, 358, https://www.dndbeyond.com/srd
---

# Octopus


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 3 (1d6) |
| Initiative | +2 (12) |
| Speed | 5 ft., Swim 30 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Skills | Perception +2, Stealth +6 |
| Senses | Darkvision 30 ft.; Passive Perception 12 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 4 | -3 | -3 |
| Dex | 15 | +2 | +2 |
| Con | 11 | +0 | +0 |
| Int | 3 | -4 | -4 |
| Wis | 10 | +0 | +0 |
| Cha | 4 | -3 | -3 |


## Source text

<!-- SRD page 357 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Beast, Unaligned\
> AC 12  Initiative +2 (12)\
> HP 3 (1d6)\
> Speed 5 ft., Swim 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str  4 −3 −3 Dex 15 +2 +2 Con 11 +0 +0\
> Int 3 −4 −4 Wis 10 +0 +0 Cha  4 −3 −3\
> Skills Perception +2, Stealth +6\
> Senses Darkvision 30 ft.; Passive Perception 12\
> Languages None\
> CR 0 (XP 10; PB +2)


<!-- SRD page 358 -->

### Traits

Compression. The octopus can move through a space as narrow as 1 inch without expending extra movement to do so.

Water Breathing. The octopus can breathe only underwater.

### Actions

Tentacles. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Bludgeoning damage.

### Reactions

Ink Cloud (1/Day). Trigger: A creature ends its turn within 5 feet of the octopus while underwater. Response: The octopus releases ink that fills a 5-foot Cube centered on itself, and the octopus moves up to its Swim Speed. The Cube is Heavily Obscured for 1 minute or until a strong current or similar effect disperses the ink.

Original layout: [[../../srd-5.2.1/pages/357|page 357]], [[../../srd-5.2.1/pages/358|page 358]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/octopus ` |
| ` implementation ` | ` dnd2024:creature_octopus ` |

### Parameters

#### Features

##### Compression

<!-- splat:values path=splat_rule/parameters/features/compression -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` minimum_gap_inches ` | ` 1 ` |
| ` extra_movement_cost ` | ` 0 ` |

##### Water breathing

<!-- splat:values path=splat_rule/parameters/features/water_breathing -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` breathing ` | ` ["water"] ` |

##### Tentacles

<!-- splat:yaml path=splat_rule/parameters/features/tentacles -->

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
  - average: 1
    roll:
      constant: 1
    type: bludgeoning
```

##### Ink cloud

<!-- splat:yaml path=splat_rule/parameters/features/ink_cloud -->

```yaml
economy: reaction
resource:
  kind: daily
  uses: 1
  recovery: long_rest
operation: sequence
trigger: creature.turn_end
requires:
  self_underwater: true
  triggering_creature_within_ft: 5
effects:
- operation: create_area
  area:
    shape: cube
    size_ft: 5
    center: self
  obscurement: heavy
  duration_seconds: 60
  ends_on:
  - strong_current
  - similar_dispersal_effect
- operation: move
  maximum_ft:
    multiply:
    - stat: swim_speed
    - 1
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 2 ` |
| ` stealth ` | ` 6 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 30 ` |
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

