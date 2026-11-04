---
splat_schema: 2
splat_id: 5afec75c-7981-5ce1-b7f7-3469de75b409
splat_kind: creature
splat_key: dnd2024:creatures/awakened_shrub
splat_name: Awakened Shrub
splat_origin: dnd2024:reference/creature/awakened_shrub
splat_sources:
- SRD 5.2.1, pp. 260, https://www.dndbeyond.com/srd
---

# Awakened Shrub


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Plant |
| Alignment | Neutral |
| Armor Class | 9 |
| Hit Points | 10 (3d6) |
| Initiative | -1 (9) |
| Speed | 20 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Senses | Passive Perception 10 |
| Languages | Common plus one other language |
| Resistances | Piercing |
| Vulnerabilities | Fire |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 3 | -4 | -4 |
| Dex | 8 | -1 | -1 |
| Con | 11 | +0 | +0 |
| Int | 10 | +0 | +0 |
| Wis | 10 | +0 | +0 |
| Cha | 6 | -2 | -2 |


## Source text

<!-- SRD page 260 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Plant, Neutral\
> AC 9 Initiative −1 (9)\
> HP 10 (3d6)\
> Speed 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 3 −4 −4 Dex 8 −1 −1 Con 11 +0 +0\
> Int 10 +0 +0 Wis 10 +0 +0 Cha 6 −2 −2\
> Vulnerabilities Fire\
> Resistances Piercing\
> Senses Passive Perception 10\
> Languages Common plus one other language\
> CR 0 (XP 10; PB +2)


### Actions

Rake. Melee Attack Roll: +1, reach 5 ft. Hit: 1 Slashing damage.

Original layout: [[../../srd-5.2.1/pages/260|page 260]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/awakened_shrub ` |
| ` implementation ` | ` dnd2024:creature_awakened_shrub ` |

### Parameters

#### Features

##### Rake

<!-- splat:yaml path=splat_rule/parameters/features/rake -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 1
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

#### Defenses

##### Vulnerabilities

<!-- splat:values path=splat_rule/parameters/defenses/vulnerabilities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["fire"] ` |

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["piercing"] ` |

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

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common"] ` |
| ` understands ` | ` ["common"] ` |

##### Additional languages

<!-- splat:values path=splat_rule/parameters/communication/additional_languages -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` chooser ` | ` gm ` |
| ` distinct_from_known ` | ` true ` |
| ` can_speak ` | ` true ` |
| ` can_understand ` | ` true ` |

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
