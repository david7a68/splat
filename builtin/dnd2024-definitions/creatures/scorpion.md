---
splat_schema: 2
splat_id: 108d8349-a58e-56b2-99d6-524205e33c86
splat_kind: creature
splat_key: dnd2024:creatures/scorpion
splat_name: Scorpion
splat_origin: dnd2024:reference/creature/scorpion
splat_sources:
- SRD 5.2.1, pp. 360, https://www.dndbeyond.com/srd
---

# Scorpion


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 1 (1d4 - 1) |
| Initiative | +0 (10) |
| Speed | 10 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Senses | Blindsight 10 ft.; Passive Perception 9 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 2 | -4 | -4 |
| Dex | 11 | +0 | +0 |
| Con | 8 | -1 | -1 |
| Int | 1 | -5 | -5 |
| Wis | 8 | -1 | -1 |
| Cha | 2 | -4 | -4 |


## Source text

<!-- SRD page 360 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Beast, Unaligned\
> AC 11  Initiative +0 (10)\
> HP 1 (1d4 − 1)\
> Speed 10 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 2 −4 −4 Dex 11 +0 +0 Con 8 −1 −1\
> Int 1 −5 −5 Wis 8 −1 −1 Cha 2 −4 −4\
> Senses Blindsight 10 ft.; Passive Perception 9\
> Languages None\
> CR 0 (XP 10; PB +2)


### Actions

Sting. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing damage plus 3 (1d6) Poison damage.

Original layout: [[../../srd-5.2.1/pages/360|page 360]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/scorpion ` |
| ` implementation ` | ` dnd2024:creature_scorpion ` |

### Parameters

#### Features

##### Sting

<!-- splat:yaml path=splat_rule/parameters/features/sting -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 2
targets:
  count: 1
hit:
- op: damage
  average: 1
  amount:
    constant: 1
  damage_type: piercing
- op: damage
  average: 3
  amount:
    dice:
      count: 1
      sides: 6
    add: 0
  damage_type: poison
reach_feet: 5
```

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 9 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 10 ` |

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
