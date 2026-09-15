---
splat_schema: 1
splat_id: 15fa8c5b-a51d-5241-88c1-ffdd197e9675
splat_kind: creature
splat_key: dnd2024:creatures/gibbering_mouther
splat_name: Gibbering Mouther
splat_origin:
  key: dnd2024:reference/creature/gibbering_mouther
splat_sources:
- title: SRD 5.2.1
  locator: pp. 288
  url: https://www.dndbeyond.com/srd
---

# Gibbering Mouther

## Source text

<!-- SRD page 288 -->

Medium Aberration, Chaotic Neutral\
AC 9 Initiative −1 (9)\
HP 52 (7d8 + 21)\
Speed 20 ft., Swim 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 8 −1 −1 Con 16 +3 +3\
Int 3 −4 −4 Wis 10 +0 +0 Cha 6 −2 −2\
Immunities Prone\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages None\
CR 2 (XP 450; PB +2)\

### Traits

Aberrant Ground. The ground in a 10-foot Emanation\
originating from the mouther is Difficult Terrain.\
Gibbering. The mouther babbles incoherently while\
it doesn’t have the Incapacitated condition. Wisdom\
Saving Throw: DC 10, any creature that starts its turn\
within 20 feet of the mouther while it is babbling.\
Failure: The target rolls 1d8 to determine what it does\
during the current turn:\
1–4. The target does nothing.\
5–6. The target takes no action or Bonus Action and\
uses all its movement to move in a random direction.\
7–8. The target makes a melee attack against a ran-\
domly determined creature within its reach or does\
nothing if it can’t make such an attack.\

### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 7 (2d6)\
Piercing damage. If the target is a Medium or smaller\
creature, it has the Prone condition. The target dies if\
it is reduced to 0 Hit Points by this attack. Its body is\
then absorbed into the mouther, leaving only equip-\
ment behind.\
Blinding Spittle (Recharge 5–6). Dexterity Saving\
Throw: DC 10, each creature in a 10-foot-radius Sphere\
centered on a point within 30 feet. Failure: 7 (2d6) Ra-\
diant damage, and the target has the Blinded condition\
until the end of the mouther’s next turn.\

Original layout: [[../../srd-5.2.1/pages/288|page 288]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` aberration ` |
| ` alignment ` | ` Chaotic Neutral ` |
| ` movement_text ` | ` 20 ft., Swim 20 ft. ` |
| ` immunities_text ` | ` Prone ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 52 (7d8 + 21) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 0 ` |
| ` Dexterity ` | ` 8 ` | ` -1 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 3 ` | ` -4 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 9 ` |  |
| ` max_hp ` | ` 52 ` |  |
| ` initiative ` | ` -1 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |
| ` swim_speed ` | ` 20 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/gibbering_mouther ` |
| ` implementation ` | ` dnd2024:creature_gibbering_mouther ` |

### Parameters

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

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
  average: 7
  amount:
    dice:
      count: 2
      sides: 6
    add: 0
  damage_type: piercing
- op: if
  test:
    target_kind: creature
    target_size_max: medium
  then:
  - op: condition
    condition: prone
- op: if
  test:
    attack_reduces_target_hp_to: 0
  then:
  - op: kill
  - op: absorb_body
    equipment_remains: true
reach_feet: 5
```

##### Blinding spittle

<!-- splat:yaml path=splat_rule/parameters/features/blinding_spittle -->

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
ability: dexterity
dc: 10
targets:
  kind: creature
  count: all
  area:
    shape: sphere
    size_feet: 10
    origin: chosen_point
    origin_range_feet: 30
failure:
- op: damage
  average: 7
  amount:
    dice:
      count: 2
      sides: 6
    add: 0
  damage_type: radiant
- op: condition
  condition: blinded
  ends:
    event: turn.end
    participant: self
    occurrence: next
success: []
```

##### Aberrant ground

<!-- splat:yaml path=splat_rule/parameters/features/aberrant_ground -->

```yaml
economy: passive
terrain:
  area:
    shape: emanation
    radius_feet: 10
    origin: self
  surface: ground
  difficult: true
```

##### Gibbering

<!-- splat:yaml path=splat_rule/parameters/features/gibbering -->

```yaml
economy: passive
trigger: creature.turn.start
requires:
  self_incapacitated: false
op: save
ability: wisdom
dc: 10
targets:
  kind: creature
  count: all
  range_feet: 20
failure:
- op: roll_behavior
  roll:
    dice:
      count: 1
      sides: 8
    add: 0
  duration: current_turn
  table:
  - roll_min: 1
    roll_max: 4
    behavior:
      does_nothing: true
  - roll_min: 5
    roll_max: 6
    behavior:
      action_allowed: false
      bonus_action_allowed: false
      movement: all
      direction: random
  - roll_min: 7
    roll_max: 8
    behavior:
      op: melee_attack
      target: random_creature_within_reach
      fallback: does_nothing
success: []
```

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` conditions ` | ` ["prone"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

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
