---
splat_schema: 1
splat_id: ed70a67c-f9f5-5278-ac81-426da49fed85
splat_kind: creature
splat_key: dnd2024:creatures/dretch
splat_name: Dretch
splat_origin:
  key: dnd2024:reference/creature/dretch
splat_sources:
- title: SRD 5.2.1
  locator: pp. 281
  url: https://www.dndbeyond.com/srd
---

# Dretch

## Source text

<!-- SRD page 281 -->

Small Fiend (Demon), Chaotic Evil\
AC 11 Initiative +0 (10)\
HP 18 (4d6 + 4)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 11 +0 +0 Con 12 +1 +1\
Int 5 −3 −3 Wis 8 −1 −1 Cha 3 −4 −4\
Resistances Cold, Fire, Lightning\
Immunities Poison; Poisoned\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages Abyssal; telepathy 60 ft. (works only with creatures that understand Abyssal)\
CR 1/4 (XP 50; PB +2)

### Actions

Rend. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1) Slashing damage.

Fetid Cloud (1/Day). Constitution Saving Throw: DC 11, each creature in a 10-foot Emanation originating from the dretch. Failure: The target has the Poisoned condition until the end of its next turn. While Poisoned, the creature can take either an action or a Bonus Action on its turn, not both, and it can’t take Reactions.

Original layout: [[../../srd-5.2.1/pages/281|page 281]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` small ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Demon) ` |
| ` movement_text ` | ` 20 ft. ` |
| ` resistances_text ` | ` Cold, Fire, Lightning ` |
| ` immunities_text ` | ` Poison; Poisoned ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 9 ` |
| ` languages_text ` | ` Abyssal; telepathy 60 ft. (works only with creatures that understand Abyssal) ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 18 (4d6 + 4) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 12 ` | ` 1 ` |
| ` Dexterity ` | ` 11 ` | ` 0 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 5 ` | ` -3 ` |
| ` Wisdom ` | ` 8 ` | ` -1 ` |
| ` Charisma ` | ` 3 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 18 ` |  |
| ` initiative ` | ` 0 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/dretch ` |
| ` implementation ` | ` dnd2024:creature_dretch ` |

### Parameters

#### Features

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 3
targets:
  count: 1
hit:
- op: damage
  average: 4
  amount:
    dice:
      count: 1
      sides: 6
    add: 1
  damage_type: slashing
reach_feet: 5
```

##### Fetid cloud

<!-- splat:yaml path=splat_rule/parameters/features/fetid_cloud -->

```yaml
economy: action
resource:
  capacity: 1
  recovery: long_rest
  cost: 1
op: save
ability: constitution
dc: 11
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    size_feet: 10
    origin: self
failure:
- op: condition
  condition: poisoned
  ends:
    event: turn.end
    participant: target
    occurrence: next
  while_active:
    turn_economy_exclusive_choice:
    - action
    - bonus_action
    reactions_allowed: false
success: []
```

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["cold", "fire", "lightning"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["poison"] ` |
| ` conditions ` | ` ["poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 9 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["abyssal"] ` |
| ` understands ` | ` ["abyssal"] ` |

##### Telepathy

<!-- splat:values path=splat_rule/parameters/communication/telepathy -->

| Field | Value |
| --- | --- |
| ` range_feet ` | ` 60 ` |
| ` target_understands ` | ` abyssal ` |

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
