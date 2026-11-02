---
splat_schema: 1
splat_id: 901a4cef-4135-5abc-adf5-810c1e85a6a8
splat_kind: creature
splat_key: dnd2024:creatures/ice_mephit
splat_name: Ice Mephit
splat_origin:
  key: dnd2024:reference/creature/ice_mephit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 307
  url: https://www.dndbeyond.com/srd
---

# Ice Mephit

## Source text

<!-- SRD page 307 -->

Small Elemental, Neutral Evil\
AC 11 Initiative +1 (11)\
HP 21 (6d6)\
Speed 30 ft., Fly 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 7 −2 −2 Dex 13 +1 +1 Con 10 +0 +0\
Int 9 −1 −1 Wis 11 +0 +0 Cha 12 +1 +1\
Skills Perception +2, Stealth +3\
Vulnerabilities Fire\
Immunities Cold, Poison; Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Primordial (Aquan, Auran)\
CR 1/2 (XP 100; PB +2)

### Traits

Death Burst. The mephit explodes when it dies. Constitution Saving Throw: DC 10, each creature in a 5-foot Emanation originating from the mephit. Failure: 5 (2d4) Cold damage. Success: Half damage.

### Actions

Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 3 (1d4 + 1) Slashing damage plus 2 (1d4) Cold damage.

Fog Cloud (1/Day). The mephit casts Fog Cloud, requiring no spell components and using Charisma as the spellcasting ability.

Frost Breath (Recharge 6). Constitution Saving Throw: DC 10, each creature in a 15-foot Cone. Failure: 7 (3d4) Cold damage. Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/307|page 307]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` small ` |
| ` type ` | ` elemental ` |
| ` alignment ` | ` Neutral Evil ` |
| ` movement_text ` | ` 30 ft., Fly 30 ft. ` |
| ` skills_text ` | ` Perception +2, Stealth +3 ` |
| ` vulnerabilities_text ` | ` Fire ` |
| ` immunities_text ` | ` Cold, Poison; Exhaustion, Poisoned ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 12 ` |
| ` languages_text ` | ` Primordial (Aquan, Auran) ` |
| ` cr_text ` | ` 1/2 (XP 100; PB +2) ` |
| ` hit_points_text ` | ` 21 (6d6) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 7 ` | ` -2 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 9 ` | ` -1 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 12 ` | ` 1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 21 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/ice_mephit ` |
| ` implementation ` | ` dnd2024:creature_ice_mephit ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_vulnerabilities ` | ` ["fire"] ` |
| ` damage_immunities ` | ` ["cold", "poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "poisoned"] ` |

#### Features

##### Death burst

<!-- splat:yaml path=splat_rule/parameters/features/death_burst -->

```yaml
economy: trait
operation: saving_throw
ability: constitution
dc: 10
targets:
  kind: creature
  selection: each
  area:
    shape: emanation
    size_ft: 5
    origin: self
failure:
  damage:
  - average: 5
    roll:
      count: 2
      sides: 4
      modifier: 0
    type: cold
success:
  damage_multiplier: 0.5
  other_effects: false
trigger: self.dies
```

##### Claw

<!-- splat:values path=splat_rule/parameters/features/claw -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/claw/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 3
reach_ft: 5
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/claw/hit/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Average:
    path: average
    type: integer
  Roll count:
    path: roll/count
    type: integer
  Roll sides:
    path: roll/sides
    type: integer
  Roll modifier:
    path: roll/modifier
    type: integer
  Type:
    path: type
    type: symbol
-->

| Key | Average | Roll count | Roll sides | Roll modifier | Type |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 3 ` | ` 1 ` | ` 4 ` | ` 1 ` | ` Slashing ` |
| ` row_2 ` | ` 2 ` | ` 1 ` | ` 4 ` | ` 0 ` | ` Cold ` |

##### Fog cloud

<!-- splat:yaml path=splat_rule/parameters/features/fog_cloud -->

```yaml
economy: action
resource:
  kind: daily
  uses: 1
  recovery: long_rest
operation: cast_spell
spell:
  key: dnd2024:spells/fog_cloud
ability: charisma
components_waived:
- verbal
- somatic
- material
```

##### Frost breath

<!-- splat:yaml path=splat_rule/parameters/features/frost_breath -->

```yaml
economy: action
resource:
  kind: recharge
  die: 6
  success_faces:
  - 6
  check: self.turn_start
  rest_recovery:
  - short
  - long
  capacity: 1
  initial_uses: 1
operation: saving_throw
ability: constitution
dc: 10
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 15
    origin: self
failure:
  damage:
  - average: 7
    roll:
      count: 3
      sides: 4
      modifier: 0
    type: cold
success:
  damage_multiplier: 0.5
  other_effects: false
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 2 ` |
| ` stealth ` | ` 3 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 12 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- language: primordial
  dialects:
  - aquan
  - auran
understands: []
```

#### Resolution conventions

<!-- splat:yaml path=splat_rule/parameters/resolution_conventions -->

```yaml
distance_unit: foot
duration_unit: second
damage_roll_mode:
  choice:
  - listed_average
  - roll_dice
  chooser: gm
supplied_totals: before_additional_effect_modifiers
multiattack_nested_economy_cost: 0
nested_feature_resource_cost: normal
spell_level_unless_specified: lowest_possible
spell_upcast_without_permission: false
spell_casting_time_at_least_one_minute: normal_casting_procedure
geometry:
  input: adjudication
  assumptions:
  - range_and_visibility_verified
  - area_membership_verified
  - path_and_destination_legal
unlisted_success_branch: no_effect
unlisted_duration: until_removed_by_applicable_rule
```
