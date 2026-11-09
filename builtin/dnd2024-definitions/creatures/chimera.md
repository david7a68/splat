---
splat_schema: 2
splat_id: 95b8c704-07ab-5065-a7da-d47075365b7e
splat_kind: creature
splat_key: dnd2024:creatures/chimera
splat_name: Chimera
splat_origin: dnd2024:reference/creature/chimera
splat_sources:
- SRD 5.2.1, pp. 273, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 70, Habitat
---

# Chimera


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Monstrosity |
| Alignment | Chaotic Evil |
| Armor Class | 14 |
| Hit Points | 114 (12d10 + 48) |
| Initiative | +0 (10) |
| Speed | 30 ft., Fly 60 ft. |
| Challenge | 6 (XP 2,300; PB +3) |
| Skills | Perception +8 |
| Senses | Darkvision 60 ft.; Passive Perception 18 |
| Languages | Understands Draconic but can’t speak |
| Habitat | Grassland, Hill, Mountain |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +4 |
| Dex | 11 | +0 | +0 |
| Con | 19 | +4 | +4 |
| Int | 3 | -4 | -4 |
| Wis | 14 | +2 | +2 |
| Cha | 10 | +0 | +0 |


## Source text

<!-- SRD page 273 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Monstrosity, Chaotic Evil\
> AC 14 Initiative +0 (10)\
> HP 114 (12d10 + 48)\
> Speed 30 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 19 +4 +4 Dex 11 +0 +0 Con 19 +4 +4\
> Int 3 −4 −4 Wis 14 +2 +2 Cha 10 +0 +0\
> Skills Perception +8\
> Senses Darkvision 60 ft.; Passive Perception 18\
> Languages Understands Draconic but can’t speak\
> CR 6 (XP 2,300; PB +3)


### Actions

Multiattack. The chimera makes one Ram attack, one Bite attack, and one Claw attack. It can replace the Claw attack with a use of Fire Breath if available.

Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 11 (2d6 + 4) Piercing damage, or 18 (4d6 + 4) Piercing damage if the chimera had Advantage on the attack roll.

Claw. Melee Attack Roll: +7, reach 5 ft. Hit: 7 (1d6 + 4) Slashing damage.

Ram. Melee Attack Roll: +7, reach 5 ft. Hit: 10 (1d12 + 4) Bludgeoning damage. If the target is a Medium or smaller creature, it has the Prone condition.

Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 15, each creature in a 15-foot Cone. Failure: 31 (7d8) Fire damage. Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/273|page 273]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/chimera ` |
| ` implementation ` | ` dnd2024:creature_chimera ` |

### Parameters

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 7
targets:
  count: 1
hit:
- op: choose_by_predicate
  test:
    attack_had_advantage: true
  then:
  - op: damage
    average: 18
    amount:
      dice:
        count: 4
        sides: 6
      add: 4
    damage_type: piercing
  else:
  - op: damage
    average: 11
    amount:
      dice:
        count: 2
        sides: 6
      add: 4
    damage_type: piercing
reach_feet: 5
```

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

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
  average: 7
  amount:
    dice:
      count: 1
      sides: 6
    add: 4
  damage_type: slashing
reach_feet: 5
```

##### Ram

<!-- splat:yaml path=splat_rule/parameters/features/ram -->

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
  average: 10
  amount:
    dice:
      count: 1
      sides: 12
    add: 4
  damage_type: bludgeoning
- op: if
  test:
    target_kind: creature
    target_size_max: medium
  then:
  - op: condition
    condition: prone
reach_feet: 5
```

##### Fire breath

<!-- splat:yaml path=splat_rule/parameters/features/fire_breath -->

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
    shape: cone
    size_feet: 15
    origin: self
failure:
- op: damage
  average: 31
  amount:
    dice:
      count: 7
      sides: 8
    add: 0
  damage_type: fire
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
  feature: ram
  count: 1
- op: use_feature
  feature: bite
  count: 1
- op: choose
  choices:
  - op: use_feature
    feature: claw
    count: 1
  - op: use_feature
    feature: fire_breath
    count: 1
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 8 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 18 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` understands ` | ` ["draconic"] ` |
| ` can_speak ` | ` false ` |

##### Speaks

<!-- splat:yaml path=splat_rule/parameters/communication/speaks -->

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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 60
> ```

