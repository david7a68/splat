---
splat_schema: 2
splat_id: 0f221b09-8d98-5661-9896-385e53f3f1e6
splat_kind: creature
splat_key: dnd2024:creatures/worg
splat_name: Worg
splat_origin: dnd2024:reference/creature/worg
splat_sources:
- SRD 5.2.1, pp. 342, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 335, Habitat
---

# Worg


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Fey |
| Alignment | Neutral Evil |
| Armor Class | 13 |
| Hit Points | 26 (4d10 + 4) |
| Initiative | +1 (11) |
| Speed | 50 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Skills | Perception +4 |
| Senses | Darkvision 60 ft.; Passive Perception 14 |
| Languages | Goblin, Worg |
| Habitat | Forest, Grassland, Hill, Planar (Feywild) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 13 | +1 | +1 |
| Con | 13 | +1 | +1 |
| Int | 7 | -2 | -2 |
| Wis | 11 | +0 | +0 |
| Cha | 8 | -1 | -1 |


## Source text

<!-- SRD page 342 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Fey, Neutral Evil\
> AC 13 Initiative +1 (11)\
> HP 26 (4d10 + 4)\
> Speed 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 13 +1 +1 Con 13 +1 +1\
> Int 7 −2 −2 Wis 11 +0 +0 Cha 8 −1 −1\
> Skills Perception +4\
> Senses Darkvision 60 ft.; Passive Perception 14\
> Languages Goblin, Worg\
> CR 1/2 (XP 100; PB +2)


### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Piercing damage, and the next attack roll made against the target before the start of the worg’s next turn has Advantage.

Original layout: [[../../srd-5.2.1/pages/342|page 342]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/worg ` |
| ` implementation ` | ` dnd2024:creature_worg ` |

### Parameters

#### Features

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
  supplied_attack_total: 5
  reach_ft: 5
hit:
  damage:
  - average: 7
    roll:
      count: 1
      sides: 8
      modifier: 3
    type: piercing
  next_attack_against_target:
    advantage: true
    expires: self.next_turn_start
    uses: 1
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 14 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- goblin
- worg
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
