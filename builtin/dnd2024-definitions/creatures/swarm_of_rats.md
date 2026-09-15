---
splat_schema: 1
splat_id: 8f82174e-ed19-5709-85e6-a3e3d2a1d8a7
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_rats
splat_name: Swarm of Rats
splat_origin:
  key: dnd2024:reference/creature/swarm_of_rats
splat_sources:
- title: SRD 5.2.1
  locator: pp. 362
  url: https://www.dndbeyond.com/srd
---

# Swarm of Rats

## Source text

<!-- SRD page 362 -->

Medium Swarm of Tiny Beasts, Unaligned\
AC 10 Initiative +0 (10)\
HP 14 (4d8 − 4)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 9 −1 −1 Dex 11 +0 +2 Con 9 −1 −1\
Int 2 −4 −4 Wis 10 +0 +0 Cha 3 −4 −4\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Charmed, Frightened, Grappled, Paralyzed,\
Petrified, Prone, Restrained, Stunned\
Senses Darkvision 30 ft.; Passive Perception 10\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Traits

Swarm. The swarm can occupy another creature’s\
space and vice versa, and the swarm can move through\
any opening large enough for a Tiny rat. The swarm\
can’t regain Hit Points or gain Temporary Hit Points.\

### Actions

Bites. Melee Attack Roll: +2, reach 5 ft. Hit: 5 (2d4)\
Piercing damage, or 2 (1d4) Piercing damage if the\
swarm is Bloodied.\

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
| ` movement_text ` | ` 30 ft., Climb 30 ft. ` |
| ` resistances_text ` | ` Bludgeoning, Piercing, Slashing ` |
| ` immunities_text ` | ` Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned ` |
| ` senses_text ` | ` Darkvision 30 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 14 (4d8 - 4) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 9 ` | ` -1 ` |
| ` Dexterity ` | ` 11 ` | ` 2 ` |
| ` Constitution ` | ` 9 ` | ` -1 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 3 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 10 ` |  |
| ` max_hp ` | ` 14 ` |  |
| ` initiative ` | ` 0 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` climb_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/swarm_of_rats ` |
| ` implementation ` | ` dnd2024:creature_swarm_of_rats ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["bludgeoning", "piercing", "slashing"] ` |
| ` condition_immunities ` | ` ["charmed", "frightened", "grappled", "paralyzed", "petrified", "prone", "restrained", "stunned"] ` |

#### Features

##### Swarm

<!-- splat:values path=splat_rule/parameters/features/swarm -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` share_creature_spaces ` | ` true ` |
| ` minimum_opening_size ` | ` tiny ` |
| ` can_regain_hp ` | ` false ` |
| ` can_gain_temporary_hp ` | ` false ` |

##### Bites

<!-- splat:yaml path=splat_rule/parameters/features/bites -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 2
  reach_ft: 5
hit:
  damage:
    if:
      self_bloodied: true
    then:
    - average: 2
      roll:
        count: 1
        sides: 4
        modifier: 0
      type: piercing
    else:
    - average: 5
      roll:
        count: 2
        sides: 4
        modifier: 0
      type: piercing
```

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 30 ` |
| ` passive_perception ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
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
