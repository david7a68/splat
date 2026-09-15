---
splat_schema: 1
splat_id: b9070a54-028a-52b6-a468-1414db2a2d04
splat_kind: creature
splat_key: dnd2024:creatures/earth_elemental
splat_name: Earth Elemental
splat_origin:
  key: dnd2024:reference/creature/earth_elemental
splat_sources:
- title: SRD 5.2.1
  locator: pp. 282, 283
  url: https://www.dndbeyond.com/srd
---

# Earth Elemental

## Source text

<!-- SRD page 282 -->

Large Elemental, Neutral\
AC 17 Initiative −1 (9)\
HP 147 (14d10 + 70)\
Speed 30 ft., Burrow 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +5 Dex 8 −1 −1 Con 20 +5 +5\
Int 5 −3 −3 Wis 10 +0 +0 Cha 5 −3 −3\
Vulnerabilities Thunder\
Immunities Poison; Exhaustion, Paralyzed, Petrified,\
Poisoned, Unconscious\
Senses Darkvision 60 ft., Tremorsense 60 ft.;\
Passive Perception 10\
Languages Primordial (Terran)\
CR 5 (XP 1,800; PB +3)\

### Traits

Earth Glide. The elemental can burrow through\
nonmagical, unworked earth and stone. While do-\
ing so, the elemental doesn’t disturb the material it\
moves through.\
Siege Monster. The elemental deals double damage to\
objects and structures.\

### Actions

Multiattack. The elemental makes two attacks, using\
Slam or Rock Launch in any combination.\
Slam. Melee Attack Roll: +8, reach 10 ft. Hit: 14 (2d8\
+ 5) Bludgeoning damage.\

<!-- SRD page 283 -->

Rock Launch. Ranged Attack Roll: +8, range 60 ft. Hit:\
8 (1d6 + 5) Bludgeoning damage. If the target is a Large\
or smaller creature, it has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/282|page 282]], [[../../srd-5.2.1/pages/283|page 283]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` elemental ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 30 ft., Burrow 30 ft. ` |
| ` vulnerabilities_text ` | ` Thunder ` |
| ` immunities_text ` | ` Poison; Exhaustion, Paralyzed, Petrified, Poisoned, Unconscious ` |
| ` senses_text ` | ` Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Primordial (Terran) ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 147 (14d10 + 70) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 20 ` | ` 5 ` |
| ` Dexterity ` | ` 8 ` | ` -1 ` |
| ` Constitution ` | ` 20 ` | ` 5 ` |
| ` Intelligence ` | ` 5 ` | ` -3 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 5 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 147 ` |  |
| ` initiative ` | ` -1 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` burrow_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/earth_elemental ` |
| ` implementation ` | ` dnd2024:creature_earth_elemental ` |

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
attack_bonus: 8
targets:
  count: 1
hit:
- op: damage
  average: 14
  amount:
    dice:
      count: 2
      sides: 8
    add: 5
  damage_type: bludgeoning
reach_feet: 10
```

##### Rock launch

<!-- splat:yaml path=splat_rule/parameters/features/rock_launch -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 8
targets:
  count: 1
hit:
- op: damage
  average: 8
  amount:
    dice:
      count: 1
      sides: 6
    add: 5
  damage_type: bludgeoning
- op: if
  test:
    target_kind: creature
    target_size_max: large
  then:
  - op: condition
    condition: prone
range_feet: 60
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` repeat_choice ` |
| ` count ` | ` 2 ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/multiattack/choices
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Feature:
    path: feature
    type: symbol
-->

| Key | Feature |
| --- | --- |
| ` row_1 ` | ` Slam ` |
| ` row_2 ` | ` Rock launch ` |

##### Earth glide

<!-- splat:yaml path=splat_rule/parameters/features/earth_glide -->

```yaml
economy: passive
burrow_through:
  materials:
  - earth
  - stone
  magical: false
  worked: false
disturbs_material: false
```

#### Defenses

##### Vulnerabilities

<!-- splat:values path=splat_rule/parameters/defenses/vulnerabilities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["thunder"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["poison"] ` |
| ` conditions ` | ` ["exhaustion", "paralyzed", "petrified", "poisoned", "unconscious"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

##### Special

<!-- splat:table
path: splat_rule/parameters/senses/special
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Sense:
    path: sense
    type: symbol
  Range feet:
    path: range_feet
    type: integer
-->

| Key | Sense | Range feet |
| --- | --- | --- |
| ` row_1 ` | ` Darkvision ` | ` 60 ` |
| ` row_2 ` | ` Tremorsense ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["primordial"] ` |
| ` understands ` | ` ["primordial"] ` |
| ` dialects ` | ` ["terran"] ` |

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
