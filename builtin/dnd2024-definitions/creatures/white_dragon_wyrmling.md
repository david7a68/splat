---
splat_schema: 1
splat_id: dd46792d-7e1b-558d-806e-a0264cb53911
splat_kind: creature
splat_key: dnd2024:creatures/white_dragon_wyrmling
splat_name: White Dragon Wyrmling
splat_origin:
  key: dnd2024:reference/creature/white_dragon_wyrmling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 339, 340
  url: https://www.dndbeyond.com/srd
---

# White Dragon Wyrmling

## Source text

<!-- SRD page 339 -->

Medium Dragon (Chromatic), Chaotic Evil\
AC 16 Initiative +2 (12)\
HP 32 (5d8 + 10)\
Speed 30 ft., Burrow 15 ft., Fly 60 ft., Swim 30 ft.\

<!-- SRD page 340 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 10 +0 +2 Con 14 +2 +2\
Int 5 −3 −3 Wis 10 +0 +2 Cha 11 +0 +0\
Skills Perception +4, Stealth +2\
Immunities Cold\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 14\
Languages Draconic\
CR 2 (450 XP; PB +2)\

### Traits

Ice Walk. The dragon can move across and climb icy\
surfaces without needing to make an ability check.\
Additionally, Difficult Terrain composed of ice or snow\
doesn’t cost it extra movement.\

### Actions

Multiattack. The dragon makes two Rend attacks.\
Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2)\
Slashing damage plus 2 (1d4) Cold damage.\
Cold Breath (Recharge 5–6). Constitution Saving\
Throw: DC 12, each creature in a 15-foot Cone. Failure:\
22 (5d8) Cold damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/339|page 339]], [[../../srd-5.2.1/pages/340|page 340]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Chromatic) ` |
| ` movement_text ` | ` 30 ft., Burrow 15 ft., Fly 60 ft., Swim 30 ft. ` |
| ` skills_text ` | ` Perception +4, Stealth +2 ` |
| ` immunities_text ` | ` Cold ` |
| ` senses_text ` | ` Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 ` |
| ` languages_text ` | ` Draconic ` |
| ` cr_text ` | ` 2 (450 XP; PB +2) ` |
| ` hit_points_text ` | ` 32 (5d8 + 10) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 14 ` | ` 2 ` |
| ` Dexterity ` | ` 10 ` | ` 2 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 5 ` | ` -3 ` |
| ` Wisdom ` | ` 10 ` | ` 2 ` |
| ` Charisma ` | ` 11 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 16 ` |  |
| ` max_hp ` | ` 32 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` burrow_speed ` | ` 15 ` | ` ft ` |
| ` fly_speed ` | ` 60 ` | ` ft ` |
| ` swim_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/white_dragon_wyrmling ` |
| ` implementation ` | ` dnd2024:creature_white_dragon_wyrmling ` |

### Parameters

#### Features

##### Ice walk

<!-- splat:yaml path=splat_rule/parameters/features/ice_walk -->

```yaml
economy: passive
movement_checks_waived:
  surfaces:
  - ice
  modes:
  - walk
  - climb
difficult_terrain_extra_cost_ignored:
- ice
- snow
```

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

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
  average: 6
  amount:
    dice:
      count: 1
      sides: 8
    add: 2
  damage_type: slashing
- op: damage
  average: 2
  amount:
    dice:
      count: 1
      sides: 4
    add: 0
  damage_type: cold
reach_feet: 5
```

##### Cold breath

<!-- splat:yaml path=splat_rule/parameters/features/cold_breath -->

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
ability: constitution
dc: 12
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 15
    origin: self
failure:
- op: damage
  average: 22
  amount:
    dice:
      count: 5
      sides: 8
    add: 0
  damage_type: cold
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
| ` damage_types ` | ` ["cold"] ` |

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
