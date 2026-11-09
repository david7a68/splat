---
splat_schema: 2
splat_id: 5a484d75-2077-5cd5-a2b5-e9a89e725e0e
splat_kind: creature
splat_key: dnd2024:creatures/hawk
splat_name: Hawk
splat_origin: dnd2024:reference/creature/hawk
splat_sources:
- SRD 5.2.1, pp. 355, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 362, Habitat
---

# Hawk


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 1 (1d4 - 1) |
| Initiative | +3 (13) |
| Speed | 10 ft., Fly 60 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Skills | Perception +6 |
| Senses | Passive Perception 16 |
| Languages | None |
| Habitat | Arctic, Coastal, Forest, Grassland, Hill, Mountain |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 5 | -3 | -3 |
| Dex | 16 | +3 | +3 |
| Con | 8 | -1 | -1 |
| Int | 2 | -4 | -4 |
| Wis | 14 | +2 | +2 |
| Cha | 6 | -2 | -2 |


## Source text

<!-- SRD page 355 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Beast, Unaligned\
> AC 13  Initiative +3 (13)\
> HP 1 (1d4 − 1)\
> Speed 10 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 5 −3 −3 Dex 16 +3 +3 Con 8 −1 −1\
> Int 2 −4 −4 Wis 14 +2 +2 Cha 6 −2 −2\
> Skills Perception +6\
> Senses Passive Perception 16\
> Languages None\
> CR 0 (XP 10; PB +2)


### Actions

Talons. Melee Attack Roll: +5, reach 5 ft. Hit: 1 Slashing damage.

Original layout: [[../../srd-5.2.1/pages/355|page 355]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/hawk ` |
| ` implementation ` | ` dnd2024:creature_hawk ` |

### Parameters

#### Features

##### Talons

<!-- splat:yaml path=splat_rule/parameters/features/talons -->

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
  average: 1
  amount:
    constant: 1
  damage_type: slashing
reach_feet: 5
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 6 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 16 ` |

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
>   value: 60
> ```

