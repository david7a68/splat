---
splat_schema: 2
splat_id: b0db3fcd-7eeb-57af-8bfc-24f15e948cb0
splat_kind: creature
splat_key: dnd2024:creatures/bugbear_stalker
splat_name: Bugbear Stalker
splat_origin: dnd2024:reference/creature/bugbear_stalker
splat_sources:
- SRD 5.2.1, pp. 271, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 62, Habitat
---

# Bugbear Stalker


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Fey (Goblinoid) |
| Alignment | Chaotic Evil |
| Armor Class | 15 |
| Hit Points | 65 (10d8 + 20) |
| Initiative | +2 (12) |
| Speed | 30 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Skills | Stealth +6, Survival +3 |
| Senses | Darkvision 60 ft.; Passive Perception 11 |
| Languages | Common, Goblin |
| Gear | Chain Shirt, Javelins (6), Morningstar |
| Habitat | Forest, Grassland, Underdark, Planar (Feywild) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 14 | +2 | +2 |
| Con | 14 | +2 | +4 |
| Int | 11 | +0 | +0 |
| Wis | 12 | +1 | +3 |
| Cha | 11 | +0 | +0 |


## Source text

<!-- SRD page 271 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Fey (Goblinoid), Chaotic Evil\
> AC 15 Initiative +2 (12)\
> HP 65 (10d8 + 20)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 14 +2 +2 Con 14 +2 +4\
> Int 11 +0 +0 Wis 12 +1 +3 Cha 11 +0 +0\
> Skills Stealth +6, Survival +3\
> Gear Chain Shirt, Javelins (6), Morningstar\
> Senses Darkvision 60 ft.; Passive Perception 11\
> Languages Common, Goblin\
> CR 3 (XP 700; PB +2)


### Traits

Abduct. The bugbear needn’t spend extra movement to move a creature it is grappling.

### Actions

Multiattack. The bugbear makes two Javelin or Morningstar attacks.

Javelin. Melee or Ranged Attack Roll: +5, reach 10 ft. or range 30/120 ft. Hit: 13 (3d6 + 3) Piercing damage.

Morningstar. Melee Attack Roll: +5 (with Advantage if the target is Grappled by the bugbear), reach 10 ft. Hit: 12 (2d8 + 3) Piercing damage.

### Bonus Actions

Quick Grapple. Dexterity Saving Throw: DC 13, one Medium or smaller creature the bugbear can see within 10 feet. Failure: The target has the Grappled condition (escape DC 13).

Original layout: [[../../srd-5.2.1/pages/271|page 271]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/bugbear_stalker ` |
| ` implementation ` | ` dnd2024:creature_bugbear_stalker ` |

### Parameters

#### Features

##### Abduct

<!-- splat:values path=splat_rule/parameters/features/abduct -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` grappled_creature_movement_extra_cost ` | ` 0 ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_choice
  count: 2
  options:
  - javelin
  - morningstar
```

##### Javelin

<!-- splat:yaml path=splat_rule/parameters/features/javelin -->

```yaml
economy: action
operation: attack
attack:
  mode: melee_or_ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  reach_ft: 10
  range_ft: 30
  long_range_ft: 120
hit:
  damage:
  - average: 13
    roll:
      count: 3
      sides: 6
      modifier: 3
    type: piercing
```

##### Morningstar

<!-- splat:yaml path=splat_rule/parameters/features/morningstar -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  reach_ft: 10
  advantage_if:
    target_grappled_by: self
hit:
  damage:
  - average: 12
    roll:
      count: 2
      sides: 8
      modifier: 3
    type: piercing
```

##### Quick grapple

<!-- splat:yaml path=splat_rule/parameters/features/quick_grapple -->

```yaml
economy: bonus_action
operation: saving_throw
ability: dexterity
dc: 13
targets:
  kind: creature
  count: 1
  range_ft: 10
  visible: true
  maximum_size: medium
failure:
  grapple:
    condition: grappled
    source: self
    escape_dc: 13
success:
  effects: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 6 ` |
| ` survival ` | ` 3 ` |

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
| ` row_1 ` | ` Chain shirt ` | ` 1 ` |
| ` row_2 ` | ` Javelins ` | ` 6 ` |
| ` row_3 ` | ` Morningstar ` | ` 1 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 11 ` |

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
