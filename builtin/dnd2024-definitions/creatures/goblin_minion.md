---
splat_schema: 2
splat_id: 299cbf5b-b33b-59e4-80a9-b38f344dc807
splat_kind: creature
splat_key: dnd2024:creatures/goblin_minion
splat_name: Goblin Minion
splat_origin: dnd2024:reference/creature/goblin_minion
splat_sources:
- SRD 5.2.1, pp. 290, https://www.dndbeyond.com/srd
---

# Goblin Minion


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Fey (Goblinoid) |
| Alignment | Chaotic Neutral |
| Armor Class | 12 |
| Hit Points | 7 (2d6) |
| Initiative | +2 (12) |
| Speed | 30 ft. |
| Challenge | 1/8 (XP 25; PB +2) |
| Skills | Stealth +6 |
| Senses | Darkvision 60 ft.; Passive Perception 9 |
| Languages | Common, Goblin |
| Gear | Daggers (3) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 8 | -1 | -1 |
| Dex | 15 | +2 | +2 |
| Con | 10 | +0 | +0 |
| Int | 10 | +0 | +0 |
| Wis | 8 | -1 | -1 |
| Cha | 8 | -1 | -1 |


## Source text

<!-- SRD page 290 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Fey (Goblinoid), Chaotic Neutral\
> AC 12 Initiative +2 (12)\
> HP 7 (2d6)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 8 −1 −1 Dex 15 +2 +2 Con 10 +0 +0\
> Int 10 +0 +0 Wis 8 −1 −1 Cha 8 −1 −1\
> Skills Stealth +6\
> Gear Daggers (3)\
> Senses Darkvision 60 ft.; Passive Perception 9\
> Languages Common, Goblin\
> CR 1/8 (XP 25; PB +2)


### Actions

Dagger. Melee or Ranged Attack Roll: +4, reach 5 ft. or range 20/60 ft. Hit: 4 (1d4 + 2) Piercing damage.

### Bonus Actions

Nimble Escape. The goblin takes the Disengage or Hide action.

Original layout: [[../../srd-5.2.1/pages/290|page 290]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/goblin_minion ` |
| ` implementation ` | ` dnd2024:creature_goblin_minion ` |

### Parameters

#### Features

##### Dagger

<!-- splat:yaml path=splat_rule/parameters/features/dagger -->

```yaml
economy: action
op: attack
attack_modes:
- melee
- ranged
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
  damage_type: piercing
reach_feet: 5
range_feet: 20
long_range_feet: 60
```

##### Nimble escape

<!-- splat:values path=splat_rule/parameters/features/nimble_escape -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` take_action ` |
| ` choice ` | ` ["disengage", "hide"] ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 6 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 9 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item_name | quantity |
| --- | --- | --- |
| ` 0 ` | ` Daggers ` | ` 3 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common", "goblin"] ` |
| ` understands ` | ` ["common", "goblin"] ` |

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
