---
splat_schema: 1
splat_id: 7e9cb656-6ad5-5a5c-9c31-0effa594bc32
splat_kind: creature
splat_key: dnd2024:creatures/succubus
splat_name: Succubus
splat_origin:
  key: dnd2024:reference/creature/succubus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 331
  url: https://www.dndbeyond.com/srd
---

# Succubus

## Source text

<!-- SRD page 331 -->

Medium Fiend, Neutral Evil\
AC 15 Initiative +3 (13)\
HP 71 (13d8 + 13)\
Speed 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 8 −1 −1 Dex 17 +3 +3 Con 13 +1 +1\
Int 15 +2 +2 Wis 12 +1 +1 Cha 20 +5 +5\
Skills Deception +9, Insight +5, Perception +5, Persuasion\
+9, Stealth +7\
Resistances Cold, Fire, Poison, Psychic\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages Abyssal, Common, Infernal; telepathy 60 ft.\
CR 4 (XP 1,100; PB +2)\

### Traits

Incubus Form. When the succubus finishes a Long\
Rest, it can shape-shift into an Incubus, using that stat\
block instead of this one.\

### Actions

Multiattack. The succubus makes one Fiendish Touch\
attack and uses Charm or Draining Kiss.\
Fiendish Touch. Melee Attack Roll: +7, reach 5 ft. Hit:\
16 (2d10 + 5) Psychic damage.\
Charm. The succubus casts Dominate Person (level 8\
version), requiring no spell components and using Cha-\
risma as the spellcasting ability (spell save DC 15).\
Draining Kiss. Constitution Saving Throw: DC 15, one\
creature Charmed by the succubus within 5 feet. Fail-\
ure: 13 (3d8) Psychic damage. Success: Half damage.\
Failure or Success: The target’s Hit Point maximum de-\
creases by an amount equal to the damage taken.\

### Bonus Actions

Shape-Shift. The succubus shape-shifts into a Medium\
or Small Humanoid, or it returns to its true form. Its\
game statistics are the same in each form, except its Fly\
Speed is available only in its true form. Any equipment\
it is wearing or carrying isn’t transformed.\

Original layout: [[../../srd-5.2.1/pages/331|page 331]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Neutral Evil ` |
| ` movement_text ` | ` 30 ft., Fly 60 ft. ` |
| ` skills_text ` | ` Deception +9, Insight +5, Perception +5, Persuasion +9, Stealth +7 ` |
| ` resistances_text ` | ` Cold, Fire, Poison, Psychic ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 15 ` |
| ` languages_text ` | ` Abyssal, Common, Infernal; telepathy 60 ft. ` |
| ` cr_text ` | ` 4 (XP 1,100; PB +2) ` |
| ` hit_points_text ` | ` 71 (13d8 + 13) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 8 ` | ` -1 ` |
| ` Dexterity ` | ` 17 ` | ` 3 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 15 ` | ` 2 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 20 ` | ` 5 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 15 ` |  |
| ` max_hp ` | ` 71 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` fly_speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/succubus ` |
| ` implementation ` | ` dnd2024:creature_succubus ` |

### Parameters

#### Features

##### Fiendish touch

<!-- splat:yaml path=splat_rule/parameters/features/fiendish_touch -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 7
targets:
  count: 1
hit:
- op: damage
  average: 16
  amount:
    dice:
      count: 2
      sides: 10
    add: 5
  damage_type: psychic
reach_feet: 5
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: fiendish_touch
  count: 1
- op: choose
  choices:
  - op: use_feature
    feature: charm
    count: 1
  - op: use_feature
    feature: draining_kiss
    count: 1
```

##### Charm

<!-- splat:yaml path=splat_rule/parameters/features/charm -->

```yaml
economy: action
op: cast_spell
spell:
  key: dnd2024:spells/dominate_person
ability: charisma
save_dc: 15
components_waived:
- verbal
- somatic
- material
cast_level: 8
```

##### Draining kiss

<!-- splat:yaml path=splat_rule/parameters/features/draining_kiss -->

```yaml
economy: action
op: save
ability: constitution
dc: 15
targets:
  kind: creature
  count: 1
  range_feet: 5
  charmed_by: self
failure:
- op: damage
  average: 13
  amount:
    dice:
      count: 3
      sides: 8
    add: 0
  damage_type: psychic
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
after_save:
- op: reduce_hit_point_maximum
  amount:
    event_value: damage_taken
```

##### Shape shift

<!-- splat:values path=splat_rule/parameters/features/shape_shift -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` shape_shift ` |
| ` retain_statistics_except ` | ` ["fly_speed"] ` |
| ` equipment_transformed ` | ` false ` |

###### Forms

<!-- splat:table
path: splat_rule/parameters/features/shape_shift/forms
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Form:
    path: form
    type: symbol
  Size choice:
    path: size_choice
    type: yaml
    optional: true
  Fly speed available:
    path: fly_speed_available
    type: boolean
-->

| Key | Form | Size choice | Fly speed available |
| --- | --- | --- | --- |
| ` row_1 ` | ` Humanoid ` | ` ["medium", "small"] ` | false |
| ` row_2 ` | ` True ` |  | true |

##### Incubus form

<!-- splat:yaml path=splat_rule/parameters/features/incubus_form -->

```yaml
economy: passive
trigger: self.long_rest.finished
optional: true
op: replace_stat_block
definition:
  key: dnd2024:creatures/incubus
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 9 ` |
| ` insight ` | ` 5 ` |
| ` perception ` | ` 5 ` |
| ` persuasion ` | ` 9 ` |
| ` stealth ` | ` 7 ` |

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["cold", "fire", "poison", "psychic"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["abyssal", "common", "infernal"] ` |
| ` understands ` | ` ["abyssal", "common", "infernal"] ` |

##### Telepathy

<!-- splat:values path=splat_rule/parameters/communication/telepathy -->

| Field | Value |
| --- | --- |
| ` range_feet ` | ` 60 ` |

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
