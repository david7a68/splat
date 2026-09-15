---
splat_schema: 1
splat_id: e2376c5f-c7b9-5a91-aab6-787c9980d3f7
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_insects
splat_name: Swarm of Insects
splat_origin:
  key: dnd2024:reference/creature/swarm_of_insects
splat_sources:
- title: SRD 5.2.1
  locator: pp. 361
  url: https://www.dndbeyond.com/srd
---

# Swarm of Insects

## Source text

<!-- SRD page 361 -->

Medium Swarm of Tiny Beasts, Unaligned\
AC 11 Initiative +1 (11)\
HP 19 (3d8 + 6)\
Speed 20 ft., Climb or Fly 20 ft. (GM’s choice)\
MOD SAVE MOD SAVE MOD SAVE\
Str 3 −4 −4 Dex 13 +1 +1 Con 14 +2 +2\
Int 1 −5 −5 Wis 7 −2 −2 Cha 1 −5 −5\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Charmed, Frightened, Grappled, Paralyzed,\
Petrified, Prone, Restrained, Stunned\
Senses Blindsight 30 ft.; Passive Perception 8\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Traits

Spider Climb. If the swarm has a Climb Speed, the\
swarm can climb difficult surfaces, including along ceil-\
ings, without needing to make an ability check.\
Swarm. The swarm can occupy another creature’s\
space and vice versa, and the swarm can move through\
any opening large enough for a Tiny insect. The swarm\
can’t regain Hit Points or gain Temporary Hit Points.\

### Actions

Bites. Melee Attack Roll: +3, reach 5 ft. Hit: 6 (2d4 + 1)\
Poison damage, or 3 (1d4 + 1) Poison damage if the\
swarm is Bloodied.\

Original layout: [[../../srd-5.2.1/pages/361|page 361]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` size_options_text ` | ` Medium Swarm of Tiny ` |
| ` tags_text ` | ` s ` |
| ` movement_text ` | ` 20 ft., Climb or Fly 20 ft. (GM’s choice) ` |
| ` resistances_text ` | ` Bludgeoning, Piercing, Slashing ` |
| ` immunities_text ` | ` Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned ` |
| ` senses_text ` | ` Blindsight 30 ft.; Passive Perception 8 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/2 (XP 100; PB +2) ` |
| ` hit_points_text ` | ` 19 (3d8 + 6) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 3 ` | ` -4 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 7 ` | ` -2 ` |
| ` Charisma ` | ` 1 ` | ` -5 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 19 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |
| ` fly_speed ` | ` 20 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/swarm_of_insects ` |
| ` implementation ` | ` dnd2024:creature_swarm_of_insects ` |

### Parameters

#### Features

##### Spider climb

<!-- splat:yaml path=splat_rule/parameters/features/spider_climb -->

```yaml
economy: passive
climb_ability_check_required: false
surfaces:
- difficult
- ceiling
requires:
  has_climb_speed: true
```

##### Swarm

<!-- splat:values path=splat_rule/parameters/features/swarm -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` space_sharing ` | ` bidirectional ` |
| ` minimum_passage_size ` | ` tiny ` |
| ` regain_hit_points ` | ` false ` |
| ` gain_temporary_hit_points ` | ` false ` |

##### Bites

<!-- splat:yaml path=splat_rule/parameters/features/bites -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 3
targets:
  count: 1
hit:
- op: choose_by_predicate
  test:
    self_bloodied: true
  then:
  - op: damage
    average: 3
    amount:
      dice:
        count: 1
        sides: 4
      add: 1
    damage_type: poison
  else:
  - op: damage
    average: 6
    amount:
      dice:
        count: 2
        sides: 4
      add: 1
    damage_type: poison
reach_feet: 5
```

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["bludgeoning", "piercing", "slashing"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` conditions ` | ` ["charmed", "frightened", "grappled", "paralyzed", "petrified", "prone", "restrained", "stunned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 8 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 30 ` |

#### Communication

##### Speaks

<!-- splat:yaml path=splat_rule/parameters/communication/speaks -->

```yaml
[]
```

##### Understands

<!-- splat:yaml path=splat_rule/parameters/communication/understands -->

```yaml
[]
```

#### Movement variant

<!-- splat:values path=splat_rule/parameters/movement_variant -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` gm ` |

##### Choose one

<!-- splat:table
path: splat_rule/parameters/movement_variant/choose_one
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Climb speed feet:
    path: climb_speed_feet
    type: integer
    optional: true
  Fly speed feet:
    path: fly_speed_feet
    type: integer
    optional: true
-->

| Key | Climb speed feet | Fly speed feet |
| --- | --- | --- |
| ` row_1 ` | ` 20 ` |  |
| ` row_2 ` |  | ` 20 ` |

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
