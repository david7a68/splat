---
splat_schema: 1
splat_id: ce8d49a6-733a-5061-9adb-1c1c5c142862
splat_kind: creature
splat_key: dnd2024:creatures/magma_mephit
splat_name: Magma Mephit
splat_origin:
  key: dnd2024:reference/creature/magma_mephit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 307
  url: https://www.dndbeyond.com/srd
---

# Magma Mephit

## Source text

<!-- SRD page 307 -->

Small Elemental, Neutral Evil\
AC 11 Initiative +1 (11)\
HP 18 (4d6 + 4)\
Speed 30 ft., Fly 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 8 −1 −1 Dex 12 +1 +1 Con 12 +1 +1\
Int 7 −2 −2 Wis 10 +0 +0 Cha 10 +0 +0\
Skills Stealth +3\
Vulnerabilities Cold\
Immunities Fire, Poison; Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Primordial (Ignan, Terran)\
CR 1/2 (XP 100; PB +2)\

### Traits

Death Burst. The mephit explodes when it dies. Dex-\
terity Saving Throw: DC 11, each creature in a 5-foot\
Emanation originating from the mephit. Failure: 7 (2d6)\
Fire damage. Success: Half damage.\

### Actions

Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 3 (1d4 + 1)\
Slashing damage plus 3 (1d6) Fire damage.\
Fire Breath (Recharge 6). Dexterity Saving Throw: DC\
11, each creature in a 15-foot Cone. Failure: 7 (2d6) Fire\
damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/307|page 307]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` small ` |
| ` type ` | ` elemental ` |
| ` alignment ` | ` Neutral Evil ` |
| ` movement_text ` | ` 30 ft., Fly 30 ft. ` |
| ` skills_text ` | ` Stealth +3 ` |
| ` vulnerabilities_text ` | ` Cold ` |
| ` immunities_text ` | ` Fire, Poison; Exhaustion, Poisoned ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Primordial (Ignan, Terran) ` |
| ` cr_text ` | ` 1/2 (XP 100; PB +2) ` |
| ` hit_points_text ` | ` 18 (4d6 + 4) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 8 ` | ` -1 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 7 ` | ` -2 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 18 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` fly_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/magma_mephit ` |
| ` implementation ` | ` dnd2024:creature_magma_mephit ` |

### Parameters

#### Features

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 3
targets:
  count: 1
hit:
- op: damage
  average: 3
  amount:
    dice:
      count: 1
      sides: 4
    add: 1
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
    - 6
  cost: 1
  recovery_also:
  - short_rest
  - long_rest
op: save
ability: dexterity
dc: 11
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 15
    origin: self
failure:
- op: damage
  average: 7
  amount:
    dice:
      count: 2
      sides: 6
    add: 0
  damage_type: fire
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
```

##### Death burst

<!-- splat:yaml path=splat_rule/parameters/features/death_burst -->

```yaml
economy: passive
trigger: self.died
op: save
ability: dexterity
dc: 11
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    size_feet: 5
    origin: self
failure:
- op: damage
  average: 7
  amount:
    dice:
      count: 2
      sides: 6
    add: 0
  damage_type: fire
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 3 ` |

#### Defenses

##### Vulnerabilities

<!-- splat:values path=splat_rule/parameters/defenses/vulnerabilities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["cold"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["fire", "poison"] ` |
| ` conditions ` | ` ["exhaustion", "poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["primordial"] ` |
| ` understands ` | ` ["primordial"] ` |
| ` dialects ` | ` ["ignan", "terran"] ` |

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
