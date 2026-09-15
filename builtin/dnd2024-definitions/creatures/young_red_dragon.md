---
splat_schema: 1
splat_id: 968ba029-f6ed-5e58-94a3-afa191c0535e
splat_kind: creature
splat_key: dnd2024:creatures/young_red_dragon
splat_name: Young Red Dragon
splat_origin:
  key: dnd2024:reference/creature/young_red_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 318
  url: https://www.dndbeyond.com/srd
---

# Young Red Dragon

## Source text

<!-- SRD page 318 -->

Large Dragon (Chromatic), Chaotic Evil\
AC 18 Initiative +4 (14)\
HP 178 (17d10 + 85)\
Speed 40 ft., Climb 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 10 +0 +4 Con 21 +5 +5\
Int 14 +2 +2 Wis 11 +0 +4 Cha 19 +4 +4\
Skills Perception +8, Stealth +4\
Immunities Fire\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 18\
Languages Common, Draconic\
CR 10 (XP 5,900; PB +4)\

### Actions

Multiattack. The dragon makes three Rend attacks.\
Rend. Melee Attack Roll: +10, reach 10 ft. Hit: 13 (2d6\
+ 6) Slashing damage plus 3 (1d6) Fire damage.\
Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 17, each creature in a 30-foot Cone. Failure: 56\
(16d6) Fire damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/318|page 318]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Chromatic) ` |
| ` movement_text ` | ` 40 ft., Climb 40 ft., Fly 80 ft. ` |
| ` skills_text ` | ` Perception +8, Stealth +4 ` |
| ` immunities_text ` | ` Fire ` |
| ` senses_text ` | ` Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 18 ` |
| ` languages_text ` | ` Common, Draconic ` |
| ` cr_text ` | ` 10 (XP 5,900; PB +4) ` |
| ` hit_points_text ` | ` 178 (17d10 + 85) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 23 ` | ` 6 ` |
| ` Dexterity ` | ` 10 ` | ` 4 ` |
| ` Constitution ` | ` 21 ` | ` 5 ` |
| ` Intelligence ` | ` 14 ` | ` 2 ` |
| ` Wisdom ` | ` 11 ` | ` 4 ` |
| ` Charisma ` | ` 19 ` | ` 4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 178 ` |  |
| ` initiative ` | ` 4 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` climb_speed ` | ` 40 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/young_red_dragon ` |
| ` implementation ` | ` dnd2024:creature_young_red_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |
| ` passive_perception ` | ` 18 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 8 ` |
| ` stealth ` | ` 4 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 30 ` |
| ` 1 ` | ` darkvision ` | ` 120 ` |

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
attack_bonus: 10
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
      add: 6
    average: 13
  - type: fire
    amount:
      dice:
        count: 1
        sides: 6
      add: 0
    average: 3
```

##### Fire breath

<!-- splat:yaml path=splat_rule/parameters/features/fire_breath -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 17
targets:
  kind: creature
  count: all
  area:
    shape: cone
    origin: actor
    size_ft: 30
failure:
- op: damage
  components:
  - type: fire
    amount:
      dice:
        count: 16
        sides: 6
      add: 0
    average: 56
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
