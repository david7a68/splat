---
splat_schema: 2
splat_id: cfbe1a2b-5d03-5718-a015-b18ad52be8c0
splat_kind: creature
splat_key: dnd2024:creatures/adult_brass_dragon
splat_name: Adult Brass Dragon
splat_origin: dnd2024:reference/creature/adult_brass_dragon
splat_sources:
- SRD 5.2.1, pp. 268, 269, https://www.dndbeyond.com/srd
---

# Adult Brass Dragon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Dragon (Metallic) |
| Alignment | Chaotic Good |
| Armor Class | 18 |
| Hit Points | 172 (15d12 + 75) |
| Initiative | +10 (20) |
| Speed | 40 ft., Burrow 30 ft., Fly 80 ft. |
| Challenge | 13 (XP 10,000, or 11,500 in lair; PB +5) |
| Skills | History +7, Perception +11, Persuasion +8, Stealth +5 |
| Senses | Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 21 |
| Languages | Common, Draconic |
| Immunities | Fire |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 23 | +6 | +6 |
| Dex | 10 | +0 | +5 |
| Con | 21 | +5 | +5 |
| Int | 14 | +2 | +2 |
| Wis | 13 | +1 | +6 |
| Cha | 17 | +3 | +3 |


## Source text

<!-- SRD page 268 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Dragon (Metallic), Chaotic Good\
> AC 18 Initiative +10 (20)\
> HP 172 (15d12 + 75)\
> Speed 40 ft., Burrow 30 ft., Fly 80 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 23 +6 +6 Dex 10 +0 +5 Con 21 +5 +5\
> Int 14 +2 +2 Wis 13 +1 +6 Cha 17 +3 +3\
> Skills History +7, Perception +11, Persuasion +8, Stealth +5\
> Immunities Fire\
> Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 21\
> Languages Common, Draconic\
> CR 13 (XP 10,000, or 11,500 in lair; PB +5)


### Traits

Legendary Resistance (3/Day, or 4/Day in Lair). If the dragon fails a saving throw, it can choose to succeed instead.

### Actions

Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of (A) Sleep Breath or (B) Spellcasting to cast Scorching Ray.

Rend. Melee Attack Roll: +11, reach 10 ft. Hit: 17 (2d10 + 6) Slashing damage plus 4 (1d8) Fire damage.

Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 18, each creature in a 60-foot-long, 5-foot-wide Line. Failure: 45 (10d8) Fire damage. Success: Half damage.

Sleep Breath. Constitution Saving Throw: DC 18, each creature in a 60-foot Cone. Failure: The target has the Incapacitated condition until the end of its next turn, at which point it repeats the save. Second Failure: The target has the Unconscious condition for 10 minutes. This effect ends for the target if it takes damage or a creature within 5 feet of it takes an action to wake it.

<!-- SRD page 269 -->

Spellcasting. The dragon casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 16):

At Will: Detect Magic, Minor Illusion, Scorching Ray, Shapechange (Beast or Humanoid form only, no Temporary Hit Points gained from the spell, and no Concentration or Temporary Hit Points required to maintain the spell), Speak with Animals

1/Day Each: Detect Thoughts, Control Weather

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend a use to take one of the following actions. The dragon regains all expended uses at the start of each of its turns.

Blazing Light. The dragon uses Spellcasting to cast Scorching Ray.

Pounce. The dragon moves up to half its Speed, and it makes one Rend attack.

Scorching Sands. Dexterity Saving Throw: DC 16, one creature the dragon can see within 120 feet. Failure: 27 (6d8) Fire damage, and the target’s Speed is halved until the end of its next turn. Failure or Success: The dragon can’t take this action again until the start of its next turn.

Original layout: [[../../srd-5.2.1/pages/268|page 268]], [[../../srd-5.2.1/pages/269|page 269]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/adult_brass_dragon ` |
| ` implementation ` | ` dnd2024:creature_adult_brass_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |

#### Features

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

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: rend
  count: 3
replacements:
- replace_count: 1
  maximum_uses: 1
  operation:
    operation: choice
    options:
    - operation: use_feature
      feature: sleep_breath
    - operation: cast_spell
      spell:
        key: dnd2024:spells/scorching_ray
      cast_level: 2
      spellcasting_profile: spellcasting
```

##### Rend

<!-- splat:values path=splat_rule/parameters/features/rend -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/rend/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 11
reach_ft: 10
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/rend/hit/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Average:
    path: average
    type: integer
  Roll count:
    path: roll/count
    type: integer
  Roll sides:
    path: roll/sides
    type: integer
  Roll modifier:
    path: roll/modifier
    type: integer
  Type:
    path: type
    type: symbol
-->

| Key | Average | Roll count | Roll sides | Roll modifier | Type |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 17 ` | ` 2 ` | ` 10 ` | ` 6 ` | ` Slashing ` |
| ` row_2 ` | ` 4 ` | ` 1 ` | ` 8 ` | ` 0 ` | ` Fire ` |

##### Fire breath

<!-- splat:yaml path=splat_rule/parameters/features/fire_breath -->

```yaml
economy: action
resource:
  kind: recharge
  die: 6
  success_faces:
  - 5
  - 6
  check: self.turn_start
  rest_recovery:
  - short
  - long
  capacity: 1
  initial_uses: 1
operation: saving_throw
ability: dexterity
dc: 18
targets:
  kind: creature
  selection: each
  area:
    shape: line
    size_ft: 60
    origin: self
    width_ft: 5
failure:
  damage:
  - average: 45
    roll:
      count: 10
      sides: 8
      modifier: 0
    type: fire
success:
  damage_multiplier: 0.5
  other_effects: false
```

##### Sleep breath

<!-- splat:yaml path=splat_rule/parameters/features/sleep_breath -->

```yaml
economy: action
operation: saving_throw
ability: constitution
dc: 18
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 60
    origin: self
failure:
  conditions:
  - condition: incapacitated
    until: target.next_turn_end
  repeat_save:
    event: target.next_turn_end
    failure:
      conditions:
      - condition: unconscious
        duration_seconds: 600
      ends_on:
      - event: target.damage_taken
      - event: creature.action
        operation: wake_target
        within_ft: 5
    success: end_effect
success:
  effects: []
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` spellcasting ` |
| ` ability ` | ` charisma ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 16 ` |
| ` material_components_available ` | ` true ` |

###### Options

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/options
key: Spell
order:
- detect_magic
- minor_illusion
- scorching_ray
- shapechange
- speak_with_animals
- detect_thoughts
- control_weather
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: usage/kind
    type: symbol
  Overrides allowed types:
    path: overrides/allowed_types
    type: yaml
    optional: true
  Overrides gain temporary hp:
    path: overrides/gain_temporary_hp
    type: boolean
    optional: true
  Overrides maintain requires concentration:
    path: overrides/maintain_requires_concentration
    type: boolean
    optional: true
  Overrides maintain requires temporary hp:
    path: overrides/maintain_requires_temporary_hp
    type: boolean
    optional: true
  Uses:
    path: usage/uses
    type: integer
    optional: true
  Use pool:
    path: usage/pool
    type: symbol
    optional: true
  Recovery:
    path: usage/recovery
    type: symbol
    optional: true
-->

| Spell | Frequency | Overrides allowed types | Overrides gain temporary hp | Overrides maintain requires concentration | Overrides maintain requires temporary hp | Uses | Use pool | Recovery |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` At will ` |  |  |  |  |  |  |  |
| [Minor Illusion](<../spells/minor_illusion.md> "dnd2024:spells/minor_illusion") | ` At will ` |  |  |  |  |  |  |  |
| [Scorching Ray](<../spells/scorching_ray.md> "dnd2024:spells/scorching_ray") | ` At will ` |  |  |  |  |  |  |  |
| [Shapechange](<../spells/shapechange.md> "dnd2024:spells/shapechange") | ` At will ` | ` ["beast", "humanoid"] ` | false | false | false |  |  |  |
| [Speak with Animals](<../spells/speak_with_animals.md> "dnd2024:spells/speak_with_animals") | ` At will ` |  |  |  |  |  |  |  |
| [Detect Thoughts](<../spells/detect_thoughts.md> "dnd2024:spells/detect_thoughts") | ` Daily ` |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Control Weather](<../spells/control_weather.md> "dnd2024:spells/control_weather") | ` Daily ` |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |

##### Blazing light

<!-- splat:yaml path=splat_rule/parameters/features/blazing_light -->

```yaml
economy: legendary_action
legendary_cost: 1
operation: cast_spell
spell:
  key: dnd2024:spells/scorching_ray
cast_level: 2
spellcasting_profile: spellcasting
```

##### Pounce

<!-- splat:yaml path=splat_rule/parameters/features/pounce -->

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
- operation: attack_sequence
  feature: rend
  count: 1
```

##### Scorching sands

<!-- splat:yaml path=splat_rule/parameters/features/scorching_sands -->

```yaml
economy: legendary_action
legendary_cost: 1
cooldown_until: self.next_turn_start
operation: saving_throw
ability: dexterity
dc: 16
targets:
  kind: creature
  count: 1
  range_ft: 120
  visible: true
failure:
  damage:
  - average: 27
    roll:
      count: 6
      sides: 8
      modifier: 0
    type: fire
  speed_multiplier: 0.5
  until: target.next_turn_end
success:
  effects: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` history ` | ` 7 ` |
| ` perception ` | ` 11 ` |
| ` persuasion ` | ` 8 ` |
| ` stealth ` | ` 5 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 60 ` |
| ` darkvision ` | ` 120 ` |
| ` passive_perception ` | ` 21 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
- draconic
understands: []
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
> fly_speed:
>   unit: ft
>   value: 80
> burrow_speed:
>   unit: ft
>   value: 30
> ```

