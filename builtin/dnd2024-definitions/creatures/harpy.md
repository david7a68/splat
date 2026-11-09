---
splat_schema: 2
splat_id: 3ddc1ac6-e9a6-55e1-86b8-acebc64ab05e
splat_kind: creature
splat_key: dnd2024:creatures/harpy
splat_name: Harpy
splat_origin: dnd2024:reference/creature/harpy
splat_sources:
- SRD 5.2.1, pp. 297, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 164, Habitat
---

# Harpy


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Monstrosity |
| Alignment | Chaotic Evil |
| Armor Class | 11 |
| Hit Points | 38 (7d8 + 7) |
| Initiative | +1 (11) |
| Speed | 20 ft., Fly 40 ft. |
| Challenge | 1 (XP 200; PB +2) |
| Senses | Passive Perception 10 |
| Languages | Common |
| Habitat | Coastal, Forest, Hill, Mountain |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 12 | +1 | +1 |
| Dex | 13 | +1 | +1 |
| Con | 12 | +1 | +1 |
| Int | 7 | -2 | -2 |
| Wis | 10 | +0 | +0 |
| Cha | 13 | +1 | +1 |


## Source text

<!-- SRD page 297 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Monstrosity, Chaotic Evil\
> AC 11 Initiative +1 (11)\
> HP 38 (7d8 + 7)\
> Speed 20 ft., Fly 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 12 +1 +1 Dex 13 +1 +1 Con 12 +1 +1\
> Int 7 −2 −2 Wis 10 +0 +0 Cha 13 +1 +1\
> Senses Passive Perception 10\
> Languages Common\
> CR 1 (XP 200; PB +2)


### Actions

Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 6 (2d4 + 1) Slashing damage.

Luring Song. The harpy sings a magical melody, which lasts until the harpy’s Concentration ends on it. Wisdom Saving Throw: DC 11, each Humanoid and Giant in a 300-foot Emanation originating from the harpy when the song starts. Failure: The target has the Charmed condition until the song ends and repeats the save at the end of each of its turns. While Charmed, the target has the Incapacitated condition and ignores the Luring Song of other harpies. If the target is more than 5 feet from the harpy, the target moves on its turn toward the harpy by the most direct route, trying to get within 5 feet of the harpy. It doesn’t avoid Opportunity Attacks; however, before moving into damaging terrain (such as lava or a pit) and whenever it takes damage from a source other than the harpy, the target repeats the save. Success: The target is immune to this harpy’s Luring Song for 24 hours.

Original layout: [[../../srd-5.2.1/pages/297|page 297]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/harpy ` |
| ` implementation ` | ` dnd2024:creature_harpy ` |

### Parameters

#### Features

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
  supplied_attack_total: 3
  reach_ft: 5
hit:
  damage:
  - average: 6
    roll:
      count: 2
      sides: 4
      modifier: 1
    type: slashing
```

##### Luring song

<!-- splat:yaml path=splat_rule/parameters/features/luring_song -->

```yaml
economy: action
operation: saving_throw
ability: wisdom
dc: 11
targets:
  kind: creature
  types:
  - humanoid
  - giant
  selection: each
  area:
    shape: emanation
    size_ft: 300
    origin: self
  trigger: song.starts
failure:
  conditions:
  - condition: charmed
    until: song.ends
  repeat_save:
    events:
    - target.turn_end
    - before_target_enters_damaging_terrain
    - event: target.damage_taken
      source_excludes: self
    success: end_effect_and_grant_immunity
  while_charmed:
    conditions:
    - incapacitated
    ignore_other_harpy_luring_songs: true
    movement:
      event: target.turn
      if_distance_greater_ft: 5
      destination: within_5ft_of_self
      route: most_direct
      avoid_opportunity_attacks: false
concentration: true
magical: true
success:
  immunity:
    effect: this_harpy_luring_song
    duration_seconds: 86400
```

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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 40
> ```

