---
splat_schema: 2
splat_id: 8cff2e8f-1067-58f4-ac6f-975eedd92bdd
splat_kind: creature
splat_key: dnd2024:creatures/merrow
splat_name: Merrow
splat_origin: dnd2024:reference/creature/merrow
splat_sources:
- SRD 5.2.1, pp. 308, https://www.dndbeyond.com/srd
---

# Merrow


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Monstrosity |
| Alignment | Chaotic Evil |
| Armor Class | 13 |
| Hit Points | 45 (6d10 + 12) |
| Initiative | +2 (12) |
| Speed | 10 ft., Swim 40 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 10 |
| Languages | Abyssal, Primordial (Aquan) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 15 | +2 | +2 |
| Con | 15 | +2 | +2 |
| Int | 8 | -1 | -1 |
| Wis | 10 | +0 | +0 |
| Cha | 9 | -1 | -1 |


## Source text

<!-- SRD page 308 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Monstrosity, Chaotic Evil\
> AC 13 Initiative +2 (12)\
> HP 45 (6d10 + 12)\
> Speed 10 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 15 +2 +2 Con 15 +2 +2\
> Int 8 −1 −1 Wis 10 +0 +0 Cha 9 −1 −1\
> Senses Darkvision 60 ft.; Passive Perception 10\
> Languages Abyssal, Primordial (Aquan)\
> CR 2 (XP 450; PB +2)


### Traits

Amphibious. The merrow can breathe air and water.

### Actions

Multiattack. The merrow makes two attacks, using Bite, Claw, or Harpoon in any combination.

Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 6 (1d4 + 4) Piercing damage, and the target has the Poisoned condition until the end of the merrow’s next turn.

Claw. Melee Attack Roll: +6, reach 5 ft. Hit: 9 (2d4 + 4) Slashing damage.

Harpoon. Melee or Ranged Attack Roll: +6, reach 5 ft. or range 20/60 ft. Hit: 11 (2d6 + 4) Piercing damage. If the target is a Large or smaller creature, the merrow pulls the target up to 15 feet straight toward itself.

Original layout: [[../../srd-5.2.1/pages/308|page 308]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/merrow ` |
| ` implementation ` | ` dnd2024:creature_merrow ` |

### Parameters

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
- operation: attack_choice
  count: 2
  options:
  - bite
  - claw
  - harpoon
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

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
  - average: 6
    roll:
      count: 1
      sides: 4
      modifier: 4
    type: piercing
  conditions:
  - condition: poisoned
    until: self.next_turn_end
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
  - average: 9
    roll:
      count: 2
      sides: 4
      modifier: 4
    type: slashing
```

##### Harpoon

<!-- splat:yaml path=splat_rule/parameters/features/harpoon -->

```yaml
economy: action
operation: attack
attack:
  mode: melee_or_ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 6
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
  pull:
    maximum_ft: 15
    direction: straight_toward_self
    maximum_size: large
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- abyssal
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
>   value: 40
> ```

