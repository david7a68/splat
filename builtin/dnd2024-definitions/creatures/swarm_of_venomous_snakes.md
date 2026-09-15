---
splat_schema: 1
splat_id: 1a810d5d-1e00-5c72-be2b-30118d102ace
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_venomous_snakes
splat_name: Swarm of Venomous Snakes
splat_origin:
  key: dnd2024:reference/creature/swarm_of_venomous_snakes
splat_sources:
- title: SRD 5.2.1
  locator: pp. 362, 363
  url: https://www.dndbeyond.com/srd
---

# Swarm of Venomous Snakes

## Source text

<!-- SRD page 362 -->

Medium Swarm of Tiny Beasts, Unaligned\
AC 14 Initiative +4 (14)\
HP 36 (8d8)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 8 −1 −1 Dex 18 +4 +4 Con 11 +0 +0\
Int 1 −5 −5 Wis 10 +0 +0 Cha 3 −4 −4\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Charmed, Frightened, Grappled, Paralyzed,\
Petrified, Prone, Restrained, Stunned\
Senses Blindsight 10 ft.; Passive Perception 10\
Languages None\
CR 2 (XP 450; PB +2)\

<!-- SRD page 363 -->


### Traits

Swarm. The swarm can occupy another creature’s\
space and vice versa, and the swarm can move through\
any opening large enough for a Tiny snake. The swarm\
can’t regain Hit Points or gain Temporary Hit Points.\

### Actions

Bites. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (1d8 + 4)\
Piercing damage—or 6 (1d4 + 4) Piercing damage if the\
swarm is Bloodied—plus 10 (3d6) Poison damage.\

Original layout: [[../../srd-5.2.1/pages/362|page 362]], [[../../srd-5.2.1/pages/363|page 363]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` size_options_text ` | ` Medium Swarm of Tiny ` |
| ` tags_text ` | ` s ` |
| ` movement_text ` | ` 30 ft., Swim 30 ft. ` |
| ` resistances_text ` | ` Bludgeoning, Piercing, Slashing ` |
| ` immunities_text ` | ` Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned ` |
| ` senses_text ` | ` Blindsight 10 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 36 (8d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 8 ` | ` -1 ` |
| ` Dexterity ` | ` 18 ` | ` 4 ` |
| ` Constitution ` | ` 11 ` | ` 0 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 3 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 36 ` |  |
| ` initiative ` | ` 4 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/swarm_of_venomous_snakes ` |
| ` implementation ` | ` dnd2024:creature_swarm_of_venomous_snakes ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["bludgeoning", "piercing", "slashing"] ` |
| ` condition_immunities ` | ` ["charmed", "frightened", "grappled", "paralyzed", "petrified", "prone", "restrained", "stunned"] ` |
| ` passive_perception ` | ` 10 ` |

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Swarm

<!-- splat:yaml path=splat_rule/parameters/features/swarm -->

```yaml
economy: passive
op: swarm.form
shared_spaces_bidirectional: true
minimum_opening:
  creature_size: tiny
  creature_shape: snake
regain_hp: false
gain_temporary_hp: false
```

##### Bites

<!-- splat:yaml path=splat_rule/parameters/features/bites -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 6
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 8
      add: 4
    average: 8
  - type: poison
    amount:
      dice:
        count: 3
        sides: 6
      add: 0
    average: 10
  replacement_when:
    if:
      actor_bloodied: true
    components:
    - type: piercing
      amount:
        dice:
          count: 1
          sides: 4
        add: 4
      average: 6
    - type: poison
      amount:
        dice:
          count: 3
          sides: 6
        add: 0
      average: 10
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
