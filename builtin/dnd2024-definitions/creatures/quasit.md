---
splat_schema: 1
splat_id: 85d3ac67-eb3a-59c1-9d26-3976f026a7be
splat_kind: creature
splat_key: dnd2024:creatures/quasit
splat_name: Quasit
splat_origin:
  key: dnd2024:reference/creature/quasit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 317
  url: https://www.dndbeyond.com/srd
---

# Quasit

## Source text

<!-- SRD page 317 -->

Tiny Fiend (Demon), Chaotic Evil\
AC 13  Initiative +3 (13)\
HP 25 (10d4)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 5 −3 −3 Dex 17 +3 +3 Con 10 +0 +0\
Int 7 −2 −2 Wis 10 +0 +0 Cha 10 +0 +0\
Skills Stealth +5\
Resistances Cold, Fire, Lightning\
Immunities Poison; Poisoned\
Senses Darkvision 120 ft.; Passive Perception 10\
Languages Abyssal, Common\
CR 1 (XP 200; PB +2)\

### Traits

Magic Resistance. The quasit has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3)\
Slashing damage, and the target has the Poisoned con-\
dition until the start of the quasit’s next turn.\
Invisibility. The quasit casts Invisibility on itself, requir-\
ing no spell components and using Charisma as the\
spellcasting ability.\
Scare (1/Day). Wisdom Saving Throw: DC 10, one\
creature within 20 feet. Failure: The target has the\
Frightened condition. At the end of each of its turns, the\
target repeats the save, ending the effect on itself on a\
success. After 1 minute, it succeeds automatically.\
Shape-Shift. The quasit shape-shifts to resemble a bat\
(Speed 10 ft., Fly 40 ft.), a centipede (40 ft., Climb 40\
ft.), or a toad (40 ft., Swim 40 ft.), or it returns to its\
true form. Its game statistics are the same in each form,\
except for its Speed. Any equipment it is wearing or\
carrying isn’t transformed.\

Original layout: [[../../srd-5.2.1/pages/317|page 317]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` tiny ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Demon) ` |
| ` movement_text ` | ` 40 ft. ` |
| ` skills_text ` | ` Stealth +5 ` |
| ` resistances_text ` | ` Cold, Fire, Lightning ` |
| ` immunities_text ` | ` Poison; Poisoned ` |
| ` senses_text ` | ` Darkvision 120 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Abyssal, Common ` |
| ` cr_text ` | ` 1 (XP 200; PB +2) ` |
| ` hit_points_text ` | ` 25 (10d4) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 5 ` | ` -3 ` |
| ` Dexterity ` | ` 17 ` | ` 3 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 7 ` | ` -2 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 25 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/quasit ` |
| ` implementation ` | ` dnd2024:creature_quasit ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold", "fire", "lightning"] ` |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["poisoned"] ` |

#### Features

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` saving_throw_advantage_against ` | ` ["spells", "magical_effects"] ` |

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  reach_ft: 5
hit:
  damage:
  - average: 5
    roll:
      count: 1
      sides: 4
      modifier: 3
    type: slashing
  conditions:
  - condition: poisoned
    until: self.next_turn_start
```

##### Invisibility

<!-- splat:yaml path=splat_rule/parameters/features/invisibility -->

```yaml
economy: action
operation: cast_spell
spell:
  key: dnd2024:spells/invisibility
ability: charisma
targets:
- self
components_waived:
- verbal
- somatic
- material
```

##### Scare

<!-- splat:yaml path=splat_rule/parameters/features/scare -->

```yaml
economy: action
resource:
  kind: daily
  uses: 1
  recovery: long_rest
operation: saving_throw
ability: wisdom
dc: 10
targets:
  kind: creature
  count: 1
  range_ft: 20
  visible: false
failure:
  conditions:
  - condition: frightened
  repeat_save:
    event: target.turn_end
    success: end_effect
  automatic_success_after_seconds: 60
success:
  effects: []
```

##### Shape shift

<!-- splat:values path=splat_rule/parameters/features/shape_shift -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` shape_shift ` |
| ` statistics_preserved_except ` | ` ["speed"] ` |
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
  Speed ft:
    path: speed_ft
    type: integer
    optional: true
  Fly speed ft:
    path: fly_speed_ft
    type: integer
    optional: true
  Climb speed ft:
    path: climb_speed_ft
    type: integer
    optional: true
  Swim speed ft:
    path: swim_speed_ft
    type: integer
    optional: true
-->

| Key | Form | Speed ft | Fly speed ft | Climb speed ft | Swim speed ft |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Bat ` | ` 10 ` | ` 40 ` |  |  |
| ` row_2 ` | ` Centipede ` | ` 40 ` |  | ` 40 ` |  |
| ` row_3 ` | ` Toad ` | ` 40 ` |  |  | ` 40 ` |
| ` row_4 ` | ` True form ` |  |  |  |  |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 5 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 120 ` |
| ` passive_perception ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- abyssal
- common
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
