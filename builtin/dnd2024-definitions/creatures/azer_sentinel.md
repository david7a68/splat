---
splat_schema: 1
splat_id: 03f09722-b0fb-58dd-b1ec-e2e30de53d62
splat_kind: creature
splat_key: dnd2024:creatures/azer_sentinel
splat_name: Azer Sentinel
splat_origin:
  key: dnd2024:reference/creature/azer_sentinel
splat_sources:
- title: SRD 5.2.1
  locator: pp. 261
  url: https://www.dndbeyond.com/srd
---

# Azer Sentinel

## Source text

<!-- SRD page 261 -->

Medium Elemental, Lawful Neutral\
AC 17 Initiative +1 (11)\
HP 39 (6d8 + 12)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 12 +1 +1 Con 15 +2 +4\
Int 12 +1 +1 Wis 13 +1 +1 Cha 10 +0 +0\
Immunities Fire, Poison; Poisoned\
Senses Passive Perception 11\
Languages Primordial (Ignan)\
CR 2 (XP 450; PB +2)

### Traits

Fire Aura. At the end of each of the azer’s turns, each creature of the azer’s choice in a 5-foot Emanation originating from the azer takes 5 (1d10) Fire damage unless the azer has the Incapacitated condition.

Illumination. The azer sheds Bright Light in a 10-foot radius and Dim Light for an additional 10 feet.

### Actions

Burning Hammer. Melee Attack Roll: +5, reach 5 ft. Hit: 8 (1d10 + 3) Bludgeoning damage plus 3 (1d6) Fire damage.

Original layout: [[../../srd-5.2.1/pages/261|page 261]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` elemental ` |
| ` alignment ` | ` Lawful Neutral ` |
| ` movement_text ` | ` 30 ft. ` |
| ` immunities_text ` | ` Fire, Poison; Poisoned ` |
| ` senses_text ` | ` Passive Perception 11 ` |
| ` languages_text ` | ` Primordial (Ignan) ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 39 (6d8 + 12) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 17 ` | ` 3 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 15 ` | ` 4 ` |
| ` Intelligence ` | ` 12 ` | ` 1 ` |
| ` Wisdom ` | ` 13 ` | ` 1 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 39 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/azer_sentinel ` |
| ` implementation ` | ` dnd2024:creature_azer_sentinel ` |

### Parameters

#### Features

##### Burning hammer

<!-- splat:yaml path=splat_rule/parameters/features/burning_hammer -->

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
      count: 1
      sides: 10
    add: 3
  damage_type: bludgeoning
- op: damage
  average: 3
  amount:
    dice:
      count: 1
      sides: 6
    add: 0
  damage_type: fire
reach_feet: 5
```

##### Fire aura

<!-- splat:yaml path=splat_rule/parameters/features/fire_aura -->

```yaml
economy: passive
trigger: self.turn.end
requires:
  self_incapacitated: false
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    size_feet: 5
    origin: self
  selection: self_choice
effects:
- op: damage
  average: 5
  amount:
    dice:
      count: 1
      sides: 10
    add: 0
  damage_type: fire
```

##### Illumination

<!-- splat:values path=splat_rule/parameters/features/illumination -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` bright_radius_feet ` | ` 10 ` |
| ` dim_additional_feet ` | ` 10 ` |

#### Defenses

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

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["primordial"] ` |
| ` understands ` | ` ["primordial"] ` |
| ` dialects ` | ` ["ignan"] ` |

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
