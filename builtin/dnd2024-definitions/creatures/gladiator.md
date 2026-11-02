---
splat_schema: 1
splat_id: af7b6e6c-088d-5f7c-be06-016ec7d3d773
splat_kind: creature
splat_key: dnd2024:creatures/gladiator
splat_name: Gladiator
splat_origin:
  key: dnd2024:reference/creature/gladiator
splat_sources:
- title: SRD 5.2.1
  locator: pp. 289
  url: https://www.dndbeyond.com/srd
---

# Gladiator

## Source text

<!-- SRD page 289 -->

Medium or Small Humanoid, Neutral\
AC 16 Initiative +5 (15)\
HP 112 (15d8 + 45)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +7 Dex 15 +2 +5 Con 16 +3 +6\
Int 10 +0 +0 Wis 12 +1 +4 Cha 15 +2 +2\
Skills Athletics +10, Performance +5\
Gear Shield, Spears (3), Studded Leather Armor\
Senses Passive Perception 11\
Languages Common\
CR 5 (XP 1,800; PB +3)

### Actions

Multiattack. The gladiator makes three Spear attacks. It can replace one attack with a use of Shield Bash.

Spear. Melee or Ranged Attack Roll: +7, reach 5 ft. or range 20/60 ft. Hit: 11 (2d6 + 4) Piercing damage.

Shield Bash. Strength Saving Throw: DC 15, one creature within 5 feet that the gladiator can see. Failure: 9 (2d4 + 4) Bludgeoning damage. If the target is a Medium or smaller creature, it has the Prone condition.

### Reactions

Parry. Trigger: The gladiator is hit by a melee attack roll while holding a weapon. Response: The gladiator adds 3 to its AC against that attack, possibly causing it to miss.

Original layout: [[../../srd-5.2.1/pages/289|page 289]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Athletics +10, Performance +5 ` |
| ` gear_text ` | ` Shield, Spears (3), Studded Leather Armor ` |
| ` senses_text ` | ` Passive Perception 11 ` |
| ` languages_text ` | ` Common ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 112 (15d8 + 45) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 7 ` |
| ` Dexterity ` | ` 15 ` | ` 5 ` |
| ` Constitution ` | ` 16 ` | ` 6 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 12 ` | ` 4 ` |
| ` Charisma ` | ` 15 ` | ` 2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 16 ` |  |
| ` max_hp ` | ` 112 ` |  |
| ` initiative ` | ` 5 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/gladiator ` |
| ` implementation ` | ` dnd2024:creature_gladiator ` |

### Parameters

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: spear
  count: 3
replacements:
- replace_count: 1
  maximum_uses: 1
  operation:
    operation: use_feature
    feature: shield_bash
```

##### Spear

<!-- splat:yaml path=splat_rule/parameters/features/spear -->

```yaml
economy: action
operation: attack
attack:
  mode: melee_or_ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 7
  reach_ft: 5
  range_ft: 20
  long_range_ft: 60
hit:
  damage:
  - average: 11
    roll:
      count: 2
      sides: 6
      modifier: 4
    type: piercing
```

##### Shield bash

<!-- splat:yaml path=splat_rule/parameters/features/shield_bash -->

```yaml
economy: action
operation: saving_throw
ability: strength
dc: 15
targets:
  kind: creature
  count: 1
  range_ft: 5
  visible: true
failure:
  damage:
  - average: 9
    roll:
      count: 2
      sides: 4
      modifier: 4
    type: bludgeoning
  prone:
    maximum_size: medium
success:
  effects: []
```

##### Parry

<!-- splat:yaml path=splat_rule/parameters/features/parry -->

```yaml
economy: reaction
operation: modify_ac
trigger: self.hit_by_melee_attack
requires:
  holding_weapon: true
bonus: 3
scope: triggering_attack
reevaluate_hit: true
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` athletics ` | ` 10 ` |
| ` performance ` | ` 5 ` |

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
| ` row_1 ` | ` Shield ` | ` 1 ` |
| ` row_2 ` | ` Spears ` | ` 3 ` |
| ` row_3 ` | ` Studded leather armor ` | ` 1 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
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
