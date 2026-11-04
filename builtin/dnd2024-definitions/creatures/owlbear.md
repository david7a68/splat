---
splat_schema: 2
splat_id: 72cc0187-32ce-54cd-b408-09dc5fb4205c
splat_kind: creature
splat_key: dnd2024:creatures/owlbear
splat_name: Owlbear
splat_origin: dnd2024:reference/creature/owlbear
splat_sources:
- SRD 5.2.1, pp. 313, https://www.dndbeyond.com/srd
---

# Owlbear


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 59 (7d10 + 21) |
| Initiative | +1 (11) |
| Speed | 40 ft., Climb 40 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Skills | Perception +5 |
| Senses | Darkvision 60 ft.; Passive Perception 15 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 20 | +5 | +5 |
| Dex | 12 | +1 | +1 |
| Con | 17 | +3 | +3 |
| Int | 3 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 313 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Monstrosity, Unaligned\
> AC 13 Initiative +1 (11)\
> HP 59 (7d10 + 21)\
> Speed 40 ft., Climb 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 20 +5 +5 Dex 12 +1 +1 Con 17 +3 +3\
> Int 3 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
> Skills Perception +5\
> Senses Darkvision 60 ft.; Passive Perception 15\
> Languages None\
> CR 3 (XP 700; PB +2)


### Actions

Multiattack. The owlbear makes two Rend attacks.

Rend. Melee Attack Roll: +7, reach 5 ft. Hit: 14 (2d8 + 5) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/313|page 313]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/owlbear ` |
| ` implementation ` | ` dnd2024:creature_owlbear ` |

### Parameters

#### Features

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

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
  damage_type: slashing
reach_feet: 5
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: rend
  count: 2
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

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

