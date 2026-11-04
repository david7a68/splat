---
splat_schema: 2
splat_id: 4a2827a5-4abb-5488-965b-d32d2333a195
splat_kind: creature
splat_key: dnd2024:creatures/giant_elk
splat_name: Giant Elk
splat_origin: dnd2024:reference/creature/giant_elk
splat_sources:
- SRD 5.2.1, pp. 351, https://www.dndbeyond.com/srd
---

# Giant Elk


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Celestial |
| Alignment | Neutral Good |
| Armor Class | 14 |
| Hit Points | 42 (5d12 + 10) |
| Initiative | +6 (16) |
| Speed | 60 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Perception +4 |
| Senses | Darkvision 90 ft.; Passive Perception 14 |
| Languages | Celestial; understands Common, Elvish, and Sylvan but can’t speak them |
| Resistances | Necrotic, Radiant |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +6 |
| Dex | 18 | +4 | +6 |
| Con | 14 | +2 | +2 |
| Int | 7 | -2 | -2 |
| Wis | 14 | +2 | +2 |
| Cha | 10 | +0 | +0 |


## Source text

<!-- SRD page 351 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Celestial, Neutral Good\
> AC 14 Initiative +6 (16)\
> HP 42 (5d12 + 10)\
> Speed 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 19 +4 +6 Dex 18 +4 +6 Con 14 +2 +2\
> Int 7 −2 −2 Wis 14 +2 +2 Cha 10 +0 +0\
> Skills Perception +4\
> Resistances Necrotic, Radiant\
> Senses Darkvision 90 ft.; Passive Perception 14\
> Languages Celestial; understands Common, Elvish, and Sylvan but can’t speak them\
> CR 2 (XP 450; PB +2)


### Actions

Ram. Melee Attack Roll: +6, reach 10 ft. Hit: 11 (2d6 + 4) Bludgeoning damage plus 5 (2d4) Radiant damage. If the target is a Huge or smaller creature and the elk moved 20+ feet straight toward it immediately before the hit, the target takes an extra 5 (2d4) Bludgeoning damage and has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/351|page 351]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_elk ` |
| ` implementation ` | ` dnd2024:creature_giant_elk ` |

### Parameters

#### Features

##### Ram

<!-- splat:yaml path=splat_rule/parameters/features/ram -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 6
targets:
  count: 1
hit:
- op: damage
  average: 11
  amount:
    dice:
      count: 2
      sides: 6
    add: 4
  damage_type: bludgeoning
- op: damage
  average: 5
  amount:
    dice:
      count: 2
      sides: 4
    add: 0
  damage_type: radiant
- op: if
  test:
    target_kind: creature
    target_size_max: huge
    immediately_preceding_movement:
      minimum_feet: 20
      direction: straight_toward_target
  then:
  - op: damage
    average: 5
    amount:
      dice:
        count: 2
        sides: 4
      add: 0
    damage_type: bludgeoning
  - op: condition
    condition: prone
reach_feet: 10
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["necrotic", "radiant"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 90 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["celestial"] ` |
| ` understands ` | ` ["celestial", "common", "elvish", "sylvan"] ` |

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
