---
splat_schema: 2
splat_id: 1cb10e40-4724-5948-8930-5e7b5876854a
splat_kind: creature
splat_key: dnd2024:creatures/giant_fly
splat_name: Giant Fly
splat_origin: dnd2024:reference/creature/giant_fly
splat_sources:
- SRD 5.2.1, pp. 222, https://www.dndbeyond.com/srd
---

# Giant Fly


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 19 (3d10 + 3) |
| Initiative | +1 (11) |
| Speed | 30 ft., Fly 60 ft. |
| Challenge | 0 (XP 0; PB +2) |
| Senses | Darkvision 60 ft., Passive Perception 10 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +2 |
| Dex | 13 | +1 | +1 |
| Con | 13 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 10 | +0 | +0 |
| Cha | 3 | -4 | -4 |


## Source text

<!-- SRD page 222 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 11  Initiative +1 (11)\
> HP 19 (3d10 + 3)\
> Speed 30 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 14 +2 +2 Dex 13 +1 +1 Con 13 +1 +1\
> Int 2 −4 −4 Wis 10 +0 +0 Cha 3 −4 −4\
> Senses Darkvision 60 ft., Passive Perception 10\
> Languages None\
> CR 0 (XP 0; PB +2)


Original layout: [[../../srd-5.2.1/pages/222|page 222]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_fly ` |
| ` implementation ` | ` dnd2024:creature_giant_fly ` |

### Parameters

#### Features

<!-- splat:yaml path=splat_rule/parameters/features -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 60
> ```

