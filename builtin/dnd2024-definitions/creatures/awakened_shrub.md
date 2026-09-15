---
splat_schema: 1
splat_id: 5afec75c-7981-5ce1-b7f7-3469de75b409
splat_kind: creature
splat_key: dnd2024:creatures/awakened_shrub
splat_name: Awakened Shrub
splat_origin:
  key: dnd2024:reference/creature/awakened_shrub
splat_sources:
- title: SRD 5.2.1
  locator: pp. 260
  url: https://www.dndbeyond.com/srd
---

# Awakened Shrub

## Source text

<!-- SRD page 260 -->

Small Plant, Neutral\
AC 9 Initiative −1 (9)\
HP 10 (3d6)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 3 −4 −4 Dex 8 −1 −1 Con 11 +0 +0\
Int 10 +0 +0 Wis 10 +0 +0 Cha 6 −2 −2\
Vulnerabilities Fire\
Resistances Piercing\
Senses Passive Perception 10\
Languages Common plus one other language\
CR 0 (XP 10; PB +2)\

### Actions

Rake. Melee Attack Roll: +1, reach 5 ft. Hit: 1 Slash-\
ing damage.\

Original layout: [[../../srd-5.2.1/pages/260|page 260]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` small ` |
| ` type ` | ` plant ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 20 ft. ` |
| ` resistances_text ` | ` Piercing ` |
| ` vulnerabilities_text ` | ` Fire ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` Common plus one other language ` |
| ` cr_text ` | ` 0 (XP 10; PB +2) ` |
| ` hit_points_text ` | ` 10 (3d6) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 3 ` | ` -4 ` |
| ` Dexterity ` | ` 8 ` | ` -1 ` |
| ` Constitution ` | ` 11 ` | ` 0 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 9 ` |  |
| ` max_hp ` | ` 10 ` |  |
| ` initiative ` | ` -1 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/awakened_shrub ` |
| ` implementation ` | ` dnd2024:creature_awakened_shrub ` |

### Parameters

#### Features

##### Rake

<!-- splat:yaml path=splat_rule/parameters/features/rake -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 1
targets:
  count: 1
hit:
- op: damage
  average: 1
  amount:
    constant: 1
  damage_type: slashing
reach_feet: 5
```

#### Defenses

##### Vulnerabilities

<!-- splat:values path=splat_rule/parameters/defenses/vulnerabilities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["fire"] ` |

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["piercing"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common"] ` |
| ` understands ` | ` ["common"] ` |

##### Additional languages

<!-- splat:values path=splat_rule/parameters/communication/additional_languages -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` chooser ` | ` gm ` |
| ` distinct_from_known ` | ` true ` |
| ` can_speak ` | ` true ` |
| ` can_understand ` | ` true ` |

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
