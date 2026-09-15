---
splat_schema: 1
splat_id: 89e2602e-5fd7-5f0c-82c6-d5bb78e0fcf5
splat_kind: creature
splat_key: dnd2024:creatures/wererat
splat_name: Wererat
splat_origin:
  key: dnd2024:reference/creature/wererat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 338
  url: https://www.dndbeyond.com/srd
---

# Wererat

## Source text

<!-- SRD page 338 -->

Medium or Small Monstrosity (Lycanthrope), Lawful Evil\
AC 13 Initiative +3 (13)\
HP 60 (11d8 + 11)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 16 +3 +3 Con 12 +1 +1\
Int 11 +0 +0 Wis 10 +0 +0 Cha 8 −1 −1\
Skills Perception +4, Stealth +5\
Gear Hand Crossbow\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Common (can’t speak in rat form)\
CR 2 (XP 450; PB +2)\

### Actions

Multiattack. The wererat makes two attacks, using\
Scratch or Hand Crossbow in any combination. It can\
replace one attack with a Bite attack.\
Bite (Rat or Hybrid Form Only). Melee Attack Roll:\
+5, reach 5 ft. Hit: 8 (2d4 + 3) Piercing damage. If the\
target is a Humanoid, it is subjected to the following\
effect. Constitution Saving Throw: DC 11. Failure: The\
target is cursed. If the cursed target drops to 0 Hit\
Points, it instead becomes a Wererat under the GM’s\
control and has 10 Hit Points. Success: The target is im-\
mune to this wererat’s curse for 24 hours.\
Scratch. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6\
+ 3) Slashing damage.\
Hand Crossbow (Humanoid or Hybrid Form Only).\
Ranged Attack Roll: +5, range 30/120 ft. Hit: 6 (1d6\
+ 3) Piercing damage.\

### Bonus Actions

Shape-Shift. The wererat shape-shifts into a Medium\
rat-humanoid hybrid or a Small rat, or it returns to its\
true humanoid form. Its game statistics, other than its\
size, are the same in each form. Any equipment it is\
wearing or carrying isn’t transformed.\

Original layout: [[../../srd-5.2.1/pages/338|page 338]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` monstrosity ` |
| ` alignment ` | ` Lawful Evil ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` tags_text ` | ` (Lycanthrope) ` |
| ` movement_text ` | ` 30 ft., Climb 30 ft. ` |
| ` skills_text ` | ` Perception +4, Stealth +5 ` |
| ` gear_text ` | ` Hand Crossbow ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 14 ` |
| ` languages_text ` | ` Common (can’t speak in rat form) ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 60 (11d8 + 11) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 0 ` |
| ` Dexterity ` | ` 16 ` | ` 3 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 11 ` | ` 0 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 8 ` | ` -1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 60 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/wererat ` |
| ` implementation ` | ` dnd2024:creature_wererat ` |

### Parameters

#### Features

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
      count: 2
      sides: 4
    add: 3
  damage_type: piercing
- op: if
  test:
    target_type: humanoid
  then:
  - op: save
    ability: constitution
    dc: 11
    targets:
      participant: target
    failure:
    - op: curse
      trigger: target.hit_points_would_become_zero
      replacement:
        op: replace_stat_block
        definition:
          key: dnd2024:creatures/wererat
        controller: gm
        hit_points: 10
    success:
    - op: feature_immunity
      source: self
      feature: bite_curse
      duration:
        hours: 24
reach_feet: 5
requires_form:
- rat
- hybrid
```

##### Scratch

<!-- splat:yaml path=splat_rule/parameters/features/scratch -->

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
  damage_type: slashing
reach_feet: 5
range_feet: 30
long_range_feet: 120
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
| ` row_1 ` | ` Scratch ` |
| ` row_2 ` | ` Hand crossbow ` |

###### Replacement

<!-- splat:yaml path=splat_rule/parameters/features/multiattack/replacement -->

```yaml
max_attacks: 1
choices:
- feature: bite
```

##### Shape shift

<!-- splat:values path=splat_rule/parameters/features/shape_shift -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` shape_shift ` |
| ` retain_statistics_except ` | ` ["size"] ` |
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
  Form:
    path: form
    type: symbol
  Size:
    path: size
    type: symbol
    optional: true
-->

| Key | Form | Size |
| --- | --- | --- |
| ` row_1 ` | ` Hybrid ` | ` Medium ` |
| ` row_2 ` | ` Rat ` | ` Small ` |
| ` row_3 ` | ` True humanoid ` |  |

##### Hand crossbow

<!-- splat:yaml path=splat_rule/parameters/features/hand_crossbow -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 5
range_feet: 30
long_range_feet: 120
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
requires_form:
- humanoid
- hybrid
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |
| ` stealth ` | ` 5 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item_name | quantity |
| --- | --- | --- |
| ` 0 ` | ` Hand Crossbow ` | ` 1 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common"] ` |
| ` understands ` | ` ["common"] ` |
| ` cannot_speak_in_forms ` | ` ["rat"] ` |

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
