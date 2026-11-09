---
splat_schema: 2
splat_id: d72a9024-caed-5593-870f-acc73edd73e4
splat_kind: creature
splat_key: dnd2024:creatures/ancient_green_dragon
splat_name: Ancient Green Dragon
splat_origin: dnd2024:reference/creature/ancient_green_dragon
splat_sources:
- SRD 5.2.1, pp. 294, 295, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 154, Habitat
---

# Ancient Green Dragon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Gargantuan |
| Type | Dragon (Chromatic) |
| Alignment | Lawful Evil |
| Armor Class | 21 |
| Hit Points | 402 (23d20 + 161) |
| Initiative | +15 (25) |
| Speed | 40 ft., Fly 80 ft., Swim 40 ft. |
| Challenge | 22 (XP 41,000, or 50,000 in lair; PB +7) |
| Skills | Deception +13, Perception +17, Persuasion +13, Stealth +8 |
| Senses | Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 27 |
| Languages | Common, Draconic |
| Immunities | Poison; Poisoned |
| Habitat | Forest |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 27 | +8 | +8 |
| Dex | 12 | +1 | +8 |
| Con | 25 | +7 | +7 |
| Int | 20 | +5 | +5 |
| Wis | 17 | +3 | +10 |
| Cha | 22 | +6 | +6 |


## Source text

<!-- SRD page 294 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Gargantuan Dragon (Chromatic), Lawful Evil\
> AC 21 Initiative +15 (25)\
> HP 402 (23d20 + 161)\
> Speed 40 ft., Fly 80 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 27 +8 +8 Dex 12 +1 +8 Con 25 +7 +7\
> Int 20 +5 +5 Wis 17 +3 +10 Cha 22 +6 +6\
> Skills Deception +13, Perception +17, Persuasion +13, Stealth +8\
> Immunities Poison; Poisoned\
> Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 27\
> Languages Common, Draconic\
> CR 22 (XP 41,000, or 50,000 in lair; PB +7)


### Traits

Amphibious. The dragon can breathe air and water.

Legendary Resistance (4/Day, or 5/Day in Lair). If the dragon fails a saving throw, it can choose to succeed instead.

### Actions

Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of Spellcasting to cast Mind Spike (level 5 version).

Rend. Melee Attack Roll: +15, reach 15 ft. Hit: 17 (2d8 + 8) Slashing damage plus 10 (3d6) Poison damage.

Poison Breath (Recharge 5–6). Constitution Saving Throw: DC 22, each creature in a 90-foot Cone. Failure: 77 (22d6) Poison damage. Success: Half damage.

Spellcasting. The dragon casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 21):

At Will: Detect Magic, Mind Spike (level 5 version)\
1/Day Each: Geas, Modify Memory

<!-- SRD page 295 -->

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend a use to take one of the following actions. The dragon regains all expended uses at the start of each of its turns.

Mind Invasion. The dragon uses Spellcasting to cast Mind Spike (level 5 version).

Noxious Miasma. Constitution Saving Throw: DC 21, each creature in a 30-foot-radius Sphere centered on a point the dragon can see within 90 feet. Failure: 17 (5d6) Poison damage, and the target takes a −2 penalty to AC until the end of its next turn. Failure or Success: The dragon can’t take this action again until the start of its next turn.

Pounce. The dragon moves up to half its Speed, and it makes one Rend attack.

Original layout: [[../../srd-5.2.1/pages/294|page 294]], [[../../srd-5.2.1/pages/295|page 295]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/ancient_green_dragon ` |
| ` implementation ` | ` dnd2024:creature_ancient_green_dragon ` |

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
  average: 10
  amount:
    dice:
      count: 3
      sides: 6
    add: 0
  damage_type: poison
reach_feet: 15
```

##### Poison breath

<!-- splat:yaml path=splat_rule/parameters/features/poison_breath -->

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
ability: constitution
dc: 22
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 90
    origin: self
failure:
- op: damage
  average: 77
  amount:
    dice:
      count: 22
      sides: 6
    add: 0
  damage_type: poison
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
      key: dnd2024:spells/mind_spike
    cast_level: 5
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

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/spells
key: Spell
order:
- detect_magic
- mind_spike
- geas
- modify_memory
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
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` At will ` |  |  |  | [Rules](#^splat-68a383d1-detect_magic "section:detect_magic") |
| [Mind Spike](<../spells/mind_spike.md> "dnd2024:spells/mind_spike") | ` At will ` |  |  |  | [Rules](#^splat-68a383d1-mind_spike "section:mind_spike") |
| [Geas](<../spells/geas.md> "dnd2024:spells/geas") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-geas "section:geas") |
| [Modify Memory](<../spells/modify_memory.md> "dnd2024:spells/modify_memory") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-modify_memory "section:modify_memory") |

###### Detect magic

^splat-68a383d1-detect_magic

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@detect_magic/cast_level -->

```yaml
spell_minimum
```

###### Mind spike

^splat-68a383d1-mind_spike

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@mind_spike/cast_level -->

```yaml
5
```

###### Geas

^splat-68a383d1-geas

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@geas/cast_level -->

```yaml
spell_minimum
```

###### Modify memory

^splat-68a383d1-modify_memory

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@modify_memory/cast_level -->

```yaml
spell_minimum
```

##### Noxious miasma

<!-- splat:yaml path=splat_rule/parameters/features/noxious_miasma -->

```yaml
economy: legendary_action
op: save
ability: constitution
dc: 21
targets:
  kind: creature
  count: all
  area:
    shape: sphere
    size_feet: 30
    origin: chosen_point
    origin_visible_to: self
    origin_range_feet: 90
failure:
- op: damage
  average: 17
  amount:
    dice:
      count: 5
      sides: 6
    add: 0
  damage_type: poison
- op: modify_ac
  add: -2
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

##### Mind invasion

<!-- splat:yaml path=splat_rule/parameters/features/mind_invasion -->

```yaml
economy: legendary_action
op: cast_spell
spell:
  key: dnd2024:spells/mind_spike
via: spellcasting
cast_level: 5
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
| ` deception ` | ` 13 ` |
| ` perception ` | ` 17 ` |
| ` persuasion ` | ` 13 ` |
| ` stealth ` | ` 8 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["poison"] ` |
| ` conditions ` | ` ["poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 27 ` |

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

