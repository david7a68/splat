---
splat_schema: 1
splat_id: 76207bbe-547f-5d70-94ce-cb0c8ab9bccc
splat_kind: creature
splat_key: dnd2024:creatures/young_copper_dragon
splat_name: Young Copper Dragon
splat_origin:
  key: dnd2024:reference/creature/young_copper_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 276
  url: https://www.dndbeyond.com/srd
---

# Young Copper Dragon

## Source text

<!-- SRD page 276 -->

Large Dragon (Metallic), Chaotic Good\
AC 17  Initiative +4 (14)\
HP 119 (14d10 + 42)\
Speed 40 ft., Climb 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 12 +1 +4 Con 17 +3 +3\
Int 16 +3 +3 Wis 13 +1 +4 Cha 15 +2 +2\
Skills Deception +5, Perception +7, Stealth +4\
Immunities Acid\
Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 17\
Languages Common, Draconic\
CR 7 (XP 2,900; PB +3)

### Actions

Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of Slowing Breath.

Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 15 (2d10 + 4) Slashing damage.

Acid Breath (Recharge 5–6). Dexterity Saving Throw: DC 14, each creature in a 40-foot-long, 5-foot-wide Line. Failure: 40 (9d8) Acid damage. Success: Half damage.

Slowing Breath. Constitution Saving Throw: DC 14, each creature in a 30-foot Cone. Failure: The target can’t take Reactions; its Speed is halved; and it can take either an action or a Bonus Action on its turn, not both. This effect lasts until the end of its next turn.

Original layout: [[../../srd-5.2.1/pages/276|page 276]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Chaotic Good ` |
| ` tags_text ` | ` (Metallic) ` |
| ` movement_text ` | ` 40 ft., Climb 40 ft., Fly 80 ft. ` |
| ` skills_text ` | ` Deception +5, Perception +7, Stealth +4 ` |
| ` immunities_text ` | ` Acid ` |
| ` senses_text ` | ` Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 17 ` |
| ` languages_text ` | ` Common, Draconic ` |
| ` cr_text ` | ` 7 (XP 2,900; PB +3) ` |
| ` hit_points_text ` | ` 119 (14d10 + 42) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 19 ` | ` 4 ` |
| ` Dexterity ` | ` 12 ` | ` 4 ` |
| ` Constitution ` | ` 17 ` | ` 3 ` |
| ` Intelligence ` | ` 16 ` | ` 3 ` |
| ` Wisdom ` | ` 13 ` | ` 4 ` |
| ` Charisma ` | ` 15 ` | ` 2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 119 ` |  |
| ` initiative ` | ` 4 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` climb_speed ` | ` 40 ` | ` ft ` |
| ` fly_speed ` | ` 80 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/young_copper_dragon ` |
| ` implementation ` | ` dnd2024:creature_young_copper_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["acid"] ` |
| ` passive_perception ` | ` 17 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 5 ` |
| ` perception ` | ` 7 ` |
| ` stealth ` | ` 4 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:table
path: splat_rule/parameters/senses
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Sense:
    path: sense
    type: symbol
  Range ft:
    path: range_ft
    type: integer
-->

| Key | Sense | Range ft |
| --- | --- | --- |
| ` row_1 ` | ` Blindsight ` | ` 30 ` |
| ` row_2 ` | ` Darkvision ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common", "draconic"] ` |

#### Features

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 7
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 2
        sides: 10
      add: 4
    average: 15
```

##### Acid breath

<!-- splat:yaml path=splat_rule/parameters/features/acid_breath -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 14
targets:
  kind: creature
  count: all
  area:
    shape: line
    origin: actor
    size_ft: 40
    width_ft: 5
failure:
- op: damage
  components:
  - type: acid
    amount:
      dice:
        count: 9
        sides: 8
      add: 0
    average: 40
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

##### Slowing breath

<!-- splat:yaml path=splat_rule/parameters/features/slowing_breath -->

```yaml
economy: action
op: saving_throw
ability: constitution
dc: 14
targets:
  kind: creature
  count: all
  area:
    shape: cone
    origin: actor
    size_ft: 30
failure:
- op: effect.apply
  duration:
    until: target.next_turn_end
  rules:
    reactions_allowed: false
    speed_multiplier: 0.5
    turn_economy_choice:
    - action
    - bonus_action
    both_allowed: false
success: []
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 3
  feature: rend
replace_attack:
  maximum: 1
  choices:
  - op: feature.use
    feature: slowing_breath
```

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
