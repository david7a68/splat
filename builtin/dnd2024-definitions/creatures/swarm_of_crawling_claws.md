---
splat_schema: 1
splat_id: f343b847-244a-535f-97ae-6aef632c3431
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_crawling_claws
splat_name: Swarm of Crawling Claws
splat_origin:
  key: dnd2024:reference/creature/swarm_of_crawling_claws
splat_sources:
- title: SRD 5.2.1
  locator: pp. 278
  url: https://www.dndbeyond.com/srd
---

# Swarm of Crawling Claws

## Source text

<!-- SRD page 278 -->

Medium Swarm of Tiny Undead, Neutral Evil\
AC 12 Initiative +2 (12)\
HP 49 (11d8)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 14 +2 +2 Con 11 +0 +0\
Int 5 −3 −3 Wis 10 +0 +0 Cha 4 −3 −3\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Necrotic, Poison; Charmed, Exhaustion, Frightened, Grappled, Incapacitated, Paralyzed, Petrified, Poisoned, Prone, Restrained, Stunned\
Senses Blindsight 30 ft.; Passive Perception 10\
Languages Understands Common but can’t speak\
CR 3 (XP 700; PB +2)

### Traits

Swarm. The swarm can occupy another creature’s space and vice versa, and the swarm can move through any opening large enough for a Tiny creature. The swarm can’t regain Hit Points or gain Temporary Hit Points.

### Actions

Swarm of Grasping Hands. Melee Attack Roll: +4, reach 5 ft. Hit: 20 (4d8 + 2) Necrotic damage, or 11 (2d8 + 2) Necrotic damage if the swarm is Bloodied. If the target is a Medium or smaller creature, it has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/278|page 278]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` undead ` |
| ` alignment ` | ` Neutral Evil ` |
| ` size_options_text ` | ` Medium Swarm of Tiny ` |
| ` movement_text ` | ` 30 ft., Climb 30 ft. ` |
| ` resistances_text ` | ` Bludgeoning, Piercing, Slashing ` |
| ` immunities_text ` | ` Necrotic, Poison; Charmed, Exhaustion, Frightened, Grappled, Incapacitated, Paralyzed, Petrified, Poisoned, Prone, Restrained, Stunned ` |
| ` senses_text ` | ` Blindsight 30 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Understands Common but can’t speak ` |
| ` cr_text ` | ` 3 (XP 700; PB +2) ` |
| ` hit_points_text ` | ` 49 (11d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 14 ` | ` 2 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 11 ` | ` 0 ` |
| ` Intelligence ` | ` 5 ` | ` -3 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 4 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 49 ` |  |
| ` initiative ` | ` 2 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/swarm_of_crawling_claws ` |
| ` implementation ` | ` dnd2024:creature_swarm_of_crawling_claws ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["bludgeoning", "piercing", "slashing"] ` |
| ` damage_immunities ` | ` ["necrotic", "poison"] ` |
| ` condition_immunities ` | ` ["charmed", "exhaustion", "frightened", "grappled", "incapacitated", "paralyzed", "petrified", "poisoned", "prone", "restrained", "stunned"] ` |

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

##### Swarm of grasping hands

<!-- splat:yaml path=splat_rule/parameters/features/swarm_of_grasping_hands -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 4
  reach_ft: 5
hit:
  damage:
    if:
      self_bloodied: true
    then:
    - average: 11
      roll:
        count: 2
        sides: 8
        modifier: 2
      type: necrotic
    else:
    - average: 20
      roll:
        count: 4
        sides: 8
        modifier: 2
      type: necrotic
  prone:
    condition: prone
    maximum_size: medium
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 30 ` |
| ` passive_perception ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands:
- common
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
