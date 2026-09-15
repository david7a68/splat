---
splat_schema: 1
splat_id: 5677cfbd-fc93-53ed-bd55-361eacb4d4b0
splat_kind: creature
splat_key: dnd2024:creatures/warrior_veteran
splat_name: Warrior Veteran
splat_origin:
  key: dnd2024:reference/creature/warrior_veteran
splat_sources:
- title: SRD 5.2.1
  locator: pp. 337
  url: https://www.dndbeyond.com/srd
---

# Warrior Veteran

## Source text

<!-- SRD page 337 -->

Medium or Small Humanoid, Neutral\
AC 17 Initiative +3 (13)\
HP 65 (10d8 + 20)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 13 +1 +1 Con 14 +2 +2\
Int 10 +0 +0 Wis 11 +0 +0 Cha 10 +0 +0\
Skills Athletics +5, Perception +2\
Gear Greatsword, Heavy Crossbow, Splint Armor\
Senses Passive Perception 12\
Languages Common plus one other language\
CR 3 (XP 700; PB +2)\

### Actions

Multiattack. The warrior makes two Greatsword or\
Heavy Crossbow attacks.\
Greatsword. Melee Attack Roll: +5, reach 5 ft. Hit: 10\
(2d6 + 3) Slashing damage.\
Heavy Crossbow. Ranged Attack Roll: +3, range\
100/400 ft. Hit: 12 (2d10 + 1) Piercing damage.\

### Reactions

Parry. Trigger: The warrior is hit by a melee attack roll\
while holding a weapon. Response: The warrior adds 2\
to its AC against that attack, possibly causing it to miss.\

Original layout: [[../../srd-5.2.1/pages/337|page 337]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Athletics +5, Perception +2 ` |
| ` gear_text ` | ` Greatsword, Heavy Crossbow, Splint Armor ` |
| ` senses_text ` | ` Passive Perception 12 ` |
| ` languages_text ` | ` Common plus one other language ` |
| ` cr_text ` | ` 3 (XP 700; PB +2) ` |
| ` hit_points_text ` | ` 65 (10d8 + 20) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 16 ` | ` 3 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 65 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/warrior_veteran ` |
| ` implementation ` | ` dnd2024:creature_warrior_veteran ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` athletics ` | ` 5 ` |
| ` perception ` | ` 2 ` |

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
  Count:
    path: count
    type: integer
-->

| Key | Item | Count |
| --- | --- | --- |
| ` row_1 ` | ` Greatsword ` | ` 1 ` |
| ` row_2 ` | ` Heavy crossbow ` | ` 1 ` |
| ` row_3 ` | ` Splint armor ` | ` 1 ` |

#### Senses

<!-- splat:yaml path=splat_rule/parameters/senses -->

```yaml
[]
```

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common"] ` |
| ` additional_choice_count ` | ` 1 ` |

#### Features

##### Greatsword

<!-- splat:yaml path=splat_rule/parameters/features/greatsword -->

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
  - type: slashing
    amount:
      dice:
        count: 2
        sides: 6
      add: 3
    average: 10
```

##### Heavy crossbow

<!-- splat:yaml path=splat_rule/parameters/features/heavy_crossbow -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 3
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 100
  long: 400
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 10
      add: 1
    average: 12
```

##### Parry

<!-- splat:yaml path=splat_rule/parameters/features/parry -->

```yaml
economy: reaction
trigger: melee_attack.hit_actor
requires:
  holding_weapon: true
op: ac.add
amount: 2
scope: triggering_attack
recheck_hit: true
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` multiattack ` |

###### Choose sequence

<!-- splat:table
path: splat_rule/parameters/features/multiattack/choose_sequence
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Feature:
    path: feature
    type: symbol
  Count:
    path: count
    type: integer
-->

| Key | Feature | Count |
| --- | --- | --- |
| ` row_1 ` | ` Greatsword ` | ` 2 ` |
| ` row_2 ` | ` Heavy crossbow ` | ` 2 ` |

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
