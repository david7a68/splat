---
splat_schema: 2
splat_id: 16552e43-3ddf-5ebd-a18e-b9ff80cedf44
splat_kind: creature
splat_key: dnd2024:creatures/cat
splat_name: Cat
splat_origin: dnd2024:reference/creature/cat
splat_sources:
- SRD 5.2.1, pp. 346, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 351, Habitat
---

# Cat


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 2 (1d4) |
| Initiative | +2 (12) |
| Speed | 40 ft., Climb 40 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Skills | Perception +3, Stealth +4 |
| Senses | Darkvision 60 ft.; Passive Perception 13 |
| Languages | None |
| Habitat | Desert, Forest, Grassland, Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 3 | -4 | -4 |
| Dex | 15 | +2 | +4 |
| Con | 10 | +0 | +0 |
| Int | 3 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 346 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Beast, Unaligned\
> AC 12  Initiative +2 (12)\
> HP 2 (1d4)\
> Speed 40 ft., Climb 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 3 −4 −4 Dex 15 +2 +4 Con 10 +0 +0\
> Int 3 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
> Skills Perception +3, Stealth +4\
> Senses Darkvision 60 ft.; Passive Perception 13\
> Languages None\
> CR 0 (XP 10; PB +2)


### Traits

Jumper. The cat’s jump distance is determined using its Dexterity rather than its Strength.

### Actions

Scratch. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Slashing damage.

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
| ` mechanic ` | ` dnd2024:creatures/cat ` |
| ` implementation ` | ` dnd2024:creature_cat ` |

### Parameters

#### Features

##### Scratch

<!-- splat:yaml path=splat_rule/parameters/features/scratch -->

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
  average: 1
  amount:
    constant: 1
  damage_type: slashing
reach_feet: 5
```

##### Jumper

<!-- splat:values path=splat_rule/parameters/features/jumper -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` jump_distance_ability ` | ` dexterity ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |
| ` stealth ` | ` 4 ` |

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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> climb_speed:
>   unit: ft
>   value: 40
> ```

