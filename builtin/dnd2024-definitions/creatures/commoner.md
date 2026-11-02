---
splat_schema: 1
splat_id: 64bcc909-76f0-5047-bcbb-66ce3c75d0f8
splat_kind: creature
splat_key: dnd2024:creatures/commoner
splat_name: Commoner
splat_origin:
  key: dnd2024:reference/creature/commoner
splat_sources:
- title: SRD 5.2.1
  locator: pp. 275
  url: https://www.dndbeyond.com/srd
---

# Commoner

## Source text

<!-- SRD page 275 -->

Medium or Small Humanoid, Neutral\
AC 10 Initiative +0 (10)\
HP 4 (1d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 10 +0 +0 Con 10 +0 +0\
Int 10 +0 +0 Wis 10 +0 +0 Cha 10 +0 +0\
Gear Club\
Senses Passive Perception 10\
Languages Common\
CR 0 (XP 10; PB +2)

### Traits

Training. The commoner has proficiency in one skill of the GM’s choice and has Advantage whenever it makes an ability check using that skill.

### Actions

Club. Melee Attack Roll: +2, reach 5 ft. Hit: 2 (1d4) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/275|page 275]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` gear_text ` | ` Club ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` Common ` |
| ` cr_text ` | ` 0 (XP 10; PB +2) ` |
| ` hit_points_text ` | ` 4 (1d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 0 ` |
| ` Dexterity ` | ` 10 ` | ` 0 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 10 ` |  |
| ` max_hp ` | ` 4 ` |  |
| ` initiative ` | ` 0 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/commoner ` |
| ` implementation ` | ` dnd2024:creature_commoner ` |

### Parameters

#### Features

##### Club

<!-- splat:yaml path=splat_rule/parameters/features/club -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 2
targets:
  count: 1
hit:
- op: damage
  average: 2
  amount:
    dice:
      count: 1
      sides: 4
    add: 0
  damage_type: bludgeoning
reach_feet: 5
```

##### Training

<!-- splat:yaml path=splat_rule/parameters/features/training -->

```yaml
economy: passive
skill_choice:
  chooser: gm
  count: 1
  from: skills
grant_skill_proficiency: true
advantage:
  roll: ability_check
  skill: chosen_skill
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

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item_name | quantity |
| --- | --- | --- |
| ` 0 ` | ` Club ` | ` 1 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common"] ` |
| ` understands ` | ` ["common"] ` |

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
