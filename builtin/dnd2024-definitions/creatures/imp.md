---
splat_schema: 1
splat_id: 7e990459-90d3-5c1a-bd68-288c41d1cfe0
splat_kind: creature
splat_key: dnd2024:creatures/imp
splat_name: Imp
splat_origin:
  key: dnd2024:reference/creature/imp
splat_sources:
- title: SRD 5.2.1
  locator: pp. 300, 301
  url: https://www.dndbeyond.com/srd
---

# Imp

## Source text

<!-- SRD page 300 -->

Tiny Fiend (Devil), Lawful Evil\
AC 13  Initiative +3 (13)\
HP 21 (6d4 + 6)\
Speed 20 ft., Fly 40 ft.\

<!-- SRD page 301 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 17 +3 +3 Con 13 +1 +1\
Int 11 +0 +0 Wis 12 +1 +1 Cha 14 +2 +2\
Skills Deception +4, Insight +3, Stealth +5\
Resistances Cold\
Immunities Fire, Poison; Poisoned\
Senses Darkvision 120 ft. (unimpeded by magical\
Darkness); Passive Perception 11\
Languages Common, Infernal\
CR 1 (XP 200; PB +2)\

### Traits

Magic Resistance. The imp has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Sting. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3)\
Piercing damage plus 7 (2d6) Poison damage.\
Invisibility. The imp casts Invisibility on itself, requiring\
no spell components and using Charisma as the spell-\
casting ability.\
Shape-Shift. The imp shape-shifts to resemble a rat\
(Speed 20 ft.), a raven (20 ft., Fly 60 ft.), or a spider\
(20 ft., Climb 20 ft.), or it returns to its true form. Its\
game statistics are the same in each form, except for\
its Speed. Any equipment it is wearing or carrying isn’t\
transformed.\

Original layout: [[../../srd-5.2.1/pages/300|page 300]], [[../../srd-5.2.1/pages/301|page 301]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` tiny ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Lawful Evil ` |
| ` tags_text ` | ` (Devil) ` |
| ` movement_text ` | ` 20 ft., Fly 40 ft. ` |
| ` skills_text ` | ` Deception +4, Insight +3, Stealth +5 ` |
| ` resistances_text ` | ` Cold ` |
| ` immunities_text ` | ` Fire, Poison; Poisoned ` |
| ` senses_text ` | ` Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 11 ` |
| ` languages_text ` | ` Common, Infernal ` |
| ` cr_text ` | ` 1 (XP 200; PB +2) ` |
| ` hit_points_text ` | ` 21 (6d4 + 6) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 6 ` | ` -2 ` |
| ` Dexterity ` | ` 17 ` | ` 3 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 11 ` | ` 0 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 14 ` | ` 2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 21 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |
| ` fly_speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/imp ` |
| ` implementation ` | ` dnd2024:creature_imp ` |

### Parameters

#### Features

##### Magic resistance

<!-- splat:yaml path=splat_rule/parameters/features/magic_resistance -->

```yaml
economy: passive
advantage:
  roll: saving_throw
  against:
  - spell
  - magical_effect
```

##### Sting

<!-- splat:yaml path=splat_rule/parameters/features/sting -->

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
  average: 6
  amount:
    dice:
      count: 1
      sides: 6
    add: 3
  damage_type: piercing
- op: damage
  average: 7
  amount:
    dice:
      count: 2
      sides: 6
    add: 0
  damage_type: poison
reach_feet: 5
```

##### Invisibility

<!-- splat:yaml path=splat_rule/parameters/features/invisibility -->

```yaml
economy: action
op: cast_spell
spell:
  key: dnd2024:spells/invisibility
ability: charisma
components_waived:
- verbal
- somatic
- material
targets: self
cast_level: spell_minimum
```

##### Shape shift

<!-- splat:values path=splat_rule/parameters/features/shape_shift -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` shape_shift ` |
| ` retain_statistics_except ` | ` ["speed"] ` |
| ` equipment_transformed ` | ` false ` |

###### Forms

<!-- splat:table
path: splat_rule/parameters/features/shape_shift/forms
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Form:
    path: form
    type: symbol
  Speeds feet walk:
    path: speeds_feet/walk
    type: integer
    optional: true
  Speeds feet fly:
    path: speeds_feet/fly
    type: integer
    optional: true
  Speeds feet climb:
    path: speeds_feet/climb
    type: integer
    optional: true
-->

| Key | Form | Speeds feet walk | Speeds feet fly | Speeds feet climb |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` Rat ` | ` 20 ` |  |  |
| ` row_2 ` | ` Raven ` | ` 20 ` | ` 60 ` |  |
| ` row_3 ` | ` Spider ` | ` 20 ` |  | ` 20 ` |
| ` row_4 ` | ` True ` |  |  |  |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 4 ` |
| ` insight ` | ` 3 ` |
| ` stealth ` | ` 5 ` |

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["cold"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["fire", "poison"] ` |
| ` conditions ` | ` ["poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet | magical_darkness_unimpeded |
| --- | --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` | ` true ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common", "infernal"] ` |
| ` understands ` | ` ["common", "infernal"] ` |

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
