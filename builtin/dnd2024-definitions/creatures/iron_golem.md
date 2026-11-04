---
splat_schema: 2
splat_id: 294e6c1c-6e74-574f-a834-775f3ca30093
splat_kind: creature
splat_key: dnd2024:creatures/iron_golem
splat_name: Iron Golem
splat_origin: dnd2024:reference/creature/iron_golem
splat_sources:
- SRD 5.2.1, pp. 302, https://www.dndbeyond.com/srd
---

# Iron Golem


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Construct |
| Alignment | Unaligned |
| Armor Class | 20 |
| Hit Points | 252 (24d10 + 120) |
| Initiative | +9 (19) |
| Speed | 30 ft. |
| Challenge | 16 (XP 15,000; PB +5) |
| Senses | Darkvision 120 ft.; Passive Perception 10 |
| Languages | Understands Common plus two other languages but can’t speak |
| Immunities | Fire, Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 24 | +7 | +7 |
| Dex | 9 | -1 | -1 |
| Con | 20 | +5 | +5 |
| Int | 3 | -4 | -4 |
| Wis | 11 | +0 | +0 |
| Cha | 1 | -5 | -5 |


## Source text

<!-- SRD page 302 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Construct, Unaligned\
> AC 20 Initiative +9 (19)\
> HP 252 (24d10 + 120)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 24 +7 +7 Dex 9 −1 −1 Con 20 +5 +5\
> Int 3 −4 −4 Wis  11 +0 +0 Cha  1 −5 −5\
> Immunities Fire, Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned\
> Senses Darkvision 120 ft.; Passive Perception 10\
> Languages Understands Common plus two other languages but can’t speak\
> CR 16 (XP 15,000; PB +5)


### Traits

Fire Absorption. Whenever the golem is subjected to Fire damage, it regains a number of Hit Points equal to the Fire damage dealt.

Immutable Form. The golem can’t shape-shift.

Magic Resistance. The golem has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The golem makes two attacks, using Bladed Arm or Fiery Bolt in any combination.

Bladed Arm. Melee Attack Roll: +12, reach 10 ft. Hit: 20 (3d8 + 7) Slashing damage plus 10 (3d6) Fire damage.

Fiery Bolt. Ranged Attack Roll: +10, range 120 ft. Hit: 36 (8d8) Fire damage.

Poison Breath (Recharge 6). Constitution Saving Throw: DC 18, each creature in a 60-foot Cone. Failure: 55 (10d10) Poison damage. Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/302|page 302]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/iron_golem ` |
| ` implementation ` | ` dnd2024:creature_iron_golem ` |

### Parameters

#### Features

##### Immutable form

<!-- splat:values path=splat_rule/parameters/features/immutable_form -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` shape_shift_allowed ` | ` false ` |

##### Magic resistance

<!-- splat:yaml path=splat_rule/parameters/features/magic_resistance -->

```yaml
economy: passive
advantage:
  roll: saving_throw
  against:
  - spell
  - magical_effect
```

##### Bladed arm

<!-- splat:yaml path=splat_rule/parameters/features/bladed_arm -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 12
targets:
  count: 1
hit:
- op: damage
  average: 20
  amount:
    dice:
      count: 3
      sides: 8
    add: 7
  damage_type: slashing
- op: damage
  average: 10
  amount:
    dice:
      count: 3
      sides: 6
    add: 0
  damage_type: fire
reach_feet: 10
```

##### Fiery bolt

<!-- splat:yaml path=splat_rule/parameters/features/fiery_bolt -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 10
targets:
  count: 1
hit:
- op: damage
  average: 36
  amount:
    dice:
      count: 8
      sides: 8
    add: 0
  damage_type: fire
range_feet: 120
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
    - 6
  cost: 1
  recovery_also:
  - short_rest
  - long_rest
op: save
ability: constitution
dc: 18
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 60
    origin: self
failure:
- op: damage
  average: 55
  amount:
    dice:
      count: 10
      sides: 10
    add: 0
  damage_type: poison
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` repeat_choice ` |
| ` count ` | ` 2 ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/multiattack/choices
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Feature:
    path: feature
    type: symbol
-->

| Key | Feature |
| --- | --- |
| ` row_1 ` | ` Bladed arm ` |
| ` row_2 ` | ` Fiery bolt ` |

##### Fire absorption

<!-- splat:yaml path=splat_rule/parameters/features/fire_absorption -->

```yaml
economy: passive
trigger: damage.subjected
damage_type: fire
op: heal
amount:
  event_value: incoming_damage_of_matching_type
```

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["fire", "poison", "psychic"] ` |
| ` conditions ` | ` ["charmed", "exhaustion", "frightened", "paralyzed", "petrified", "poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` understands ` | ` ["common"] ` |
| ` can_speak ` | ` false ` |

##### Speaks

<!-- splat:yaml path=splat_rule/parameters/communication/speaks -->

```yaml
[]
```

##### Additional languages

<!-- splat:values path=splat_rule/parameters/communication/additional_languages -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` chooser ` | ` gm ` |
| ` distinct_from_known ` | ` true ` |
| ` can_speak ` | ` false ` |
| ` can_understand ` | ` true ` |

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
