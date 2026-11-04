---
splat_schema: 2
splat_id: 4a242baf-74a2-51bb-a6e6-728d5d8ff0e1
splat_kind: creature
splat_key: dnd2024:creatures/bronze_dragon_wyrmling
splat_name: Bronze Dragon Wyrmling
splat_origin: dnd2024:reference/creature/bronze_dragon_wyrmling
splat_sources:
- SRD 5.2.1, pp. 269, 270, https://www.dndbeyond.com/srd
---

# Bronze Dragon Wyrmling


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Dragon (Metallic) |
| Alignment | Lawful Good |
| Armor Class | 15 |
| Hit Points | 39 (6d8 + 12) |
| Initiative | +2 (12) |
| Speed | 30 ft., Fly 60 ft., Swim 30 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Perception +4, Stealth +2 |
| Senses | Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 |
| Languages | Draconic |
| Immunities | Lightning |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 10 | +0 | +2 |
| Con | 15 | +2 | +2 |
| Int | 12 | +1 | +1 |
| Wis | 11 | +0 | +2 |
| Cha | 15 | +2 | +2 |


## Source text

<!-- SRD page 269 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Dragon (Metallic), Lawful Good\
> AC 15 Initiative +2 (12)\
> HP 39 (6d8 + 12)\
> Speed 30 ft., Fly 60 ft., Swim 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 10 +0 +2 Con 15 +2 +2\
> Int 12 +1 +1 Wis 11 +0 +2 Cha 15 +2 +2\
> Skills Perception +4, Stealth +2\
> Immunities Lightning\
> Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14\
> Languages Draconic\
> CR 2 (XP 450; PB +2)


<!-- SRD page 270 -->

### Traits

Amphibious. The dragon can breathe air and water.

### Actions

Multiattack. The dragon makes two Rend attacks.

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 8 (1d10 + 3) Slashing damage.

Lightning Breath (Recharge 5–6). Dexterity Saving Throw: DC 12, each creature in a 40-foot-long, 5-foot-wide Line. Failure: 16 (3d10) Lightning damage. Success: Half damage.

Repulsion Breath. Strength Saving Throw: DC 12, each creature in a 30-foot Cone. Failure: The target is pushed up to 30 feet straight away from the dragon and has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/269|page 269]], [[../../srd-5.2.1/pages/270|page 270]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/bronze_dragon_wyrmling ` |
| ` implementation ` | ` dnd2024:creature_bronze_dragon_wyrmling ` |

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
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 8
  amount:
    dice:
      count: 1
      sides: 10
    add: 3
  damage_type: slashing
reach_feet: 5
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
dc: 12
targets:
  kind: creature
  count: all
  area:
    shape: line
    size_feet: 40
    origin: self
    width_feet: 5
failure:
- op: damage
  average: 16
  amount:
    dice:
      count: 3
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
  count: 2
```

##### Repulsion breath

<!-- splat:yaml path=splat_rule/parameters/features/repulsion_breath -->

```yaml
economy: action
op: save
ability: strength
dc: 12
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 30
    origin: self
failure:
- op: push
  distance_max_feet: 30
  direction: straight_away_from_self
- op: condition
  condition: prone
success: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |
| ` stealth ` | ` 2 ` |

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
| ` passive_perception ` | ` 14 ` |

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
| ` row_1 ` | ` Blindsight ` | ` 10 ` |
| ` row_2 ` | ` Darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["draconic"] ` |
| ` understands ` | ` ["draconic"] ` |

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
>   value: 60
> swim_speed:
>   unit: ft
>   value: 30
> ```

