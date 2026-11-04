---
splat_schema: 2
splat_id: 31472007-19a6-5177-82fe-878e11e1740f
splat_kind: creature
splat_key: dnd2024:creatures/giant_wasp
splat_name: Giant Wasp
splat_origin: dnd2024:reference/creature/giant_wasp
splat_sources:
- SRD 5.2.1, pp. 354, 355, https://www.dndbeyond.com/srd
---

# Giant Wasp


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 22 (5d8) |
| Initiative | +2 (12) |
| Speed | 10 ft., Fly 50 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Senses | Passive Perception 10 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 10 | +0 | +0 |
| Dex | 14 | +2 | +2 |
| Con | 10 | +0 | +0 |
| Int | 1 | -5 | -5 |
| Wis | 10 | +0 | +0 |
| Cha | 3 | -4 | -4 |


## Source text

<!-- SRD page 354 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 13  Initiative +2 (12)\
> HP 22 (5d8)\
> Speed 10 ft., Fly 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 10 +0 +0 Dex 14 +2 +2 Con 10 +0 +0\
> Int 1 −5 −5 Wis 10 +0 +0 Cha 3 −4 −4\
> Senses Passive Perception 10\
> Languages None\
> CR 1/2 (XP 100; PB +2)


<!-- SRD page 355 -->

### Traits

Flyby. The wasp doesn’t provoke an Opportunity Attack when it flies out of an enemy’s reach.

### Actions

Sting. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage plus 5 (2d4) Poison damage.

Original layout: [[../../srd-5.2.1/pages/354|page 354]], [[../../srd-5.2.1/pages/355|page 355]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_wasp ` |
| ` implementation ` | ` dnd2024:creature_giant_wasp ` |

### Parameters

#### Features

##### Flyby

<!-- splat:yaml path=splat_rule/parameters/features/flyby -->

```yaml
economy: passive
opportunity_attack_immunity:
  trigger: fly_out_of_enemy_reach
```

##### Sting

<!-- splat:yaml path=splat_rule/parameters/features/sting -->

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
  damage_type: piercing
- op: damage
  average: 5
  amount:
    dice:
      count: 2
      sides: 4
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
| ` passive_perception ` | ` 10 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

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
> fly_speed:
>   unit: ft
>   value: 50
> ```

