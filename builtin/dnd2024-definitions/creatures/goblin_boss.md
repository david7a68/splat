---
splat_schema: 1
splat_id: a3cdd71e-c01d-5b58-8196-b24aaa70472a
splat_kind: creature
splat_key: dnd2024:creatures/goblin_boss
splat_name: Goblin Boss
splat_origin:
  key: dnd2024:reference/creature/goblin_boss
splat_sources:
- title: SRD 5.2.1
  locator: pp. 290
  url: https://www.dndbeyond.com/srd
---

# Goblin Boss

## Source text

<!-- SRD page 290 -->

Small Fey (Goblinoid), Chaotic Neutral\
AC 17 Initiative +2 (12)\
HP 21 (6d6)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 15 +2 +2 Con 10 +0 +0\
Int 10 +0 +0 Wis 8 −1 −1 Cha 10 +0 +0\
Skills Stealth +6\
Gear Chain Shirt, Scimitar, Shield, Shortbow\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages Common, Goblin\
CR 1 (XP 200; PB +2)

### Actions

Multiattack. The goblin makes two attacks, using Scimitar or Shortbow in any combination.

Scimitar. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Slashing damage, plus 2 (1d4) Slashing damage if the attack roll had Advantage.

Shortbow. Ranged Attack Roll: +4, range 80/320 ft. Hit: 5 (1d6 + 2) Piercing damage, plus 2 (1d4) Piercing damage if the attack roll had Advantage.

### Bonus Actions

Nimble Escape. The goblin takes the Disengage or Hide action.

### Reactions

Redirect Attack. Trigger: A creature the goblin can see makes an attack roll against it. Response: The goblin chooses a Small or Medium ally within 5 feet of itself. The goblin and that ally swap places, and the ally becomes the target of the attack instead.

Original layout: [[../../srd-5.2.1/pages/290|page 290]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` small ` |
| ` type ` | ` fey ` |
| ` alignment ` | ` Chaotic Neutral ` |
| ` tags_text ` | ` (Goblinoid) ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Stealth +6 ` |
| ` gear_text ` | ` Chain Shirt, Scimitar, Shield, Shortbow ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 9 ` |
| ` languages_text ` | ` Common, Goblin ` |
| ` cr_text ` | ` 1 (XP 200; PB +2) ` |
| ` hit_points_text ` | ` 21 (6d6) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 0 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 8 ` | ` -1 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 21 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/goblin_boss ` |
| ` implementation ` | ` dnd2024:creature_goblin_boss ` |

### Parameters

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_choice
  count: 2
  options:
  - scimitar
  - shortbow
```

##### Scimitar

<!-- splat:yaml path=splat_rule/parameters/features/scimitar -->

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
  - average: 5
    roll:
      count: 1
      sides: 6
      modifier: 2
    type: slashing
  conditional_damage:
    if:
      attack_had_advantage: true
    damage:
    - roll:
        count: 1
        sides: 4
        modifier: 0
      type: slashing
```

##### Shortbow

<!-- splat:yaml path=splat_rule/parameters/features/shortbow -->

```yaml
economy: action
operation: attack
attack:
  mode: ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 4
  range_ft: 80
  long_range_ft: 320
hit:
  damage:
  - average: 5
    roll:
      count: 1
      sides: 6
      modifier: 2
    type: piercing
  conditional_damage:
    if:
      attack_had_advantage: true
    damage:
    - roll:
        count: 1
        sides: 4
        modifier: 0
      type: piercing
```

##### Nimble escape

<!-- splat:values path=splat_rule/parameters/features/nimble_escape -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` operation ` | ` choose_action ` |
| ` options ` | ` ["disengage", "hide"] ` |
| ` count ` | ` 1 ` |

##### Redirect attack

<!-- splat:yaml path=splat_rule/parameters/features/redirect_attack -->

```yaml
economy: reaction
operation: redirect_attack
trigger: visible_creature.attack_roll_against_self
ally:
  within_ft: 5
  size:
  - small
  - medium
  chooser: self
swap_positions: true
new_target: chosen_ally
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 6 ` |

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Item:
    path: item
    type: symbol
  Quantity:
    path: quantity
    type: integer
-->

| Key | Item | Quantity |
| --- | --- | --- |
| ` row_1 ` | ` Chain shirt ` | ` 1 ` |
| ` row_2 ` | ` Scimitar ` | ` 1 ` |
| ` row_3 ` | ` Shield ` | ` 1 ` |
| ` row_4 ` | ` Shortbow ` | ` 1 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 9 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
- goblin
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
