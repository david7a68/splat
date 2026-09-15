---
splat_schema: 1
splat_id: 98271f17-1270-51f4-8b3c-fbcafb58085c
splat_kind: creature
splat_key: dnd2024:creatures/wraith
splat_name: Wraith
splat_origin:
  key: dnd2024:reference/creature/wraith
splat_sources:
- title: SRD 5.2.1
  locator: pp. 342, 343
  url: https://www.dndbeyond.com/srd
---

# Wraith

## Source text

<!-- SRD page 342 -->

Medium or Small Undead, Neutral Evil\
AC 13 Initiative +3 (13)\
HP 67 (9d8 + 27)\
Speed 5 ft., Fly 60 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 16 +3 +3 Con 16 +3 +3\
Int 12 +1 +1 Wis 14 +2 +2 Cha 15 +2 +2\
Resistances Acid, Bludgeoning, Cold, Fire, Piercing,\
Slashing\
Immunities Necrotic, Poison; Charmed, Exhaustion,\
Grappled, Paralyzed, Petrified, Poisoned, Prone,\
Restrained, Unconscious\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Common plus two other languages\
CR 5 (XP 1,800; PB +3)\

<!-- SRD page 343 -->


### Traits

Incorporeal Movement. The wraith can move through\
other creatures and objects as if they were Difficult\
Terrain. It takes 5 (1d10) Force damage if it ends its turn\
inside an object.\
Sunlight Sensitivity. While in sunlight, the wraith has\
Disadvantage on ability checks and attack rolls.\

### Actions

Life Drain. Melee Attack Roll: +6, reach 5 ft. Hit: 21\
(4d8 + 3) Necrotic damage. If the target is a creature, its\
Hit Point maximum decreases by an amount equal to\
the damage taken.\
Create Specter. The wraith targets a Humanoid corpse\
within 10 feet of itself that has been dead for no longer\
than 1 minute. The target’s spirit rises as a Specter in\
the space of its corpse or in the nearest unoccupied\
space. The specter is under the wraith’s control. The\
wraith can have no more than seven specters under its\
control at a time.\

Original layout: [[../../srd-5.2.1/pages/342|page 342]], [[../../srd-5.2.1/pages/343|page 343]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` undead ` |
| ` alignment ` | ` Neutral Evil ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 5 ft., Fly 60 ft. (hover) ` |
| ` resistances_text ` | ` Acid, Bludgeoning, Cold, Fire, Piercing, Slashing ` |
| ` immunities_text ` | ` Necrotic, Poison; Charmed, Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 12 ` |
| ` languages_text ` | ` Common plus two other languages ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 67 (9d8 + 27) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 6 ` | ` -2 ` |
| ` Dexterity ` | ` 16 ` | ` 3 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 12 ` | ` 1 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 15 ` | ` 2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 67 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 5 ` | ` ft ` |
| ` fly_speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/wraith ` |
| ` implementation ` | ` dnd2024:creature_wraith ` |

### Parameters

#### Features

##### Sunlight sensitivity

<!-- splat:yaml path=splat_rule/parameters/features/sunlight_sensitivity -->

```yaml
economy: passive
disadvantage:
  rolls:
  - attack
  - ability_check
  when: self_in_sunlight
```

##### Life drain

<!-- splat:yaml path=splat_rule/parameters/features/life_drain -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 6
targets:
  count: 1
hit:
- op: damage
  average: 21
  amount:
    dice:
      count: 4
      sides: 8
    add: 3
  damage_type: necrotic
- op: if
  test:
    target_kind: creature
  then:
  - op: reduce_hit_point_maximum
    amount:
      event_value: damage_taken
reach_feet: 5
```

##### Incorporeal movement

<!-- splat:yaml path=splat_rule/parameters/features/incorporeal_movement -->

```yaml
economy: passive
move_through:
- creatures
- objects
movement_cost: difficult_terrain
turn_end_inside_object:
- op: damage
  average: 5
  amount:
    dice:
      count: 1
      sides: 10
    add: 0
  damage_type: force
```

##### Create specter

<!-- splat:yaml path=splat_rule/parameters/features/create_specter -->

```yaml
economy: action
op: create_creature
targets:
  kind: corpse
  former_creature_type: humanoid
  range_feet: 10
  dead_for_max:
    minutes: 1
definition:
  key: dnd2024:creatures/specter
destination_choice:
- corpse_space
- nearest_unoccupied_space
controller: self
controlled_capacity: 7
```

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["acid", "bludgeoning", "cold", "fire", "piercing", "slashing"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["necrotic", "poison"] ` |
| ` conditions ` | ` ["charmed", "exhaustion", "grappled", "paralyzed", "petrified", "poisoned", "prone", "restrained", "unconscious"] ` |

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
| ` speaks ` | ` ["common"] ` |
| ` understands ` | ` ["common"] ` |

##### Additional languages

<!-- splat:values path=splat_rule/parameters/communication/additional_languages -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
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
