---
splat_schema: 2
splat_id: f8b9d24b-5839-5f42-8249-f828ca119c19
splat_kind: creature
splat_key: dnd2024:creatures/boar
splat_name: Boar
splat_origin: dnd2024:reference/creature/boar
splat_sources:
- SRD 5.2.1, pp. 346, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 350, Habitat
---

# Boar


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 13 (2d8 + 4) |
| Initiative | +0 (10) |
| Speed | 40 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Senses | Passive Perception 9 |
| Languages | None |
| Habitat | Forest, Grassland, Hill |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 13 | +1 | +1 |
| Dex | 11 | +0 | +0 |
| Con | 14 | +2 | +2 |
| Int | 2 | -4 | -4 |
| Wis | 9 | -1 | -1 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 346 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 11  Initiative +0 (10)\
> HP 13 (2d8 + 4)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 13 +1 +1 Dex 11 +0 +0 Con 14 +2 +2\
> Int 2 −4 −4 Wis 9 −1 −1 Cha 5 −3 −3\
> Senses Passive Perception 9\
> Languages None\
> CR 1/4 (XP 50; PB +2)


### Traits

Bloodied Fury. While Bloodied, the boar has Advantage on attack rolls.

### Actions

Gore. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1) Piercing damage. If the target is a Medium or smaller creature and the boar moved 20+ feet straight toward it immediately before the hit, the target takes an extra 3 (1d6) Piercing damage and has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/346|page 346]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/boar ` |
| ` implementation ` | ` dnd2024:creature_boar ` |

### Parameters

#### Features

##### Gore

<!-- splat:yaml path=splat_rule/parameters/features/gore -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 3
targets:
  count: 1
hit:
- op: damage
  average: 4
  amount:
    dice:
      count: 1
      sides: 6
    add: 1
  damage_type: piercing
- op: if
  test:
    target_kind: creature
    target_size_max: medium
    immediately_preceding_movement:
      minimum_feet: 20
      direction: straight_toward_target
  then:
  - op: damage
    average: 3
    amount:
      dice:
        count: 1
        sides: 6
      add: 0
    damage_type: piercing
  - op: condition
    condition: prone
reach_feet: 5
```

##### Bloodied fury

<!-- splat:yaml path=splat_rule/parameters/features/bloodied_fury -->

```yaml
economy: passive
advantage:
  roll: attack
  when: self_bloodied
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
