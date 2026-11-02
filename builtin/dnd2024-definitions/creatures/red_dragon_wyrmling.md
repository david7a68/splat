---
splat_schema: 1
splat_id: 68c1923d-664c-53d9-a003-89d96fb80dcc
splat_kind: creature
splat_key: dnd2024:creatures/red_dragon_wyrmling
splat_name: Red Dragon Wyrmling
splat_origin:
  key: dnd2024:reference/creature/red_dragon_wyrmling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 318
  url: https://www.dndbeyond.com/srd
---

# Red Dragon Wyrmling

## Source text

<!-- SRD page 318 -->

Medium Dragon (Chromatic), Chaotic Evil\
AC 17 Initiative +2 (12)\
HP 75 (10d8 + 30)\
Speed 30 ft., Climb 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 10 +0 +2 Con 17 +3 +3\
Int 12 +1 +1 Wis 11 +0 +2 Cha 15 +2 +2\
Skills Perception +4, Stealth +2\
Immunities Fire\
Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14\
Languages Draconic\
CR 4 (XP 1,100; PB +2)

### Actions

Multiattack. The dragon makes two Rend attacks.

Rend. Melee Attack Roll: +6, reach 5 ft. Hit: 9 (1d10 + 4) Slashing damage plus 3 (1d6) Fire damage.

Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 13, each creature in a 15-foot Cone. Failure: 24 (7d6) Fire damage. Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/318|page 318]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Chromatic) ` |
| ` movement_text ` | ` 30 ft., Climb 30 ft., Fly 60 ft. ` |
| ` skills_text ` | ` Perception +4, Stealth +2 ` |
| ` immunities_text ` | ` Fire ` |
| ` senses_text ` | ` Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 ` |
| ` languages_text ` | ` Draconic ` |
| ` cr_text ` | ` 4 (XP 1,100; PB +2) ` |
| ` hit_points_text ` | ` 75 (10d8 + 30) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 19 ` | ` 4 ` |
| ` Dexterity ` | ` 10 ` | ` 2 ` |
| ` Constitution ` | ` 17 ` | ` 3 ` |
| ` Intelligence ` | ` 12 ` | ` 1 ` |
| ` Wisdom ` | ` 11 ` | ` 2 ` |
| ` Charisma ` | ` 15 ` | ` 2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 75 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` climb_speed ` | ` 30 ` | ` ft ` |
| ` fly_speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/red_dragon_wyrmling ` |
| ` implementation ` | ` dnd2024:creature_red_dragon_wyrmling ` |

### Parameters

#### Features

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

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
  average: 9
  amount:
    dice:
      count: 1
      sides: 10
    add: 4
  damage_type: slashing
- op: damage
  average: 3
  amount:
    dice:
      count: 1
      sides: 6
    add: 0
  damage_type: fire
reach_feet: 5
```

##### Fire breath

<!-- splat:yaml path=splat_rule/parameters/features/fire_breath -->

```yaml
economy: action
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
op: save
ability: dexterity
dc: 13
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 15
    origin: self
failure:
- op: damage
  average: 24
  amount:
    dice:
      count: 7
      sides: 6
    add: 0
  damage_type: fire
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
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
| ` perception ` | ` 4 ` |
| ` stealth ` | ` 2 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["fire"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

##### Special

<!-- splat:table
path: splat_rule/parameters/senses/special
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Sense:
    path: sense
    type: symbol
  Range feet:
    path: range_feet
    type: integer
-->

| Key | Sense | Range feet |
| --- | --- | --- |
| ` row_1 ` | ` Blindsight ` | ` 10 ` |
| ` row_2 ` | ` Darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["draconic"] ` |
| ` understands ` | ` ["draconic"] ` |

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
