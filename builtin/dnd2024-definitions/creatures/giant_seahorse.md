---
splat_schema: 2
splat_id: 2c3d32aa-4010-5638-a265-553da3cc8030
splat_kind: creature
splat_key: dnd2024:creatures/giant_seahorse
splat_name: Giant Seahorse
splat_origin: dnd2024:reference/creature/giant_seahorse
splat_sources:
- SRD 5.2.1, pp. 353, https://www.dndbeyond.com/srd
---

# Giant Seahorse


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 14 |
| Hit Points | 16 (3d10) |
| Initiative | +1 (11) |
| Speed | 5 ft., Swim 40 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Senses | Passive Perception 11 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +2 |
| Dex | 12 | +1 | +1 |
| Con | 11 | +0 | +0 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 353 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 14  Initiative +1 (11)\
> HP 16 (3d10)\
> Speed 5 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 15 +2 +2 Dex 12 +1 +1 Con 11 +0 +0\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
> Senses Passive Perception 11\
> Languages None\
> CR 1/2 (XP 100; PB +2)


### Traits

Water Breathing. The seahorse can breathe only underwater.

### Actions

Ram. Melee Attack Roll: +4, reach 5 ft. Hit: 9 (2d6 + 2) Bludgeoning damage, or 11 (2d8 + 2) Bludgeoning damage if the seahorse moved 20+ feet straight toward the target immediately before the hit.

### Bonus Actions

Bubble Dash. While underwater, the seahorse moves up to half its Swim Speed without provoking Opportunity Attacks.

Original layout: [[../../srd-5.2.1/pages/353|page 353]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_seahorse ` |
| ` implementation ` | ` dnd2024:creature_giant_seahorse ` |

### Parameters

#### Features

##### Water breathing

<!-- splat:values path=splat_rule/parameters/features/water_breathing -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` breathing_media ` | ` ["water"] ` |

##### Ram

<!-- splat:yaml path=splat_rule/parameters/features/ram -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 4
targets:
  count: 1
hit:
- op: choose_by_predicate
  test:
    immediately_preceding_movement:
      minimum_feet: 20
      direction: straight_toward_target
  then:
  - op: damage
    average: 11
    amount:
      dice:
        count: 2
        sides: 8
      add: 2
    damage_type: bludgeoning
  else:
  - op: damage
    average: 9
    amount:
      dice:
        count: 2
        sides: 6
      add: 2
    damage_type: bludgeoning
reach_feet: 5
```

##### Bubble dash

<!-- splat:values path=splat_rule/parameters/features/bubble_dash -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` move ` |
| ` provokes_opportunity_attacks ` | ` false ` |

###### Requires

<!-- splat:values path=splat_rule/parameters/features/bubble_dash/requires -->

| Field | Value |
| --- | --- |
| ` underwater ` | ` true ` |

###### Distance max

###### Multiply

<!-- splat:table
path: splat_rule/parameters/features/bubble_dash/distance_max/multiply
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Stat:
    path: stat
    type: symbol
    optional: true
  Numerator:
    path: numerator
    type: integer
    optional: true
  Denominator:
    path: denominator
    type: integer
    optional: true
-->

| Key | Stat | Numerator | Denominator |
| --- | --- | --- | --- |
| ` row_1 ` | ` Swim speed ` |  |  |
| ` row_2 ` |  | ` 1 ` | ` 2 ` |

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
> swim_speed:
>   unit: ft
>   value: 40
> ```

