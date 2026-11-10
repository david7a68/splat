---
splat_schema: 2
splat_id: 4716d703-8690-5db4-80ac-cd5400ee9f4b
splat_kind: creature
splat_key: dnd2024:creatures/seahorse
splat_name: Seahorse
splat_origin: dnd2024:reference/creature/seahorse
splat_sources:
- SRD 5.2.1, pp. 361, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 369, Habitat
---

# Seahorse


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 1 (1d4 - 1) |
| Initiative | +1 (11) |
| Speed | 5 ft., Swim 20 ft. |
| Challenge | 0 (XP 0; PB +2) |
| Skills | Perception +2, Stealth +5 |
| Senses | Passive Perception 12 |
| Languages | None |
| Habitat | Underwater |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 1 | -5 | -5 |
| Dex | 12 | +1 | +1 |
| Con | 8 | -1 | -1 |
| Int | 1 | -5 | -5 |
| Wis | 10 | +0 | +0 |
| Cha | 2 | -4 | -4 |


## Source text

<!-- SRD page 361 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Beast, Unaligned\
> AC 12  Initiative +1 (11)\
> HP 1 (1d4 − 1)\
> Speed 5 ft., Swim 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 1 −5 −5 Dex 12 +1 +1 Con 8 −1 −1\
> Int 1 −5 −5 Wis 10 +0 +0 Cha 2 −4 −4\
> Skills Perception +2, Stealth +5\
> Senses Passive Perception 12\
> Languages None\
> CR 0 (XP 0; PB +2)


### Traits

Water Breathing. The seahorse can breathe only underwater.

### Actions

Bubble Dash. While underwater, the seahorse moves up to its Swim Speed without provoking Opportunity Attacks.

Original layout: [[../../srd-5.2.1/pages/361|page 361]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/seahorse ` |
| ` implementation ` | ` dnd2024:creature_seahorse ` |

### Parameters

#### Features

##### Water breathing

<!-- splat:values path=splat_rule/parameters/features/water_breathing -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` breathing_media ` | ` ["water"] ` |

##### Bubble dash

<!-- splat:yaml path=splat_rule/parameters/features/bubble_dash -->

```yaml
economy: action
requires:
  underwater: true
op: move
distance_max:
  multiply:
  - stat: swim_speed
  - 1
provokes_opportunity_attacks: false
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 2 ` |
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
| ` passive_perception ` | ` 12 ` |

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
>   value: 20
> ```

