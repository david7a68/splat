---
splat_schema: 1
splat_id: 71f360e6-f6cd-5406-a452-273caff9f01f
splat_kind: creature
splat_key: dnd2024:creatures/assassin
splat_name: Assassin
splat_origin:
  key: dnd2024:reference/creature/assassin
splat_sources:
- title: SRD 5.2.1
  locator: pp. 260
  url: https://www.dndbeyond.com/srd
---

# Assassin

## Source text

<!-- SRD page 260 -->

Medium or Small Humanoid, Neutral\
AC 16 Initiative +10 (20)\
HP 97 (15d8 + 30)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 18 +4 +7 Con 14 +2 +2\
Int 16 +3 +6 Wis 11 +0 +0 Cha 10 +0 +0\
Skills Acrobatics +7, Perception +6, Stealth +10\
Resistances Poison\
Gear Light Crossbow, Shortsword, Studded Leather Armor\
Senses Passive Perception 16\
Languages Common, Thieves’ Cant\
CR 8 (XP 3,900; PB +3)\

### Traits

Evasion. If the assassin is subjected to an effect that\
allows it to make a Dexterity saving throw to take only\
half damage, the assassin instead takes no damage if it\
succeeds on the save and only half damage if it fails. It\
can’t use this trait if it has the Incapacitated condition.\

### Actions

Multiattack. The assassin makes three attacks, using\
Shortsword or Light Crossbow in any combination.\
Shortsword. Melee Attack Roll: +7, reach 5 ft. Hit: 7\
(1d6 + 4) Piercing damage plus 17 (5d6) Poison dam-\
age, and the target has the Poisoned condition until the\
start of the assassin’s next turn.\
Light Crossbow. Ranged Attack Roll: +7, range\
80/320 ft. Hit: 8 (1d8 + 4) Piercing damage plus 21\
(6d6) Poison damage.\

### Bonus Actions

Cunning Action. The assassin takes the Dash, Disen-\
gage, or Hide action.\

Original layout: [[../../srd-5.2.1/pages/260|page 260]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Acrobatics +7, Perception +6, Stealth +10 ` |
| ` resistances_text ` | ` Poison ` |
| ` gear_text ` | ` Light Crossbow, Shortsword, Studded Leather Armor ` |
| ` senses_text ` | ` Passive Perception 16 ` |
| ` languages_text ` | ` Common, Thieves’ Cant ` |
| ` cr_text ` | ` 8 (XP 3,900; PB +3) ` |
| ` hit_points_text ` | ` 97 (15d8 + 30) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 11 ` | ` 0 ` |
| ` Dexterity ` | ` 18 ` | ` 7 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 16 ` | ` 6 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 16 ` |  |
| ` max_hp ` | ` 97 ` |  |
| ` initiative ` | ` 10 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/assassin ` |
| ` implementation ` | ` dnd2024:creature_assassin ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["poison"] ` |
| ` passive_perception ` | ` 16 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` acrobatics ` | ` 7 ` |
| ` perception ` | ` 6 ` |
| ` stealth ` | ` 10 ` |

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
| ` row_1 ` | ` Light crossbow ` | ` 1 ` |
| ` row_2 ` | ` Shortsword ` | ` 1 ` |
| ` row_3 ` | ` Studded leather armor ` | ` 1 ` |

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
| ` known ` | ` ["common", "thieves_cant"] ` |

#### Features

##### Shortsword

<!-- splat:yaml path=splat_rule/parameters/features/shortsword -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 7
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
        sides: 6
      add: 4
    average: 7
  - type: poison
    amount:
      dice:
        count: 5
        sides: 6
      add: 0
    average: 17
- op: condition.apply
  condition: poisoned
  duration:
    until: actor.next_turn_start
```

##### Light crossbow

<!-- splat:yaml path=splat_rule/parameters/features/light_crossbow -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 7
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 80
  long: 320
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
        count: 6
        sides: 6
      add: 0
    average: 21
```

##### Cunning action

<!-- splat:values path=splat_rule/parameters/features/cunning_action -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` action.choose ` |
| ` choices ` | ` ["dash", "disengage", "hide"] ` |

##### Evasion

<!-- splat:yaml path=splat_rule/parameters/features/evasion -->

```yaml
economy: passive
when:
  save_ability: dexterity
  success_damage_fraction: 0.5
  actor_excluded_conditions:
  - incapacitated
op: damage_fraction.override
success: 0
failure: 0.5
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
attacks:
  count: 3
  choices:
  - shortsword
  - light_crossbow
  combination: any
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
