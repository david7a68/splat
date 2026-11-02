---
splat_schema: 1
splat_id: d7d62b80-e321-5292-b081-c21b13ca111a
splat_kind: creature
splat_key: dnd2024:creatures/giant_toad
splat_name: Giant Toad
splat_origin:
  key: dnd2024:reference/creature/giant_toad
splat_sources:
- title: SRD 5.2.1
  locator: pp. 354
  url: https://www.dndbeyond.com/srd
---

# Giant Toad

## Source text

<!-- SRD page 354 -->

Large Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 39 (6d10 + 6)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 13 +1 +1 Con 13 +1 +1\
Int 2 −4 −4 Wis 10 +0 +0 Cha 3 −4 −4\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages None\
CR 1 (XP 200; PB +2)

### Traits

Amphibious. The toad can breathe air and water.

Standing Leap. The toad’s Long Jump is up to 20 feet and its High Jump is up to 10 feet with or without a running start.

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage plus 5 (2d4) Poison damage. If the target is a Medium or smaller creature, it has the Grappled condition (escape DC 12).

Swallow. The toad swallows a Medium or smaller target it is grappling. While swallowed, the target isn’t Grappled but has the Blinded and Restrained conditions, and it has Total Cover against attacks and other effects outside the toad. In addition, the target takes 10 (3d6) Acid damage at the end of each of the toad’s turns. The toad can have only one target swallowed at a time, and it can’t use Bite while it has a swallowed target. If the toad dies, a swallowed creature is no longer Restrained and can escape from the corpse using 5 feet of movement, exiting with the Prone condition.

Original layout: [[../../srd-5.2.1/pages/354|page 354]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 30 ft., Swim 30 ft. ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1 (XP 200; PB +2) ` |
| ` hit_points_text ` | ` 39 (6d10 + 6) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 15 ` | ` 2 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 3 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 39 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` swim_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_toad ` |
| ` implementation ` | ` dnd2024:creature_giant_toad ` |

### Parameters

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` breathing_media ` | ` ["air", "water"] ` |

##### Standing leap

<!-- splat:values path=splat_rule/parameters/features/standing_leap -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` long_jump_max_feet ` | ` 20 ` |
| ` high_jump_max_feet ` | ` 10 ` |
| ` running_start_required ` | ` false ` |

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
  average: 5
  amount:
    dice:
      count: 1
      sides: 6
    add: 2
  damage_type: piercing
- op: damage
  average: 5
  amount:
    dice:
      count: 2
      sides: 4
    add: 0
  damage_type: poison
- op: if
  test:
    target_kind: creature
    target_size_max: medium
  then:
  - op: condition
    condition: grappled
    escape_dc: 12
    source: self
reach_feet: 5
```

##### Swallow

<!-- splat:yaml path=splat_rule/parameters/features/swallow -->

```yaml
economy: action
op: swallow
targets:
  count: 1
  grappled_by: self
  size_max: medium
on_swallow:
- op: remove_condition
  condition: grappled
- op: condition
  condition: blinded
- op: condition
  condition: restrained
while_swallowed:
  total_cover_against: external_attacks_and_effects
  self_cannot_use:
  - bite
self_death:
  remove_conditions:
  - restrained
  escape:
    movement_cost_feet: 5
    exit_condition: prone
digest:
  event: self.turn.end
  occurrence: each
  effects:
  - op: damage
    average: 10
    amount:
      dice:
        count: 3
        sides: 6
      add: 0
    damage_type: acid
swallowed_capacity: 1
```

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
