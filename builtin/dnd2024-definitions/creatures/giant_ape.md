---
splat_schema: 2
splat_id: 34922b69-57c3-5971-a0d5-45a3c6005c88
splat_kind: creature
splat_key: dnd2024:creatures/giant_ape
splat_name: Giant Ape
splat_origin: dnd2024:reference/creature/giant_ape
splat_sources:
- SRD 5.2.1, pp. 349, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 354, Habitat
---

# Giant Ape


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 168 (16d12 + 64) |
| Initiative | +5 (15) |
| Speed | 40 ft., Climb 40 ft. |
| Challenge | 7 (XP 2,900; PB +3) |
| Skills | Athletics +9, Perception +4, Survival +4 |
| Senses | Passive Perception 14 |
| Languages | None |
| Habitat | Forest |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 23 | +6 | +6 |
| Dex | 14 | +2 | +2 |
| Con | 18 | +4 | +4 |
| Int | 5 | -3 | -3 |
| Wis | 12 | +1 | +1 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 349 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Beast, Unaligned\
> AC 12 Initiative +5 (15)\
> HP 168 (16d12 + 64)\
> Speed 40 ft., Climb 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 23 +6 +6 Dex 14 +2 +2 Con 18 +4 +4\
> Int 5 −3 −3 Wis 12 +1 +1 Cha 7 −2 −2\
> Skills Athletics +9, Perception +4, Survival +4\
> Senses Passive Perception 14\
> Languages None\
> CR 7 (XP 2,900; PB +3)


### Actions

Multiattack. The ape makes two Fist attacks.

Fist. Melee Attack Roll: +9, reach 10 ft. Hit: 22 (3d10 + 6) Bludgeoning damage.

Boulder Toss (Recharge 6). The ape hurls a boulder at a point it can see within 90 feet. Dexterity Saving Throw: DC 17, each creature in a 5-foot-radius Sphere centered on that point. Failure: 24 (7d6) Bludgeoning damage. If the target is a Large or smaller creature, it has the Prone condition. Success: Half damage only.

### Bonus Actions

Leap. The ape jumps up to 30 feet by spending 10 feet of movement.

Original layout: [[../../srd-5.2.1/pages/349|page 349]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_ape ` |
| ` implementation ` | ` dnd2024:creature_giant_ape ` |

### Parameters

#### Features

##### Fist

<!-- splat:yaml path=splat_rule/parameters/features/fist -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 9
targets:
  count: 1
hit:
- op: damage
  average: 22
  amount:
    dice:
      count: 3
      sides: 10
    add: 6
  damage_type: bludgeoning
reach_feet: 10
```

##### Leap

<!-- splat:values path=splat_rule/parameters/features/leap -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` jump ` |
| ` distance_max_feet ` | ` 30 ` |
| ` movement_cost_feet ` | ` 10 ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: fist
  count: 2
```

##### Boulder toss

<!-- splat:yaml path=splat_rule/parameters/features/boulder_toss -->

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
    - 6
  cost: 1
  recovery_also:
  - short_rest
  - long_rest
op: save
ability: dexterity
dc: 17
targets:
  kind: creature
  count: all
  area:
    shape: sphere
    size_feet: 5
    origin: chosen_point
    origin_visible_to: self
    origin_range_feet: 90
failure:
- op: damage
  average: 24
  amount:
    dice:
      count: 7
      sides: 6
    add: 0
  damage_type: bludgeoning
- op: if
  test:
    target_size_max: large
  then:
  - op: condition
    condition: prone
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` athletics ` | ` 9 ` |
| ` perception ` | ` 4 ` |
| ` survival ` | ` 4 ` |

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

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> climb_speed:
>   unit: ft
>   value: 40
> ```

