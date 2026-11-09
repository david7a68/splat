---
splat_schema: 2
splat_id: 5df7adb8-5218-5c6f-b1ff-7232ef3527bc
splat_kind: creature
splat_key: dnd2024:creatures/tiger
splat_name: Tiger
splat_origin: dnd2024:reference/creature/tiger
splat_sources:
- SRD 5.2.1, pp. 363, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 371, Habitat
---

# Tiger


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 30 (4d10 + 8) |
| Initiative | +3 (13) |
| Speed | 40 ft. |
| Challenge | 1 (XP 200; PB +2) |
| Skills | Perception +3, Stealth +7 |
| Senses | Darkvision 60 ft.; Passive Perception 13 |
| Languages | None |
| Habitat | Forest, Grassland |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 16 | +3 | +3 |
| Con | 14 | +2 | +2 |
| Int | 3 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 8 | -1 | -1 |


## Source text

<!-- SRD page 363 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 13  Initiative +3 (13)\
> HP 30 (4d10 + 8)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 16 +3 +3 Con 14 +2 +2\
> Int 3 −4 −4 Wis 12 +1 +1 Cha 8 −1 −1\
> Skills Perception +3, Stealth +7\
> Senses Darkvision 60 ft.; Passive Perception 13\
> Languages None\
> CR 1 (XP 200; PB +2)


### Actions

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage. If the target is a Large or smaller creature, it has the Prone condition.

### Bonus Actions

Nimble Escape. The tiger takes the Disengage or Hide action.

Original layout: [[../../srd-5.2.1/pages/363|page 363]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/tiger ` |
| ` implementation ` | ` dnd2024:creature_tiger ` |

### Parameters

#### Features

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

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
  damage_type: slashing
- op: if
  test:
    target_kind: creature
    target_size_max: large
  then:
  - op: condition
    condition: prone
reach_feet: 5
```

##### Nimble escape

<!-- splat:values path=splat_rule/parameters/features/nimble_escape -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` take_action ` |
| ` choice ` | ` ["disengage", "hide"] ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |
| ` stealth ` | ` 7 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 13 ` |

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
