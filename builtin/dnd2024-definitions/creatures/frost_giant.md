---
splat_schema: 1
splat_id: 70219a9d-618c-52de-80dc-6918dcb1b6da
splat_kind: creature
splat_key: dnd2024:creatures/frost_giant
splat_name: Frost Giant
splat_origin:
  key: dnd2024:reference/creature/frost_giant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 285, 286
  url: https://www.dndbeyond.com/srd
---

# Frost Giant

## Source text

<!-- SRD page 285 -->

Huge Giant, Neutral Evil\
AC 15 Initiative +2 (12)\
HP 149 (13d12 + 65)\
Speed 40 ft.\

<!-- SRD page 286 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 9 −1 −1 Con 21 +5 +8\
Int 9 −1 −1 Wis 10 +0 +3 Cha 12 +1 +4\
Skills Athletics +9, Perception +3\
Immunities Cold\
Senses Passive Perception 13\
Languages Giant\
CR 8 (XP 3,900; PB +3)\

### Actions

Multiattack. The giant makes two attacks, using Frost\
Axe or Great Bow in any combination.\
Frost Axe. Melee Attack Roll: +9, reach 10 ft. Hit: 19\
(2d12 + 6) Slashing damage plus 9 (2d8) Cold damage.\
Great Bow. Ranged Attack Roll: +9, range 150/600 ft.\
Hit: 17 (2d10 + 6) Piercing damage plus 7 (2d6) Cold\
damage, and the target’s Speed decreases by 10 feet\
until the end of its next turn.\

### Bonus Actions

War Cry (Recharge 5–6). The giant or one creature of\
its choice that can see or hear it gains 16 (2d10 + 5)\
Temporary Hit Points and has Advantage on attack rolls\
until the start of the giant’s next turn.\

Original layout: [[../../srd-5.2.1/pages/285|page 285]], [[../../srd-5.2.1/pages/286|page 286]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` giant ` |
| ` alignment ` | ` Neutral Evil ` |
| ` movement_text ` | ` 40 ft. ` |
| ` skills_text ` | ` Athletics +9, Perception +3 ` |
| ` immunities_text ` | ` Cold ` |
| ` senses_text ` | ` Passive Perception 13 ` |
| ` languages_text ` | ` Giant ` |
| ` cr_text ` | ` 8 (XP 3,900; PB +3) ` |
| ` hit_points_text ` | ` 149 (13d12 + 65) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 23 ` | ` 6 ` |
| ` Dexterity ` | ` 9 ` | ` -1 ` |
| ` Constitution ` | ` 21 ` | ` 8 ` |
| ` Intelligence ` | ` 9 ` | ` -1 ` |
| ` Wisdom ` | ` 10 ` | ` 3 ` |
| ` Charisma ` | ` 12 ` | ` 4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 15 ` |  |
| ` max_hp ` | ` 149 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/frost_giant ` |
| ` implementation ` | ` dnd2024:creature_frost_giant ` |

### Parameters

#### Features

##### Frost axe

<!-- splat:yaml path=splat_rule/parameters/features/frost_axe -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 9
targets:
  count: 1
hit:
- op: damage
  average: 19
  amount:
    dice:
      count: 2
      sides: 12
    add: 6
  damage_type: slashing
- op: damage
  average: 9
  amount:
    dice:
      count: 2
      sides: 8
    add: 0
  damage_type: cold
reach_feet: 10
```

##### Great bow

<!-- splat:yaml path=splat_rule/parameters/features/great_bow -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 9
targets:
  count: 1
hit:
- op: damage
  average: 17
  amount:
    dice:
      count: 2
      sides: 10
    add: 6
  damage_type: piercing
- op: damage
  average: 7
  amount:
    dice:
      count: 2
      sides: 6
    add: 0
  damage_type: cold
- op: modify_speed
  add_feet: -10
  ends:
    event: turn.end
    participant: target
    occurrence: next
range_feet: 150
long_range_feet: 600
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: repeat_choice
count: 2
choices:
- feature: frost_axe
- feature: great_bow
```

##### War cry

<!-- splat:yaml path=splat_rule/parameters/features/war_cry -->

```yaml
economy: bonus_action
resource:
  capacity: 1
  initial: 1
  recharge:
    event: turn.start
    participant: self
    roll:
      dice:
        count: 1
        sides: 6
      add: 0
    succeeds_on:
    - 5
    - 6
  cost: 1
  recovery_also:
  - short_rest
  - long_rest
targets:
  choice:
  - self
  - one_creature_that_can_see_or_hear_self
effects:
- op: temporary_hit_points
  average: 16
  amount:
    dice:
      count: 2
      sides: 10
    add: 5
- op: advantage
  roll: attack
  ends:
    event: turn.start
    participant: self
    occurrence: next
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` athletics ` | ` 9 ` |
| ` perception ` | ` 3 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["cold"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 13 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["giant"] ` |
| ` understands ` | ` ["giant"] ` |

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
