---
splat_schema: 1
splat_id: 0aa80369-235f-55b4-97f6-a1479ebe1f5b
splat_kind: creature
splat_key: dnd2024:creatures/treant
splat_name: Treant
splat_origin:
  key: dnd2024:reference/creature/treant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 333
  url: https://www.dndbeyond.com/srd
---

# Treant

## Source text

<!-- SRD page 333 -->

Huge Plant, Chaotic Good\
AC 16 Initiative +3 (13)\
HP 138 (12d12 + 60)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 8 −1 −1 Con 21 +5 +5\
Int 12 +1 +1 Wis 16 +3 +3 Cha 12 +1 +1\
Vulnerabilities Fire\
Resistances Bludgeoning, Piercing\
Senses Passive Perception 13\
Languages Common, Druidic, Elvish, Sylvan\
CR 9 (XP 5,000; PB +4)\

### Traits

Siege Monster. The treant deals double damage to ob-\
jects and structures.\

### Actions

Multiattack. The treant makes two Slam attacks.\
Slam. Melee Attack Roll: +10, reach 5 ft. Hit: 16 (3d6\
+ 6) Bludgeoning damage.\
Hail of Bark. Ranged Attack Roll: +10, range 180 ft.\
Hit: 28 (4d10 + 6) Piercing damage.\
Animate Trees (1/Day). The treant magically animates\
up to two trees it can see within 60 feet of itself. Each\
tree uses the Treant stat block, except it has Intelligence\
and Charisma scores of 1, it can’t speak, and it lacks\
this action. The tree takes its turn immediately after the\
treant on the same Initiative count, and it obeys the tre-\
ant. A tree remains animate for 1 day or until it dies, the\
treant dies, or it is more than 120 feet from the treant.\
The tree then takes root if possible.\

Original layout: [[../../srd-5.2.1/pages/333|page 333]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` plant ` |
| ` alignment ` | ` Chaotic Good ` |
| ` movement_text ` | ` 30 ft. ` |
| ` resistances_text ` | ` Bludgeoning, Piercing ` |
| ` vulnerabilities_text ` | ` Fire ` |
| ` senses_text ` | ` Passive Perception 13 ` |
| ` languages_text ` | ` Common, Druidic, Elvish, Sylvan ` |
| ` cr_text ` | ` 9 (XP 5,000; PB +4) ` |
| ` hit_points_text ` | ` 138 (12d12 + 60) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 23 ` | ` 6 ` |
| ` Dexterity ` | ` 8 ` | ` -1 ` |
| ` Constitution ` | ` 21 ` | ` 5 ` |
| ` Intelligence ` | ` 12 ` | ` 1 ` |
| ` Wisdom ` | ` 16 ` | ` 3 ` |
| ` Charisma ` | ` 12 ` | ` 1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 16 ` |  |
| ` max_hp ` | ` 138 ` |  |
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

<!-- splat:yaml path=splat_rule/parameters/features/animate_trees -->

```yaml
economy: action
resource:
  capacity: 1
  recovery: long_rest
  cost: 1
op: animate_objects
targets:
  kind: tree
  count_max: 2
  range_feet: 60
  visible_to: self
stat_block:
  key: dnd2024:creatures/treant
overrides:
  ability_scores:
    intelligence: 1
    charisma: 1
  can_speak: false
  remove_features:
  - animate_trees
initiative:
  same_count_as: self
  turn_order: immediately_after_self
controller: self
ends_any:
- days: 1
- animated_tree_dies: true
- self_dies: true
- distance_from_self_exceeds_feet: 120
on_end:
  op: take_root
  if_possible: true
```

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
