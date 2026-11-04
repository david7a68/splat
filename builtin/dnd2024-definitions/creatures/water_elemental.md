---
splat_schema: 2
splat_id: fed2df36-1e53-5e3f-8b77-aef7a2b16ae5
splat_kind: creature
splat_key: dnd2024:creatures/water_elemental
splat_name: Water Elemental
splat_origin: dnd2024:reference/creature/water_elemental
splat_sources:
- SRD 5.2.1, pp. 337, https://www.dndbeyond.com/srd
---

# Water Elemental


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Elemental |
| Alignment | Neutral |
| Armor Class | 14 |
| Hit Points | 114 (12d10 + 48) |
| Initiative | +2 (12) |
| Speed | 30 ft., Swim 90 ft. |
| Challenge | 5 (XP 1,800; PB +3) |
| Senses | Darkvision 60 ft.; Passive Perception 10 |
| Languages | Primordial (Aquan) |
| Resistances | Acid, Fire |
| Immunities | Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 14 | +2 | +2 |
| Con | 18 | +4 | +4 |
| Int | 5 | -3 | -3 |
| Wis | 10 | +0 | +0 |
| Cha | 8 | -1 | -1 |


## Source text

<!-- SRD page 337 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Elemental, Neutral\
> AC 14 Initiative +2 (12)\
> HP 114 (12d10 + 48)\
> Speed 30 ft., Swim 90 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 14 +2 +2 Con 18 +4 +4\
> Int 5 −3 −3 Wis 10 +0 +0 Cha 8 −1 −1\
> Resistances Acid, Fire\
> Immunities Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious\
> Senses Darkvision 60 ft.; Passive Perception 10\
> Languages Primordial (Aquan)\
> CR 5 (XP 1,800; PB +3)


### Traits

Freeze. If the elemental takes Cold damage, its Speed decreases by 20 feet until the end of its next turn.

Water Form. The elemental can enter an enemy’s space and stop there. It can move through a space as narrow as 1 inch without expending extra movement to do so.

### Actions

Multiattack. The elemental makes two Slam attacks.

Slam. Melee Attack Roll: +7, reach 5 ft. Hit: 13 (2d8 + 4) Bludgeoning damage. If the target is a Medium or smaller creature, it has the Prone condition.

Whelm (Recharge 4–6). Strength Saving Throw: DC 15, each creature in the elemental’s space. Failure: 22 (4d8 + 4) Bludgeoning damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 14). Until the grapple ends, the target has the Restrained condition, is suffocating unless it can breathe water, and takes 9 (2d8) Bludgeoning damage at the start of each of the elemental’s turns. The elemental can grapple one Large creature or up to two Medium or smaller creatures at a time with Whelm. As an action, a creature within 5 feet of the elemental can pull a creature out of it by succeeding on a DC 14 Strength (Athletics) check. Success: Half damage only.

Original layout: [[../../srd-5.2.1/pages/337|page 337]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/water_elemental ` |
| ` implementation ` | ` dnd2024:creature_water_elemental ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["acid", "fire"] ` |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "grappled", "paralyzed", "petrified", "poisoned", "prone", "restrained", "unconscious"] ` |

#### Features

##### Freeze

<!-- splat:values path=splat_rule/parameters/features/freeze -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` trigger ` | ` self.cold_damage_taken ` |
| ` speed_modifier_ft ` | ` -20 ` |
| ` until ` | ` self.next_turn_end ` |

##### Water form

<!-- splat:values path=splat_rule/parameters/features/water_form -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` occupy_enemy_space ` | ` true ` |
| ` minimum_gap_inches ` | ` 1 ` |
| ` extra_movement_cost ` | ` 0 ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: slam
  count: 2
```

##### Slam

<!-- splat:yaml path=splat_rule/parameters/features/slam -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 7
  reach_ft: 5
hit:
  damage:
  - average: 13
    roll:
      count: 2
      sides: 8
      modifier: 4
    type: bludgeoning
  prone:
    condition: prone
    maximum_size: medium
```

##### Whelm

<!-- splat:values path=splat_rule/parameters/features/whelm -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` saving_throw ` |
| ` ability ` | ` strength ` |
| ` dc ` | ` 15 ` |

###### Resource

<!-- splat:values path=splat_rule/parameters/features/whelm/resource -->

| Field | Value |
| --- | --- |
| ` kind ` | ` recharge ` |
| ` die ` | ` 6 ` |
| ` success_faces ` | ` [4, 5, 6] ` |
| ` check ` | ` self.turn_start ` |
| ` rest_recovery ` | ` ["short", "long"] ` |
| ` capacity ` | ` 1 ` |
| ` initial_uses ` | ` 1 ` |

###### Targets

<!-- splat:values path=splat_rule/parameters/features/whelm/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` selection ` | ` each ` |
| ` location ` | ` self.space ` |

###### Failure

<!-- splat:yaml path=splat_rule/parameters/features/whelm/failure -->

```yaml
damage:
- roll:
    count: 4
    sides: 8
    modifier: 4
  type: bludgeoning
grapple:
  condition: grappled
  source: self
  escape_dc: 14
  maximum_size: large
  while_grappled:
    conditions:
    - restrained
    suffocating_unless_water_breathing: true
    damage_each_self_turn_start:
    - roll:
        count: 2
        sides: 8
        modifier: 0
      type: bludgeoning
```

###### Success

<!-- splat:values path=splat_rule/parameters/features/whelm/success -->

| Field | Value |
| --- | --- |
| ` damage_multiplier ` | ` 0.5 ` |
| ` other_effects ` | ` false ` |

###### Capacity

###### Choose one

<!-- splat:table
path: splat_rule/parameters/features/whelm/capacity/choose_one
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Size:
    path: size
    type: symbol
    optional: true
  Count:
    path: count
    type: integer
  Maximum size:
    path: maximum_size
    type: symbol
    optional: true
-->

| Key | Size | Count | Maximum size |
| --- | --- | --- | --- |
| ` row_1 ` | ` Large ` | ` 1 ` |  |
| ` row_2 ` |  | ` 2 ` | ` Medium ` |

###### Pull out

<!-- splat:values path=splat_rule/parameters/features/whelm/pull_out -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` actor_within_ft ` | ` 5 ` |
| ` check ` | ` strength_athletics ` |
| ` dc ` | ` 14 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- language: primordial
  dialects:
  - aquan
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> swim_speed:
>   unit: ft
>   value: 90
> ```

