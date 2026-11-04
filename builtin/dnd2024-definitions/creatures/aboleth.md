---
splat_schema: 2
splat_id: 8dc91c9b-4e6d-51e8-97a0-6f23bafeb1ef
splat_kind: creature
splat_key: dnd2024:creatures/aboleth
splat_name: Aboleth
splat_origin: dnd2024:reference/creature/aboleth
splat_sources:
- SRD 5.2.1, pp. 258, https://www.dndbeyond.com/srd
---

# Aboleth


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Aberration |
| Alignment | Lawful Evil |
| Armor Class | 17 |
| Hit Points | 150 (20d10 + 40) |
| Initiative | +7 (17) |
| Speed | 10 ft., Swim 40 ft. |
| Challenge | 10 (XP 5,900, or 7,200 in lair; PB +4) |
| Skills | History +12, Perception +10 |
| Senses | Darkvision 120 ft.; Passive Perception 20 |
| Languages | Deep Speech; telepathy 120 ft. |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 21 | +5 | +5 |
| Dex | 9 | -1 | +3 |
| Con | 15 | +2 | +6 |
| Int | 18 | +4 | +8 |
| Wis | 15 | +2 | +6 |
| Cha | 18 | +4 | +4 |


## Source text

<!-- SRD page 258 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Aberration, Lawful Evil\
> AC 17  Initiative +7 (17)\
> HP 150 (20d10 + 40)\
> Speed 10 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 21 +5 +5 Dex 9 −1 +3 Con 15 +2 +6\
> Int 18 +4 +8 Wis 15 +2 +6 Cha 18 +4 +4\
> Skills History +12, Perception +10\
> Senses Darkvision 120 ft.; Passive Perception 20\
> Languages Deep Speech; telepathy 120 ft.\
> CR 10 (XP 5,900, or 7,200 in lair; PB +4)


### Traits

Amphibious. The aboleth can breathe air and water.

Eldritch Restoration. If destroyed, the aboleth gains a new body in 5d10 days, reviving with all its Hit Points in the Far Realm or another location chosen by the GM.

Legendary Resistance (3/Day, or 4/Day in Lair). If the aboleth fails a saving throw, it can choose to succeed instead.

Mucus Cloud. While underwater, the aboleth is surrounded by mucus. Constitution Saving Throw: DC 14, each creature in a 5-foot Emanation originating from the aboleth at the end of the aboleth’s turn. Failure: The target is cursed. Until the curse ends, the target’s skin becomes slimy, the target can breathe air and water, and it can’t regain Hit Points unless it is underwater.

While the cursed creature is outside a body of water, the creature takes 6 (1d12) Acid damage at the end of every 10 minutes unless moisture is applied to its skin before those minutes have passed.

Probing Telepathy. If a creature the aboleth can see communicates telepathically with the aboleth, the aboleth learns the creature’s greatest desires.

### Actions

Multiattack. The aboleth makes two Tentacle attacks and uses either Consume Memories or Dominate Mind if available.

Tentacle. Melee Attack Roll: +9, reach 15 ft. Hit: 12 (2d6 + 5) Bludgeoning damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 14) from one of four tentacles.

Consume Memories. Intelligence Saving Throw: DC 16, one creature within 30 feet that is Charmed or Grappled by the aboleth. Failure: 10 (3d6) Psychic damage. Success: Half damage. Failure or Success: The aboleth gains the target’s memories if the target is a Humanoid and is reduced to 0 Hit Points by this action.

Dominate Mind (2/Day). Wisdom Saving Throw: DC 16, one creature the aboleth can see within 30 feet. Failure: The target has the Charmed condition until the aboleth dies or is on a different plane of existence from the target. While Charmed, the target acts as an ally to the aboleth and is under its control while within 60 feet of it. In addition, the aboleth and the target can communicate telepathically with each other over any distance.

The target repeats the save whenever it takes damage as well as after every 24 hours it spends at least 1 mile away from the aboleth, ending the effect on itself on a success.

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the aboleth can expend a use to take one of the following actions. The aboleth regains all expended uses at the start of each of its turns.

Lash. The aboleth makes one Tentacle attack.

Psychic Drain. If the aboleth has at least one creature Charmed or Grappled, it uses Consume Memories and regains 5 (1d10) Hit Points.

Original layout: [[../../srd-5.2.1/pages/258|page 258]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/aboleth ` |
| ` implementation ` | ` dnd2024:creature_aboleth ` |

### Parameters

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` breathing ` | ` ["air", "water"] ` |

##### Eldritch restoration

<!-- splat:yaml path=splat_rule/parameters/features/eldritch_restoration -->

```yaml
economy: trait
operation: passive_rule
trigger: self.destroyed
new_body:
  delay:
    roll:
      count: 5
      sides: 10
      modifier: 0
    unit: day
  hp: maximum
  location:
    choice:
    - far_realm
    - gm_selected_location
    chooser: gm
```

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: trait
resource:
  kind: daily
  uses: 3
  recovery: long_rest
  lair_uses: 4
operation: replace_save_result
trigger: self.save_failed
optional: true
result: success
```

##### Mucus cloud

<!-- splat:yaml path=splat_rule/parameters/features/mucus_cloud -->

```yaml
economy: trait
operation: saving_throw
ability: constitution
dc: 14
targets:
  kind: creature
  selection: each
  area:
    shape: emanation
    size_ft: 5
    origin: self
failure:
  curse:
    skin: slimy
    breathing:
    - air
    - water
    can_regain_hp_if:
      underwater: true
    dry_damage:
      interval_seconds: 600
      while:
        outside_body_of_water: true
      unless:
        skin_moistened_during_interval: true
      damage:
      - roll:
          count: 1
          sides: 12
          modifier: 0
        type: acid
trigger: self.turn_end
requires:
  self_underwater: true
success:
  effects: []
```

##### Probing telepathy

<!-- splat:yaml path=splat_rule/parameters/features/probing_telepathy -->

```yaml
economy: trait
operation: passive_rule
trigger: telepathic_message_received
requires:
  sender_visible: true
learn:
- sender.greatest_desires
```

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
    feature: consume_memories
  - operation: use_feature
    feature: dominate_mind
  require_available: true
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
  supplied_attack_total: 9
  reach_ft: 15
hit:
  damage:
  - average: 12
    roll:
      count: 2
      sides: 6
      modifier: 5
    type: bludgeoning
  grapple:
    condition: grappled
    source: self
    escape_dc: 14
    maximum_size: large
    appendage_pool:
      count: 4
      type: tentacles
      cost: 1
```

##### Consume memories

<!-- splat:values path=splat_rule/parameters/features/consume_memories -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` saving_throw ` |
| ` ability ` | ` intelligence ` |
| ` dc ` | ` 16 ` |

###### Targets

<!-- splat:values path=splat_rule/parameters/features/consume_memories/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` range_ft ` | ` 30 ` |

###### Any of

<!-- splat:table
path: splat_rule/parameters/features/consume_memories/targets/any_of
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Charmed by:
    path: charmed_by
    type: symbol
    optional: true
  Grappled by:
    path: grappled_by
    type: symbol
    optional: true
-->

| Key | Charmed by | Grappled by |
| --- | --- | --- |
| ` row_1 ` | ` Self ` |  |
| ` row_2 ` |  | ` Self ` |

###### Failure

<!-- splat:yaml path=splat_rule/parameters/features/consume_memories/failure -->

```yaml
damage:
- average: 10
  roll:
    count: 3
    sides: 6
    modifier: 0
  type: psychic
```

###### Success

<!-- splat:values path=splat_rule/parameters/features/consume_memories/success -->

| Field | Value |
| --- | --- |
| ` damage_multiplier ` | ` 0.5 ` |
| ` other_effects ` | ` false ` |

###### After resolution

<!-- splat:yaml path=splat_rule/parameters/features/consume_memories/after_resolution -->

```yaml
if:
  target_type: humanoid
  target_reduced_to_zero_hp_by_this_action: true
learn: target.memories
```

##### Dominate mind

<!-- splat:yaml path=splat_rule/parameters/features/dominate_mind -->

```yaml
economy: action
resource:
  kind: daily
  uses: 2
  recovery: long_rest
operation: saving_throw
ability: wisdom
dc: 16
targets:
  kind: creature
  count: 1
  range_ft: 30
  visible: true
failure:
  conditions:
  - condition: charmed
  ends_on:
  - self.dies
  - self_and_target_on_different_planes
  attitude: ally_of_self
  control:
    controller: self
    within_ft: 60
  telepathy:
    with: self
    range: unlimited
  repeat_save:
    events:
    - target.damage_taken
    - event: elapsed_time
      seconds: 86400
      while_distance_at_least_miles: 1
    success: end_effect
success:
  effects: []
```

##### Lash

<!-- splat:values path=splat_rule/parameters/features/lash -->

| Field | Value |
| --- | --- |
| ` economy ` | ` legendary_action ` |
| ` legendary_cost ` | ` 1 ` |
| ` operation ` | ` attack_sequence ` |
| ` feature ` | ` tentacle ` |
| ` count ` | ` 1 ` |

##### Psychic drain

<!-- splat:values path=splat_rule/parameters/features/psychic_drain -->

| Field | Value |
| --- | --- |
| ` economy ` | ` legendary_action ` |
| ` legendary_cost ` | ` 1 ` |
| ` operation ` | ` sequence ` |

###### Requires

###### Any creature

###### Any of

<!-- splat:table
path: splat_rule/parameters/features/psychic_drain/requires/any_creature/any_of
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Charmed by:
    path: charmed_by
    type: symbol
    optional: true
  Grappled by:
    path: grappled_by
    type: symbol
    optional: true
-->

| Key | Charmed by | Grappled by |
| --- | --- | --- |
| ` row_1 ` | ` Self ` |  |
| ` row_2 ` |  | ` Self ` |

###### Effects

<!-- splat:rows path=splat_rule/parameters/features/psychic_drain/effects -->

| Row | operation | feature | target |
| --- | --- | --- | --- |
| ` 0 ` | ` use_feature ` | ` consume_memories ` |  |
| ` 1 ` | ` heal ` |  | ` self ` |

###### Roll

<!-- splat:values path=splat_rule/parameters/features/psychic_drain/effects/1/roll -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sides ` | ` 10 ` |
| ` modifier ` | ` 0 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` history ` | ` 12 ` |
| ` perception ` | ` 10 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 120 ` |
| ` passive_perception ` | ` 20 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- deep_speech
understands: []
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
>   value: 40
> ```

