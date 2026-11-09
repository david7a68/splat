---
splat_schema: 2
splat_id: cb346188-c152-5a56-8d74-da37fa5a3a29
splat_kind: creature
splat_key: dnd2024:creatures/young_bronze_dragon
splat_name: Young Bronze Dragon
splat_origin: dnd2024:reference/creature/young_bronze_dragon
splat_sources:
- SRD 5.2.1, pp. 270, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 58, Habitat
---

# Young Bronze Dragon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Dragon (Metallic) |
| Alignment | Lawful Good |
| Armor Class | 17 |
| Hit Points | 142 (15d10 + 60) |
| Initiative | +3 (13) |
| Speed | 40 ft., Fly 80 ft., Swim 40 ft. |
| Challenge | 8 (XP 3,900; PB +3) |
| Skills | Insight +4, Perception +7, Stealth +3 |
| Senses | Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 17 |
| Languages | Common, Draconic |
| Immunities | Lightning |
| Habitat | Coastal |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 21 | +5 | +5 |
| Dex | 10 | +0 | +3 |
| Con | 19 | +4 | +4 |
| Int | 14 | +2 | +2 |
| Wis | 13 | +1 | +4 |
| Cha | 17 | +3 | +3 |


## Source text

<!-- SRD page 270 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Dragon (Metallic), Lawful Good\
> AC 17 Initiative +3 (13)\
> HP 142 (15d10 + 60)\
> Speed 40 ft., Fly 80 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 21 +5 +5 Dex 10 +0 +3 Con 19 +4 +4\
> Int 14 +2 +2 Wis 13 +1 +4 Cha 17 +3 +3\
> Skills Insight +4, Perception +7, Stealth +3\
> Immunities Lightning\
> Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 17\
> Languages Common, Draconic\
> CR 8 (XP 3,900; PB +3)


### Traits

Amphibious. The dragon can breathe air and water.

### Actions

Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of Repulsion Breath.

Rend. Melee Attack Roll: +8, reach 10 ft. Hit: 16 (2d10 + 5) Slashing damage.

Lightning Breath (Recharge 5–6). Dexterity Saving Throw: DC 15, each creature in a 60-foot-long, 5-foot-wide Line. Failure: 49 (9d10) Lightning damage. Success: Half damage.

Repulsion Breath. Strength Saving Throw: DC 15, each creature in a 30-foot Cone. Failure: The target is pushed up to 40 feet straight away from the dragon and has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/270|page 270]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/young_bronze_dragon ` |
| ` implementation ` | ` dnd2024:creature_young_bronze_dragon ` |

### Parameters

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` breathing_media ` | ` ["air", "water"] ` |

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

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
  average: 16
  amount:
    dice:
      count: 2
      sides: 10
    add: 5
  damage_type: slashing
reach_feet: 10
```

##### Lightning breath

<!-- splat:yaml path=splat_rule/parameters/features/lightning_breath -->

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
dc: 15
targets:
  kind: creature
  count: all
  area:
    shape: line
    size_feet: 60
    origin: self
    width_feet: 5
failure:
- op: damage
  average: 49
  amount:
    dice:
      count: 9
      sides: 10
    add: 0
  damage_type: lightning
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: rend
  count: 3
replacement:
  max_attacks: 1
  choices:
  - feature: repulsion_breath
```

##### Repulsion breath

<!-- splat:yaml path=splat_rule/parameters/features/repulsion_breath -->

```yaml
economy: action
op: save
ability: strength
dc: 15
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 30
    origin: self
failure:
- op: push
  distance_max_feet: 40
  direction: straight_away_from_self
- op: condition
  condition: prone
success: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` insight ` | ` 4 ` |
| ` perception ` | ` 7 ` |
| ` stealth ` | ` 3 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["lightning"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 17 ` |

##### Special

<!-- splat:table
path: splat_rule/parameters/senses/special
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
  Range feet:
    path: range_feet
    type: integer
-->

| Key | Sense | Range feet |
| --- | --- | --- |
| ` row_1 ` | ` Blindsight ` | ` 30 ` |
| ` row_2 ` | ` Darkvision ` | ` 120 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common", "draconic"] ` |
| ` understands ` | ` ["common", "draconic"] ` |

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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 80
> swim_speed:
>   unit: ft
>   value: 40
> ```

