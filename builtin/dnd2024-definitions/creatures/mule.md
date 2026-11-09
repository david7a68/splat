---
splat_schema: 2
splat_id: 40b2a9a6-ad8a-554f-8b8b-eb21dd929b2e
splat_kind: creature
splat_key: dnd2024:creatures/mule
splat_name: Mule
splat_origin: dnd2024:reference/creature/mule
splat_sources:
- SRD 5.2.1, pp. 357, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 365, Habitat
---

# Mule


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 10 |
| Hit Points | 11 (2d8 + 2) |
| Initiative | +0 (10) |
| Speed | 40 ft. |
| Challenge | 1/8 (XP 25; PB +2) |
| Senses | Passive Perception 10 |
| Languages | None |
| Habitat | Desert, Hill, Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +4 |
| Dex | 10 | +0 | +0 |
| Con | 13 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 10 | +0 | +0 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 357 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 10  Initiative +0 (10)\
> HP 11 (2d8 + 2)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str  14 +2 +4 Dex 10 +0 +0 Con 13 +1 +1\
> Int 2 −4 −4 Wis 10 +0 +0 Cha  5 −3 −3\
> Senses Passive Perception 10\
> Languages None\
> CR 1/8 (XP 25; PB +2)


### Traits

Beast of Burden. The mule counts as one size larger for the purpose of determining its carrying capacity.

### Actions

Hooves. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/357|page 357]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/mule ` |
| ` implementation ` | ` dnd2024:creature_mule ` |

### Parameters

#### Features

##### Hooves

<!-- splat:yaml path=splat_rule/parameters/features/hooves -->

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
  average: 4
  amount:
    dice:
      count: 1
      sides: 4
    add: 2
  damage_type: bludgeoning
reach_feet: 5
```

##### Beast of burden

<!-- splat:values path=splat_rule/parameters/features/beast_of_burden -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` carrying_capacity_size_increment ` | ` 1 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

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
