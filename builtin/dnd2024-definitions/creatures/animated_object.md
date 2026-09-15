---
splat_schema: 1
splat_id: b4fa59d2-2ff7-5217-95b9-00d9abaa06a6
splat_kind: creature
splat_key: dnd2024:creatures/animated_object
splat_name: Animated Object
splat_origin:
  key: dnd2024:reference/creature/animated_object
splat_sources:
- title: SRD 5.2.1
  locator: pp. 109
  url: https://www.dndbeyond.com/srd
---

# Animated Object

## Source text

<!-- SRD page 109 -->

Huge or Smaller Construct, Unaligned\
AC 15\
HP 10 (Medium or smaller), 20 (Large), 40 (Huge)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 10 +0 +0 Con10 +0 +0\
Int 3 −4 −4 Wis 3 −4 −4 Cha 1 −5 −5\
Immunities Poison, Psychic; Charmed, Exhaustion,\
Frightened, Paralyzed, Poisoned\
Senses Blindsight 30 ft.; Passive Perception 6\
Languages Understands the languages you know\
CR None (XP 0; PB equals your Proficiency Bonus)\

### Actions

Slam. Melee Attack Roll: Bonus equals your spell attack\
modifier, reach 5 ft. Hit: Force damage equal to 1d4\
+ 3 (Medium or smaller), 2d6 + 3 + your spellcasting\
ability modifier (Large), or 2d12 + 3 + your spellcasting\
ability modifier (Huge).\

Original layout: [[../../srd-5.2.1/pages/109|page 109]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` construct ` |
| ` alignment ` | ` Unaligned ` |
| ` size_options_text ` | ` Huge or Smaller ` |
| ` movement_text ` | ` 30 ft. ` |
| ` immunities_text ` | ` Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Poisoned ` |
| ` senses_text ` | ` Blindsight 30 ft.; Passive Perception 6 ` |
| ` languages_text ` | ` Understands the languages you know ` |
| ` cr_text ` | ` None (XP 0; PB equals your Proficiency Bonus) ` |
| ` hit_points_text ` | ` 10 (Medium or smaller), 20 (Large), 40 (Huge) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 16 ` | ` 3 ` |
| ` Dexterity ` | ` 10 ` | ` 0 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 3 ` | ` -4 ` |
| ` Wisdom ` | ` 3 ` | ` -4 ` |
| ` Charisma ` | ` 1 ` | ` -5 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 15 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/animated_object ` |
| ` implementation ` | ` dnd2024:creature_animated_object ` |

### Parameters

#### Features

##### Slam

<!-- splat:yaml path=splat_rule/parameters/features/slam -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus:
  input: summoner_spell_attack_modifier
reach_feet: 5
targets:
  count: 1
hit:
- op: switch
  value:
    stat: size
  cases:
  - values:
    - tiny
    - small
    - medium
    effects:
    - op: damage
      damage_type: force
      amount:
        add:
        - dice:
            count: 1
            sides: 4
          add: 3
  - values:
    - large
    effects:
    - op: damage
      damage_type: force
      amount:
        add:
        - dice:
            count: 2
            sides: 6
          add: 3
        - input: summoner_spellcasting_ability_modifier
  - values:
    - huge
    effects:
    - op: damage
      damage_type: force
      amount:
        add:
        - dice:
            count: 2
            sides: 12
          add: 3
        - input: summoner_spellcasting_ability_modifier
```

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["poison", "psychic"] ` |
| ` conditions ` | ` ["charmed", "exhaustion", "frightened", "paralyzed", "poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 6 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 30 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` can_speak ` | ` false ` |

##### Speaks

<!-- splat:yaml path=splat_rule/parameters/communication/speaks -->

```yaml
[]
```

##### Understands

<!-- splat:values path=splat_rule/parameters/communication/understands -->

| Field | Value |
| --- | --- |
| ` languages_known_by ` | ` summoner ` |

#### Proficiency bonus

<!-- splat:values path=splat_rule/parameters/proficiency_bonus -->

| Field | Value |
| --- | --- |
| ` input ` | ` summoner_proficiency_bonus ` |

#### Variant stats

<!-- splat:yaml path=splat_rule/parameters/variant_stats -->

```yaml
size_choice:
- tiny
- small
- medium
- large
- huge
hit_points_by_size:
  tiny: 10
  small: 10
  medium: 10
  large: 20
  huge: 40
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
