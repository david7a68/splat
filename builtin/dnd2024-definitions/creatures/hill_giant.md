---
splat_schema: 1
splat_id: f0ab06b4-adb6-53ac-b983-62c72d1555cd
splat_kind: creature
splat_key: dnd2024:creatures/hill_giant
splat_name: Hill Giant
splat_origin:
  key: dnd2024:reference/creature/hill_giant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 298
  url: https://www.dndbeyond.com/srd
---

# Hill Giant

## Source text

<!-- SRD page 298 -->

Huge Giant, Chaotic Evil\
AC 13 Initiative +2 (12)\
HP 105 (10d12 + 40)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 8 −1 −1 Con 19 +4 +4\
Int 5 −3 −3 Wis 9 −1 −1 Cha 6 −2 −2\
Skills Perception +2\
Senses Passive Perception 12\
Languages Giant\
CR 5 (XP 1,800; PB +3)\

### Actions

Multiattack. The giant makes two attacks, using Tree\
Club or Trash Lob in any combination.\
Tree Club. Melee Attack Roll: +8, reach 10 ft. Hit: 18\
(3d8 + 5) Bludgeoning damage. If the target is a Large\
or smaller creature, it has the Prone condition.\
Trash Lob. Ranged Attack Roll: +8, range 60/240\
ft. Hit: 16 (2d10 + 5) Bludgeoning damage, and the\
target has the Poisoned condition until the end of its\
next turn.\

Original layout: [[../../srd-5.2.1/pages/298|page 298]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` giant ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` movement_text ` | ` 40 ft. ` |
| ` skills_text ` | ` Perception +2 ` |
| ` senses_text ` | ` Passive Perception 12 ` |
| ` languages_text ` | ` Giant ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 105 (10d12 + 40) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 21 ` | ` 5 ` |
| ` Dexterity ` | ` 8 ` | ` -1 ` |
| ` Constitution ` | ` 19 ` | ` 4 ` |
| ` Intelligence ` | ` 5 ` | ` -3 ` |
| ` Wisdom ` | ` 9 ` | ` -1 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 105 ` |  |
| ` initiative ` | ` 2 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/hill_giant ` |
| ` implementation ` | ` dnd2024:creature_hill_giant ` |

### Parameters

#### Features

##### Tree club

<!-- splat:yaml path=splat_rule/parameters/features/tree_club -->

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
  average: 18
  amount:
    dice:
      count: 3
      sides: 8
    add: 5
  damage_type: bludgeoning
- op: if
  test:
    target_kind: creature
    target_size_max: large
  then:
  - op: condition
    condition: prone
reach_feet: 10
```

##### Trash lob

<!-- splat:yaml path=splat_rule/parameters/features/trash_lob -->

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
  average: 16
  amount:
    dice:
      count: 2
      sides: 10
    add: 5
  damage_type: bludgeoning
- op: condition
  condition: poisoned
  ends:
    event: turn.end
    participant: target
    occurrence: next
range_feet: 60
long_range_feet: 240
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: repeat_choice
count: 2
choices:
- feature: tree_club
- feature: trash_lob
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 2 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["giant"] ` |
| ` understands ` | ` ["giant"] ` |

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
