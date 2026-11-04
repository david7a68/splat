---
splat_schema: 2
splat_id: 247e28ca-ee08-53c5-846e-4fcd417019fd
splat_kind: creature
splat_key: dnd2024:creatures/hyena
splat_name: Hyena
splat_origin: dnd2024:reference/creature/hyena
splat_sources:
- SRD 5.2.1, pp. 356, https://www.dndbeyond.com/srd
---

# Hyena


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 5 (1d8 + 1) |
| Initiative | +1 (11) |
| Speed | 50 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Skills | Perception +3 |
| Senses | Darkvision 60 ft.; Passive Perception 13 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 11 | +0 | +0 |
| Dex | 13 | +1 | +1 |
| Con | 12 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 356 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 11  Initiative +1 (11)\
> HP 5 (1d8 + 1)\
> Speed 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 11 +0 +0 Dex 13 +1 +1 Con 12 +1 +1\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
> Skills Perception +3\
> Senses Darkvision 60 ft.; Passive Perception 13\
> Languages None\
> CR 0 (XP 10; PB +2)


### Traits

Pack Tactics. The hyena has Advantage on an attack roll against a creature if at least one of the hyena’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 3 (1d6) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/356|page 356]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/hyena ` |
| ` implementation ` | ` dnd2024:creature_hyena ` |

### Parameters

#### Features

##### Pack tactics

<!-- splat:yaml path=splat_rule/parameters/features/pack_tactics -->

```yaml
economy: passive
advantage:
  roll: attack
  target_kind: creature
  requires:
    ally_distance_to_target_max_feet: 5
    ally_incapacitated: false
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 2
targets:
  count: 1
hit:
- op: damage
  average: 3
  amount:
    dice:
      count: 1
      sides: 6
    add: 0
  damage_type: piercing
reach_feet: 5
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |

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
