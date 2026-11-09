---
splat_schema: 2
splat_id: 273b9cd7-f749-5b98-a216-65592f4c1360
splat_kind: creature
splat_key: dnd2024:creatures/ghast
splat_name: Ghast
splat_origin: dnd2024:reference/creature/ghast
splat_sources:
- SRD 5.2.1, pp. 287, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 130, Habitat
---

# Ghast


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Undead |
| Alignment | Chaotic Evil |
| Armor Class | 13 |
| Hit Points | 36 (8d8) |
| Initiative | +3 (13) |
| Speed | 30 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 10 |
| Languages | Common |
| Resistances | Necrotic |
| Immunities | Poison; Charmed, Exhaustion, Poisoned |
| Habitat | Swamp, Underdark, Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 17 | +3 | +3 |
| Con | 10 | +0 | +0 |
| Int | 11 | +0 | +0 |
| Wis | 10 | +0 | +2 |
| Cha | 8 | -1 | -1 |


## Source text

<!-- SRD page 287 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Undead, Chaotic Evil\
> AC 13 Initiative +3 (13)\
> HP 36 (8d8)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 17 +3 +3 Con 10 +0 +0\
> Int 11 +0 +0 Wis 10 +0 +2 Cha 8 −1 −1\
> Resistances Necrotic\
> Immunities Poison; Charmed, Exhaustion, Poisoned\
> Senses Darkvision 60 ft.; Passive Perception 10\
> Languages Common\
> CR 2 (XP 450; PB +2)


### Traits

Stench. Constitution Saving Throw: DC 10, any creature that starts its turn in a 5-foot Emanation originating from the ghast. Failure: The target has the Poisoned condition until the start of its next turn. Success: The target is immune to this ghast’s Stench for 24 hours.

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Piercing damage plus 9 (2d8) Necrotic damage.

Claw. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage. If the target is a non-Undead creature, it is subjected to the following effect. Constitution Saving Throw: DC 10. Failure: The target has the Paralyzed condition until the end of its next turn.

Original layout: [[../../srd-5.2.1/pages/287|page 287]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/ghast ` |
| ` implementation ` | ` dnd2024:creature_ghast ` |

### Parameters

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

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
  average: 7
  amount:
    dice:
      count: 1
      sides: 8
    add: 3
  damage_type: piercing
- op: damage
  average: 9
  amount:
    dice:
      count: 2
      sides: 8
    add: 0
  damage_type: necrotic
reach_feet: 5
```

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

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
  average: 10
  amount:
    dice:
      count: 2
      sides: 6
    add: 3
  damage_type: slashing
- op: if
  test:
    target_kind: creature
    target_type_not: undead
  then:
  - op: save
    ability: constitution
    dc: 10
    targets:
      participant: target
    failure:
    - op: condition
      condition: paralyzed
      ends:
        event: turn.end
        participant: target
        occurrence: next
    success: []
reach_feet: 5
```

##### Stench

<!-- splat:yaml path=splat_rule/parameters/features/stench -->

```yaml
economy: passive
trigger: creature.turn.start
op: save
ability: constitution
dc: 10
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    size_feet: 5
    origin: self
failure:
- op: condition
  condition: poisoned
  ends:
    event: turn.start
    participant: target
    occurrence: next
success:
- op: feature_immunity
  feature: stench
  source: self
  duration:
    hours: 24
```

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["necrotic"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["poison"] ` |
| ` conditions ` | ` ["charmed", "exhaustion", "poisoned"] ` |

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

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common"] ` |
| ` understands ` | ` ["common"] ` |

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
