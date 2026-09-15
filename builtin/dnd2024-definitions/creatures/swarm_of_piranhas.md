---
splat_schema: 1
splat_id: 9c5354de-82e7-5e8a-8bb0-7514dd3a73c0
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_piranhas
splat_name: Swarm of Piranhas
splat_origin:
  key: dnd2024:reference/creature/swarm_of_piranhas
splat_sources:
- title: SRD 5.2.1
  locator: pp. 362
  url: https://www.dndbeyond.com/srd
---

# Swarm of Piranhas

## Source text

<!-- SRD page 362 -->

Medium Swarm of Tiny Beasts, Unaligned\
AC 13 Initiative +3 (13)\
HP 28 (8d8 − 8)\
Speed 5 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 16 +3 +3 Con 9 −1 −1\
Int 1 −5 −5 Wis 7 −2 −2 Cha 2 −4 −4\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Charmed, Frightened, Grappled, Paralyzed,\
Petrified, Prone, Restrained, Stunned\
Senses Darkvision 60 ft.; Passive Perception 8\
Languages None\
CR 1 (XP 200; PB +2)\

### Traits

Swarm. The swarm can occupy another creature’s\
space and vice versa, and the swarm can move\
through any opening large enough for a Tiny piranha.\
The swarm can’t regain Hit Points or gain Temporary\
Hit Points.\
Water Breathing. The swarm can breathe only\
underwater.\

### Actions

Bites. Melee Attack Roll: +5 (with Advantage if the\
target doesn’t have all its Hit Points), reach 5 ft. Hit: 8\
(2d4 + 3) Piercing damage, or 5 (1d4 + 3) Piercing dam-\
age if the swarm is Bloodied.\

Original layout: [[../../srd-5.2.1/pages/362|page 362]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` size_options_text ` | ` Medium Swarm of Tiny ` |
| ` tags_text ` | ` s ` |
| ` movement_text ` | ` 5 ft., Swim 40 ft. ` |
| ` resistances_text ` | ` Bludgeoning, Piercing, Slashing ` |
| ` immunities_text ` | ` Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 8 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1 (XP 200; PB +2) ` |
| ` hit_points_text ` | ` 28 (8d8 - 8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 13 ` | ` 1 ` |
| ` Dexterity ` | ` 16 ` | ` 3 ` |
| ` Constitution ` | ` 9 ` | ` -1 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 7 ` | ` -2 ` |
| ` Charisma ` | ` 2 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 28 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 5 ` | ` ft ` |
| ` swim_speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/swarm_of_piranhas ` |
| ` implementation ` | ` dnd2024:creature_swarm_of_piranhas ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["bludgeoning", "piercing", "slashing"] ` |
| ` condition_immunities ` | ` ["charmed", "frightened", "grappled", "paralyzed", "petrified", "prone", "restrained", "stunned"] ` |
| ` passive_perception ` | ` 8 ` |

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

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
  creature_shape: piranha
regain_hp: false
gain_temporary_hp: false
```

##### Water breathing

<!-- splat:values path=splat_rule/parameters/features/water_breathing -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` breathing.allow ` |
| ` media ` | ` ["water"] ` |
| ` exclusive ` | ` true ` |

##### Bites

<!-- splat:yaml path=splat_rule/parameters/features/bites -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 5
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
        count: 2
        sides: 4
      add: 3
    average: 8
  replacement_when:
    if:
      actor_bloodied: true
    components:
    - type: piercing
      amount:
        dice:
          count: 1
          sides: 4
        add: 3
      average: 5
advantage_when:
  target_hp_below_maximum: true
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
