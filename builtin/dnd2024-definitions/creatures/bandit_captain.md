---
splat_schema: 1
splat_id: 34d57125-2ab3-533f-8cfc-df44ed4a96ee
splat_kind: creature
splat_key: dnd2024:creatures/bandit_captain
splat_name: Bandit Captain
splat_origin:
  key: dnd2024:reference/creature/bandit_captain
splat_sources:
- title: SRD 5.2.1
  locator: pp. 261, 262
  url: https://www.dndbeyond.com/srd
---

# Bandit Captain

## Source text

<!-- SRD page 261 -->

Medium or Small Humanoid, Neutral\
AC 15 Initiative +3 (13)\
HP 52 (8d8 + 16)\
Speed 30 ft.\

<!-- SRD page 262 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +4 Dex 16 +3 +5 Con 14 +2 +2\
Int 14 +2 +2 Wis 11 +0 +2 Cha 14 +2 +2\
Skills Athletics +4, Deception +4\
Gear Pistol, Scimitar, Studded Leather Armor\
Senses Passive Perception 10\
Languages Common, Thieves’ Cant\
CR 2 (XP 450; PB +2)\

### Actions

Multiattack. The bandit makes two attacks, using Scim-\
itar and Pistol in any combination.\
Scimitar. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6\
+ 3) Slashing damage.\
Pistol. Ranged Attack Roll: +5, range 30/90 ft. Hit: 8\
(1d10 + 3) Piercing damage.\

### Reactions

Parry. Trigger: The bandit is hit by a melee attack roll\
while holding a weapon. Response: The bandit adds 2\
to its AC against that attack, possibly causing it to miss.\

Original layout: [[../../srd-5.2.1/pages/261|page 261]], [[../../srd-5.2.1/pages/262|page 262]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Athletics +4, Deception +4 ` |
| ` gear_text ` | ` Pistol, Scimitar, Studded Leather Armor ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` Common, Thieves’ Cant ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 52 (8d8 + 16) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 15 ` | ` 4 ` |
| ` Dexterity ` | ` 16 ` | ` 5 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 14 ` | ` 2 ` |
| ` Wisdom ` | ` 11 ` | ` 2 ` |
| ` Charisma ` | ` 14 ` | ` 2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 15 ` |  |
| ` max_hp ` | ` 52 ` |  |
| ` initiative ` | ` 3 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/bandit_captain ` |
| ` implementation ` | ` dnd2024:creature_bandit_captain ` |

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
  - pistol
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
  supplied_attack_total: 5
  reach_ft: 5
hit:
  damage:
  - average: 6
    roll:
      count: 1
      sides: 6
      modifier: 3
    type: slashing
```

##### Pistol

<!-- splat:yaml path=splat_rule/parameters/features/pistol -->

```yaml
economy: action
operation: attack
attack:
  mode: ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  range_ft: 30
  long_range_ft: 90
hit:
  damage:
  - average: 8
    roll:
      count: 1
      sides: 10
      modifier: 3
    type: piercing
```

##### Parry

<!-- splat:yaml path=splat_rule/parameters/features/parry -->

```yaml
economy: reaction
operation: modify_ac
trigger: self.hit_by_melee_attack
requires:
  holding_weapon: true
bonus: 2
scope: triggering_attack
reevaluate_hit: true
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` athletics ` | ` 4 ` |
| ` deception ` | ` 4 ` |

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
- row_3
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
| ` row_1 ` | ` Pistol ` | ` 1 ` |
| ` row_2 ` | ` Scimitar ` | ` 1 ` |
| ` row_3 ` | ` Studded leather armor ` | ` 1 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
- thieves_cant
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
