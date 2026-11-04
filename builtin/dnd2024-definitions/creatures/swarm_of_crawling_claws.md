---
splat_schema: 2
splat_id: f343b847-244a-535f-97ae-6aef632c3431
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_crawling_claws
splat_name: Swarm of Crawling Claws
splat_origin: dnd2024:reference/creature/swarm_of_crawling_claws
splat_sources:
- SRD 5.2.1, pp. 278, https://www.dndbeyond.com/srd
---

# Swarm of Crawling Claws


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium Swarm of Tiny |
| Type | Undead |
| Alignment | Neutral Evil |
| Armor Class | 12 |
| Hit Points | 49 (11d8) |
| Initiative | +2 (12) |
| Speed | 30 ft., Climb 30 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Senses | Blindsight 30 ft.; Passive Perception 10 |
| Languages | Understands Common but can’t speak |
| Resistances | Bludgeoning, Piercing, Slashing |
| Immunities | Necrotic, Poison; Charmed, Exhaustion, Frightened, Grappled, Incapacitated, Paralyzed, Petrified, Poisoned, Prone, Restrained, Stunned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +2 |
| Dex | 14 | +2 | +2 |
| Con | 11 | +0 | +0 |
| Int | 5 | -3 | -3 |
| Wis | 10 | +0 | +0 |
| Cha | 4 | -3 | -3 |


## Source text

<!-- SRD page 278 -->

Medium Swarm of Tiny Undead, Neutral Evil\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 12 Initiative +2 (12)\
> HP 49 (11d8)\
> Speed 30 ft., Climb 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 14 +2 +2 Dex 14 +2 +2 Con 11 +0 +0\
> Int 5 −3 −3 Wis 10 +0 +0 Cha 4 −3 −3\
> Resistances Bludgeoning, Piercing, Slashing\
> Immunities Necrotic, Poison; Charmed, Exhaustion, Frightened, Grappled, Incapacitated, Paralyzed, Petrified, Poisoned, Prone, Restrained, Stunned\
> Senses Blindsight 30 ft.; Passive Perception 10\
> Languages Understands Common but can’t speak\
> CR 3 (XP 700; PB +2)


### Traits

Swarm. The swarm can occupy another creature’s space and vice versa, and the swarm can move through any opening large enough for a Tiny creature. The swarm can’t regain Hit Points or gain Temporary Hit Points.

### Actions

Swarm of Grasping Hands. Melee Attack Roll: +4, reach 5 ft. Hit: 20 (4d8 + 2) Necrotic damage, or 11 (2d8 + 2) Necrotic damage if the swarm is Bloodied. If the target is a Medium or smaller creature, it has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/278|page 278]].







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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> climb_speed:
>   unit: ft
>   value: 30
> ```

