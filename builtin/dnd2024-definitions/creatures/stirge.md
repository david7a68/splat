---
splat_schema: 2
splat_id: 27b3cb24-91c4-5726-baa2-73389c952cf6
splat_kind: creature
splat_key: dnd2024:creatures/stirge
splat_name: Stirge
splat_origin: dnd2024:reference/creature/stirge
splat_sources:
- SRD 5.2.1, pp. 329, 330, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 299, Habitat
---

# Stirge


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 5 (2d4) |
| Initiative | +3 (13) |
| Speed | 10 ft., Fly 40 ft. |
| Challenge | 1/8 (XP 25; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 9 |
| Languages | None |
| Habitat | Desert, Forest, Grassland, Hill, Mountain, Swamp, Underdark, Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 4 | -3 | -3 |
| Dex | 16 | +3 | +3 |
| Con | 11 | +0 | +0 |
| Int | 2 | -4 | -4 |
| Wis | 8 | -1 | -1 |
| Cha | 6 | -2 | -2 |


## Source text

<!-- SRD page 329 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Monstrosity, Unaligned\
> AC 13 Initiative +3 (13)\
> HP 5 (2d4)\
> Speed 10 ft., Fly 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 4 −3 −3 Dex 16 +3 +3 Con 11 +0 +0\
> Int 2 −4 −4 Wis 8 −1 −1 Cha 6 −2 −2\
> Senses Darkvision 60 ft.; Passive Perception 9\
> Languages None\
> CR 1/8 (XP 25; PB +2)


<!-- SRD page 330 -->

### Actions

Proboscis. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Piercing damage, and the stirge attaches to the target. While attached, the stirge can’t make Proboscis attacks, and the target takes 5 (2d4) Necrotic damage at the start of each of the stirge’s turns.

The stirge can detach itself by spending 5 feet of its movement. The target or a creature within 5 feet of it can detach the stirge as an action.

Original layout: [[../../srd-5.2.1/pages/329|page 329]], [[../../srd-5.2.1/pages/330|page 330]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/stirge ` |
| ` implementation ` | ` dnd2024:creature_stirge ` |

### Parameters

#### Features

##### Proboscis

<!-- splat:yaml path=splat_rule/parameters/features/proboscis -->

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
    type: piercing
  attach:
    target: hit_target
    while_attached:
      can_use_proboscis: false
      damage_each_self_turn_start:
      - roll:
          count: 2
          sides: 4
          modifier: 0
        type: necrotic
    detach_self:
      movement_cost_ft: 5
    detach_other:
      economy: action
      actor_any_of:
      - target
      - within_target_ft: 5
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 40
> ```

