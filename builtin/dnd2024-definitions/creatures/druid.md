---
splat_schema: 1
splat_id: e9b6ff57-018a-5931-b144-c9338a133035
splat_kind: creature
splat_key: dnd2024:creatures/druid
splat_name: Druid
splat_origin:
  key: dnd2024:reference/creature/druid
splat_sources:
- title: SRD 5.2.1
  locator: pp. 282
  url: https://www.dndbeyond.com/srd
---

# Druid

## Source text

<!-- SRD page 282 -->

Medium or Small Humanoid (Druid), Neutral\
AC 13 Initiative +1 (11)\
HP 44 (8d8 + 8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 12 +1 +1 Con 13 +1 +1\
Int 12 +1 +1 Wis 16 +3 +3 Cha 11 +0 +0\
Skills Medicine +5, Nature +3, Perception +5\
Gear Studded Leather Armor\
Senses Passive Perception 15\
Languages Common, Druidic, Sylvan\
CR 2 (XP 450; PB +2)\

### Actions

Multiattack. The druid makes two attacks, using Vine\
Staff or Verdant Wisp in any combination.\
Vine Staff. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8\
+ 3) Bludgeoning damage plus 2 (1d4) Poison damage.\
Verdant Wisp. Ranged Attack Roll: +5, range 90 ft. Hit:\
10 (3d6) Radiant damage.\
Spellcasting. The druid casts one of the following\
spells, using Wisdom as the spellcasting ability (spell\
save DC 13):\
At Will: Druidcraft, Speak with Animals\
2/Day Each: Entangle, Thunderwave\
1/Day Each: Animal Messenger, Long-\
strider, Moonbeam\

Original layout: [[../../srd-5.2.1/pages/282|page 282]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` tags_text ` | ` (Druid) ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Medicine +5, Nature +3, Perception +5 ` |
| ` gear_text ` | ` Studded Leather Armor ` |
| ` senses_text ` | ` Passive Perception 15 ` |
| ` languages_text ` | ` Common, Druidic, Sylvan ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 44 (8d8 + 8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 0 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 12 ` | ` 1 ` |
| ` Wisdom ` | ` 16 ` | ` 3 ` |
| ` Charisma ` | ` 11 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 44 ` |  |
| ` initiative ` | ` 1 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/druid ` |
| ` implementation ` | ` dnd2024:creature_druid ` |

### Parameters

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_choice
  count: 2
  options:
  - vine_staff
  - verdant_wisp
```

##### Vine staff

<!-- splat:values path=splat_rule/parameters/features/vine_staff -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/vine_staff/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 5
reach_ft: 5
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/vine_staff/hit/damage
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
| ` row_1 ` | ` 7 ` | ` 1 ` | ` 8 ` | ` 3 ` | ` Bludgeoning ` |
| ` row_2 ` | ` 2 ` | ` 1 ` | ` 4 ` | ` 0 ` | ` Poison ` |

##### Verdant wisp

<!-- splat:yaml path=splat_rule/parameters/features/verdant_wisp -->

```yaml
economy: action
operation: attack
attack:
  mode: ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  range_ft: 90
  long_range_ft: null
hit:
  damage:
  - average: 10
    roll:
      count: 3
      sides: 6
      modifier: 0
    type: radiant
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` spellcasting ` |
| ` ability ` | ` wisdom ` |
| ` save_dc ` | ` 13 ` |
| ` material_components_available ` | ` true ` |

###### Components waived

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/components_waived -->

```yaml
[]
```

###### Options

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/options
key: Spell
order:
- druidcraft
- speak_with_animals
- entangle
- thunderwave
- animal_messenger
- longstrider
- moonbeam
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
    optional: true
  Use pool:
    path: usage/pool
    type: symbol
    optional: true
  Recovery:
    path: usage/recovery
    type: symbol
    optional: true
-->

| Spell | Frequency | Uses | Use pool | Recovery |
| --- | --- | --- | --- | --- |
| [Druidcraft](<../spells/druidcraft.md> "dnd2024:spells/druidcraft") | ` At will ` |  |  |  |
| [Speak with Animals](<../spells/speak_with_animals.md> "dnd2024:spells/speak_with_animals") | ` At will ` |  |  |  |
| [Entangle](<../spells/entangle.md> "dnd2024:spells/entangle") | ` Daily ` | ` 2 ` | ` Per spell ` | ` Long rest ` |
| [Thunderwave](<../spells/thunderwave.md> "dnd2024:spells/thunderwave") | ` Daily ` | ` 2 ` | ` Per spell ` | ` Long rest ` |
| [Animal Messenger](<../spells/animal_messenger.md> "dnd2024:spells/animal_messenger") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Longstrider](<../spells/longstrider.md> "dnd2024:spells/longstrider") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Moonbeam](<../spells/moonbeam.md> "dnd2024:spells/moonbeam") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` medicine ` | ` 5 ` |
| ` nature ` | ` 3 ` |
| ` perception ` | ` 5 ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item | quantity |
| --- | --- | --- |
| ` 0 ` | ` studded_leather_armor ` | ` 1 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
- druidic
- sylvan
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
