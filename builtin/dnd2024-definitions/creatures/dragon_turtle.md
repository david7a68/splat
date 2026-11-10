---
splat_schema: 2
splat_id: e6257a3a-9490-5c23-9b3c-7c8a2f10f404
splat_kind: creature
splat_key: dnd2024:creatures/dragon_turtle
splat_name: Dragon Turtle
splat_origin: dnd2024:reference/creature/dragon_turtle
splat_sources:
- SRD 5.2.1, pp. 281, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 103, Habitat
---

# Dragon Turtle


## Stat block

| Stat | Value |
| --- | --- |
| Size | Gargantuan |
| Type | Dragon |
| Alignment | Neutral |
| Armor Class | 20 |
| Hit Points | 356 (23d20 + 115) |
| Initiative | +6 (16) |
| Speed | 20 ft., Swim 50 ft. |
| Challenge | 17 (XP 18,000; PB +6) |
| Senses | Darkvision 120 ft.; Passive Perception 11 |
| Languages | Draconic, Primordial (Aquan) |
| Resistances | Fire |
| Habitat | Coastal, Underwater |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 25 | +7 | +7 |
| Dex | 10 | +0 | +0 |
| Con | 20 | +5 | +11 |
| Int | 10 | +0 | +0 |
| Wis | 12 | +1 | +7 |
| Cha | 12 | +1 | +1 |


## Source text

<!-- SRD page 281 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Gargantuan Dragon, Neutral\
> AC 20 Initiative +6 (16)\
> HP 356 (23d20 + 115)\
> Speed 20 ft., Swim 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 25 +7 +7 Dex 10 +0 +0 Con 20 +5 +11\
> Int 10 +0 +0 Wis 12 +1 +7 Cha 12 +1 +1\
> Resistances Fire\
> Senses Darkvision 120 ft.; Passive Perception 11\
> Languages Draconic, Primordial (Aquan)\
> CR 17 (XP 18,000; PB +6)


### Traits

Amphibious. The dragon can breathe air and water.

### Actions

Multiattack. The dragon makes three Bite attacks. It can replace one attack with a Tail attack.

Bite. Melee Attack Roll: +13, reach 15 ft. Hit: 23 (3d10 + 7) Piercing damage plus 7 (2d6) Fire damage. Being underwater doesn’t grant Resistance to this Fire damage.

Tail. Melee Attack Roll: +13, reach 15 ft. Hit: 18 (2d10 + 7) Bludgeoning damage. If the target is a Huge or smaller creature, it has the Prone condition.

Steam Breath (Recharge 5–6). Constitution Saving Throw: DC 19, each creature in a 60-foot Cone. Failure: 56 (16d6) Fire damage. Success: Half damage. Failure or Success: Being underwater doesn’t grant Resistance to this Fire damage.

Original layout: [[../../srd-5.2.1/pages/281|page 281]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/dragon_turtle ` |
| ` implementation ` | ` dnd2024:creature_dragon_turtle ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["fire"] ` |

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` breathing ` | ` ["air", "water"] ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: bite
  count: 3
replacements:
- replace_count: 1
  maximum_uses: 1
  operation:
    operation: use_feature
    feature: tail
```

##### Bite

<!-- splat:values path=splat_rule/parameters/features/bite -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |
| ` underwater_fire_resistance_applies ` | ` false ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/bite/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 13
reach_ft: 15
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/bite/hit/damage
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
| ` row_1 ` | ` 23 ` | ` 3 ` | ` 10 ` | ` 7 ` | ` Piercing ` |
| ` row_2 ` | ` 7 ` | ` 2 ` | ` 6 ` | ` 0 ` | ` Fire ` |

##### Tail

<!-- splat:yaml path=splat_rule/parameters/features/tail -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 13
  reach_ft: 15
hit:
  damage:
  - average: 18
    roll:
      count: 2
      sides: 10
      modifier: 7
    type: bludgeoning
  prone:
    condition: prone
    maximum_size: huge
```

##### Steam breath

<!-- splat:yaml path=splat_rule/parameters/features/steam_breath -->

```yaml
economy: action
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
operation: saving_throw
ability: constitution
dc: 19
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 60
    origin: self
failure:
  damage:
  - average: 56
    roll:
      count: 16
      sides: 6
      modifier: 0
    type: fire
success:
  damage_multiplier: 0.5
  other_effects: false
underwater_fire_resistance_applies: false
```

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
- draconic
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
>   value: 50
> ```

