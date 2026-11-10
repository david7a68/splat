---
splat_schema: 2
splat_id: e76f4d97-c2ca-599e-a26e-7ded8834ee22
splat_kind: creature
splat_key: dnd2024:creatures/black_bear
splat_name: Black Bear
splat_origin: dnd2024:reference/creature/black_bear
splat_sources:
- SRD 5.2.1, pp. 345, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 349, Habitat
---

# Black Bear


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 19 (3d8 + 6) |
| Initiative | +1 (11) |
| Speed | 30 ft., Climb 30 ft., Swim 30 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Skills | Perception +5 |
| Senses | Darkvision 60 ft.; Passive Perception 15 |
| Languages | None |
| Habitat | Forest |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +2 |
| Dex | 12 | +1 | +1 |
| Con | 14 | +2 | +2 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 345 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 11 Initiative +1 (11)\
> HP 19 (3d8 + 6)\
> Speed 30 ft., Climb 30 ft., Swim 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 15 +2 +2 Dex 12 +1 +1 Con 14 +2 +2\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
> Skills Perception +5\
> Senses Darkvision 60 ft.; Passive Perception 15\
> Languages None\
> CR 1/2 (XP 100; PB +2)


### Actions

Multiattack. The bear makes two Rend attacks.

Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/345|page 345]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/black_bear ` |
| ` implementation ` | ` dnd2024:creature_black_bear ` |

### Parameters

#### Features

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 4
targets:
  count: 1
hit:
- op: damage
  average: 5
  amount:
    dice:
      count: 1
      sides: 6
    add: 2
  damage_type: slashing
reach_feet: 5
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: rend
  count: 2
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

##### Speaks

<!-- splat:yaml path=splat_rule/parameters/communication/speaks -->

```yaml
[]
```

##### Understands

<!-- splat:yaml path=splat_rule/parameters/communication/understands -->

```yaml
[]
```

#### Stat block conventions

<!-- splat:yaml path=splat_rule/parameters/stat_block_conventions -->

```yaml
attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
save_dc: supplied_total
damage_mode_choice:
- fixed_average
- rolled_dice
condition_source: self
multiattack_child_economy_cost: 0
multiattack_child_resource_costs_retained: true
spellcasting:
  default_level: spell_minimum
  spell_slots_expended: false
  required_materials: available
  casting_time_at_least_one_minute:
    uses_normal_casting_duration: true
    magic_action_each_turn: true
    concentration_required: true
geometry_assumptions:
  chooser: gm
  supplies:
  - range
  - visibility
  - line_of_sight
  - area_membership
  - reachable_locations
  - unoccupied_locations
  - terrain
  - plane
  - form_appearance
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> swim_speed:
>   unit: ft
>   value: 30
> climb_speed:
>   unit: ft
>   value: 30
> ```

