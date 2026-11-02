---
splat_schema: 1
splat_id: a959432d-bbcc-5cab-810d-29dd255b407b
splat_kind: creature
splat_key: dnd2024:creatures/hezrou
splat_name: Hezrou
splat_origin:
  key: dnd2024:reference/creature/hezrou
splat_sources:
- title: SRD 5.2.1
  locator: pp. 298
  url: https://www.dndbeyond.com/srd
---

# Hezrou

## Source text

<!-- SRD page 298 -->

Large Fiend (Demon), Chaotic Evil\
AC 18 Initiative +6 (16)\
HP 157 (15d10 + 75)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +7 Dex 17 +3 +3 Con 20 +5 +8\
Int 5 −3 −3 Wis 12 +1 +4 Cha 13 +1 +1\
Resistances Cold, Fire, Lightning\
Immunities Poison; Poisoned\
Senses Darkvision 120 ft.; Passive Perception 11\
Languages Abyssal; telepathy 120 ft.\
CR 8 (XP 3,900; PB +3)

### Traits

Demonic Restoration. If the hezrou dies outside the Abyss, its body dissolves into ichor, and it gains a new body instantly, reviving with all its Hit Points somewhere in the Abyss.

Magic Resistance. The hezrou has Advantage on saving throws against spells and other magical effects.

Stench. Constitution Saving Throw: DC 16, any creature that starts its turn in a 10-foot Emanation originating from the hezrou. Failure: The target has the Poisoned condition until the start of its next turn.

### Actions

Multiattack. The hezrou makes three Rend attacks.

Rend. Melee Attack Roll: +7, reach 5 ft. Hit: 6 (1d4 + 4) Slashing damage plus 9 (2d8) Poison damage.

### Bonus Actions

Leap. The hezrou jumps up to 30 feet by spending 10 feet of movement.

Original layout: [[../../srd-5.2.1/pages/298|page 298]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Demon) ` |
| ` movement_text ` | ` 30 ft. ` |
| ` resistances_text ` | ` Cold, Fire, Lightning ` |
| ` immunities_text ` | ` Poison; Poisoned ` |
| ` senses_text ` | ` Darkvision 120 ft.; Passive Perception 11 ` |
| ` languages_text ` | ` Abyssal; telepathy 120 ft. ` |
| ` cr_text ` | ` 8 (XP 3,900; PB +3) ` |
| ` hit_points_text ` | ` 157 (15d10 + 75) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 19 ` | ` 7 ` |
| ` Dexterity ` | ` 17 ` | ` 3 ` |
| ` Constitution ` | ` 20 ` | ` 8 ` |
| ` Intelligence ` | ` 5 ` | ` -3 ` |
| ` Wisdom ` | ` 12 ` | ` 4 ` |
| ` Charisma ` | ` 13 ` | ` 1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 157 ` |  |
| ` initiative ` | ` 6 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/hezrou ` |
| ` implementation ` | ` dnd2024:creature_hezrou ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold", "fire", "lightning"] ` |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["poisoned"] ` |

#### Features

##### Demonic restoration

<!-- splat:yaml path=splat_rule/parameters/features/demonic_restoration -->

```yaml
economy: trait
operation: passive_rule
trigger: self.dies
unless:
  plane: abyss
remove_old_body: true
new_body:
  delay_seconds: 0
  plane: abyss
  hp: maximum
```

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` saving_throw_advantage_against ` | ` ["spells", "magical_effects"] ` |

##### Stench

<!-- splat:yaml path=splat_rule/parameters/features/stench -->

```yaml
economy: trait
operation: saving_throw
ability: constitution
dc: 16
targets:
  kind: creature
  selection: each
  area:
    shape: emanation
    size_ft: 10
    origin: self
failure:
  conditions:
  - condition: poisoned
    until: target.next_turn_start
trigger: target.turn_start
success:
  effects: []
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: rend
  count: 3
```

##### Rend

<!-- splat:values path=splat_rule/parameters/features/rend -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/rend/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 7
reach_ft: 5
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/rend/hit/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Average:
    path: average
    type: integer
  Roll count:
    path: roll/count
    type: integer
  Roll sides:
    path: roll/sides
    type: integer
  Roll modifier:
    path: roll/modifier
    type: integer
  Type:
    path: type
    type: symbol
-->

| Key | Average | Roll count | Roll sides | Roll modifier | Type |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 6 ` | ` 1 ` | ` 4 ` | ` 4 ` | ` Slashing ` |
| ` row_2 ` | ` 9 ` | ` 2 ` | ` 8 ` | ` 0 ` | ` Poison ` |

##### Leap

<!-- splat:values path=splat_rule/parameters/features/leap -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` operation ` | ` jump ` |
| ` maximum_ft ` | ` 30 ` |
| ` movement_cost_ft ` | ` 10 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 120 ` |
| ` passive_perception ` | ` 11 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- abyssal
understands: []
telepathy_range_ft: 120
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
