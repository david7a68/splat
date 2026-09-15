---
splat_schema: 1
splat_id: 3c69da88-f30f-5f64-b083-7cc44ef33822
splat_kind: creature
splat_key: dnd2024:creatures/young_blue_dragon
splat_name: Young Blue Dragon
splat_origin:
  key: dnd2024:reference/creature/young_blue_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 266
  url: https://www.dndbeyond.com/srd
---

# Young Blue Dragon

## Source text

<!-- SRD page 266 -->

Large Dragon (Chromatic), Lawful Evil\
AC 18 Initiative +4 (14)\
HP 152 (16d10 + 64)\
Speed 40 ft., Burrow 20 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 10 +0 +4 Con 19 +4 +4\
Int 14 +2 +2 Wis 13 +1 +5 Cha 17 +3 +3\
Skills Perception +9, Stealth +4\
Immunities Lightning\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 19\
Languages Common, Draconic\
CR 9 (XP 5,000; PB +4)\

### Actions

Multiattack. The dragon makes three Rend attacks.\
Rend. Melee Attack Roll: +9, reach 10 ft. Hit: 12 (2d6\
+ 5) Slashing damage plus 5 (1d10) Lightning damage.\
Lightning Breath (Recharge 5–6). Dexterity Saving\
Throw: DC 16, each creature in a 60-foot-long, 5-foot-\
wide Line. Failure: 55 (10d10) Lightning damage. Suc-\
cess: Half damage.\

Original layout: [[../../srd-5.2.1/pages/266|page 266]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Lawful Evil ` |
| ` tags_text ` | ` (Chromatic) ` |
| ` movement_text ` | ` 40 ft., Burrow 20 ft., Fly 80 ft. ` |
| ` skills_text ` | ` Perception +9, Stealth +4 ` |
| ` immunities_text ` | ` Lightning ` |
| ` senses_text ` | ` Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 19 ` |
| ` languages_text ` | ` Common, Draconic ` |
| ` cr_text ` | ` 9 (XP 5,000; PB +4) ` |
| ` hit_points_text ` | ` 152 (16d10 + 64) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 21 ` | ` 5 ` |
| ` Dexterity ` | ` 10 ` | ` 4 ` |
| ` Constitution ` | ` 19 ` | ` 4 ` |
| ` Intelligence ` | ` 14 ` | ` 2 ` |
| ` Wisdom ` | ` 13 ` | ` 5 ` |
| ` Charisma ` | ` 17 ` | ` 3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 152 ` |  |
| ` initiative ` | ` 4 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` burrow_speed ` | ` 20 ` | ` ft ` |
| ` fly_speed ` | ` 80 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/young_blue_dragon ` |
| ` implementation ` | ` dnd2024:creature_young_blue_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["lightning"] ` |
| ` passive_perception ` | ` 19 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 9 ` |
| ` stealth ` | ` 4 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:table
path: splat_rule/parameters/senses
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
  Range ft:
    path: range_ft
    type: integer
-->

| Key | Sense | Range ft |
| --- | --- | --- |
| ` row_1 ` | ` Blindsight ` | ` 30 ` |
| ` row_2 ` | ` Darkvision ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common", "draconic"] ` |

#### Features

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 9
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 2
        sides: 6
      add: 5
    average: 12
  - type: lightning
    amount:
      dice:
        count: 1
        sides: 10
      add: 0
    average: 5
```

##### Lightning breath

<!-- splat:yaml path=splat_rule/parameters/features/lightning_breath -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 16
targets:
  kind: creature
  count: all
  area:
    shape: line
    origin: actor
    size_ft: 60
    width_ft: 5
failure:
- op: damage
  components:
  - type: lightning
    amount:
      dice:
        count: 10
        sides: 10
      add: 0
    average: 55
success:
- op: damage
  from: failure_damage
  multiplier: 0.5
  round: down
recharge:
  die_sides: 6
  success_min: 5
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 3
  feature: rend
```

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |
