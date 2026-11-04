---
splat_schema: 2
splat_id: 33a71319-c74a-5cb6-9de9-2cd913a6f3f4
splat_kind: creature
splat_key: dnd2024:creatures/kraken
splat_name: Kraken
splat_origin: dnd2024:reference/creature/kraken
splat_sources:
- SRD 5.2.1, pp. 303, https://www.dndbeyond.com/srd
---

# Kraken


## Stat block

| Stat | Value |
| --- | --- |
| Size | Gargantuan |
| Type | Monstrosity (Titan) |
| Alignment | Chaotic Evil |
| Armor Class | 18 |
| Hit Points | 481 (26d20 + 208) |
| Initiative | +14 (24) |
| Speed | 30 ft., Swim 120 ft. |
| Challenge | 23 (XP 50,000, or 62,000 in lair; PB +7) |
| Skills | History +13, Perception +11 |
| Senses | Truesight 120 ft.; Passive Perception 21 |
| Languages | Understands Abyssal, Celestial, Infernal, and Primordial but can’t speak; telepathy 120 ft. |
| Immunities | Cold, Lightning; Frightened, Grappled, Paralyzed, Restrained |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 30 | +10 | +17 |
| Dex | 11 | +0 | +7 |
| Con | 26 | +8 | +15 |
| Int | 22 | +6 | +6 |
| Wis | 18 | +4 | +11 |
| Cha | 20 | +5 | +5 |


## Source text

<!-- SRD page 303 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Gargantuan Monstrosity (Titan), Chaotic Evil\
> AC 18 Initiative +14 (24)\
> HP 481 (26d20 + 208)\
> Speed 30 ft., Swim 120 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 30 +10 +17 Dex 11 +0 +7 Con 26 +8 +15\
> Int 22 +6 +6 Wis 18 +4 +11 Cha 20 +5 +5\
> Skills History +13, Perception +11\
> Immunities Cold, Lightning; Frightened, Grappled, Paralyzed, Restrained\
> Senses Truesight 120 ft.; Passive Perception 21\
> Languages Understands Abyssal, Celestial, Infernal, and Primordial but can’t speak; telepathy 120 ft.\
> CR 23 (XP 50,000, or 62,000 in lair; PB +7)


### Traits

Amphibious. The kraken can breathe air and water.

Legendary Resistance (4/Day, or 5/Day in Lair). If the kraken fails a saving throw, it can choose to succeed instead.

Siege Monster. The kraken deals double damage to objects and structures.

### Actions

Multiattack. The kraken makes two Tentacle attacks and uses Fling, Lightning Strike, or Swallow.

Tentacle. Melee Attack Roll: +17, reach 30 ft. Hit: 24 (4d6 + 10) Bludgeoning damage. The target has the Grappled condition (escape DC 20) from one of ten tentacles, and it has the Restrained condition until the grapple ends.

Fling. The kraken throws a Large or smaller creature Grappled by it to a space it can see within 60 feet of itself that isn’t in the air. Dexterity Saving Throw: DC 25, the creature thrown and each creature in the destination space. Failure: 18 (4d8) Bludgeoning damage, and the target has the Prone condition. Success: Half damage only.

Lightning Strike. Dexterity Saving Throw: DC 23, one creature the kraken can see within 120 feet. Failure: 33 (6d10) Lightning damage. Success: Half damage.

Swallow. Dexterity Saving Throw: DC 25, one creature Grappled by the kraken (it can have up to four creatures swallowed at a time). Failure: 23 (3d8 + 10) Piercing damage. If the target is Large or smaller, it is swallowed and no longer Grappled. A swallowed creature has the Restrained condition, has Total Cover against attacks and other effects outside the kraken, and takes 24 (7d6) Acid damage at the start of each of its turns.

If the kraken takes 50 damage or more on a single turn from a creature inside it, the kraken must succeed on a DC 25 Constitution saving throw at the end of that turn or regurgitate all swallowed creatures, each of which falls in a space within 10 feet of the kraken with the Prone condition. If the kraken dies, any swallowed creature no longer has the Restrained condition and can escape from the corpse using 15 feet of movement, exiting Prone.

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the kraken can expend a use to take one of the following actions. The kraken regains all expended uses at the start of each of its turns.

Storm Bolt. The kraken uses Lightning Strike.

Toxic Ink. Constitution Saving Throw: DC 23, each creature in a 15-foot Emanation originating from the kraken while it is underwater. Failure: The target has the Blinded and Poisoned conditions until the end of the kraken’s next turn. The kraken then moves up to its Speed. Failure or Success: The kraken can’t take this action again until the start of its next turn.

Original layout: [[../../srd-5.2.1/pages/303|page 303]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/kraken ` |
| ` implementation ` | ` dnd2024:creature_kraken ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["cold", "lightning"] ` |
| ` condition_immunities ` | ` ["frightened", "grappled", "paralyzed", "restrained"] ` |

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` breathing ` | ` ["air", "water"] ` |

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: trait
resource:
  kind: daily
  uses: 4
  recovery: long_rest
  lair_uses: 5
operation: replace_save_result
trigger: self.save_failed
optional: true
result: success
```

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
  feature: tentacle
  count: 2
- operation: choice
  options:
  - operation: use_feature
    feature: fling
  - operation: use_feature
    feature: lightning_strike
  - operation: use_feature
    feature: swallow
```

##### Tentacle

<!-- splat:yaml path=splat_rule/parameters/features/tentacle -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 17
  reach_ft: 30
hit:
  damage:
  - average: 24
    roll:
      count: 4
      sides: 6
      modifier: 10
    type: bludgeoning
  grapple:
    condition: grappled
    source: self
    escape_dc: 20
    appendage_pool:
      count: 10
      type: tentacles
      cost: 1
    while_grappled:
      conditions:
      - restrained
```

##### Fling

<!-- splat:yaml path=splat_rule/parameters/features/fling -->

```yaml
economy: action
operation: saving_throw
ability: dexterity
dc: 25
targets:
  selection: thrown_creature_and_creatures_in_destination
failure:
  damage:
  - roll:
      count: 4
      sides: 8
      modifier: 0
    type: bludgeoning
  conditions:
  - condition: prone
success:
  damage_multiplier: 0.5
  other_effects: false
before_save:
  throw:
    target:
      grappled_by: self
      maximum_size: large
    destination:
      self_visible: true
      within_self_ft: 60
      in_air: false
```

##### Lightning strike

<!-- splat:yaml path=splat_rule/parameters/features/lightning_strike -->

```yaml
economy: action
operation: saving_throw
ability: dexterity
dc: 23
targets:
  kind: creature
  count: 1
  range_ft: 120
  visible: true
failure:
  damage:
  - average: 33
    roll:
      count: 6
      sides: 10
      modifier: 0
    type: lightning
success:
  damage_multiplier: 0.5
  other_effects: false
```

##### Swallow

<!-- splat:yaml path=splat_rule/parameters/features/swallow -->

```yaml
economy: action
operation: saving_throw
ability: dexterity
dc: 25
failure:
  end_grapple: true
  swallow:
    maximum_size: large
    conditions:
    - restrained
    cover_against_outside: total
    periodic_damage:
      event: target.turn_start
      damage:
      - roll:
          count: 7
          sides: 6
          modifier: 0
        type: acid
    regurgitate:
      trigger: turn_end
      if_damage_on_single_turn_from_one_inside_creature_at_least: 50
      save:
        ability: constitution
        dc: 25
        failure:
          eject: all
          within_ft: 10
          conditions:
          - prone
    on_self_death:
      remove_conditions:
      - restrained
      escape_movement_ft: 15
      exit_condition: prone
  damage:
  - roll:
      count: 3
      sides: 8
      modifier: 10
    type: piercing
  end_grapple_requires_swallowed: true
targets:
  kind: creature
  count: 1
  grappled_by: self
capacity: 4
success:
  effects: []
```

##### Storm bolt

<!-- splat:values path=splat_rule/parameters/features/storm_bolt -->

| Field | Value |
| --- | --- |
| ` economy ` | ` legendary_action ` |
| ` legendary_cost ` | ` 1 ` |
| ` operation ` | ` use_feature ` |
| ` feature ` | ` lightning_strike ` |

##### Toxic ink

<!-- splat:values path=splat_rule/parameters/features/toxic_ink -->

| Field | Value |
| --- | --- |
| ` economy ` | ` legendary_action ` |
| ` legendary_cost ` | ` 1 ` |
| ` cooldown_until ` | ` self.next_turn_start ` |
| ` operation ` | ` saving_throw ` |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 23 ` |

###### Targets

<!-- splat:yaml path=splat_rule/parameters/features/toxic_ink/targets -->

```yaml
kind: creature
selection: each
area:
  shape: emanation
  size_ft: 15
  origin: self
```

###### Failure

###### Conditions

<!-- splat:table
path: splat_rule/parameters/features/toxic_ink/failure/conditions
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
| ` row_1 ` | ` Blinded ` | ` self.next_turn_end ` |
| ` row_2 ` | ` Poisoned ` | ` self.next_turn_end ` |

###### Then move

<!-- splat:yaml path=splat_rule/parameters/features/toxic_ink/failure/then_move -->

```yaml
operation: move
maximum_ft:
  multiply:
  - stat: speed
  - 1
```

###### Requires

<!-- splat:values path=splat_rule/parameters/features/toxic_ink/requires -->

| Field | Value |
| --- | --- |
| ` self_underwater ` | ` true ` |

###### Success

<!-- splat:yaml path=splat_rule/parameters/features/toxic_ink/success -->

```yaml
effects: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` history ` | ` 13 ` |
| ` perception ` | ` 11 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` truesight ` | ` 120 ` |
| ` passive_perception ` | ` 21 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands:
- abyssal
- celestial
- infernal
- primordial
telepathy_range_ft: 120
```

#### Legendary actions

<!-- splat:yaml path=splat_rule/parameters/legendary_actions -->

```yaml
uses: 3
recovery: self.turn_start
trigger: other_creature.turn_end
cost_per_action: 1
lair_uses: 4
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
> swim_speed:
>   unit: ft
>   value: 120
> ```

