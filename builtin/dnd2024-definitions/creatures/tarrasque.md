---
splat_schema: 2
splat_id: e6888c77-0bcb-516f-a305-13de712c5bd1
splat_kind: creature
splat_key: dnd2024:creatures/tarrasque
splat_name: Tarrasque
splat_origin: dnd2024:reference/creature/tarrasque
splat_sources:
- SRD 5.2.1, pp. 331, 332, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 305, Habitat
---

# Tarrasque


## Stat block

| Stat | Value |
| --- | --- |
| Size | Gargantuan |
| Type | Monstrosity (Titan) |
| Alignment | Unaligned |
| Armor Class | 25 |
| Hit Points | 697 (34d20 + 340) |
| Initiative | +18 (28) |
| Speed | 60 ft., Burrow 40 ft., Climb 60 ft. |
| Challenge | 30 (XP 155,000; PB +9) |
| Skills | Perception +9 |
| Senses | Blindsight 120 ft.; Passive Perception 19 |
| Languages | None |
| Resistances | Bludgeoning, Piercing, Slashing |
| Immunities | Fire, Poison; Charmed, Deafened, Frightened, Paralyzed, Poisoned |
| Habitat | Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 30 | +10 | +10 |
| Dex | 11 | +0 | +9 |
| Con | 30 | +10 | +10 |
| Int | 3 | -4 | +5 |
| Wis | 11 | +0 | +9 |
| Cha | 11 | +0 | +9 |


## Source text

<!-- SRD page 331 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Gargantuan Monstrosity (Titan), Unaligned\
> AC 25 Initiative +18 (28)\
> HP 697 (34d20 + 340)\
> Speed 60 ft., Burrow 40 ft., Climb 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 30 +10 +10 Dex 11 +0 +9 Con 30 +10 +10\
> Int 3 −4 +5 Wis 11 +0 +9 Cha 11 +0 +9\
> Skills Perception +9\
> Resistances Bludgeoning, Piercing, Slashing\
> Immunities Fire, Poison; Charmed, Deafened, Frightened, Paralyzed, Poisoned\
> Senses Blindsight 120 ft.; Passive Perception 19\
> Languages None\
> CR 30 (XP 155,000; PB +9)


### Traits

Legendary Resistance (6/Day). If the tarrasque fails a saving throw, it can choose to succeed instead.

Magic Resistance. The tarrasque has Advantage on saving throws against spells and other magical effects.

Reflective Carapace. If the tarrasque is targeted by a Magic Missile spell or a spell that requires a ranged attack roll, roll 1d6. On a 1–5, the tarrasque is unaffected. On a 6, the tarrasque is unaffected and reflects the spell, turning the caster into the target.

Siege Monster. The tarrasque deals double damage to objects and structures.

### Actions

Multiattack. The tarrasque makes one Bite attack and three other attacks, using Claw or Tail in any combination.

Bite. Melee Attack Roll: +19, reach 15 ft. Hit: 36 (4d12 + 10) Piercing damage, and the target has the Grappled condition (escape DC 20). Until the grapple ends, the target has the Restrained condition and can’t teleport.

<!-- SRD page 332 -->

Claw. Melee Attack Roll: +19, reach 15 ft. Hit: 28 (4d8 + 10) Slashing damage.

Tail. Melee Attack Roll: +19, reach 30 ft. Hit: 23 (3d8 + 10) Bludgeoning damage. If the target is a Huge or smaller creature, it has the Prone condition.

Thunderous Bellow (Recharge 5–6). Constitution Saving Throw: DC 27, each creature and each object that isn’t being worn or carried in a 150-foot Cone. Failure: 78 (12d12) Thunder damage, and the target has the Deafened and Frightened conditions until the end of its next turn. Success: Half damage only.

### Bonus Actions

Swallow. Strength Saving Throw: DC 27, one Large or smaller creature Grappled by the tarrasque (it can have up to six creatures swallowed at a time). Failure: The target is swallowed, and the Grappled condition ends. A swallowed creature has the Blinded and Restrained conditions and can’t teleport, it has Total Cover against attacks and other effects outside the tarrasque, and it takes 56 (16d6) Acid damage at the start of each of the tarrasque’s turns.

If the tarrasque takes 60 damage or more on a single turn from a creature inside it, the tarrasque must succeed on a DC 20 Constitution saving throw at the end of that turn or regurgitate all swallowed creatures, each of which falls in a space within 10 feet of the tarrasque and has the Prone condition. If the tarrasque dies, any swallowed creature no longer has the Restrained condition and can escape from the corpse using 20 feet of movement, exiting Prone.

### Legendary Actions

Legendary Action Uses: 3. Immediately after another creature’s turn, the tarrasque can expend a use to take one of the following actions. The tarrasque regains all expended uses at the start of each of its turns.

Onslaught. The tarrasque moves up to half its Speed, and it makes one Claw or Tail attack.

World-Shaking Movement. The tarrasque moves up to its Speed. At the end of this movement, the tarrasque creates an instantaneous shock wave in a 60-foot Emanation originating from itself. Creatures in that area lose Concentration and, if Medium or smaller, have the Prone condition. The tarrasque can’t take this action again until the start of its next turn.

Original layout: [[../../srd-5.2.1/pages/331|page 331]], [[../../srd-5.2.1/pages/332|page 332]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/tarrasque ` |
| ` implementation ` | ` dnd2024:creature_tarrasque ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["bludgeoning", "piercing", "slashing"] ` |
| ` damage_immunities ` | ` ["fire", "poison"] ` |
| ` condition_immunities ` | ` ["charmed", "deafened", "frightened", "paralyzed", "poisoned"] ` |

#### Features

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: trait
resource:
  kind: daily
  uses: 6
  recovery: long_rest
operation: replace_save_result
trigger: self.save_failed
optional: true
result: success
```

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` saving_throw_advantage_against ` | ` ["spells", "magical_effects"] ` |

##### Reflective carapace

<!-- splat:values path=splat_rule/parameters/features/reflective_carapace -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` trigger ` | ` self.targeted_by_spell ` |

###### Spell eligibility

###### Any of

<!-- splat:table
path: splat_rule/parameters/features/reflective_carapace/spell_eligibility/any_of
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Spell:
    path: spell
    type: reference
    optional: true
  Requires ranged attack roll:
    path: requires_ranged_attack_roll
    type: boolean
    optional: true
-->

| Key | Spell | Requires ranged attack roll |
| --- | --- | --- |
| ` row_1 ` | [Magic Missile](<../spells/magic_missile.md> "dnd2024:spells/magic_missile") |  |
| ` row_2 ` |  | true |

###### Roll

<!-- splat:values path=splat_rule/parameters/features/reflective_carapace/roll -->

| Field | Value |
| --- | --- |
| ` die ` | ` 6 ` |

###### Results

<!-- splat:table
path: splat_rule/parameters/features/reflective_carapace/roll/results
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Faces:
    path: faces
    type: yaml
  Self affected:
    path: self_affected
    type: boolean
  Reflect to:
    path: reflect_to
    type: symbol
    optional: true
-->

| Key | Faces | Self affected | Reflect to |
| --- | --- | --- | --- |
| ` row_1 ` | ` [1, 2, 3, 4, 5] ` | false |  |
| ` row_2 ` | ` [6] ` | false | ` Caster ` |

##### Siege monster

<!-- splat:values path=splat_rule/parameters/features/siege_monster -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` damage_multiplier ` | ` 2 ` |
| ` target_kinds ` | ` ["object", "structure"] ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: bite
  count: 1
- operation: attack_choice
  count: 3
  options:
  - claw
  - tail
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 19
  reach_ft: 15
hit:
  damage:
  - average: 36
    roll:
      count: 4
      sides: 12
      modifier: 10
    type: piercing
  grapple:
    condition: grappled
    source: self
    escape_dc: 20
    while_grappled:
      conditions:
      - restrained
      can_teleport: false
```

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 19
  reach_ft: 15
hit:
  damage:
  - average: 28
    roll:
      count: 4
      sides: 8
      modifier: 10
    type: slashing
```

##### Tail

<!-- splat:yaml path=splat_rule/parameters/features/tail -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 19
  reach_ft: 30
hit:
  damage:
  - average: 23
    roll:
      count: 3
      sides: 8
      modifier: 10
    type: bludgeoning
  prone:
    condition: prone
    maximum_size: huge
```

##### Thunderous bellow

<!-- splat:values path=splat_rule/parameters/features/thunderous_bellow -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` saving_throw ` |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 27 ` |

###### Resource

<!-- splat:values path=splat_rule/parameters/features/thunderous_bellow/resource -->

| Field | Value |
| --- | --- |
| ` kind ` | ` recharge ` |
| ` die ` | ` 6 ` |
| ` success_faces ` | ` [5, 6] ` |
| ` check ` | ` self.turn_start ` |
| ` rest_recovery ` | ` ["short", "long"] ` |
| ` capacity ` | ` 1 ` |
| ` initial_uses ` | ` 1 ` |

###### Targets

<!-- splat:yaml path=splat_rule/parameters/features/thunderous_bellow/targets -->

```yaml
kinds:
- creature
- object
objects_exclude:
- worn
- carried
selection: each
area:
  shape: cone
  size_ft: 150
  origin: self
```

###### Failure

###### Damage

<!-- splat:rows path=splat_rule/parameters/features/thunderous_bellow/failure/damage -->

| Row | average | type |
| --- | --- | --- |
| ` 0 ` | ` 78 ` | ` thunder ` |

###### Roll

<!-- splat:values path=splat_rule/parameters/features/thunderous_bellow/failure/damage/0/roll -->

| Field | Value |
| --- | --- |
| ` count ` | ` 12 ` |
| ` sides ` | ` 12 ` |
| ` modifier ` | ` 0 ` |

###### Conditions

<!-- splat:table
path: splat_rule/parameters/features/thunderous_bellow/failure/conditions
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Condition:
    path: condition
    type: symbol
  Until:
    path: until
    type: string
-->

| Key | Condition | Until |
| --- | --- | --- |
| ` row_1 ` | ` Deafened ` | ` target.next_turn_end ` |
| ` row_2 ` | ` Frightened ` | ` target.next_turn_end ` |

###### Success

<!-- splat:values path=splat_rule/parameters/features/thunderous_bellow/success -->

| Field | Value |
| --- | --- |
| ` damage_multiplier ` | ` 0.5 ` |
| ` other_effects ` | ` false ` |

##### Swallow

<!-- splat:yaml path=splat_rule/parameters/features/swallow -->

```yaml
economy: bonus_action
operation: saving_throw
ability: strength
dc: 27
failure:
  end_grapple: true
  swallow:
    maximum_size: large
    conditions:
    - restrained
    - blinded
    cover_against_outside: total
    periodic_damage:
      event: self.turn_start
      damage:
      - roll:
          count: 16
          sides: 6
          modifier: 0
        type: acid
    regurgitate:
      trigger: turn_end
      if_damage_on_single_turn_from_one_inside_creature_at_least: 60
      save:
        ability: constitution
        dc: 20
        failure:
          eject: all
          within_ft: 10
          conditions:
          - prone
    on_self_death:
      remove_conditions:
      - restrained
      escape_movement_ft: 20
      exit_condition: prone
    can_teleport: false
targets:
  kind: creature
  count: 1
  grappled_by: self
  maximum_size: large
capacity: 6
success:
  effects: []
```

##### Onslaught

<!-- splat:yaml path=splat_rule/parameters/features/onslaught -->

```yaml
economy: legendary_action
legendary_cost: 1
operation: sequence
effects:
- operation: move
  maximum_ft:
    multiply:
    - stat: speed
    - 0.5
- operation: attack_choice
  count: 1
  options:
  - claw
  - tail
```

##### World shaking movement

<!-- splat:yaml path=splat_rule/parameters/features/world_shaking_movement -->

```yaml
economy: legendary_action
legendary_cost: 1
cooldown_until: self.next_turn_start
operation: sequence
effects:
- operation: move
  maximum_ft:
    multiply:
    - stat: speed
    - 1
- operation: shock_wave
  area:
    shape: emanation
    size_ft: 60
    origin: self
  duration: instantaneous
  targets: creatures
  end_concentration: true
  prone:
    maximum_size: medium
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 9 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 120 ` |
| ` passive_perception ` | ` 19 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands: []
```

#### Legendary actions

<!-- splat:yaml path=splat_rule/parameters/legendary_actions -->

```yaml
uses: 3
recovery: self.turn_start
trigger: other_creature.turn_end
cost_per_action: 1
requires:
  not_incapacitated: true
maximum_actions_per_trigger: 1
```

#### Resolution conventions

<!-- splat:yaml path=splat_rule/parameters/resolution_conventions -->

```yaml
distance_unit: foot
duration_unit: second
damage_roll_mode:
  choice:
  - listed_average
  - roll_dice
  chooser: gm
supplied_totals: before_additional_effect_modifiers
multiattack_nested_economy_cost: 0
nested_feature_resource_cost: normal
spell_level_unless_specified: lowest_possible
spell_upcast_without_permission: false
spell_casting_time_at_least_one_minute: normal_casting_procedure
geometry:
  input: adjudication
  assumptions:
  - range_and_visibility_verified
  - area_membership_verified
  - path_and_destination_legal
unlisted_success_branch: no_effect
unlisted_duration: until_removed_by_applicable_rule
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> climb_speed:
>   unit: ft
>   value: 60
> burrow_speed:
>   unit: ft
>   value: 40
> ```

