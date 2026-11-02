---
splat_schema: 1
splat_id: a022e1fe-c390-5c20-90fe-93dfddfb57be
splat_kind: creature
splat_key: dnd2024:creatures/gorgon
splat_name: Gorgon
splat_origin:
  key: dnd2024:reference/creature/gorgon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 292, 293
  url: https://www.dndbeyond.com/srd
---

# Gorgon

## Source text

<!-- SRD page 292 -->

Large Construct, Unaligned\
AC 19 Initiative +0 (10)\
HP 114 (12d10 + 48)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +5 Dex 11 +0 +0 Con 18 +4 +4\
Int 2 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Perception +7\
Immunities Exhaustion, Petrified\
Senses Darkvision 60 ft.; Passive Perception 17\
Languages None\
CR 5 (XP 1,800; PB +3)

<!-- SRD page 293 -->

### Actions

Gore. Melee Attack Roll: +8, reach 5 ft. Hit: 18 (2d12 + 5) Piercing damage. If the target is a Large or smaller creature and the gorgon moved 20+ feet straight toward it immediately before the hit, the target has the Prone condition.

Petrifying Breath (Recharge 5–6). Constitution Saving Throw: DC 15, each creature in a 30-foot Cone. First Failure: The target has the Restrained condition and repeats the save at the end of its next turn if it is still Restrained, ending the effect on itself on a success. Second Failure: The target has the Petrified condition instead of the Restrained condition.

### Bonus Actions

Trample. Dexterity Saving Throw: DC 16, one creature within 5 feet that has the Prone condition. Failure: 16 (2d10 + 5) Bludgeoning damage. Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/292|page 292]], [[../../srd-5.2.1/pages/293|page 293]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` construct ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 40 ft. ` |
| ` skills_text ` | ` Perception +7 ` |
| ` immunities_text ` | ` Exhaustion, Petrified ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 17 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 114 (12d10 + 48) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 20 ` | ` 5 ` |
| ` Dexterity ` | ` 11 ` | ` 0 ` |
| ` Constitution ` | ` 18 ` | ` 4 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 7 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 19 ` |  |
| ` max_hp ` | ` 114 ` |  |
| ` initiative ` | ` 0 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/gorgon ` |
| ` implementation ` | ` dnd2024:creature_gorgon ` |

### Parameters

#### Features

##### Gore

<!-- splat:yaml path=splat_rule/parameters/features/gore -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 8
targets:
  count: 1
hit:
- op: damage
  average: 18
  amount:
    dice:
      count: 2
      sides: 12
    add: 5
  damage_type: piercing
- op: if
  test:
    target_kind: creature
    target_size_max: large
    immediately_preceding_movement:
      minimum_feet: 20
      direction: straight_toward_target
  then:
  - op: condition
    condition: prone
reach_feet: 5
```

##### Trample

<!-- splat:yaml path=splat_rule/parameters/features/trample -->

```yaml
economy: bonus_action
op: save
ability: dexterity
dc: 16
targets:
  kind: creature
  count: 1
  range_feet: 5
  condition: prone
failure:
- op: damage
  average: 16
  amount:
    dice:
      count: 2
      sides: 10
    add: 5
  damage_type: bludgeoning
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
```

##### Petrifying breath

<!-- splat:yaml path=splat_rule/parameters/features/petrifying_breath -->

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
dc: 15
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 30
    origin: self
failure:
- op: condition
  condition: restrained
  ends:
    event: repeat_save.success
  repeat_save:
    event: target.turn.end
    occurrence: next
    requires_condition: restrained
    ability: constitution
    dc: 15
    success: end_effect
    failure:
    - op: remove_condition
      condition: restrained
    - op: condition
      condition: petrified
success: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 7 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` conditions ` | ` ["exhaustion", "petrified"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 17 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

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
