---
splat_schema: 2
splat_id: 88e6df83-4160-5611-b731-883f728ff052
splat_kind: creature
splat_key: dnd2024:creatures/adult_black_dragon
splat_name: Adult Black Dragon
splat_origin: dnd2024:reference/creature/adult_black_dragon
splat_sources:
- SRD 5.2.1, pp. 264, https://www.dndbeyond.com/srd
---

# Adult Black Dragon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Huge |
| Type | Dragon (Chromatic) |
| Alignment | Chaotic Evil |
| Armor Class | 19 |
| Hit Points | 195 (17d12 + 85) |
| Initiative | +12 (22) |
| Speed | 40 ft., Fly 80 ft., Swim 40 ft. |
| Challenge | 14 (XP 11,500, or 13,000 in lair; PB +5) |
| Skills | Perception +11, Stealth +7 |
| Senses | Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 21 |
| Languages | Common, Draconic |
| Immunities | Acid |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 23 | +6 | +6 |
| Dex | 14 | +2 | +7 |
| Con | 21 | +5 | +5 |
| Int | 14 | +2 | +2 |
| Wis | 13 | +1 | +6 |
| Cha | 19 | +4 | +4 |


## Source text

<!-- SRD page 264 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Huge Dragon (Chromatic), Chaotic Evil\
> AC 19 Initiative +12 (22)\
> HP 195 (17d12 + 85)\
> Speed 40 ft., Fly 80 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 23 +6 +6 Dex 14 +2 +7 Con 21 +5 +5\
> Int 14 +2 +2 Wis 13 +1 +6 Cha 19 +4 +4\
> Skills Perception +11, Stealth +7\
> Immunities Acid\
> Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 21\
> Languages Common, Draconic\
> CR 14 (XP 11,500, or 13,000 in lair; PB +5)


### Traits

Amphibious. The dragon can breathe air and water.

Legendary Resistance (3/Day, or 4/Day in Lair). If the dragon fails a saving throw, it can choose to succeed instead.

### Actions

Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of Spellcasting to cast Acid Arrow (level 3 version).

Rend. Melee Attack Roll: +11, reach 10 ft. Hit: 13 (2d6 + 6) Slashing damage plus 4 (1d8) Acid damage.

Acid Breath (Recharge 5–6). Dexterity Saving Throw: DC 18, each creature in a 60-foot-long, 5-foot-wide Line. Failure: 54 (12d8) Acid damage. Success: Half damage.

Spellcasting. The dragon casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 17, +9 to hit with spell attacks):

At Will: Acid Arrow (level 3 version), Detect Magic, Fear\
1/Day Each: Speak with Dead, Vitriolic Sphere

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend a use to take one of the following actions. The dragon regains all expended uses at the start of each of its turns.

Cloud of Insects. Dexterity Saving Throw: DC 17, one creature the dragon can see within 120 feet. Failure: 22 (4d10) Poison damage, and the target has Disadvantage on saving throws to maintain Concentration until the end of its next turn. Failure or Success: The dragon can’t take this action again until the start of its next turn.

Frightful Presence. The dragon uses Spellcasting to cast Fear. The dragon can’t take this action again until the start of its next turn.

Pounce. The dragon moves up to half its Speed, and it makes one Rend attack.

Original layout: [[../../srd-5.2.1/pages/264|page 264]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/adult_black_dragon ` |
| ` implementation ` | ` dnd2024:creature_adult_black_dragon ` |

### Parameters

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` breathing_media ` | ` ["air", "water"] ` |

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

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 11
targets:
  count: 1
hit:
- op: damage
  average: 13
  amount:
    dice:
      count: 2
      sides: 6
    add: 6
  damage_type: slashing
- op: damage
  average: 4
  amount:
    dice:
      count: 1
      sides: 8
    add: 0
  damage_type: acid
reach_feet: 10
```

##### Acid breath

<!-- splat:yaml path=splat_rule/parameters/features/acid_breath -->

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
    - 5
    - 6
  cost: 1
  recovery_also:
  - short_rest
  - long_rest
op: save
ability: dexterity
dc: 18
targets:
  kind: creature
  count: all
  area:
    shape: line
    size_feet: 60
    origin: self
    width_feet: 5
failure:
- op: damage
  average: 54
  amount:
    dice:
      count: 12
      sides: 8
    add: 0
  damage_type: acid
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: rend
  count: 3
replacement:
  max_attacks: 1
  choices:
  - feature: spellcasting
    spell:
      key: dnd2024:spells/acid_arrow
    cast_level: 3
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` charisma ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 17 ` |
| ` spell_attack_bonus ` | ` 9 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/spells
key: Spell
order:
- acid_arrow
- detect_magic
- fear
- speak_with_dead
- vitriolic_sphere
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: frequency
    type: symbol
  Uses:
    path: uses
    type: integer
    optional: true
  Resource scope:
    path: resource_scope
    type: symbol
    optional: true
  Recovery:
    path: recovery
    type: symbol
    optional: true
  Details:
    type: section
    optional: true
-->

| Spell | Frequency | Uses | Resource scope | Recovery | Details |
| --- | --- | --- | --- | --- | --- |
| [Acid Arrow](<../spells/acid_arrow.md> "dnd2024:spells/acid_arrow") | ` At will ` |  |  |  | [Rules](#^splat-68a383d1-acid_arrow "section:acid_arrow") |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` At will ` |  |  |  | [Rules](#^splat-68a383d1-detect_magic "section:detect_magic") |
| [Fear](<../spells/fear.md> "dnd2024:spells/fear") | ` At will ` |  |  |  | [Rules](#^splat-68a383d1-fear "section:fear") |
| [Speak with Dead](<../spells/speak_with_dead.md> "dnd2024:spells/speak_with_dead") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-speak_with_dead "section:speak_with_dead") |
| [Vitriolic Sphere](<../spells/vitriolic_sphere.md> "dnd2024:spells/vitriolic_sphere") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-vitriolic_sphere "section:vitriolic_sphere") |

###### Acid arrow

^splat-68a383d1-acid_arrow

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@acid_arrow/cast_level -->

```yaml
3
```

###### Detect magic

^splat-68a383d1-detect_magic

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@detect_magic/cast_level -->

```yaml
spell_minimum
```

###### Fear

^splat-68a383d1-fear

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@fear/cast_level -->

```yaml
spell_minimum
```

###### Speak with dead

^splat-68a383d1-speak_with_dead

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@speak_with_dead/cast_level -->

```yaml
spell_minimum
```

###### Vitriolic sphere

^splat-68a383d1-vitriolic_sphere

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@vitriolic_sphere/cast_level -->

```yaml
spell_minimum
```

##### Cloud of insects

<!-- splat:yaml path=splat_rule/parameters/features/cloud_of_insects -->

```yaml
economy: legendary_action
op: save
ability: dexterity
dc: 17
targets:
  kind: creature
  count: 1
  range_feet: 120
  visible_to: self
failure:
- op: damage
  average: 22
  amount:
    dice:
      count: 4
      sides: 10
    add: 0
  damage_type: poison
- op: disadvantage
  roll: concentration_save
  ends:
    event: turn.end
    participant: target
    occurrence: next
success: []
cooldown_ends:
  event: turn.start
  participant: self
  occurrence: next
```

##### Frightful presence

<!-- splat:yaml path=splat_rule/parameters/features/frightful_presence -->

```yaml
economy: legendary_action
op: cast_spell
spell:
  key: dnd2024:spells/fear
via: spellcasting
cooldown_ends:
  event: turn.start
  participant: self
  occurrence: next
cast_level: spell_minimum
```

##### Pounce

<!-- splat:yaml path=splat_rule/parameters/features/pounce -->

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
  feature: rend
  count: 1
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 11 ` |
| ` stealth ` | ` 7 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["acid"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 21 ` |

##### Special

<!-- splat:table
path: splat_rule/parameters/senses/special
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Sense:
    path: sense
    type: symbol
  Range feet:
    path: range_feet
    type: integer
-->

| Key | Sense | Range feet |
| --- | --- | --- |
| ` row_1 ` | ` Blindsight ` | ` 60 ` |
| ` row_2 ` | ` Darkvision ` | ` 120 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common", "draconic"] ` |
| ` understands ` | ` ["common", "draconic"] ` |

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
> fly_speed:
>   unit: ft
>   value: 80
> swim_speed:
>   unit: ft
>   value: 40
> ```

