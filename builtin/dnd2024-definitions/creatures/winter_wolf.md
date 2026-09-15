---
splat_schema: 1
splat_id: b02d5610-4e8b-510d-9951-89f98e719302
splat_kind: creature
splat_key: dnd2024:creatures/winter_wolf
splat_name: Winter Wolf
splat_origin:
  key: dnd2024:reference/creature/winter_wolf
splat_sources:
- title: SRD 5.2.1
  locator: pp. 342
  url: https://www.dndbeyond.com/srd
---

# Winter Wolf

## Source text

<!-- SRD page 342 -->

Large Monstrosity, Neutral Evil\
AC 13 Initiative +1 (11)\
HP 75 (10d10 + 20)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 13 +1 +1 Con 14 +2 +2\
Int 7 −2 −2 Wis 12 +1 +1 Cha 8 −1 −1\
Skills Perception +5, Stealth +5\
Immunities Cold\
Senses Passive Perception 15\
Languages Common, Giant\
CR 3 (XP 700; PB +2)\

### Traits

Pack Tactics. The wolf has Advantage on an attack roll\
against a creature if at least one of the wolf’s allies is\
within 5 feet of the creature and the ally doesn’t have\
the Incapacitated condition.\

### Actions

Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 11 (2d6 + 4)\
Piercing damage. If the target is a Large or smaller crea-\
ture, it has the Prone condition.\
Cold Breath (Recharge 5–6). Constitution Saving\
Throw: DC 12, each creature in a 15-foot Cone. Failure:\
18 (4d8) Cold damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/342|page 342]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` monstrosity ` |
| ` alignment ` | ` Neutral Evil ` |
| ` movement_text ` | ` 50 ft. ` |
| ` skills_text ` | ` Perception +5, Stealth +5 ` |
| ` immunities_text ` | ` Cold ` |
| ` senses_text ` | ` Passive Perception 15 ` |
| ` languages_text ` | ` Common, Giant ` |
| ` cr_text ` | ` 3 (XP 700; PB +2) ` |
| ` hit_points_text ` | ` 75 (10d10 + 20) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 7 ` | ` -2 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 8 ` | ` -1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 75 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 50 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/winter_wolf ` |
| ` implementation ` | ` dnd2024:creature_winter_wolf ` |

### Parameters

#### Features

##### Pack tactics

<!-- splat:yaml path=splat_rule/parameters/features/pack_tactics -->

```yaml
economy: passive
advantage:
  roll: attack
  target_kind: creature
  requires:
    ally_distance_to_target_max_feet: 5
    ally_incapacitated: false
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

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
  average: 11
  amount:
    dice:
      count: 2
      sides: 6
    add: 4
  damage_type: piercing
- op: if
  test:
    target_kind: creature
    target_size_max: large
  then:
  - op: condition
    condition: prone
reach_feet: 5
```

##### Cold breath

<!-- splat:yaml path=splat_rule/parameters/features/cold_breath -->

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
    - 5
    - 6
  cost: 1
  recovery_also:
  - short_rest
  - long_rest
op: save
ability: constitution
dc: 12
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 15
    origin: self
failure:
- op: damage
  average: 18
  amount:
    dice:
      count: 4
      sides: 8
    add: 0
  damage_type: cold
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |
| ` stealth ` | ` 5 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["cold"] ` |

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

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common", "giant"] ` |
| ` understands ` | ` ["common", "giant"] ` |

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
