---
splat_schema: 2
splat_id: 4dfa9536-a6f6-54e9-88fb-c0bf824be2f3
splat_kind: creature
splat_key: dnd2024:creatures/clay_golem
splat_name: Clay Golem
splat_origin: dnd2024:reference/creature/clay_golem
splat_sources:
- SRD 5.2.1, pp. 274, https://www.dndbeyond.com/srd
---

# Clay Golem


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Construct |
| Alignment | Unaligned |
| Armor Class | 14 |
| Hit Points | 123 (13d10 + 52) |
| Initiative | +3 (13) |
| Speed | 30 ft. |
| Challenge | 9 (XP 5,000; PB +4) |
| Senses | Darkvision 60 ft.; Passive Perception 9 |
| Languages | Common plus one other language |
| Resistances | Bludgeoning, Piercing, Slashing |
| Immunities | Acid, Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 20 | +5 | +5 |
| Dex | 9 | -1 | -1 |
| Con | 18 | +4 | +4 |
| Int | 3 | -4 | -4 |
| Wis | 8 | -1 | -1 |
| Cha | 1 | -5 | -5 |


## Source text

<!-- SRD page 274 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Construct, Unaligned\
> AC 14 Initiative +3 (13)\
> HP 123 (13d10 + 52)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 20 +5 +5 Dex 9 −1 −1 Con 18 +4 +4\
> Int 3 −4 −4 Wis 8 −1 −1 Cha 1 −5 −5\
> Resistances Bludgeoning, Piercing, Slashing\
> Immunities Acid, Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned\
> Senses Darkvision 60 ft.; Passive Perception 9\
> Languages Common plus one other language\
> CR 9 (XP 5,000; PB +4)


### Traits

Acid Absorption. Whenever the golem is subjected to Acid damage, it takes no damage and instead regains a number of Hit Points equal to the Acid damage dealt.

Berserk. Whenever the golem starts its turn Bloodied, roll 1d6. On a 6, the golem goes berserk. On each of its turns while berserk, the golem attacks the nearest creature it can see. If no creature is near enough to move to and attack, the golem attacks an object. Once the golem goes berserk, it continues to be berserk until it is destroyed or it is no longer Bloodied.

Immutable Form. The golem can’t shape-shift.

Magic Resistance. The golem has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The golem makes two Slam attacks, or it makes three Slam attacks if it used Hasten this turn.

Slam. Melee Attack Roll: +9, reach 5 ft. Hit: 10 (1d10 + 5) Bludgeoning damage plus 6 (1d12) Acid damage, and the target’s Hit Point maximum decreases by an amount equal to the Acid damage taken.

### Bonus Actions

Hasten (Recharge 5–6). The golem takes the Dash and Disengage actions.

Original layout: [[../../srd-5.2.1/pages/274|page 274]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/clay_golem ` |
| ` implementation ` | ` dnd2024:creature_clay_golem ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["bludgeoning", "piercing", "slashing"] ` |
| ` damage_immunities ` | ` ["acid", "poison", "psychic"] ` |
| ` condition_immunities ` | ` ["charmed", "exhaustion", "frightened", "paralyzed", "petrified", "poisoned"] ` |

#### Features

##### Acid absorption

<!-- splat:yaml path=splat_rule/parameters/features/acid_absorption -->

```yaml
economy: trait
operation: passive_rule
trigger: self.subjected_to_acid_damage
damage_taken: 0
heal:
  equal: acid_damage_dealt
```

##### Berserk

<!-- splat:values path=splat_rule/parameters/features/berserk -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` trigger ` | ` self.turn_start ` |
| ` ends_on ` | ` ["self.destroyed", "self.not_bloodied"] ` |

###### Requires

<!-- splat:values path=splat_rule/parameters/features/berserk/requires -->

| Field | Value |
| --- | --- |
| ` self_bloodied ` | ` true ` |

###### Roll

<!-- splat:values path=splat_rule/parameters/features/berserk/roll -->

| Field | Value |
| --- | --- |
| ` die ` | ` 6 ` |
| ` success_faces ` | ` [6] ` |

###### On success

<!-- splat:values path=splat_rule/parameters/features/berserk/on_success -->

| Field | Value |
| --- | --- |
| ` state ` | ` berserk ` |

###### While berserk

###### Attack priority

<!-- splat:table
path: splat_rule/parameters/features/berserk/while_berserk/attack_priority
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Target:
    path: target
    type: symbol
  If reachable for move and attack:
    path: if_reachable_for_move_and_attack
    type: boolean
    optional: true
-->

| Key | Target | If reachable for move and attack |
| --- | --- | --- |
| ` row_1 ` | ` Nearest visible creature ` | true |
| ` row_2 ` | ` Object ` |  |

##### Immutable form

<!-- splat:values path=splat_rule/parameters/features/immutable_form -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` can_shape_shift ` | ` false ` |

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` saving_throw_advantage_against ` | ` ["spells", "magical_effects"] ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: slam
  count:
    if:
      used_this_turn: hasten
    then: 3
    else: 2
```

##### Slam

<!-- splat:values path=splat_rule/parameters/features/slam -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/slam/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 9
reach_ft: 5
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/slam/hit/damage
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
| ` row_1 ` | ` 10 ` | ` 1 ` | ` 10 ` | ` 5 ` | ` Bludgeoning ` |
| ` row_2 ` | ` 6 ` | ` 1 ` | ` 12 ` | ` 0 ` | ` Acid ` |

###### Maximum hp reduction

<!-- splat:values path=splat_rule/parameters/features/slam/hit/maximum_hp_reduction -->

| Field | Value |
| --- | --- |
| ` equal ` | ` acid_damage_taken ` |

##### Hasten

<!-- splat:yaml path=splat_rule/parameters/features/hasten -->

```yaml
economy: bonus_action
resource:
  kind: recharge
  die: 6
  success_faces:
  - 5
  - 6
  check: self.turn_start
  rest_recovery:
  - short
  - long
  capacity: 1
  initial_uses: 1
operation: sequence
effects:
- operation: action
  action: dash
- operation: action
  action: disengage
```

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
