---
splat_schema: 2
splat_id: 0aa80369-235f-55b4-97f6-a1479ebe1f5b
splat_kind: creature
splat_key: dnd2024:creatures/treant
splat_name: Treant
splat_origin: dnd2024:reference/creature/treant
splat_sources:
- SRD 5.2.1, pp. 333, https://www.dndbeyond.com/srd
---

# Treant


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Plant |
| Alignment | Chaotic Good |
| Armor Class | 16 |
| Hit Points | 138 (12d12 + 60) |
| Initiative | +3 (13) |
| Speed | 30 ft. |
| Challenge | 9 (XP 5,000; PB +4) |
| Senses | Passive Perception 13 |
| Languages | Common, Druidic, Elvish, Sylvan |
| Resistances | Bludgeoning, Piercing |
| Vulnerabilities | Fire |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 23 | +6 | +6 |
| Dex | 8 | -1 | -1 |
| Con | 21 | +5 | +5 |
| Int | 12 | +1 | +1 |
| Wis | 16 | +3 | +3 |
| Cha | 12 | +1 | +1 |


## Source text

<!-- SRD page 333 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Plant, Chaotic Good\
> AC 16 Initiative +3 (13)\
> HP 138 (12d12 + 60)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 23 +6 +6 Dex 8 −1 −1 Con 21 +5 +5\
> Int 12 +1 +1 Wis 16 +3 +3 Cha 12 +1 +1\
> Vulnerabilities Fire\
> Resistances Bludgeoning, Piercing\
> Senses Passive Perception 13\
> Languages Common, Druidic, Elvish, Sylvan\
> CR 9 (XP 5,000; PB +4)


### Traits

Siege Monster. The treant deals double damage to objects and structures.

### Actions

Multiattack. The treant makes two Slam attacks.

Slam. Melee Attack Roll: +10, reach 5 ft. Hit: 16 (3d6 + 6) Bludgeoning damage.

Hail of Bark. Ranged Attack Roll: +10, range 180 ft. Hit: 28 (4d10 + 6) Piercing damage.

Animate Trees (1/Day). The treant magically animates up to two trees it can see within 60 feet of itself. Each tree uses the Treant stat block, except it has Intelligence and Charisma scores of 1, it can’t speak, and it lacks this action. The tree takes its turn immediately after the treant on the same Initiative count, and it obeys the treant. A tree remains animate for 1 day or until it dies, the treant dies, or it is more than 120 feet from the treant. The tree then takes root if possible.

Original layout: [[../../srd-5.2.1/pages/333|page 333]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/treant ` |
| ` implementation ` | ` dnd2024:creature_treant ` |

### Parameters

#### Features

##### Siege monster

<!-- splat:values path=splat_rule/parameters/features/siege_monster -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` damage_multiplier ` | ` 2 ` |
| ` targets ` | ` ["object", "structure"] ` |

##### Slam

<!-- splat:yaml path=splat_rule/parameters/features/slam -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 10
targets:
  count: 1
hit:
- op: damage
  average: 16
  amount:
    dice:
      count: 3
      sides: 6
    add: 6
  damage_type: bludgeoning
reach_feet: 5
```

##### Hail of bark

<!-- splat:yaml path=splat_rule/parameters/features/hail_of_bark -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 10
targets:
  count: 1
hit:
- op: damage
  average: 28
  amount:
    dice:
      count: 4
      sides: 10
    add: 6
  damage_type: piercing
range_feet: 180
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: slam
  count: 2
```

##### Animate trees

<!-- splat:values path=splat_rule/parameters/features/animate_trees -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` animate_objects ` |
| ` controller ` | ` self ` |

###### Resource

<!-- splat:values path=splat_rule/parameters/features/animate_trees/resource -->

| Field | Value |
| --- | --- |
| ` capacity ` | ` 1 ` |
| ` recovery ` | ` long_rest ` |
| ` cost ` | ` 1 ` |

###### Targets

<!-- splat:values path=splat_rule/parameters/features/animate_trees/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` tree ` |
| ` count_max ` | ` 2 ` |
| ` range_feet ` | ` 60 ` |
| ` visible_to ` | ` self ` |

###### Stat block

<!-- splat:values path=splat_rule/parameters/features/animate_trees/stat_block -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:creatures/treant ` |

###### Overrides

<!-- splat:yaml path=splat_rule/parameters/features/animate_trees/overrides -->

```yaml
ability_scores:
  intelligence: 1
  charisma: 1
can_speak: false
remove_features:
- animate_trees
```

###### Initiative

<!-- splat:values path=splat_rule/parameters/features/animate_trees/initiative -->

| Field | Value |
| --- | --- |
| ` same_count_as ` | ` self ` |
| ` turn_order ` | ` immediately_after_self ` |

###### Ends any

<!-- splat:table
path: splat_rule/parameters/features/animate_trees/ends_any
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Days:
    path: days
    type: integer
    optional: true
  Animated tree dies:
    path: animated_tree_dies
    type: boolean
    optional: true
  Self dies:
    path: self_dies
    type: boolean
    optional: true
  Distance from self exceeds feet:
    path: distance_from_self_exceeds_feet
    type: integer
    optional: true
-->

| Key | Days | Animated tree dies | Self dies | Distance from self exceeds feet |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` |  |  |  |
| ` row_2 ` |  | true |  |  |
| ` row_3 ` |  |  | true |  |
| ` row_4 ` |  |  |  | ` 120 ` |

###### On end

<!-- splat:values path=splat_rule/parameters/features/animate_trees/on_end -->

| Field | Value |
| --- | --- |
| ` op ` | ` take_root ` |
| ` if_possible ` | ` true ` |

#### Defenses

##### Vulnerabilities

<!-- splat:values path=splat_rule/parameters/defenses/vulnerabilities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["fire"] ` |

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["bludgeoning", "piercing"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 13 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common", "druidic", "elvish", "sylvan"] ` |
| ` understands ` | ` ["common", "druidic", "elvish", "sylvan"] ` |

#### Stat block conventions

<!-- splat:yaml path=splat_rule/parameters/stat_block_conventions -->

```yaml
attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
save_dc: supplied_total
damage_mode_choice:
- fixed_average
- rolled_dice
condition_source: self
multiattack_child_economy_cost: 0
multiattack_child_resource_costs_retained: true
spellcasting:
  default_level: spell_minimum
  spell_slots_expended: false
  required_materials: available
  casting_time_at_least_one_minute:
    uses_normal_casting_duration: true
    magic_action_each_turn: true
    concentration_required: true
geometry_assumptions:
  chooser: gm
  supplies:
  - range
  - visibility
  - line_of_sight
  - area_membership
  - reachable_locations
  - unoccupied_locations
  - terrain
  - plane
  - form_appearance
```
