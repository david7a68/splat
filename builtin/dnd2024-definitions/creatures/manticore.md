---
splat_schema: 2
splat_id: 921e78dc-60f0-595a-9332-a0261b8abd5a
splat_kind: creature
splat_key: dnd2024:creatures/manticore
splat_name: Manticore
splat_origin: dnd2024:reference/creature/manticore
splat_sources:
- SRD 5.2.1, pp. 306, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 202, Habitat
---

# Manticore


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Monstrosity |
| Alignment | Lawful Evil |
| Armor Class | 14 |
| Hit Points | 68 (8d10 + 24) |
| Initiative | +3 (13) |
| Speed | 30 ft., Fly 50 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 11 |
| Languages | Common |
| Habitat | Arctic, Coastal, Grassland, Hill, Mountain |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 16 | +3 | +3 |
| Con | 17 | +3 | +3 |
| Int | 7 | -2 | -2 |
| Wis | 12 | +1 | +1 |
| Cha | 8 | -1 | -1 |


## Source text

<!-- SRD page 306 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Monstrosity, Lawful Evil\
> AC 14 Initiative +3 (13)\
> HP 68 (8d10 + 24)\
> Speed 30 ft., Fly 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 16 +3 +3 Con 17 +3 +3\
> Int 7 −2 −2 Wis 12 +1 +1 Cha 8 −1 −1\
> Senses Darkvision 60 ft.; Passive Perception 11\
> Languages Common\
> CR 3 (XP 700; PB +2)


### Actions

Multiattack. The manticore makes three attacks, using Rend or Tail Spike in any combination.

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Slashing damage.

Tail Spike. Ranged Attack Roll: +5, range 100/200 ft. Hit: 7 (1d8 + 3) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/306|page 306]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/manticore ` |
| ` implementation ` | ` dnd2024:creature_manticore ` |

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
  average: 7
  amount:
    dice:
      count: 1
      sides: 8
    add: 3
  damage_type: slashing
reach_feet: 5
```

##### Tail spike

<!-- splat:yaml path=splat_rule/parameters/features/tail_spike -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 7
  amount:
    dice:
      count: 1
      sides: 8
    add: 3
  damage_type: piercing
range_feet: 100
long_range_feet: 200
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` repeat_choice ` |
| ` count ` | ` 3 ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/multiattack/choices
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Feature:
    path: feature
    type: symbol
-->

| Key | Feature |
| --- | --- |
| ` row_1 ` | ` Rend ` |
| ` row_2 ` | ` Tail spike ` |

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

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common"] ` |
| ` understands ` | ` ["common"] ` |

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

