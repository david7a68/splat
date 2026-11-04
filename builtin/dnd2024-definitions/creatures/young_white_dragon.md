---
splat_schema: 2
splat_id: 4544c113-2676-582c-898a-9b6ad5031d8b
splat_kind: creature
splat_key: dnd2024:creatures/young_white_dragon
splat_name: Young White Dragon
splat_origin: dnd2024:reference/creature/young_white_dragon
splat_sources:
- SRD 5.2.1, pp. 340, https://www.dndbeyond.com/srd
---

# Young White Dragon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Dragon (Chromatic) |
| Alignment | Chaotic Evil |
| Armor Class | 17 |
| Hit Points | 123 (13d10 + 52) |
| Initiative | +3 (13) |
| Speed | 40 ft., Burrow 20 ft., Fly 80 ft., Swim 40 ft. |
| Challenge | 6 (2,300 XP; PB +3) |
| Skills | Perception +6, Stealth +3 |
| Senses | Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 16 |
| Languages | Common, Draconic |
| Immunities | Cold |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 10 | +0 | +3 |
| Con | 18 | +4 | +4 |
| Int | 6 | -2 | -2 |
| Wis | 11 | +0 | +3 |
| Cha | 12 | +1 | +1 |


## Source text

<!-- SRD page 340 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Dragon (Chromatic), Chaotic Evil\
> AC 17 Initiative +3 (13)\
> HP 123 (13d10 + 52)\
> Speed 40 ft., Burrow 20 ft., Fly 80 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 10 +0 +3 Con 18 +4 +4\
> Int 6 −2 −2 Wis 11 +0 +3 Cha 12 +1 +1\
> Skills Perception +6, Stealth +3\
> Immunities Cold\
> Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 16\
> Languages Common, Draconic\
> CR 6 (2,300 XP; PB +3)


### Traits

Ice Walk. The dragon can move across and climb icy surfaces without needing to make an ability check. Additionally, Difficult Terrain composed of ice or snow doesn’t cost it extra movement.

### Actions

Multiattack. The dragon makes three Rend attacks.

Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 9 (2d4 + 4) Slashing damage plus 2 (1d4) Cold damage.

Cold Breath (Recharge 5–6). Constitution Saving Throw: DC 15, each creature in a 30-foot Cone. Failure: 40 (9d8) Cold damage. Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/340|page 340]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/young_white_dragon ` |
| ` implementation ` | ` dnd2024:creature_young_white_dragon ` |

### Parameters

#### Features

##### Ice walk

<!-- splat:yaml path=splat_rule/parameters/features/ice_walk -->

```yaml
economy: passive
movement_checks_waived:
  surfaces:
  - ice
  modes:
  - walk
  - climb
difficult_terrain_extra_cost_ignored:
- ice
- snow
```

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
  average: 9
  amount:
    dice:
      count: 2
      sides: 4
    add: 4
  damage_type: slashing
- op: damage
  average: 2
  amount:
    dice:
      count: 1
      sides: 4
    add: 0
  damage_type: cold
reach_feet: 10
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
dc: 15
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 30
    origin: self
failure:
- op: damage
  average: 40
  amount:
    dice:
      count: 9
      sides: 8
    add: 0
  damage_type: cold
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
| ` perception ` | ` 6 ` |
| ` stealth ` | ` 3 ` |

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
| ` passive_perception ` | ` 16 ` |

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
> burrow_speed:
>   unit: ft
>   value: 20
> swim_speed:
>   unit: ft
>   value: 40
> ```

