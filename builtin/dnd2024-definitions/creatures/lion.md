---
splat_schema: 2
splat_id: d59c56e9-b533-5939-92ec-dcddd24de7c7
splat_kind: creature
splat_key: dnd2024:creatures/lion
splat_name: Lion
splat_origin: dnd2024:reference/creature/lion
splat_sources:
- SRD 5.2.1, pp. 356, 357, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 364, Habitat
---

# Lion


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 22 (4d10) |
| Initiative | +2 (12) |
| Speed | 50 ft. |
| Challenge | 1 (XP 200; PB +2) |
| Skills | Perception +3, Stealth +4 |
| Senses | Darkvision 60 ft.; Passive Perception 13 |
| Languages | None |
| Habitat | Desert, Grassland, Hill, Mountain |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 15 | +2 | +2 |
| Con | 11 | +0 | +0 |
| Int | 3 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 8 | -1 | -1 |


## Source text

<!-- SRD page 356 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 12  Initiative +2 (12)\
> HP 22 (4d10)\
> Speed 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 15 +2 +2 Con 11 +0 +0\
> Int 3 −4 −4 Wis 12 +1 +1 Cha 8 −1 −1\
> Skills Perception +3, Stealth +4\
> Senses Darkvision 60 ft.; Passive Perception 13\
> Languages None\
> CR 1 (XP 200; PB +2)


<!-- SRD page 357 -->

### Traits

Pack Tactics. The lion has Advantage on an attack roll against a creature if at least one of the lion’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

Running Leap. With a 10-foot running start, the lion can Long Jump up to 25 feet.

### Actions

Multiattack. The lion makes two Rend attacks. It can replace one attack with a use of Roar.

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Slashing damage.

Roar. Wisdom Saving Throw: DC 11, one creature within 15 feet. Failure: The target has the Frightened condition until the start of the lion’s next turn.

Original layout: [[../../srd-5.2.1/pages/356|page 356]], [[../../srd-5.2.1/pages/357|page 357]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/lion ` |
| ` implementation ` | ` dnd2024:creature_lion ` |

### Parameters

#### Features

##### Pack tactics

<!-- splat:yaml path=splat_rule/parameters/features/pack_tactics -->

```yaml
economy: passive
advantage:
  roll: attack
  target_kind: creature
  requires:
    ally_distance_to_target_max_feet: 5
    ally_incapacitated: false
```

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
  average: 7
  amount:
    dice:
      count: 1
      sides: 8
    add: 3
  damage_type: slashing
reach_feet: 5
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
replacement:
  max_attacks: 1
  choices:
  - feature: roar
```

##### Running leap

<!-- splat:values path=splat_rule/parameters/features/running_leap -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` long_jump_max_feet ` | ` 25 ` |
| ` running_start_min_feet ` | ` 10 ` |

##### Roar

<!-- splat:yaml path=splat_rule/parameters/features/roar -->

```yaml
economy: action
op: save
ability: wisdom
dc: 11
targets:
  kind: creature
  count: 1
  range_feet: 15
failure:
- op: condition
  condition: frightened
  ends:
    event: turn.start
    participant: self
    occurrence: next
success: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |
| ` stealth ` | ` 4 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 13 ` |

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
