---
splat_schema: 1
splat_id: fbed3a50-0260-5326-beca-47503b2433dc
splat_kind: creature
splat_key: dnd2024:creatures/oni
splat_name: Oni
splat_origin:
  key: dnd2024:reference/creature/oni
splat_sources:
- title: SRD 5.2.1
  locator: pp. 312, 313
  url: https://www.dndbeyond.com/srd
---

# Oni

## Source text

<!-- SRD page 312 -->

Large Fiend, Lawful Evil\
AC 17  Initiative +0 (10)\
HP 119 (14d10 + 42)\
Speed 30 ft., Fly 30 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 11 +0 +3 Con 16 +3 +6\
Int 14 +2 +2 Wis 12 +1 +4 Cha 15 +2 +5\
Skills Arcana +5, Deception +8, Perception +4\
Resistances Cold\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Common, Giant\
CR 7 (XP 2,900; PB +3)\

### Traits

Regeneration. The oni regains 10 Hit Points at the start\
of each of its turns if it has at least 1 Hit Point.\

### Actions

Multiattack. The oni makes two Claw or Nightmare\
Ray attacks. It can replace one attack with a use of\
Spellcasting.\

<!-- SRD page 313 -->

Claw. Melee Attack Roll: +7, reach 10 ft. Hit: 10 (1d12\
+ 4) Slashing damage plus 9 (2d8) Necrotic damage.\
Nightmare Ray. Ranged Attack Roll: +5, range 60 ft.\
Hit: 9 (2d6 + 2) Psychic damage, and the target has\
the Frightened condition until the start of the oni’s\
next turn.\
Shape-Shift. The oni shape-shifts into a Small or Me-\
dium Humanoid or a Large Giant, or it returns to its\
true form. Other than its size, its game statistics are the\
same in each form. Any equipment it is wearing or car-\
rying isn’t transformed.\
Spellcasting. The oni casts one of the following spells,\
requiring no Material components and using Charisma\
as the spellcasting ability (spell save DC 13):\
1/Day Each: Charm Person (level 2 version), Darkness,\
Gaseous Form, Sleep\

### Bonus Actions

Invisibility. The oni casts Invisibility on itself, requiring\
no spell components and using the same spellcasting\
ability as Spellcasting.\

Original layout: [[../../srd-5.2.1/pages/312|page 312]], [[../../srd-5.2.1/pages/313|page 313]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Lawful Evil ` |
| ` movement_text ` | ` 30 ft., Fly 30 ft. (hover) ` |
| ` skills_text ` | ` Arcana +5, Deception +8, Perception +4 ` |
| ` resistances_text ` | ` Cold ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 14 ` |
| ` languages_text ` | ` Common, Giant ` |
| ` cr_text ` | ` 7 (XP 2,900; PB +3) ` |
| ` hit_points_text ` | ` 119 (14d10 + 42) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 19 ` | ` 4 ` |
| ` Dexterity ` | ` 11 ` | ` 3 ` |
| ` Constitution ` | ` 16 ` | ` 6 ` |
| ` Intelligence ` | ` 14 ` | ` 2 ` |
| ` Wisdom ` | ` 12 ` | ` 4 ` |
| ` Charisma ` | ` 15 ` | ` 5 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 119 ` |  |
| ` initiative ` | ` 0 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` fly_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/oni ` |
| ` implementation ` | ` dnd2024:creature_oni ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold"] ` |

#### Features

##### Regeneration

<!-- splat:yaml path=splat_rule/parameters/features/regeneration -->

```yaml
economy: trait
operation: passive_rule
trigger: self.turn_start
requires:
  minimum_hp: 1
heal: 10
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_choice
  count: 2
  options:
  - claw
  - nightmare_ray
replacements:
- replace_count: 1
  maximum_uses: 1
  operation:
    operation: use_feature
    feature: spellcasting
```

##### Claw

<!-- splat:values path=splat_rule/parameters/features/claw -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/claw/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 7
reach_ft: 10
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/claw/hit/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Average:
    path: average
    type: integer
  Roll count:
    path: roll/count
    type: integer
  Roll sides:
    path: roll/sides
    type: integer
  Roll modifier:
    path: roll/modifier
    type: integer
  Type:
    path: type
    type: symbol
-->

| Key | Average | Roll count | Roll sides | Roll modifier | Type |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 10 ` | ` 1 ` | ` 12 ` | ` 4 ` | ` Slashing ` |
| ` row_2 ` | ` 9 ` | ` 2 ` | ` 8 ` | ` 0 ` | ` Necrotic ` |

##### Nightmare ray

<!-- splat:yaml path=splat_rule/parameters/features/nightmare_ray -->

```yaml
economy: action
operation: attack
attack:
  mode: ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  range_ft: 60
  long_range_ft: null
hit:
  damage:
  - average: 9
    roll:
      count: 2
      sides: 6
      modifier: 2
    type: psychic
  conditions:
  - condition: frightened
    until: self.next_turn_start
```

##### Shape shift

<!-- splat:values path=splat_rule/parameters/features/shape_shift -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` shape_shift ` |
| ` statistics_preserved_except ` | ` ["size"] ` |
| ` equipment_transformed ` | ` false ` |

###### Forms

<!-- splat:table
path: splat_rule/parameters/features/shape_shift/forms
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Type:
    path: type
    type: symbol
    optional: true
  Form:
    path: form
    type: symbol
    optional: true
  Details:
    type: section
    optional: true
-->

| Key | Type | Form | Details |
| --- | --- | --- | --- |
| ` row_1 ` | ` Humanoid ` |  | [Rules](#^splat-8910289c-row_1 "section:row_1") |
| ` row_2 ` | ` Giant ` |  | [Rules](#^splat-8910289c-row_2 "section:row_2") |
| ` row_3 ` |  | ` True form ` |  |

###### Row 1

^splat-8910289c-row_1

###### Size

<!-- splat:yaml path=splat_rule/parameters/features/shape_shift/forms/@row_1/size -->

```yaml
- small
- medium
```

###### Row 2

^splat-8910289c-row_2

###### Size

<!-- splat:yaml path=splat_rule/parameters/features/shape_shift/forms/@row_2/size -->

```yaml
large
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` spellcasting ` |
| ` ability ` | ` charisma ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 13 ` |
| ` material_components_available ` | ` true ` |

###### Options

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/options
key: Spell
order:
- charm_person
- darkness
- gaseous_form
- sleep
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: usage/kind
    type: symbol
  Uses:
    path: usage/uses
    type: integer
  Use pool:
    path: usage/pool
    type: symbol
  Recovery:
    path: usage/recovery
    type: symbol
  Cast level:
    path: cast_level
    type: integer
    optional: true
-->

| Spell | Frequency | Uses | Use pool | Recovery | Cast level |
| --- | --- | --- | --- | --- | --- |
| [Charm Person](<../spells/charm_person.md> "dnd2024:spells/charm_person") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` | ` 2 ` |
| [Darkness](<../spells/darkness.md> "dnd2024:spells/darkness") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |  |
| [Gaseous Form](<../spells/gaseous_form.md> "dnd2024:spells/gaseous_form") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |  |
| [Sleep](<../spells/sleep.md> "dnd2024:spells/sleep") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |  |

##### Invisibility

<!-- splat:yaml path=splat_rule/parameters/features/invisibility -->

```yaml
economy: bonus_action
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

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` arcana ` | ` 5 ` |
| ` deception ` | ` 8 ` |
| ` perception ` | ` 4 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 14 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
- giant
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
