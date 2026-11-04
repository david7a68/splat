---
splat_schema: 2
splat_id: 55036dfe-5c31-5350-a624-942d197a4b11
splat_kind: creature
splat_key: dnd2024:creatures/young_green_dragon
splat_name: Young Green Dragon
splat_origin: dnd2024:reference/creature/young_green_dragon
splat_sources:
- SRD 5.2.1, pp. 293, 294, https://www.dndbeyond.com/srd
---

# Young Green Dragon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Dragon (Chromatic) |
| Alignment | Lawful Evil |
| Armor Class | 18 |
| Hit Points | 136 (16d10 + 48) |
| Initiative | +4 (14) |
| Speed | 40 ft., Fly 80 ft., Swim 40 ft. |
| Challenge | 8 (XP 3,900; PB +3) |
| Skills | Deception +5, Perception +7, Stealth +4 |
| Senses | Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 17 |
| Languages | Common, Draconic |
| Immunities | Poison; Poisoned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +4 |
| Dex | 12 | +1 | +4 |
| Con | 17 | +3 | +3 |
| Int | 16 | +3 | +3 |
| Wis | 13 | +1 | +4 |
| Cha | 15 | +2 | +2 |


## Source text

<!-- SRD page 293 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Dragon (Chromatic), Lawful Evil\
> AC 18 Initiative +4 (14)\
> HP 136 (16d10 + 48)\
> Speed 40 ft., Fly 80 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 19 +4 +4 Dex 12 +1 +4 Con 17 +3 +3\
> Int 16 +3 +3 Wis 13 +1 +4 Cha 15 +2 +2\
> Skills Deception +5, Perception +7, Stealth +4\
> Immunities Poison; Poisoned\
> Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 17\
> Languages Common, Draconic\
> CR 8 (XP 3,900; PB +3)


### Traits

Amphibious. The dragon can breathe air and water.

<!-- SRD page 294 -->

### Actions

Multiattack. The dragon makes three Rend attacks.

Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 11 (2d6 + 4) Slashing damage plus 7 (2d6) Poison damage.

Poison Breath (Recharge 5–6). Constitution Saving Throw: DC 14, each creature in a 30-foot Cone. Failure: 42 (12d6) Poison damage. Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/293|page 293]], [[../../srd-5.2.1/pages/294|page 294]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/young_green_dragon ` |
| ` implementation ` | ` dnd2024:creature_young_green_dragon ` |

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
attack_bonus: 7
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
  damage_type: slashing
- op: damage
  average: 7
  amount:
    dice:
      count: 2
      sides: 6
    add: 0
  damage_type: poison
reach_feet: 10
```

##### Poison breath

<!-- splat:yaml path=splat_rule/parameters/features/poison_breath -->

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
dc: 14
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 30
    origin: self
failure:
- op: damage
  average: 42
  amount:
    dice:
      count: 12
      sides: 6
    add: 0
  damage_type: poison
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
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 5 ` |
| ` perception ` | ` 7 ` |
| ` stealth ` | ` 4 ` |

#### Defenses

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

