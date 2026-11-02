---
splat_schema: 1
splat_id: 5aa5397e-f6d8-5109-9a7f-8b1b2ce96fde
splat_kind: creature
splat_key: dnd2024:creatures/spider
splat_name: Spider
splat_origin:
  key: dnd2024:reference/creature/spider
splat_sources:
- title: SRD 5.2.1
  locator: pp. 361
  url: https://www.dndbeyond.com/srd
---

# Spider

## Source text

<!-- SRD page 361 -->

Tiny Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 1 (1d4 − 1)\
Speed 20 ft., Climb 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 2 −4 −4 Dex 14 +2 +2 Con 8 −1 −1\
Int 1 −5 −5 Wis 10 +0 +0 Cha 2 −4 −4\
Skills Stealth +4\
Senses Darkvision 30 ft.; Passive Perception 10\
Languages None\
CR 0 (XP 10; PB +2)

### Traits

Spider Climb. The spider can climb difficult surfaces, including along ceilings, without needing to make an ability check.

Web Walker. The spider ignores movement restrictions caused by webs, and the spider knows the location of any other creature in contact with the same web.

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Piercing damage plus 2 (1d4) Poison damage.

Original layout: [[../../srd-5.2.1/pages/361|page 361]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` tiny ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 20 ft., Climb 20 ft. ` |
| ` skills_text ` | ` Stealth +4 ` |
| ` senses_text ` | ` Darkvision 30 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 0 (XP 10; PB +2) ` |
| ` hit_points_text ` | ` 1 (1d4 - 1) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 2 ` | ` -4 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 8 ` | ` -1 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 2 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 1 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |
| ` climb_speed ` | ` 20 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/spider ` |
| ` implementation ` | ` dnd2024:creature_spider ` |

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
attack_bonus: 4
targets:
  count: 1
hit:
- op: damage
  average: 1
  amount:
    constant: 1
  damage_type: piercing
- op: damage
  average: 2
  amount:
    dice:
      count: 1
      sides: 4
    add: 0
  damage_type: poison
reach_feet: 5
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 4 ` |

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
| ` 0 ` | ` darkvision ` | ` 30 ` |

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
