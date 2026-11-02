---
splat_schema: 1
splat_id: 56739d5d-c675-5561-8eb4-4d331e83231c
splat_kind: creature
splat_key: dnd2024:creatures/behir
splat_name: Behir
splat_origin:
  key: dnd2024:reference/creature/behir
splat_sources:
- title: SRD 5.2.1
  locator: pp. 263
  url: https://www.dndbeyond.com/srd
---

# Behir

## Source text

<!-- SRD page 263 -->

Huge Monstrosity, Neutral Evil\
AC 17 Initiative +3 (13)\
HP 168 (16d12 + 64)\
Speed 50 ft., Climb 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 16 +3 +3 Con 18 +4 +4\
Int 7 −2 −2 Wis 14 +2 +2 Cha 12 +1 +1\
Skills Perception +6, Stealth +7\
Immunities Lightning\
Senses Darkvision 90 ft.; Passive Perception 16\
Languages Draconic\
CR 11 (XP 7,200; PB +4)

### Actions

Multiattack. The behir makes one Bite attack and uses Constrict.

Bite. Melee Attack Roll: +10, reach 10 ft. Hit: 19 (2d12 + 6) Piercing damage plus 11 (2d10) Lightning damage.

Constrict. Strength Saving Throw: DC 18, one Large or smaller creature the behir can see within 5 feet. Failure: 28 (5d8 + 6) Bludgeoning damage. The target has the Grappled condition (escape DC 16), and it has the Restrained condition until the grapple ends.

Lightning Breath (Recharge 5–6). Dexterity Saving Throw: DC 16, each creature in a 90-foot-long, 5-foot-wide Line. Failure: 66 (12d10) Lightning damage. Success: Half damage.

### Bonus Actions

Swallow. Dexterity Saving Throw: DC 18, one Large or smaller creature Grappled by the behir (the behir can have only one creature swallowed at a time). Failure: The behir swallows the target, which is no longer Grappled. While swallowed, a creature has the Blinded and Restrained conditions, has Total Cover against attacks and other effects outside the behir, and takes 21 (6d6) Acid damage at the start of each of the behir’s turns.

If the behir takes 30 damage or more on a single turn from the swallowed creature, the behir must succeed on a DC 14 Constitution saving throw at the end of that turn or regurgitate the creature, which falls in a space within 10 feet of the behir and has the Prone condition. If the behir dies, a swallowed creature is no longer Restrained and can escape from the corpse by using 15 feet of movement, exiting Prone.

Original layout: [[../../srd-5.2.1/pages/263|page 263]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` monstrosity ` |
| ` alignment ` | ` Neutral Evil ` |
| ` movement_text ` | ` 50 ft., Climb 50 ft. ` |
| ` skills_text ` | ` Perception +6, Stealth +7 ` |
| ` immunities_text ` | ` Lightning ` |
| ` senses_text ` | ` Darkvision 90 ft.; Passive Perception 16 ` |
| ` languages_text ` | ` Draconic ` |
| ` cr_text ` | ` 11 (XP 7,200; PB +4) ` |
| ` hit_points_text ` | ` 168 (16d12 + 64) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 23 ` | ` 6 ` |
| ` Dexterity ` | ` 16 ` | ` 3 ` |
| ` Constitution ` | ` 18 ` | ` 4 ` |
| ` Intelligence ` | ` 7 ` | ` -2 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 12 ` | ` 1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 168 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 50 ` | ` ft ` |
| ` climb_speed ` | ` 50 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/behir ` |
| ` implementation ` | ` dnd2024:creature_behir ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["lightning"] ` |
| ` passive_perception ` | ` 16 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 6 ` |
| ` stealth ` | ` 7 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 90 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["draconic"] ` |

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 10
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 12
      add: 6
    average: 19
  - type: lightning
    amount:
      dice:
        count: 2
        sides: 10
      add: 0
    average: 11
```

##### Constrict

<!-- splat:yaml path=splat_rule/parameters/features/constrict -->

```yaml
economy: action
op: saving_throw
ability: strength
dc: 18
targets:
  kind: creature
  count: 1
  range_ft: 5
  maximum_size: large
  visible_to_actor: true
failure:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 5
        sides: 8
      add: 6
    average: 28
- op: grapple
  escape_dc: 16
  while_grappled:
  - op: condition.apply
    condition: restrained
success: []
```

##### Lightning breath

<!-- splat:yaml path=splat_rule/parameters/features/lightning_breath -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 16
targets:
  kind: creature
  count: all
  area:
    shape: line
    origin: actor
    size_ft: 90
    width_ft: 5
failure:
- op: damage
  components:
  - type: lightning
    amount:
      dice:
        count: 12
        sides: 10
      add: 0
    average: 66
success:
- op: damage
  from: failure_damage
  multiplier: 0.5
  round: down
recharge:
  die_sides: 6
  success_min: 5
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
```

##### Swallow

<!-- splat:yaml path=splat_rule/parameters/features/swallow -->

```yaml
economy: bonus_action
op: saving_throw
ability: dexterity
dc: 18
targets:
  kind: creature
  count: 1
  maximum_size: large
  grappled_by: actor
failure:
- op: swallow
  end_grapple: true
  conditions:
  - blinded
  - restrained
  total_cover_against: outside_actor
  periodic_damage:
    event: actor.turn_start
    damage:
    - type: acid
      amount:
        dice:
          count: 6
          sides: 6
        add: 0
      average: 21
success: []
swallowed_capacity: 1
regurgitation:
  trigger: turn.end
  ability: constitution
  dc: 14
  failure:
    targets: swallowed_creature
    exit_within_ft: 10
    condition: prone
  when:
    damage_to_actor_from_one_swallowed_creature_this_turn_at_least: 30
actor_death:
  remove_condition: restrained
  escape_movement_cost_ft: 15
  exit_condition: prone
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` multiattack ` |

###### Sequence

<!-- splat:table
path: splat_rule/parameters/features/multiattack/sequence
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Feature:
    path: feature
    type: symbol
  Count:
    path: count
    type: integer
-->

| Key | Feature | Count |
| --- | --- | --- |
| ` row_1 ` | ` Bite ` | ` 1 ` |
| ` row_2 ` | ` Constrict ` | ` 1 ` |

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |
