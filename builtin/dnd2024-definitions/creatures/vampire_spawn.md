---
splat_schema: 1
splat_id: 7d62036e-996d-516f-b89d-a8f1762641c4
splat_kind: creature
splat_key: dnd2024:creatures/vampire_spawn
splat_name: Vampire Spawn
splat_origin:
  key: dnd2024:reference/creature/vampire_spawn
splat_sources:
- title: SRD 5.2.1
  locator: pp. 334, 335
  url: https://www.dndbeyond.com/srd
---

# Vampire Spawn

## Source text

<!-- SRD page 334 -->

Medium or Small Undead, Neutral Evil\
AC 16 Initiative +3 (13)\
HP 90 (12d8 + 36)\
Speed 30 ft.\

<!-- SRD page 335 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 16 +3 +6 Con 16 +3 +3\
Int 11 +0 +0 Wis 10 +0 +3 Cha 12 +1 +1\
Skills Perception +3, Stealth +6\
Resistances Necrotic\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages Common plus one other language\
CR 5 (XP 1,800; PB +3)\

### Traits

Spider Climb. The vampire can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\
Vampire Weakness. The vampire has these\
weaknesses:\
Forbiddance. The vampire can’t enter a residence with-\
out an invitation from an occupant.\
Running Water. The vampire takes 20 Acid damage if it\
ends its turn in running water.\
Stake to the Heart. The vampire is destroyed if a\
weapon that deals Piercing damage is driven into the\
vampire’s heart while the vampire has the Incapaci-\
tated condition.\
Sunlight. The vampire takes 20 Radiant damage if it\
starts its turn in sunlight. While in sunlight, it has Dis-\
advantage on attack rolls and ability checks.\

### Actions

Multiattack. The vampire makes two Claw attacks and\
uses Bite.\
Claw. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (2d4 + 3)\
Slashing damage. If the target is a Medium or smaller\
creature, it has the Grappled condition (escape DC 13)\
from one of two claws.\
Bite. Constitution Saving Throw: DC 14, one creature\
within 5 feet that is willing or that has the Grappled,\
Incapacitated, or Restrained condition. Failure: 5 (1d4\
+ 3) Piercing damage plus 10 (3d6) Necrotic dam-\
age. The target’s Hit Point maximum decreases by an\
amount equal to the Necrotic damage taken, and the\
vampire regains Hit Points equal to that amount.\

### Bonus Actions

Deathless Agility. The vampire takes the Dash or Dis-\
engage action.\

Original layout: [[../../srd-5.2.1/pages/334|page 334]], [[../../srd-5.2.1/pages/335|page 335]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` undead ` |
| ` alignment ` | ` Neutral Evil ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Perception +3, Stealth +6 ` |
| ` resistances_text ` | ` Necrotic ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 13 ` |
| ` languages_text ` | ` Common plus one other language ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 90 (12d8 + 36) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 16 ` | ` 3 ` |
| ` Dexterity ` | ` 16 ` | ` 6 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 11 ` | ` 0 ` |
| ` Wisdom ` | ` 10 ` | ` 3 ` |
| ` Charisma ` | ` 12 ` | ` 1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 16 ` |  |
| ` max_hp ` | ` 90 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/vampire_spawn ` |
| ` implementation ` | ` dnd2024:creature_vampire_spawn ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["necrotic"] ` |

#### Features

##### Spider climb

<!-- splat:values path=splat_rule/parameters/features/spider_climb -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` climb_difficult_surfaces ` | ` true ` |
| ` climb_ceilings ` | ` true ` |
| ` ability_check_required ` | ` false ` |

##### Vampire weakness

<!-- splat:values path=splat_rule/parameters/features/vampire_weakness -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` weaknesses ` | ` ["forbiddance", "running_water", "stake_to_the_heart", "sunlight"] ` |

##### Forbiddance

<!-- splat:values path=splat_rule/parameters/features/forbiddance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` residence_entry_requires ` | ` occupant_invitation ` |

##### Running water

<!-- splat:yaml path=splat_rule/parameters/features/running_water -->

```yaml
economy: trait
operation: passive_rule
trigger: self.turn_end
requires:
  in_running_water: true
damage:
- roll:
    constant: 20
  type: acid
```

##### Stake to the heart

<!-- splat:yaml path=splat_rule/parameters/features/stake_to_the_heart -->

```yaml
economy: trait
operation: passive_rule
trigger: piercing_weapon_driven_into_heart
requires:
  self_condition: incapacitated
destroy_self: true
```

##### Sunlight

<!-- splat:yaml path=splat_rule/parameters/features/sunlight -->

```yaml
economy: trait
operation: passive_rule
trigger: self.turn_start
requires:
  in_sunlight: true
damage:
- roll:
    constant: 20
  type: radiant
while_sunlight:
  disadvantage_on:
  - attack_rolls
  - ability_checks
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: claw
  count: 2
- operation: use_feature
  feature: bite
```

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 6
  reach_ft: 5
hit:
  damage:
  - average: 8
    roll:
      count: 2
      sides: 4
      modifier: 3
    type: slashing
  grapple:
    condition: grappled
    source: self
    escape_dc: 13
    maximum_size: medium
    appendage_pool:
      count: 2
      type: claws
      cost: 1
```

##### Bite

<!-- splat:values path=splat_rule/parameters/features/bite -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` saving_throw ` |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 14 ` |

###### Targets

<!-- splat:values path=splat_rule/parameters/features/bite/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` range_ft ` | ` 5 ` |

###### Any of

<!-- splat:table
path: splat_rule/parameters/features/bite/targets/any_of
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Willing:
    path: willing
    type: boolean
    optional: true
  Condition:
    path: condition
    type: symbol
    optional: true
-->

| Key | Willing | Condition |
| --- | --- | --- |
| ` row_1 ` | true |  |
| ` row_2 ` |  | ` Grappled ` |
| ` row_3 ` |  | ` Incapacitated ` |
| ` row_4 ` |  | ` Restrained ` |

###### Failure

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/bite/failure/damage
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
| ` row_1 ` | ` 5 ` | ` 1 ` | ` 4 ` | ` 3 ` | ` Piercing ` |
| ` row_2 ` | ` 10 ` | ` 3 ` | ` 6 ` | ` 0 ` | ` Necrotic ` |

###### Maximum hp reduction

<!-- splat:values path=splat_rule/parameters/features/bite/failure/maximum_hp_reduction -->

| Field | Value |
| --- | --- |
| ` equal ` | ` necrotic_damage_taken ` |

###### Heal self

<!-- splat:values path=splat_rule/parameters/features/bite/failure/heal_self -->

| Field | Value |
| --- | --- |
| ` equal ` | ` necrotic_damage_taken ` |

###### Success

<!-- splat:yaml path=splat_rule/parameters/features/bite/success -->

```yaml
effects: []
```

##### Deathless agility

<!-- splat:values path=splat_rule/parameters/features/deathless_agility -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` operation ` | ` choose_action ` |
| ` count ` | ` 1 ` |
| ` options ` | ` ["dash", "disengage"] ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |
| ` stealth ` | ` 6 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 13 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
understands: []
additional_languages:
  count: 1
  chooser: creator
  can_speak: true
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
