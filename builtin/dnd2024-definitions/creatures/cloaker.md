---
splat_schema: 2
splat_id: 445df1a9-08f5-5544-9b84-d2fa512baa74
splat_kind: creature
splat_key: dnd2024:creatures/cloaker
splat_name: Cloaker
splat_origin: dnd2024:reference/creature/cloaker
splat_sources:
- SRD 5.2.1, pp. 274, https://www.dndbeyond.com/srd
---

# Cloaker


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Aberration |
| Alignment | Chaotic Neutral |
| Armor Class | 14 |
| Hit Points | 91 (14d10 + 14) |
| Initiative | +5 (15) |
| Speed | 10 ft., Fly 40 ft. |
| Challenge | 8 (XP 3,900; PB +3) |
| Skills | Stealth +5 |
| Senses | Darkvision 120 ft.; Passive Perception 12 |
| Languages | Deep Speech, Undercommon |
| Immunities | Frightened |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 15 | +2 | +2 |
| Con | 12 | +1 | +1 |
| Int | 13 | +1 | +1 |
| Wis | 14 | +2 | +2 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 274 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Aberration, Chaotic Neutral\
> AC 14 Initiative +5 (15)\
> HP 91 (14d10 + 14)\
> Speed 10 ft., Fly 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 15 +2 +2 Con 12 +1 +1\
> Int 13 +1 +1 Wis 14 +2 +2 Cha 7 −2 −2\
> Skills Stealth +5\
> Immunities Frightened\
> Senses Darkvision 120 ft.; Passive Perception 12\
> Languages Deep Speech, Undercommon\
> CR 8 (XP 3,900; PB +3)


### Traits

Light Sensitivity. While in Bright Light, the cloaker has Disadvantage on attack rolls.

### Actions

Multiattack. The cloaker makes one Attach attack and two Tail attacks.

Attach. Melee Attack Roll: +6, reach 5 ft. Hit: 13 (3d6 + 3) Piercing damage. If the target is a Large or smaller creature, the cloaker attaches to it. While the cloaker is attached, the target has the Blinded condition, and the cloaker can’t make Attach attacks against other targets. In addition, the cloaker halves the damage it takes (round down), and the target takes the same amount of damage.

The cloaker can detach itself by spending 5 feet of movement. The target or a creature within 5 feet of it can take an action to try to detach the cloaker, doing so by succeeding on a DC 14 Strength (Athletics) check.

Tail. Melee Attack Roll: +6, reach 10 ft. Hit: 8 (1d10 + 3) Slashing damage.

### Bonus Actions

Moan. Wisdom Saving Throw: DC 13, each creature in a 60-foot Emanation originating from the cloaker. Failure: The target has the Frightened condition until the end of the cloaker’s next turn. Success: The target is immune to this cloaker’s Moan for the next 24 hours.

Phantasms (Recharge after a Short or Long Rest). The cloaker casts the Mirror Image spell, requiring no spell components and using Wisdom as the spellcasting ability. The spell ends early if the cloaker starts or ends its turn in Bright Light.

Original layout: [[../../srd-5.2.1/pages/274|page 274]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/cloaker ` |
| ` implementation ` | ` dnd2024:creature_cloaker ` |

### Parameters

#### Features

##### Light sensitivity

<!-- splat:yaml path=splat_rule/parameters/features/light_sensitivity -->

```yaml
economy: passive
disadvantage:
  rolls:
  - attack
  when: self_in_bright_light
```

##### Attach

<!-- splat:yaml path=splat_rule/parameters/features/attach -->

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
  average: 13
  amount:
    dice:
      count: 3
      sides: 6
    add: 3
  damage_type: piercing
- op: if
  test:
    target_kind: creature
    target_size_max: large
  then:
  - op: attach
    effects_on_target:
    - op: condition
      condition: blinded
    self_attack_restriction:
      feature: attach
      targets: attached_target_only
    damage_taken:
      self_fraction:
        numerator: 1
        denominator: 2
        round: down
      attached_target_damage: same_amount_as_self
    detach_options:
    - actor: self
      movement_cost_feet: 5
    - actor: target_or_creature_within_5_feet
      economy: action
      check:
        ability: strength
        skill: athletics
        dc: 14
      success: detach
reach_feet: 5
```

##### Tail

<!-- splat:yaml path=splat_rule/parameters/features/tail -->

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
  average: 8
  amount:
    dice:
      count: 1
      sides: 10
    add: 3
  damage_type: slashing
reach_feet: 10
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: attach
  count: 1
- op: use_feature
  feature: tail
  count: 2
```

##### Phantasms

<!-- splat:yaml path=splat_rule/parameters/features/phantasms -->

```yaml
economy: bonus_action
resource:
  capacity: 1
  recovery:
  - short_rest
  - long_rest
  cost: 1
op: cast_spell
spell:
  key: dnd2024:spells/mirror_image
ability: wisdom
components_waived:
- verbal
- somatic
- material
ends_early:
  events:
  - self.turn.start
  - self.turn.end
  when: self_in_bright_light
cast_level: spell_minimum
```

##### Moan

<!-- splat:yaml path=splat_rule/parameters/features/moan -->

```yaml
economy: bonus_action
op: save
ability: wisdom
dc: 13
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    size_feet: 60
    origin: self
failure:
- op: condition
  condition: frightened
  ends:
    event: turn.end
    participant: self
    occurrence: next
success:
- op: feature_immunity
  source: self
  feature: moan
  duration:
    hours: 24
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 5 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` conditions ` | ` ["frightened"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["deep_speech", "undercommon"] ` |
| ` understands ` | ` ["deep_speech", "undercommon"] ` |

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
>   value: 40
> ```

