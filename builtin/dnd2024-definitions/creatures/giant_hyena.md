---
splat_schema: 2
splat_id: f72fd22b-59c0-5ac7-acd1-5f9ed6b376bd
splat_kind: creature
splat_key: dnd2024:creatures/giant_hyena
splat_name: Giant Hyena
splat_origin: dnd2024:reference/creature/giant_hyena
splat_sources:
- SRD 5.2.1, pp. 352, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 357, Habitat
---

# Giant Hyena


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 45 (6d10 + 12) |
| Initiative | +2 (12) |
| Speed | 50 ft. |
| Challenge | 1 (XP 200; PB +2) |
| Skills | Perception +3 |
| Senses | Darkvision 60 ft.; Passive Perception 13 |
| Languages | None |
| Habitat | Desert, Forest, Grassland, Hill |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 14 | +2 | +2 |
| Con | 14 | +2 | +2 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 352 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 12  Initiative +2 (12)\
> HP 45 (6d10 + 12)\
> Speed 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 14 +2 +2 Con 14 +2 +2\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
> Skills Perception +3\
> Senses Darkvision 60 ft.; Passive Perception 13\
> Languages None\
> CR 1 (XP 200; PB +2)


### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Piercing damage.

### Bonus Actions

Rampage (1/Day). Immediately after dealing damage to a creature that was already Bloodied, the hyena can move up to half its Speed, and it makes one Bite attack.

Original layout: [[../../srd-5.2.1/pages/352|page 352]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_hyena ` |
| ` implementation ` | ` dnd2024:creature_giant_hyena ` |

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
  - average: 10
    roll:
      count: 2
      sides: 6
      modifier: 3
    type: piercing
```

##### Rampage

<!-- splat:yaml path=splat_rule/parameters/features/rampage -->

```yaml
economy: bonus_action
resource:
  kind: daily
  uses: 1
  recovery: long_rest
operation: sequence
trigger: self.dealt_damage
requires:
  target_already_bloodied: true
immediate: true
effects:
- operation: move
  maximum_ft:
    multiply:
    - stat: speed
    - 0.5
- operation: attack_sequence
  feature: bite
  count: 1
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
