---
splat_schema: 2
splat_id: 4db58924-8af5-5174-8218-1a90dee028b5
splat_kind: creature
splat_key: dnd2024:creatures/medusa
splat_name: Medusa
splat_origin: dnd2024:reference/creature/medusa
splat_sources:
- SRD 5.2.1, pp. 306, 307, https://www.dndbeyond.com/srd
---

# Medusa


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Monstrosity |
| Alignment | Lawful Evil |
| Armor Class | 15 |
| Hit Points | 127 (17d8 + 51) |
| Initiative | +6 (16) |
| Speed | 30 ft. |
| Challenge | 6 (XP 2,300; PB +3) |
| Skills | Deception +5, Perception +4, Stealth +6 |
| Senses | Darkvision 150 ft.; Passive Perception 14 |
| Languages | Common plus one other language |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 10 | +0 | +0 |
| Dex | 17 | +3 | +3 |
| Con | 16 | +3 | +3 |
| Int | 12 | +1 | +1 |
| Wis | 13 | +1 | +4 |
| Cha | 15 | +2 | +2 |


## Source text

<!-- SRD page 306 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Monstrosity, Lawful Evil\
> AC 15 Initiative +6 (16)\
> HP 127 (17d8 + 51)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 10 +0 +0 Dex 17 +3 +3 Con 16 +3 +3\
> Int 12 +1 +1 Wis 13 +1 +4 Cha 15 +2 +2\
> Skills Deception +5, Perception +4, Stealth +6\
> Senses Darkvision 150 ft.; Passive Perception 14\
> Languages Common plus one other language\
> CR 6 (XP 2,300; PB +3)


### Actions

Multiattack. The medusa makes two Claw attacks and one Snake Hair attack, or it makes three Poison Ray attacks.

Claw. Melee Attack Roll: +6, reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage.

Snake Hair. Melee Attack Roll: +6, reach 5 ft. Hit: 5 (1d4 + 3) Piercing damage plus 14 (4d6) Poison damage.

Poison Ray. Ranged Attack Roll: +5, range 150 ft. Hit: 11 (2d8 + 2) Poison damage.

<!-- SRD page 307 -->

### Bonus Actions

Petrifying Gaze (Recharge 5–6). Constitution Saving Throw: DC 13, each creature in a 30-foot Cone. If the medusa sees its reflection in the Cone, the medusa must make this save. First Failure: The target has the Restrained condition and repeats the save at the end of its next turn if it is still Restrained, ending the effect on itself on a success. Second Failure: The target has the Petrified condition instead of the Restrained condition.

Original layout: [[../../srd-5.2.1/pages/306|page 306]], [[../../srd-5.2.1/pages/307|page 307]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/medusa ` |
| ` implementation ` | ` dnd2024:creature_medusa ` |

### Parameters

#### Features

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 6
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
reach_feet: 5
```

##### Snake hair

<!-- splat:yaml path=splat_rule/parameters/features/snake_hair -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 6
targets:
  count: 1
hit:
- op: damage
  average: 5
  amount:
    dice:
      count: 1
      sides: 4
    add: 3
  damage_type: piercing
- op: damage
  average: 14
  amount:
    dice:
      count: 4
      sides: 6
    add: 0
  damage_type: poison
reach_feet: 5
```

##### Poison ray

<!-- splat:yaml path=splat_rule/parameters/features/poison_ray -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 11
  amount:
    dice:
      count: 2
      sides: 8
    add: 2
  damage_type: poison
range_feet: 150
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: choose_sequence
choices:
- - op: use_feature
    feature: claw
    count: 2
  - op: use_feature
    feature: snake_hair
    count: 1
- - op: use_feature
    feature: poison_ray
    count: 3
```

##### Petrifying gaze

<!-- splat:yaml path=splat_rule/parameters/features/petrifying_gaze -->

```yaml
economy: bonus_action
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
dc: 13
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 30
    origin: self
failure:
- op: condition
  condition: restrained
  ends:
    event: repeat_save.success
  repeat_save:
    event: target.turn.end
    occurrence: next
    requires_condition: restrained
    ability: constitution
    dc: 13
    success: end_effect
    failure:
    - op: remove_condition
      condition: restrained
    - op: condition
      condition: petrified
success: []
additional_target:
  participant: self
  when: self_sees_own_reflection_in_cone
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 5 ` |
| ` perception ` | ` 4 ` |
| ` stealth ` | ` 6 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 150 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common"] ` |
| ` understands ` | ` ["common"] ` |

##### Additional languages

<!-- splat:values path=splat_rule/parameters/communication/additional_languages -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` chooser ` | ` gm ` |
| ` distinct_from_known ` | ` true ` |
| ` can_speak ` | ` true ` |
| ` can_understand ` | ` true ` |

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
