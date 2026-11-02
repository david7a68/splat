---
splat_schema: 1
splat_id: ae140097-94e9-5b72-8a9b-0f19ce7ae08a
splat_kind: creature
splat_key: dnd2024:creatures/blink_dog
splat_name: Blink Dog
splat_origin:
  key: dnd2024:reference/creature/blink_dog
splat_sources:
- title: SRD 5.2.1
  locator: pp. 266
  url: https://www.dndbeyond.com/srd
---

# Blink Dog

## Source text

<!-- SRD page 266 -->

Medium Fey, Lawful Good\
AC 13 Initiative +3 (13)\
HP 22 (4d8 + 4)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 17 +3 +3 Con 12 +1 +1\
Int 10 +0 +0 Wis 13 +1 +1 Cha 11 +0 +0\
Skills Perception +5, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages Blink Dog; understands Elvish and Sylvan but can’t speak them\
CR 1/4 (XP 50; PB +2)

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3) Piercing damage.

### Bonus Actions

Teleport (Recharge 4–6). The dog teleports up to 40 feet to an unoccupied space it can see.

Original layout: [[../../srd-5.2.1/pages/266|page 266]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` fey ` |
| ` alignment ` | ` Lawful Good ` |
| ` movement_text ` | ` 40 ft. ` |
| ` skills_text ` | ` Perception +5, Stealth +5 ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 15 ` |
| ` languages_text ` | ` Blink Dog; understands Elvish and Sylvan but can’t speak them ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 22 (4d8 + 4) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 12 ` | ` 1 ` |
| ` Dexterity ` | ` 17 ` | ` 3 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 13 ` | ` 1 ` |
| ` Charisma ` | ` 11 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 22 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/blink_dog ` |
| ` implementation ` | ` dnd2024:creature_blink_dog ` |

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
  average: 5
  amount:
    dice:
      count: 1
      sides: 4
    add: 3
  damage_type: piercing
reach_feet: 5
```

##### Teleport

<!-- splat:yaml path=splat_rule/parameters/features/teleport -->

```yaml
economy: bonus_action
resource:
  capacity: 1
  initial: 1
  recharge:
    event: turn.start
    participant: self
    roll:
      dice:
        count: 1
        sides: 6
      add: 0
    succeeds_on:
    - 4
    - 5
    - 6
  cost: 1
  recovery_also:
  - short_rest
  - long_rest
op: teleport
targets: self
range_feet: 40
destination:
  unoccupied: true
  visible_to: self
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |
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
| ` speaks ` | ` ["blink_dog"] ` |
| ` understands ` | ` ["blink_dog", "elvish", "sylvan"] ` |

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
