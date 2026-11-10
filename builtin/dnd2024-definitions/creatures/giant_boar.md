---
splat_schema: 2
splat_id: 92183bf8-0664-5742-938f-992b30280a12
splat_kind: creature
splat_key: dnd2024:creatures/giant_boar
splat_name: Giant Boar
splat_origin: dnd2024:reference/creature/giant_boar
splat_sources:
- SRD 5.2.1, pp. 349, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 355, Habitat
---

# Giant Boar


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 42 (5d10 + 15) |
| Initiative | +0 (10) |
| Speed | 40 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Senses | Passive Perception 8 |
| Languages | None |
| Habitat | Forest, Grassland, Hill |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +5 |
| Dex | 10 | +0 | +0 |
| Con | 16 | +3 | +3 |
| Int | 2 | -4 | -4 |
| Wis | 7 | -2 | -2 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 349 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 13  Initiative +0 (10)\
> HP 42 (5d10 + 15)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +5 Dex 10 +0 +0 Con 16 +3 +3\
> Int 2 −4 −4 Wis 7 −2 −2 Cha 5 −3 −3\
> Senses Passive Perception 8\
> Languages None\
> CR 2 (XP 450; PB +2)


### Traits

Bloodied Fury. The boar has Advantage on melee attack rolls while it is Bloodied.

### Actions

Gore. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Piercing damage. If the target is a Large or smaller creature and the boar moved 20+ feet straight toward it immediately before the hit, the target takes an extra 7 (2d6) Piercing damage and has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/349|page 349]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_boar ` |
| ` implementation ` | ` dnd2024:creature_giant_boar ` |

### Parameters

#### Features

##### Gore

<!-- splat:yaml path=splat_rule/parameters/features/gore -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 10
  amount:
    dice:
      count: 2
      sides: 6
    add: 3
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
    average: 7
    amount:
      dice:
        count: 2
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
  roll: melee_attack
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
| ` passive_perception ` | ` 8 ` |

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
