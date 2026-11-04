---
splat_schema: 2
splat_id: 69ba782f-9f7c-5021-b5c8-352cd3785c31
splat_kind: creature
splat_key: dnd2024:creatures/giant_vulture
splat_name: Giant Vulture
splat_origin: dnd2024:reference/creature/giant_vulture
splat_sources:
- SRD 5.2.1, pp. 354, https://www.dndbeyond.com/srd
---

# Giant Vulture


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Monstrosity |
| Alignment | Neutral Evil |
| Armor Class | 10 |
| Hit Points | 25 (3d10 + 9) |
| Initiative | +0 (10) |
| Speed | 10 ft., Fly 60 ft. |
| Challenge | 1 (XP 200; PB +2) |
| Skills | Perception +3 |
| Senses | Darkvision 60 ft.; Passive Perception 13 |
| Languages | Understands Common but can’t speak |
| Resistances | Necrotic |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +2 |
| Dex | 10 | +0 | +0 |
| Con | 16 | +3 | +3 |
| Int | 6 | -2 | -2 |
| Wis | 12 | +1 | +1 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 354 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Monstrosity, Neutral Evil\
> AC 10 Initiative +0 (10)\
> HP 25 (3d10 + 9)\
> Speed 10 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 15 +2 +2 Dex 10 +0 +0 Con 16 +3 +3\
> Int 6 −2 −2 Wis 12 +1 +1 Cha 7 −2 −2\
> Skills Perception +3\
> Resistances Necrotic\
> Senses Darkvision 60 ft.; Passive Perception 13\
> Languages Understands Common but can’t speak\
> CR 1 (XP 200; PB +2)


### Traits

Pack Tactics. The vulture has Advantage on an attack roll against a creature if at least one of the vulture’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

### Actions

Gouge. Melee Attack Roll: +4, reach 5 ft. Hit: 9 (2d6 + 2) Piercing damage, and the target has the Poisoned condition until the end of its next turn.

Original layout: [[../../srd-5.2.1/pages/354|page 354]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_vulture ` |
| ` implementation ` | ` dnd2024:creature_giant_vulture ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["necrotic"] ` |

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

##### Gouge

<!-- splat:yaml path=splat_rule/parameters/features/gouge -->

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
  - average: 9
    roll:
      count: 2
      sides: 6
      modifier: 2
    type: piercing
  conditions:
  - condition: poisoned
    until: target.next_turn_end
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 13 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands:
- common
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
>   value: 60
> ```

