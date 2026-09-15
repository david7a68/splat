---
splat_schema: 1
splat_id: 1e20ad87-337a-5f20-bd89-a170439e60fc
splat_kind: creature
splat_key: dnd2024:creatures/giant_insect
splat_name: Giant Insect
splat_origin:
  key: dnd2024:reference/creature/giant_insect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 136
  url: https://www.dndbeyond.com/srd
---

# Giant Insect

## Source text

<!-- SRD page 136 -->

Large Beast, Unaligned\
AC 11 + the spell’s level\
HP 30 + 10 for each spell level above 4\
Speed 40 ft., Climb 40 ft., Fly 40 ft. (Wasp only)\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 13 +1 +1 Con15 +2 +2\
Int 4 −3 −3 Wis 14 +2 +2 Cha 3 −4 −4\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Understands the languages you know\
CR None (XP 0; PB equals your Proficiency Bonus)\

### Traits

Spider Climb. The insect can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\

### Actions

Multiattack. The insect makes a number of attacks\
equal to half this spell’s level (round down).\
Poison Jab. Melee Attack Roll: Bonus equals your spell\
attack modifier, reach 10 ft. Hit: 1d6 + 3 plus the spell’s\
level Piercing damage plus 1d4 Poison damage.\
Web Bolt (Spider Only). Ranged Attack Roll: Bonus\
equals your spell attack modifier, range 60 ft. Hit: 1d10\
+ 3 plus the spell’s level Bludgeoning damage, and the\
target’s Speed is reduced to 0 until the start of the in-\
sect’s next turn.\

### Bonus Actions

Venomous Spew (Centipede Only). Constitution Saving\
Throw: Your spell save DC, one creature the insect can\
see within 10 feet. Failure: The target has the Poisoned\
condition until the start of the insect’s next turn.\

Original layout: [[../../srd-5.2.1/pages/136|page 136]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 40 ft., Climb 40 ft., Fly 40 ft. (Wasp only) ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 12 ` |
| ` languages_text ` | ` Understands the languages you know ` |
| ` cr_text ` | ` None (XP 0; PB equals your Proficiency Bonus) ` |
| ` hit_points_text ` | ` 30 + 10 for each spell level above 4 ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 17 ` | ` 3 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 15 ` | ` 2 ` |
| ` Intelligence ` | ` 4 ` | ` -3 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 3 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` climb_speed ` | ` 40 ` | ` ft ` |
| ` fly_speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_insect ` |
| ` implementation ` | ` dnd2024:creature_giant_insect ` |

### Parameters

#### Features

##### Spider climb

<!-- splat:values path=splat_rule/parameters/features/spider_climb -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` climb_ability_check_required ` | ` false ` |
| ` surfaces ` | ` ["difficult", "ceiling"] ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: repeat_choice
count:
  floor_divide:
  - input: spell_level
  - 2
choices:
- feature: poison_jab
- feature: web_bolt
  requires_form:
  - spider
```

##### Poison jab

<!-- splat:yaml path=splat_rule/parameters/features/poison_jab -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus:
  input: summoner_spell_attack_modifier
reach_feet: 10
targets:
  count: 1
hit:
- op: damage
  damage_type: piercing
  amount:
    add:
    - dice:
        count: 1
        sides: 6
      add: 3
    - input: spell_level
- op: damage
  damage_type: poison
  amount:
    add:
    - dice:
        count: 1
        sides: 4
      add: 0
```

##### Web bolt

<!-- splat:yaml path=splat_rule/parameters/features/web_bolt -->

```yaml
economy: action
requires_form:
- spider
op: attack
attack_modes:
- ranged
attack_bonus:
  input: summoner_spell_attack_modifier
range_feet: 60
targets:
  count: 1
hit:
- op: damage
  damage_type: bludgeoning
  amount:
    add:
    - dice:
        count: 1
        sides: 10
      add: 3
    - input: spell_level
- op: set_speed
  feet: 0
  ends:
    event: turn.start
    participant: self
    occurrence: next
```

##### Venomous spew

<!-- splat:yaml path=splat_rule/parameters/features/venomous_spew -->

```yaml
economy: bonus_action
requires_form:
- centipede
op: save
ability: constitution
dc:
  input: summoner_spell_save_dc
targets:
  kind: creature
  count: 1
  range_feet: 10
  visible_to: self
failure:
- op: condition
  condition: poisoned
  ends:
    event: turn.start
    participant: self
    occurrence: next
success: []
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
| ` passive_perception ` | ` 12 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

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

#### Scaling stats

<!-- splat:yaml path=splat_rule/parameters/scaling_stats -->

```yaml
ac:
  add:
  - 11
  - input: spell_level
hit_points:
  add:
  - 30
  - multiply:
    - 10
    - subtract:
      - input: spell_level
      - 4
```

#### Form choice

<!-- splat:yaml path=splat_rule/parameters/form_choice -->

```yaml
chooser: summoner
options:
- wasp
- spider
- centipede
fly_speed_feet:
  wasp: 40
  otherwise: 0
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
