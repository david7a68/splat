---
splat_schema: 2
splat_id: 5c49d536-cb5a-59ea-821a-e5df052847eb
splat_kind: creature
splat_key: dnd2024:creatures/weasel
splat_name: Weasel
splat_origin: dnd2024:reference/creature/weasel
splat_sources:
- SRD 5.2.1, pp. 364, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 372, Habitat
---

# Weasel


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 1 (1d4 - 1) |
| Initiative | +3 (13) |
| Speed | 30 ft., Climb 30 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Skills | Acrobatics +5, Perception +3, Stealth +5 |
| Senses | Darkvision 60 ft.; Passive Perception 13 |
| Languages | None |
| Habitat | Forest, Grassland, Hill |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 3 | -4 | -4 |
| Dex | 16 | +3 | +3 |
| Con | 8 | -1 | -1 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 3 | -4 | -4 |


## Source text

<!-- SRD page 364 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Beast, Unaligned\
> AC 13  Initiative +3 (13)\
> HP 1 (1d4 − 1)\
> Speed 30 ft., Climb 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 3 −4 −4 Dex 16 +3 +3 Con 8 −1 −1\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 3 −4 −4\
> Skills Acrobatics +5, Perception +3, Stealth +5\
> Senses Darkvision 60 ft.; Passive Perception 13\
> Languages None\
> CR 0 (XP 10; PB +2)


### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 1 Piercing damage.

Original layout: [[../../srd-5.2.1/pages/364|page 364]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/weasel ` |
| ` implementation ` | ` dnd2024:creature_weasel ` |

### Parameters

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

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
  damage_type: piercing
reach_feet: 5
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` acrobatics ` | ` 5 ` |
| ` perception ` | ` 3 ` |
| ` stealth ` | ` 5 ` |

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
>   value: 30
> ```

