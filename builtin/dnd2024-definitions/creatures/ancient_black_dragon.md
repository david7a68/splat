---
splat_schema: 2
splat_id: 501f429d-aec6-542a-98c8-ffdbd2772985
splat_kind: creature
splat_key: dnd2024:creatures/ancient_black_dragon
splat_name: Ancient Black Dragon
splat_origin: dnd2024:reference/creature/ancient_black_dragon
splat_sources:
- SRD 5.2.1, pp. 265, https://www.dndbeyond.com/srd
---

# Ancient Black Dragon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Gargantuan |
| Type | Dragon (Chromatic) |
| Alignment | Chaotic Evil |
| Armor Class | 22 |
| Hit Points | 367 (21d20 + 147) |
| Initiative | +16 (26) |
| Speed | 40 ft., Fly 80 ft., Swim 40 ft. |
| Challenge | 21 (XP 33,000, or 41,000 in lair; PB +7) |
| Skills | Perception +16, Stealth +9 |
| Senses | Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 26 |
| Languages | Common, Draconic |
| Immunities | Acid |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 27 | +8 | +8 |
| Dex | 14 | +2 | +9 |
| Con | 25 | +7 | +7 |
| Int | 16 | +3 | +3 |
| Wis | 15 | +2 | +9 |
| Cha | 22 | +6 | +6 |


## Source text

<!-- SRD page 265 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Gargantuan Dragon (Chromatic), Chaotic Evil\
> AC 22  Initiative +16 (26)\
> HP 367 (21d20 + 147)\
> Speed 40 ft., Fly 80 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 27 +8 +8 Dex 14 +2 +9 Con 25 +7 +7\
> Int 16 +3 +3 Wis 15 +2 +9 Cha 22 +6 +6\
> Skills Perception +16, Stealth +9\
> Immunities Acid\
> Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 26\
> Languages Common, Draconic\
> CR 21 (XP 33,000, or 41,000 in lair; PB +7)


### Traits

Amphibious. The dragon can breathe air and water.

Legendary Resistance (4/Day, or 5/Day in Lair). If the dragon fails a saving throw, it can choose to succeed instead.

### Actions

Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of Spellcasting to cast Acid Arrow (level 4 version).

Rend. Melee Attack Roll: +15, reach 15 ft. Hit: 17 (2d8 + 8) Slashing damage plus 9 (2d8) Acid damage.

Acid Breath (Recharge 5–6). Dexterity Saving Throw: DC 22, each creature in a 90-foot-long, 10-foot-wide Line. Failure: 67 (15d8) Acid damage. Success: Half damage.

Spellcasting. The dragon casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 21, +13 to hit with spell attacks):

At Will: Acid Arrow (level 4 version), Detect Magic, Fear\
1/Day Each: Create Undead, Speak with Dead, Vitriolic Sphere (level 5 version)

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend a use to take one of the following actions. The dragon regains all expended uses at the start of each of its turns.

Cloud of Insects. Dexterity Saving Throw: DC 21, one creature the dragon can see within 120 feet. Failure: 33 (6d10) Poison damage, and the target has Disadvantage on saving throws to maintain Concentration until the end of its next turn. Failure or Success: The dragon can’t take this action again until the start of its next turn.

Frightful Presence. The dragon uses Spellcasting to cast Fear. The dragon can’t take this action again until the start of its next turn.

Pounce. The dragon moves up to half its Speed, and it makes one Rend attack.

Original layout: [[../../srd-5.2.1/pages/265|page 265]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/ancient_black_dragon ` |
| ` implementation ` | ` dnd2024:creature_ancient_black_dragon ` |

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
  capacity: 4
  capacity_in_lair: 5
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
attack_bonus: 15
targets:
  count: 1
hit:
- op: damage
  average: 17
  amount:
    dice:
      count: 2
      sides: 8
    add: 8
  damage_type: slashing
- op: damage
  average: 9
  amount:
    dice:
      count: 2
      sides: 8
    add: 0
  damage_type: acid
reach_feet: 15
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
dc: 22
targets:
  kind: creature
  count: all
  area:
    shape: line
    size_feet: 90
    origin: self
    width_feet: 10
failure:
- op: damage
  average: 67
  amount:
    dice:
      count: 15
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
    cast_level: 4
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` charisma ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 21 ` |
| ` spell_attack_bonus ` | ` 13 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/spells
key: Spell
order:
- acid_arrow
- detect_magic
- fear
- create_undead
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
| [Create Undead](<../spells/create_undead.md> "dnd2024:spells/create_undead") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-create_undead "section:create_undead") |
| [Speak with Dead](<../spells/speak_with_dead.md> "dnd2024:spells/speak_with_dead") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-speak_with_dead "section:speak_with_dead") |
| [Vitriolic Sphere](<../spells/vitriolic_sphere.md> "dnd2024:spells/vitriolic_sphere") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-vitriolic_sphere "section:vitriolic_sphere") |

###### Acid arrow

^splat-68a383d1-acid_arrow

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@acid_arrow/cast_level -->

```yaml
4
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

###### Create undead

^splat-68a383d1-create_undead

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@create_undead/cast_level -->

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
5
```

##### Cloud of insects

<!-- splat:yaml path=splat_rule/parameters/features/cloud_of_insects -->

```yaml
economy: legendary_action
op: save
ability: dexterity
dc: 21
targets:
  kind: creature
  count: 1
  range_feet: 120
  visible_to: self
failure:
- op: damage
  average: 33
  amount:
    dice:
      count: 6
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
| ` perception ` | ` 16 ` |
| ` stealth ` | ` 9 ` |

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
| ` passive_perception ` | ` 26 ` |

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

