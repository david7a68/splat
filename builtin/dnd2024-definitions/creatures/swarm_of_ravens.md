---
splat_schema: 1
splat_id: 8533ab51-d920-544c-ac08-8d714f8de7fe
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_ravens
splat_name: Swarm of Ravens
splat_origin:
  key: dnd2024:reference/creature/swarm_of_ravens
splat_sources:
- title: SRD 5.2.1
  locator: pp. 362
  url: https://www.dndbeyond.com/srd
---

# Swarm of Ravens

## Source text

<!-- SRD page 362 -->

Medium Swarm of Tiny Beasts, Unaligned\
AC 12 Initiative +2 (12)\
HP 11 (2d8 + 2)\
Speed 10 ft., Fly 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 14 +2 +2 Con 12 +1 +1\
Int  5 −3 −3 Wis 12 +1 +1 Cha 6 −2 −2\
Skills Perception +5\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Charmed, Frightened, Grappled, Paralyzed,\
Petrified, Prone, Restrained, Stunned\
Senses Passive Perception 15\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Traits

Swarm. The swarm can occupy another creature’s\
space and vice versa, and the swarm can move through\
any opening large enough for a Tiny raven. The swarm\
can’t regain Hit Points or gain Temporary Hit Points.\

### Actions

Beaks. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6\
+ 2) Piercing damage, or 2 (1d4) Piercing damage if the\
swarm is Bloodied.\
Cacophony (Recharge 6). Wisdom Saving Throw: DC\
10, one creature in the swarm’s space. Failure: The\
target has the Deafened condition until the start of the\
swarm’s next turn. While Deafened, the target also has\
Disadvantage on ability checks and attack rolls.\

Original layout: [[../../srd-5.2.1/pages/362|page 362]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` size_options_text ` | ` Medium Swarm of Tiny ` |
| ` tags_text ` | ` s ` |
| ` movement_text ` | ` 10 ft., Fly 50 ft. ` |
| ` skills_text ` | ` Perception +5 ` |
| ` resistances_text ` | ` Bludgeoning, Piercing, Slashing ` |
| ` immunities_text ` | ` Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned ` |
| ` senses_text ` | ` Passive Perception 15 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 11 (2d8 + 2) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 6 ` | ` -2 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 5 ` | ` -3 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 11 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 10 ` | ` ft ` |
| ` fly_speed ` | ` 50 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/swarm_of_ravens ` |
| ` implementation ` | ` dnd2024:creature_swarm_of_ravens ` |

### Parameters

#### Features

##### Swarm

<!-- splat:values path=splat_rule/parameters/features/swarm -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` space_sharing ` | ` bidirectional ` |
| ` minimum_passage_size ` | ` tiny ` |
| ` regain_hit_points ` | ` false ` |
| ` gain_temporary_hit_points ` | ` false ` |

##### Beaks

<!-- splat:yaml path=splat_rule/parameters/features/beaks -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 4
targets:
  count: 1
hit:
- op: choose_by_predicate
  test:
    self_bloodied: true
  then:
  - op: damage
    average: 2
    amount:
      dice:
        count: 1
        sides: 4
      add: 0
    damage_type: piercing
  else:
  - op: damage
    average: 5
    amount:
      dice:
        count: 1
        sides: 6
      add: 2
    damage_type: piercing
reach_feet: 5
```

##### Cacophony

<!-- splat:yaml path=splat_rule/parameters/features/cacophony -->

```yaml
economy: action
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
    - 6
  cost: 1
  recovery_also:
  - short_rest
  - long_rest
op: save
ability: wisdom
dc: 10
targets:
  kind: creature
  count: 1
  location: self_space
failure:
- op: condition
  condition: deafened
  ends:
    event: turn.start
    participant: self
    occurrence: next
  while_active:
    disadvantage:
    - ability_check
    - attack
success: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["bludgeoning", "piercing", "slashing"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` conditions ` | ` ["charmed", "frightened", "grappled", "paralyzed", "petrified", "prone", "restrained", "stunned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

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
