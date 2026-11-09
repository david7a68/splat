---
splat_schema: 2
splat_id: eb0c302e-d1ce-5d59-a29c-6ca17b77f484
splat_kind: creature
splat_key: dnd2024:creatures/ancient_blue_dragon
splat_name: Ancient Blue Dragon
splat_origin: dnd2024:reference/creature/ancient_blue_dragon
splat_sources:
- SRD 5.2.1, pp. 267, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 50, Habitat
---

# Ancient Blue Dragon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Gargantuan |
| Type | Dragon (Chromatic) |
| Alignment | Lawful Evil |
| Armor Class | 22 |
| Hit Points | 481 (26d20 + 208) |
| Initiative | +14 (24) |
| Speed | 40 ft., Burrow 40 ft., Fly 80 ft. |
| Challenge | 23 (XP 50,000, or 62,000 in lair; PB +7) |
| Skills | Perception +17, Stealth +7 |
| Senses | Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 27 |
| Languages | Common, Draconic |
| Immunities | Lightning |
| Habitat | Coastal, Desert |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 29 | +9 | +9 |
| Dex | 10 | +0 | +7 |
| Con | 27 | +8 | +8 |
| Int | 18 | +4 | +4 |
| Wis | 17 | +3 | +10 |
| Cha | 25 | +7 | +7 |


## Source text

<!-- SRD page 267 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Gargantuan Dragon (Chromatic), Lawful Evil\
> AC 22 Initiative +14 (24)\
> HP 481 (26d20 + 208)\
> Speed 40 ft., Burrow 40 ft., Fly 80 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 29 +9 +9 Dex 10 +0 +7 Con 27 +8 +8\
> Int 18 +4 +4 Wis 17 +3 +10 Cha 25 +7 +7\
> Skills Perception +17, Stealth +7\
> Immunities Lightning\
> Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 27\
> Languages Common, Draconic\
> CR 23 (XP 50,000, or 62,000 in lair; PB +7)


### Traits

Legendary Resistance (4/Day, or 5/Day in Lair). If the dragon fails a saving throw, it can choose to succeed instead.

### Actions

Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of Spellcasting to cast Shatter (level 3 version).

Rend. Melee Attack Roll: +16, reach 15 ft. Hit: 18 (2d8 + 9) Slashing damage plus 11 (2d10) Lightning damage.

Lightning Breath (Recharge 5–6). Dexterity Saving Throw: DC 23, each creature in a 120-foot-long, 10-foot-wide Line. Failure: 88 (16d10) Lightning damage. Success: Half damage.

Spellcasting. The dragon casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 22):

At Will: Detect Magic, Invisibility, Mage Hand, Shatter (level 3 version)\
1/Day Each: Scrying, Sending

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend a use to take one of the following actions. The dragon regains all expended uses at the start of each of its turns.

Cloaked Flight. The dragon uses Spellcasting to cast Invisibility on itself, and it can fly up to half its Fly Speed. The dragon can’t take this action again until the start of its next turn.

Sonic Boom. The dragon uses Spellcasting to cast Shatter (level 3 version). The dragon can’t take this action again until the start of its next turn.

Tail Swipe. The dragon makes one Rend attack.

Original layout: [[../../srd-5.2.1/pages/267|page 267]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/ancient_blue_dragon ` |
| ` implementation ` | ` dnd2024:creature_ancient_blue_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["lightning"] ` |
| ` passive_perception ` | ` 27 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 17 ` |
| ` stealth ` | ` 7 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:table
path: splat_rule/parameters/senses
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
  Range ft:
    path: range_ft
    type: integer
-->

| Key | Sense | Range ft |
| --- | --- | --- |
| ` row_1 ` | ` Blindsight ` | ` 60 ` |
| ` row_2 ` | ` Darkvision ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common", "draconic"] ` |

#### Features

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: passive
trigger: saving_throw.failed
optional: true
op: saving_throw.set_result
result: success
uses_in_lair: 5
uses:
  count: 4
  recovery: long_rest
```

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 16
target:
  kind: creature_or_object
  count: 1
reach_ft: 15
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 2
        sides: 8
      add: 9
    average: 18
  - type: lightning
    amount:
      dice:
        count: 2
        sides: 10
      add: 0
    average: 11
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` spellcasting ` |

###### Profile

<!-- splat:values path=splat_rule/parameters/features/spellcasting/profile -->

| Field | Value |
| --- | --- |
| ` ability ` | ` charisma ` |
| ` waived_components ` | ` ["material"] ` |
| ` save_dc ` | ` 22 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- detect_magic
- invisibility
- mage_hand
- shatter
- scrying
- sending
columns:
  Spell:
    path: spell
    type: reference
  Uses unlimited:
    path: uses/unlimited
    type: boolean
    optional: true
  Cast level:
    path: cast_level
    type: integer
    optional: true
  Uses count:
    path: uses/count
    type: integer
    optional: true
  Uses recovery:
    path: uses/recovery
    type: symbol
    optional: true
-->

| Spell | Uses unlimited | Cast level | Uses count | Uses recovery |
| --- | --- | --- | --- | --- |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | true |  |  |  |
| [Invisibility](<../spells/invisibility.md> "dnd2024:spells/invisibility") | true |  |  |  |
| [Mage Hand](<../spells/mage_hand.md> "dnd2024:spells/mage_hand") | true |  |  |  |
| [Shatter](<../spells/shatter.md> "dnd2024:spells/shatter") | true | ` 3 ` |  |  |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") |  |  | ` 1 ` | ` Long rest ` |
| [Sending](<../spells/sending.md> "dnd2024:spells/sending") |  |  | ` 1 ` | ` Long rest ` |

##### Lightning breath

<!-- splat:yaml path=splat_rule/parameters/features/lightning_breath -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 23
targets:
  kind: creature
  count: all
  area:
    shape: line
    origin: actor
    size_ft: 120
    width_ft: 10
failure:
- op: damage
  components:
  - type: lightning
    amount:
      dice:
        count: 16
        sides: 10
      add: 0
    average: 88
success:
- op: damage
  from: failure_damage
  multiplier: 0.5
  round: down
recharge:
  die_sides: 6
  success_min: 5
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
```

##### Cloaked flight

<!-- splat:yaml path=splat_rule/parameters/features/cloaked_flight -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: spell.cast
  spell:
    key: dnd2024:spells/invisibility
  profile: spellcasting
  target: self
- op: movement
  maximum:
    multiply:
    - 0.5
    - actor_stat: fly_speed
  optional: true
lockout_until: actor.next_turn_start
legendary_cost: 1
```

##### Sonic boom

<!-- splat:yaml path=splat_rule/parameters/features/sonic_boom -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: spell.cast
  spell:
    key: dnd2024:spells/shatter
  profile: spellcasting
  cast_level: 3
lockout_until: actor.next_turn_start
legendary_cost: 1
```

##### Tail swipe

<!-- splat:values path=splat_rule/parameters/features/tail_swipe -->

| Field | Value |
| --- | --- |
| ` economy ` | ` legendary_action ` |
| ` op ` | ` feature.use ` |
| ` feature ` | ` rend ` |
| ` legendary_cost ` | ` 1 ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 3
  feature: rend
replace_attack:
  maximum: 1
  choices:
  - op: spell.cast
    spell:
      key: dnd2024:spells/shatter
    profile: spellcasting
    cast_level: 3
```

#### Legendary actions

<!-- splat:values path=splat_rule/parameters/legendary_actions -->

| Field | Value |
| --- | --- |
| ` uses ` | ` 3 ` |
| ` uses_in_lair ` | ` 4 ` |
| ` trigger ` | ` another_creature.turn_end_immediately_after ` |
| ` cost_per_action ` | ` 1 ` |
| ` recover_all ` | ` actor.turn_start ` |
| ` maximum_per_trigger ` | ` 1 ` |
| ` requires_actions_available ` | ` true ` |
| ` excluded_conditions ` | ` ["incapacitated"] ` |

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spellcasting defaults

<!-- splat:yaml path=splat_rule/parameters/spellcasting_defaults -->

```yaml
cast_level: lowest_possible_unless_overridden
upcast_allowed: only_explicit_cast_level_overrides
required_material_components_available: true
casting_time_at_least_1_minute:
  uses_normal_time: true
  magic_action_each_turn: true
  concentration_required: true
other_components: spell_definition_unless_waived
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 80
> burrow_speed:
>   unit: ft
>   value: 40
> ```

