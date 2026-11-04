---
splat_schema: 2
splat_id: 217f9689-bdcd-5867-9c0c-ed4a5b5c3313
splat_kind: creature
splat_key: dnd2024:creatures/grimlock
splat_name: Grimlock
splat_origin: dnd2024:reference/creature/grimlock
splat_sources:
- SRD 5.2.1, pp. 296, https://www.dndbeyond.com/srd
---

# Grimlock


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Aberration |
| Alignment | Neutral Evil |
| Armor Class | 11 |
| Hit Points | 11 (2d8 + 2) |
| Initiative | +1 (11) |
| Speed | 30 ft., Climb 30 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Skills | Athletics +5, Perception +3, Stealth +5 |
| Senses | Blindsight 30 ft.; Passive Perception 13 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 12 | +1 | +1 |
| Con | 12 | +1 | +1 |
| Int | 9 | -1 | -1 |
| Wis | 8 | -1 | -1 |
| Cha | 6 | -2 | -2 |


## Source text

<!-- SRD page 296 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Aberration, Neutral Evil\
> AC 11 Initiative +1 (11)\
> HP 11 (2d8 + 2)\
> Speed 30 ft., Climb 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 12 +1 +1 Con 12 +1 +1\
> Int 9 −1 −1 Wis 8 −1 −1 Cha 6 −2 −2\
> Skills Athletics +5, Perception +3, Stealth +5\
> Senses Blindsight 30 ft.; Passive Perception 13\
> Languages None\
> CR 1/4 (XP 50; PB +2)


### Actions

Bone Cudgel. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Bludgeoning damage plus 2 (1d4) Psychic damage.

Original layout: [[../../srd-5.2.1/pages/296|page 296]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/grimlock ` |
| ` implementation ` | ` dnd2024:creature_grimlock ` |

### Parameters

#### Features

##### Bone cudgel

<!-- splat:yaml path=splat_rule/parameters/features/bone_cudgel -->

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
  average: 6
  amount:
    dice:
      count: 1
      sides: 6
    add: 3
  damage_type: bludgeoning
- op: damage
  average: 2
  amount:
    dice:
      count: 1
      sides: 4
    add: 0
  damage_type: psychic
reach_feet: 5
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` athletics ` | ` 5 ` |
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
| ` 0 ` | ` blindsight ` | ` 30 ` |

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

