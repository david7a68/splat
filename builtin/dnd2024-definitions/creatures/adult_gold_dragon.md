---
splat_schema: 2
splat_id: 9705a4e4-6d7c-5510-91bb-86a3686c0c3d
splat_kind: creature
splat_key: dnd2024:creatures/adult_gold_dragon
splat_name: Adult Gold Dragon
splat_origin: dnd2024:reference/creature/adult_gold_dragon
splat_sources:
- SRD 5.2.1, pp. 291, 292, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 145, Habitat
---

# Adult Gold Dragon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Dragon (Metallic) |
| Alignment | Lawful Good |
| Armor Class | 19 |
| Hit Points | 243 (18d12 + 126) |
| Initiative | +14 (24) |
| Speed | 40 ft., Fly 80 ft., Swim 40 ft. |
| Challenge | 17 (XP 18,000, or 20,000 in lair; PB +6) |
| Skills | Insight +8, Perception +14, Persuasion +13, Stealth +8 |
| Senses | Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 24 |
| Languages | Common, Draconic |
| Immunities | Fire |
| Habitat | Forest, Grassland |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 27 | +8 | +8 |
| Dex | 14 | +2 | +8 |
| Con | 25 | +7 | +7 |
| Int | 16 | +3 | +3 |
| Wis | 15 | +2 | +8 |
| Cha | 24 | +7 | +7 |


## Source text

<!-- SRD page 291 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Dragon (Metallic), Lawful Good\
> AC 19 Initiative +14 (24)\
> HP 243 (18d12 + 126)\
> Speed 40 ft., Fly 80 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 27 +8 +8 Dex 14 +2 +8 Con 25 +7 +7\
> Int 16 +3 +3 Wis 15 +2 +8 Cha 24 +7 +7\
> Skills Insight +8, Perception +14, Persuasion +13, Stealth +8\
> Immunities Fire\
> Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 24\
> Languages Common, Draconic\
> CR 17 (XP 18,000, or 20,000 in lair; PB +6)


### Traits

Amphibious. The dragon can breathe air and water.

Legendary Resistance (3/Day, or 4/Day in Lair). If the dragon fails a saving throw, it can choose to succeed instead.

### Actions

Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of (A) Spellcasting to cast Guiding Bolt (level 2 version) or (B) Weakening Breath.

Rend. Melee Attack Roll: +14, reach 10 ft. Hit: 17 (2d8 + 8) Slashing damage plus 4 (1d8) Fire damage.

Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 21, each creature in a 60-foot Cone. Failure: 66 (12d10) Fire damage. Success: Half damage.

Spellcasting. The dragon casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 21, +13 to hit with spell attacks):

At Will: Detect Magic, Guiding Bolt (level 2 version), Shapechange (Beast or Humanoid form only, no Temporary Hit Points gained from the spell, and no Concentration or Temporary Hit Points required to maintain the spell)

1/Day Each: Flame Strike, Zone of Truth

Weakening Breath. Strength Saving Throw: DC 21, each creature that isn’t currently affected by this breath in a 60-foot Cone. Failure: The target has Disadvantage on Strength-based D20 Tests and subtracts 3 (1d6) from its damage rolls. It repeats the save at the end of each of its turns, ending the effect on itself on a success. After 1 minute, it succeeds automatically.

<!-- SRD page 292 -->

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend a use to take one of the following actions. The dragon regains all expended uses at the start of each of its turns.

Banish. Charisma Saving Throw: DC 21, one creature the dragon can see within 120 feet. Failure: 10 (3d6) Force damage, and the target has the Incapacitated condition and is transported to a harmless demiplane until the start of the dragon’s next turn, at which point it reappears in an unoccupied space of the dragon’s choice within 120 feet of the dragon. Failure or Success: The dragon can’t take this action again until the start of its next turn.

Guiding Light. The dragon uses Spellcasting to cast Guiding Bolt (level 2 version).

Pounce. The dragon moves up to half its Speed, and it makes one Rend attack.

Original layout: [[../../srd-5.2.1/pages/291|page 291]], [[../../srd-5.2.1/pages/292|page 292]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/adult_gold_dragon ` |
| ` implementation ` | ` dnd2024:creature_adult_gold_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |

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
      feature: weakening_breath
    - operation: cast_spell
      spell:
        key: dnd2024:spells/guiding_bolt
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
supplied_attack_total: 14
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
dc: 21
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 60
    origin: self
failure:
  damage:
  - average: 66
    roll:
      count: 12
      sides: 10
      modifier: 0
    type: fire
success:
  damage_multiplier: 0.5
  other_effects: false
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` spellcasting ` |
| ` ability ` | ` charisma ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 21 ` |
| ` supplied_spell_attack_total ` | ` 13 ` |
| ` material_components_available ` | ` true ` |

###### Options

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/options
key: Spell
order:
- detect_magic
- guiding_bolt
- shapechange
- flame_strike
- zone_of_truth
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: usage/kind
    type: symbol
  Cast level:
    path: cast_level
    type: integer
    optional: true
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

| Spell | Frequency | Cast level | Overrides allowed types | Overrides gain temporary hp | Overrides maintain requires concentration | Overrides maintain requires temporary hp | Uses | Use pool | Recovery |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` At will ` |  |  |  |  |  |  |  |  |
| [Guiding Bolt](<../spells/guiding_bolt.md> "dnd2024:spells/guiding_bolt") | ` At will ` | ` 2 ` |  |  |  |  |  |  |  |
| [Shapechange](<../spells/shapechange.md> "dnd2024:spells/shapechange") | ` At will ` |  | ` ["beast", "humanoid"] ` | false | false | false |  |  |  |
| [Flame Strike](<../spells/flame_strike.md> "dnd2024:spells/flame_strike") | ` Daily ` |  |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Zone of Truth](<../spells/zone_of_truth.md> "dnd2024:spells/zone_of_truth") | ` Daily ` |  |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |

##### Weakening breath

<!-- splat:yaml path=splat_rule/parameters/features/weakening_breath -->

```yaml
economy: action
operation: saving_throw
ability: strength
dc: 21
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 60
    origin: self
  exclude_already_affected_by: this_breath
failure:
  disadvantage_on:
    roll: d20_test
    ability: strength
  damage_roll_modifier:
    subtract:
      count: 1
      sides: 6
      modifier: 0
  repeat_save:
    event: target.turn_end
    success: end_effect
  automatic_success_after_seconds: 60
success:
  effects: []
```

##### Banish

<!-- splat:yaml path=splat_rule/parameters/features/banish -->

```yaml
economy: legendary_action
legendary_cost: 1
cooldown_until: self.next_turn_start
operation: saving_throw
ability: charisma
dc: 21
targets:
  kind: creature
  count: 1
  range_ft: 120
  visible: true
failure:
  damage:
  - average: 10
    roll:
      count: 3
      sides: 6
      modifier: 0
    type: force
  conditions:
  - condition: incapacitated
    until: self.next_turn_start
  transport:
    plane: harmless_demiplane
    until: self.next_turn_start
    return:
      unoccupied: true
      within_self_ft: 120
      chooser: self
success:
  effects: []
```

##### Guiding light

<!-- splat:yaml path=splat_rule/parameters/features/guiding_light -->

```yaml
economy: legendary_action
legendary_cost: 1
operation: cast_spell
spell:
  key: dnd2024:spells/guiding_bolt
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

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` insight ` | ` 8 ` |
| ` perception ` | ` 14 ` |
| ` persuasion ` | ` 13 ` |
| ` stealth ` | ` 8 ` |

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
| ` passive_perception ` | ` 24 ` |

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
> swim_speed:
>   unit: ft
>   value: 40
> ```

