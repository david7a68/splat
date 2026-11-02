---
splat_schema: 1
splat_id: 9607ffda-788a-5ade-8b84-7afa39d5dfa5
splat_kind: creature
splat_key: dnd2024:creatures/gargoyle
splat_name: Gargoyle
splat_origin:
  key: dnd2024:reference/creature/gargoyle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 286
  url: https://www.dndbeyond.com/srd
---

# Gargoyle

## Source text

<!-- SRD page 286 -->

Medium Elemental, Chaotic Evil\
AC 15 Initiative +2 (12)\
HP 67 (9d8 + 27)\
Speed 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 11 +0 +0 Con 16 +3 +3\
Int 6 −2 −2 Wis 11 +0 +0 Cha 7 −2 −2\
Skills Stealth +4\
Immunities Poison; Exhaustion, Petrified, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Primordial (Terran)\
CR 2 (XP 450; PB +2)

### Traits

Flyby. The gargoyle doesn’t provoke an Opportunity Attack when it flies out of an enemy’s reach.

### Actions

Multiattack. The gargoyle makes two Claw attacks.

Claw. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/286|page 286]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` elemental ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` movement_text ` | ` 30 ft., Fly 60 ft. ` |
| ` skills_text ` | ` Stealth +4 ` |
| ` immunities_text ` | ` Poison; Exhaustion, Petrified, Poisoned ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Primordial (Terran) ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 67 (9d8 + 27) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 15 ` | ` 2 ` |
| ` Dexterity ` | ` 11 ` | ` 0 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 6 ` | ` -2 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 7 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 15 ` |  |
| ` max_hp ` | ` 67 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/gargoyle ` |
| ` implementation ` | ` dnd2024:creature_gargoyle ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "petrified", "poisoned"] ` |
| ` passive_perception ` | ` 10 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 4 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["primordial"] ` |
| ` primordial_dialects ` | ` ["terran"] ` |

#### Features

##### Flyby

<!-- splat:values path=splat_rule/parameters/features/flyby -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` opportunity_attack.suppress ` |
| ` trigger ` | ` actor.leaves_enemy_reach ` |
| ` movement_modes ` | ` ["fly"] ` |

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 4
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 2
        sides: 4
      add: 2
    average: 7
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: claw
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
