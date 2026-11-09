---
splat_schema: 2
splat_id: 385242bd-9fd7-5a19-8d90-6affd94ceecc
splat_kind: creature
splat_key: dnd2024:creatures/rhinoceros
splat_name: Rhinoceros
splat_origin: dnd2024:reference/creature/rhinoceros
splat_sources:
- SRD 5.2.1, pp. 360, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 368, Habitat
---

# Rhinoceros


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 45 (6d10 + 12) |
| Initiative | -1 (9) |
| Speed | 40 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Senses | Passive Perception 11 |
| Languages | None |
| Habitat | Grassland |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 21 | +5 | +5 |
| Dex | 8 | -1 | -1 |
| Con | 15 | +2 | +2 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 6 | -2 | -2 |


## Source text

<!-- SRD page 360 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 13  Initiative −1 (9)\
> HP 45 (6d10 + 12)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 21 +5 +5 Dex 8 −1 −1 Con 15 +2 +2\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 6 −2 −2\
> Senses Passive Perception 11\
> Languages None\
> CR 2 (XP 450; PB +2)


### Actions

Gore. Melee Attack Roll: +7, reach 5 ft. Hit: 14 (2d8 + 5) Piercing damage. If target is a Large or smaller creature and the rhinoceros moved 20+ feet straight toward it immediately before the hit, the target takes an extra 9 (2d8) Piercing damage and has the Prone condition.

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
| ` mechanic ` | ` dnd2024:creatures/rhinoceros ` |
| ` implementation ` | ` dnd2024:creature_rhinoceros ` |

### Parameters

#### Features

##### Gore

<!-- splat:yaml path=splat_rule/parameters/features/gore -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 7
targets:
  count: 1
hit:
- op: damage
  average: 14
  amount:
    dice:
      count: 2
      sides: 8
    add: 5
  damage_type: piercing
- op: if
  test:
    target_kind: creature
    target_size_max: large
    immediately_preceding_movement:
      minimum_feet: 20
      direction: straight_toward_target
  then:
  - op: damage
    average: 9
    amount:
      dice:
        count: 2
        sides: 8
      add: 0
    damage_type: piercing
  - op: condition
    condition: prone
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
| ` passive_perception ` | ` 11 ` |

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
