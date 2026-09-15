---
splat_schema: 1
splat_id: a44921eb-156c-59ad-88f3-8372efa9b185
splat_kind: creature
splat_key: dnd2024:creatures/phase_spider
splat_name: Phase Spider
splat_origin:
  key: dnd2024:reference/creature/phase_spider
splat_sources:
- title: SRD 5.2.1
  locator: pp. 313, 314
  url: https://www.dndbeyond.com/srd
---

# Phase Spider

## Source text

<!-- SRD page 313 -->

Large Monstrosity, Unaligned\
AC 14 Initiative +3 (13)\
HP 45 (7d10 + 7)\
Speed 30 ft., Climb 30 ft.\

<!-- SRD page 314 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 16 +3 +3 Con 12 +1 +1\
Int 6 −2 −2 Wis 10 +0 +0 Cha 6 −2 −2\
Skills Stealth +7\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages None\
CR 3 (XP 700; PB +2)\

### Traits

Ethereal Sight. The spider can see 60 feet into the Ethe-\
real Plane while on the Material Plane and vice versa.\
Spider Climb. The spider can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\
Web Walker. The spider ignores movement restrictions\
caused by webs, and the spider knows the location of\
any other creature in contact with the same web.\

### Actions

Multiattack. The spider makes two Bite attacks.\
Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 8 (1d10 +\
3) Piercing damage plus 9 (2d8) Poison damage. If this\
damage reduces the target to 0 Hit Points, the target\
becomes Stable, and it has the Poisoned condition for 1\
hour. While Poisoned, the target also has the Paralyzed\
condition.\

### Bonus Actions

Ethereal Jaunt. The spider teleports from the Material\
Plane to the Ethereal Plane or vice versa.\

Original layout: [[../../srd-5.2.1/pages/313|page 313]], [[../../srd-5.2.1/pages/314|page 314]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` monstrosity ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 30 ft., Climb 30 ft. ` |
| ` skills_text ` | ` Stealth +7 ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 3 (XP 700; PB +2) ` |
| ` hit_points_text ` | ` 45 (7d10 + 7) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 15 ` | ` 2 ` |
| ` Dexterity ` | ` 16 ` | ` 3 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 6 ` | ` -2 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 45 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` climb_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/phase_spider ` |
| ` implementation ` | ` dnd2024:creature_phase_spider ` |

### Parameters

#### Features

##### Spider climb

<!-- splat:values path=splat_rule/parameters/features/spider_climb -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` climb_ability_check_required ` | ` false ` |
| ` surfaces ` | ` ["difficult", "ceiling"] ` |

##### Web walker

<!-- splat:yaml path=splat_rule/parameters/features/web_walker -->

```yaml
economy: passive
ignore_movement_restrictions:
- web
knows_location:
  targets: creatures_touching_same_web
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 8
  amount:
    dice:
      count: 1
      sides: 10
    add: 3
  damage_type: piercing
- op: damage
  average: 9
  amount:
    dice:
      count: 2
      sides: 8
    add: 0
  damage_type: poison
- op: if
  test:
    attack_reduces_target_hp_to: 0
  then:
  - op: stabilize
  - op: condition
    condition: poisoned
    ends:
      hours: 1
    while_active:
      conditions:
      - paralyzed
reach_feet: 5
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: bite
  count: 2
```

##### Ethereal sight

<!-- splat:values path=splat_rule/parameters/features/ethereal_sight -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` from_planes ` | ` ["material", "ethereal"] ` |
| ` see_other_plane ` | ` true ` |
| ` range_feet ` | ` 60 ` |

##### Ethereal jaunt

<!-- splat:yaml path=splat_rule/parameters/features/ethereal_jaunt -->

```yaml
economy: bonus_action
op: teleport
plane_pairs:
- - material
  - ethereal
- - ethereal
  - material
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 7 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

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
