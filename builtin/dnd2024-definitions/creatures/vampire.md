---
splat_schema: 2
splat_id: a841e2a2-3123-522d-9109-059fe039fb76
splat_kind: creature
splat_key: dnd2024:creatures/vampire
splat_name: Vampire
splat_origin: dnd2024:reference/creature/vampire
splat_sources:
- SRD 5.2.1, pp. 335, 336, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 317, Habitat
---

# Vampire


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Undead |
| Alignment | Lawful Evil |
| Armor Class | 16 |
| Hit Points | 195 (23d8 + 92) |
| Initiative | +14 (24) |
| Speed | 40 ft., Climb 40 ft. |
| Challenge | 13 (XP 10,000, or 11,500 in lair; PB +5) |
| Skills | Perception +7, Stealth +9 |
| Senses | Darkvision 120 ft.; Passive Perception 17 |
| Languages | Common plus two other languages |
| Resistances | Necrotic |
| Habitat | Underdark, Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 18 | +4 | +9 |
| Con | 18 | +4 | +9 |
| Int | 17 | +3 | +3 |
| Wis | 15 | +2 | +7 |
| Cha | 18 | +4 | +9 |


## Source text

<!-- SRD page 335 -->

Medium or Small Undead, Lawful Evil\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 16 Initiative +14 (24)\
> HP 195 (23d8 + 92)\
> Speed 40 ft., Climb 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 18 +4 +9 Con 18 +4 +9\
> Int 17 +3 +3 Wis 15 +2 +7 Cha 18 +4 +9\
> Skills Perception +7, Stealth +9\
> Resistances Necrotic\
> Senses Darkvision 120 ft.; Passive Perception 17\
> Languages Common plus two other languages\
> CR 13 (XP 10,000, or 11,500 in lair; PB +5)


### Traits

Legendary Resistance (3/Day, or 4/Day in Lair). If the vampire fails a saving throw, it can choose to succeed instead.

Misty Escape. If the vampire drops to 0 Hit Points outside its resting place, the vampire uses Shape-Shift to become mist (no action required). If it can’t use Shape-Shift, it is destroyed.

While it has 0 Hit Points in mist form, it can’t return to its vampire form, and it must reach its resting place within 2 hours or be destroyed. Once in its resting place, it returns to its vampire form and has the Paralyzed condition until it regains any Hit Points, and it regains 1 Hit Point after spending 1 hour there.

Spider Climb. The vampire can climb difficult surfaces, including along ceilings, without needing to make an ability check.

Vampire Weakness. The vampire has these weaknesses:

Forbiddance. The vampire can’t enter a residence without an invitation from an occupant.

Running Water. The vampire takes 20 Acid damage if it ends its turn in running water.

Stake to the Heart. If a weapon that deals Piercing damage is driven into the vampire’s heart while the vampire has the Incapacitated condition in its resting place, the vampire has the Paralyzed condition until the weapon is removed.

Sunlight. The vampire takes 20 Radiant damage if it starts its turn in sunlight. While in sunlight, it has Disadvantage on attack rolls and ability checks.

### Actions

Multiattack (Vampire Form Only). The vampire makes two Grave Strike attacks and uses Bite.

Grave Strike (Vampire Form Only). Melee Attack Roll: +9, reach 5 ft. Hit: 8 (1d8 + 4) Bludgeoning damage plus 7 (2d6) Necrotic damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 14) from one of two hands.

Bite (Bat or Vampire Form Only). Constitution Saving Throw: DC 17, one creature within 5 feet that is willing or that has the Grappled, Incapacitated, or Restrained condition. Failure: 6 (1d4 + 4) Piercing damage plus 13 (3d8) Necrotic damage. The target’s Hit Point maximum decreases by an amount equal to the Necrotic damage taken, and the vampire regains Hit Points equal to that amount. A Humanoid reduced to 0 Hit Points by this damage and then buried rises the following sunset as a Vampire Spawn under the vampire’s control.

<!-- SRD page 336 -->

### Bonus Actions

Charm (Recharge 5–6). The vampire casts Charm Person, requiring no spell components and using Charisma as the spellcasting ability (spell save DC 17), and the duration is 24 hours. The Charmed target is a willing recipient of the vampire’s Bite, the damage of which doesn’t end the spell. When the spell ends, the target is unaware it was Charmed by the vampire.

Shape-Shift. If the vampire isn’t in sunlight or running water, it shape-shifts into a Tiny bat (Speed 5 ft., Fly Speed 30 ft.) or a Medium cloud of mist (Speed 5 ft., Fly Speed 20 ft. [hover]), or it returns to its vampire form. Anything it is wearing transforms with it.

While in bat form, the vampire can’t speak. Its game statistics, other than its size and Speed, are unchanged.

While in mist form, the vampire can’t take any actions, speak, or manipulate objects. It is weightless and can enter an enemy’s space and stop there. If air can pass through a space, the mist can do so, but it can’t pass through liquid. It has Resistance to all damage, except the damage it takes from sunlight.

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the vampire can expend a use to take one of the following actions. The vampire regains all expended uses at the start of each of its turns.

Beguile. The vampire casts Command, requiring no spell components and using Charisma as the spellcasting ability (spell save DC 17). The vampire can’t take this action again until the start of its next turn.

Deathless Strike. The vampire moves up to half its Speed, and it makes one Grave Strike attack.

Original layout: [[../../srd-5.2.1/pages/335|page 335]], [[../../srd-5.2.1/pages/336|page 336]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/vampire ` |
| ` implementation ` | ` dnd2024:creature_vampire ` |

### Parameters

#### Features

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: passive
resource:
  capacity: 3
  capacity_in_lair: 4
  recovery: long_rest
  cost: 1
trigger: saving_throw.failed
optional: true
op: replace_save_outcome
outcome: success
```

##### Spider climb

<!-- splat:values path=splat_rule/parameters/features/spider_climb -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` climb_ability_check_required ` | ` false ` |
| ` surfaces ` | ` ["difficult", "ceiling"] ` |

##### Grave strike

<!-- splat:yaml path=splat_rule/parameters/features/grave_strike -->

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
  average: 8
  amount:
    dice:
      count: 1
      sides: 8
    add: 4
  damage_type: bludgeoning
- op: damage
  average: 7
  amount:
    dice:
      count: 2
      sides: 6
    add: 0
  damage_type: necrotic
- op: if
  test:
    target_kind: creature
    target_size_max: large
  then:
  - op: condition
    condition: grappled
    escape_dc: 14
    source: self
    appendages:
      kind: hand
      available: 2
      used: 1
reach_feet: 5
requires_form:
- vampire
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: grave_strike
  count: 2
- op: use_feature
  feature: bite
  count: 1
requires_form:
- vampire
```

##### Misty escape

<!-- splat:yaml path=splat_rule/parameters/features/misty_escape -->

```yaml
economy: none
trigger: self.hit_points_drop_to_zero
requires:
  outside_resting_place: true
op: use_feature
feature: shape_shift
form: mist
if_shape_shift_unavailable:
  op: destroy
while_zero_hp_mist:
  return_to_vampire_form_allowed: false
  reach_resting_place_deadline:
    hours: 2
  deadline_failure: destroy
on_reach_resting_place:
  form: vampire
  conditions:
  - op: condition
    condition: paralyzed
    ends:
      event: self.hit_points_regained
  after:
    hours: 1
    op: heal
    amount:
      constant: 1
```

##### Vampire weakness

<!-- splat:values path=splat_rule/parameters/features/vampire_weakness -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` feature_group ` |
| ` members ` | ` ["forbiddance", "running_water", "stake_to_the_heart", "sunlight"] ` |

##### Forbiddance

<!-- splat:yaml path=splat_rule/parameters/features/forbiddance -->

```yaml
economy: passive
enter_residence_requires:
  invitation_from: occupant
```

##### Running water

<!-- splat:yaml path=splat_rule/parameters/features/running_water -->

```yaml
economy: passive
trigger: self.turn.end
requires:
  in_running_water: true
effects:
- op: damage
  average: 20
  amount:
    constant: 20
  damage_type: acid
```

##### Stake to the heart

<!-- splat:yaml path=splat_rule/parameters/features/stake_to_the_heart -->

```yaml
economy: passive
trigger: piercing_weapon.driven_into_self_heart
requires:
  self_incapacitated: true
  in_resting_place: true
effects:
- op: condition
  condition: paralyzed
  ends:
    event: weapon.removed
```

##### Sunlight

<!-- splat:yaml path=splat_rule/parameters/features/sunlight -->

```yaml
economy: passive
turn_start:
  requires:
    in_sunlight: true
  effects:
  - op: damage
    average: 20
    amount:
      constant: 20
    damage_type: radiant
while_in_sunlight:
  disadvantage:
  - attack
  - ability_check
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
requires_form:
- bat
- vampire
op: save
ability: constitution
dc: 17
targets:
  kind: creature
  count: 1
  range_feet: 5
  eligibility_any:
  - willing
  - grappled
  - incapacitated
  - restrained
failure:
- op: damage
  average: 6
  amount:
    dice:
      count: 1
      sides: 4
    add: 4
  damage_type: piercing
- op: damage
  average: 13
  amount:
    dice:
      count: 3
      sides: 8
    add: 0
  damage_type: necrotic
- op: reduce_hit_point_maximum
  amount:
    event_value: necrotic_damage_taken
- op: heal
  participant: self
  amount:
    event_value: necrotic_damage_taken
- op: if
  test:
    target_type: humanoid
    attack_reduced_target_hp_to_zero: true
  then:
  - op: create_undead_on_burial
    definition:
      key: dnd2024:creatures/vampire_spawn
    timing: following_sunset
    controller: self
success: []
```

##### Charm

<!-- splat:yaml path=splat_rule/parameters/features/charm -->

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
op: cast_spell
spell:
  key: dnd2024:spells/charm_person
ability: charisma
save_dc: 17
components_waived:
- verbal
- somatic
- material
overrides:
  duration:
    hours: 24
  while_charmed:
    willing_for_self_bite: true
    self_bite_damage_ends_spell: false
  on_end:
    aware_of_charm: false
cast_level: spell_minimum
```

##### Shape shift

<!-- splat:yaml path=splat_rule/parameters/features/shape_shift -->

```yaml
economy: bonus_action
requires:
  in_sunlight: false
  in_running_water: false
op: shape_shift
worn_equipment_transformed: true
forms:
- form: bat
  size: tiny
  speeds_feet:
    walk: 5
    fly: 30
  can_speak: false
  retain_other_statistics: true
- form: mist
  size: medium
  speeds_feet:
    walk: 5
    fly: 20
  hover: true
  actions_allowed: false
  can_speak: false
  manipulate_objects: false
  weight: 0
  enter_and_stop_in_enemy_space: true
  pass_through: spaces_air_can_pass
  pass_through_liquid: false
  damage_resistance:
    all: true
    except_sources:
    - sunlight
- form: vampire
retain_statistics_except:
- size
- speed
- listed_form_rules
```

##### Beguile

<!-- splat:yaml path=splat_rule/parameters/features/beguile -->

```yaml
economy: legendary_action
cooldown_ends:
  event: turn.start
  participant: self
  occurrence: next
op: cast_spell
spell:
  key: dnd2024:spells/command
ability: charisma
save_dc: 17
components_waived:
- verbal
- somatic
- material
cast_level: spell_minimum
```

##### Deathless strike

<!-- splat:yaml path=splat_rule/parameters/features/deathless_strike -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: move
  distance_max:
    multiply:
    - stat: speed
    - numerator: 1
      denominator: 2
- op: use_feature
  feature: grave_strike
  count: 1
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 7 ` |
| ` stealth ` | ` 9 ` |

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["necrotic"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 17 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

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
| ` count ` | ` 2 ` |
| ` chooser ` | ` gm ` |
| ` distinct_from_known ` | ` true ` |
| ` can_speak ` | ` true ` |
| ` can_understand ` | ` true ` |

#### Legendary actions

<!-- splat:yaml path=splat_rule/parameters/legendary_actions -->

```yaml
uses: 3
uses_in_lair: 4
cost_per_action: 1
trigger: another_creature.turn.end
recovery:
  event: turn.start
  participant: self
  amount: all
maximum_per_trigger: 1
requires:
  self_incapacitated: false
  self_can_take_actions: true
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

