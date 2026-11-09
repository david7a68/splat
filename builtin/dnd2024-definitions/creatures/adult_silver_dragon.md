---
splat_schema: 2
splat_id: 1b7e80ec-6b0f-5ff0-96da-5be3943d0c6d
splat_kind: creature
splat_key: dnd2024:creatures/adult_silver_dragon
splat_name: Adult Silver Dragon
splat_origin: dnd2024:reference/creature/adult_silver_dragon
splat_sources:
- SRD 5.2.1, pp. 324, 325, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 279, Habitat
---

# Adult Silver Dragon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Dragon (Metallic) |
| Alignment | Lawful Good |
| Armor Class | 19 |
| Hit Points | 216 (16d12 + 112) |
| Initiative | +10 (20) |
| Speed | 40 ft., Fly 80 ft. |
| Challenge | 16 (XP 15,000, or 18,000 in lair; PB +5) |
| Skills | History +8, Perception +11, Stealth +5 |
| Senses | Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 21 |
| Languages | Common, Draconic |
| Immunities | Cold |
| Habitat | Mountain, Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 27 | +8 | +8 |
| Dex | 10 | +0 | +5 |
| Con | 25 | +7 | +7 |
| Int | 16 | +3 | +3 |
| Wis | 13 | +1 | +6 |
| Cha | 22 | +6 | +6 |


## Source text

<!-- SRD page 324 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Dragon (Metallic), Lawful Good\
> AC 19  Initiative +10 (20)\
> HP 216 (16d12 + 112)\
> Speed 40 ft., Fly 80 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 27 +8 +8 Dex 10 +0 +5 Con 25 +7 +7\
> Int 16 +3 +3 Wis 13 +1 +6 Cha 22 +6 +6\
> Skills History +8, Perception +11, Stealth +5\
> Immunities Cold\
> Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 21\
> Languages Common, Draconic\
> CR 16 (XP 15,000, or 18,000 in lair; PB +5)


### Traits

Legendary Resistance (3/Day, or 4/Day in Lair). If the dragon fails a saving throw, it can choose to succeed instead.

### Actions

Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of (A) Paralyzing Breath or (B) Spellcasting to cast Ice Knife.

Rend. Melee Attack Roll: +13, reach 10 ft. Hit: 17 (2d8 + 8) Slashing damage plus 4 (1d8) Cold damage.

Cold Breath (Recharge 5–6). Constitution Saving Throw: DC 20, each creature in a 60-foot Cone. Failure: 54 (12d8) Cold damage. Success: Half damage.

Paralyzing Breath. Constitution Saving Throw: DC 20, each creature in a 60-foot Cone. First Failure: The target has the Incapacitated condition until the end of its next turn, when it repeats the save. Second Failure: The target has the Paralyzed condition, and it repeats the save at the end of each of its turns, ending the effect on itself on a success. After 1 minute, it succeeds automatically.

Spellcasting. The dragon casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 19, +11 to hit with spell attacks):

<!-- SRD page 325 -->

At Will: Detect Magic, Hold Monster, Ice Knife, Shapechange (Beast or Humanoid form only, no Temporary Hit Points gained from the spell, and no Concentration or Temporary Hit Points required to maintain the spell)

1/Day Each: Ice Storm (level 5 version), Zone of Truth

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend a use to take one of the following actions. The dragon regains all expended uses at the start of each of its turns.

Chill. The dragon uses Spellcasting to cast Hold Monster. The dragon can’t take this action again until the start of its next turn.

Cold Gale. Dexterity Saving Throw: DC 19, each creature in a 60-foot-long, 10-foot-wide Line. Failure: 14 (4d6) Cold damage, and the target is pushed up to 30 feet straight away from the dragon. Success: Half damage only. Failure or Success: The dragon can’t take this action again until the start of its next turn.

Pounce. The dragon moves up to half its Speed, and it makes one Rend attack.

Original layout: [[../../srd-5.2.1/pages/324|page 324]], [[../../srd-5.2.1/pages/325|page 325]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/adult_silver_dragon ` |
| ` implementation ` | ` dnd2024:creature_adult_silver_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["cold"] ` |

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
      feature: paralyzing_breath
    - operation: cast_spell
      spell:
        key: dnd2024:spells/ice_knife
      cast_level: 1
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
supplied_attack_total: 13
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
| ` row_1 ` | ` 17 ` | ` 2 ` | ` 8 ` | ` 8 ` | ` Slashing ` |
| ` row_2 ` | ` 4 ` | ` 1 ` | ` 8 ` | ` 0 ` | ` Cold ` |

##### Cold breath

<!-- splat:yaml path=splat_rule/parameters/features/cold_breath -->

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
ability: constitution
dc: 20
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 60
    origin: self
failure:
  damage:
  - average: 54
    roll:
      count: 12
      sides: 8
      modifier: 0
    type: cold
success:
  damage_multiplier: 0.5
  other_effects: false
```

##### Paralyzing breath

<!-- splat:yaml path=splat_rule/parameters/features/paralyzing_breath -->

```yaml
economy: action
operation: saving_throw
ability: constitution
dc: 20
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 60
    origin: self
failure:
  damage: []
  conditions:
  - condition: incapacitated
    until: target.next_turn_end
  repeat_save:
    event: target.next_turn_end
    failure:
      conditions:
      - condition: paralyzed
      repeat_save:
        event: target.turn_end
        success: end_effect
      automatic_success_after_seconds: 60
      automatic_success_clock_origin: initial_failed_save
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
| ` save_dc ` | ` 19 ` |
| ` supplied_spell_attack_total ` | ` 11 ` |
| ` material_components_available ` | ` true ` |

###### Options

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/options
key: Spell
order:
- detect_magic
- hold_monster
- ice_knife
- shapechange
- ice_storm
- zone_of_truth
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
  Cast level:
    path: cast_level
    type: integer
    optional: true
-->

| Spell | Frequency | Overrides allowed types | Overrides gain temporary hp | Overrides maintain requires concentration | Overrides maintain requires temporary hp | Uses | Use pool | Recovery | Cast level |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` At will ` |  |  |  |  |  |  |  |  |
| [Hold Monster](<../spells/hold_monster.md> "dnd2024:spells/hold_monster") | ` At will ` |  |  |  |  |  |  |  |  |
| [Ice Knife](<../spells/ice_knife.md> "dnd2024:spells/ice_knife") | ` At will ` |  |  |  |  |  |  |  |  |
| [Shapechange](<../spells/shapechange.md> "dnd2024:spells/shapechange") | ` At will ` | ` ["beast", "humanoid"] ` | false | false | false |  |  |  |  |
| [Ice Storm](<../spells/ice_storm.md> "dnd2024:spells/ice_storm") | ` Daily ` |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` | ` 5 ` |
| [Zone of Truth](<../spells/zone_of_truth.md> "dnd2024:spells/zone_of_truth") | ` Daily ` |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |  |

##### Chill

<!-- splat:yaml path=splat_rule/parameters/features/chill -->

```yaml
economy: legendary_action
legendary_cost: 1
cooldown_until: self.next_turn_start
operation: cast_spell
spell:
  key: dnd2024:spells/hold_monster
spellcasting_profile: spellcasting
```

##### Cold gale

<!-- splat:yaml path=splat_rule/parameters/features/cold_gale -->

```yaml
economy: legendary_action
legendary_cost: 1
cooldown_until: self.next_turn_start
operation: saving_throw
ability: dexterity
dc: 19
targets:
  kind: creature
  selection: each
  area:
    shape: line
    size_ft: 60
    origin: self
    width_ft: 10
failure:
  damage:
  - average: 14
    roll:
      count: 4
      sides: 6
      modifier: 0
    type: cold
  push:
    maximum_ft: 30
    direction: straight_away_from_self
success:
  damage_multiplier: 0.5
  other_effects: false
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

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` history ` | ` 8 ` |
| ` perception ` | ` 11 ` |
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
> ```

