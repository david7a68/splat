---
splat_schema: 2
splat_id: e6484aed-61b8-567b-835d-4f18c472deb2
splat_kind: creature
splat_key: dnd2024:creatures/half_dragon
splat_name: Half-Dragon
splat_origin: dnd2024:reference/creature/half_dragon
splat_sources:
- SRD 5.2.1, pp. 297, https://www.dndbeyond.com/srd
---

# Half-Dragon


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Dragon |
| Alignment | Neutral |
| Armor Class | 18 |
| Hit Points | 105 (14d8 + 42) |
| Initiative | +5 (15) |
| Speed | 40 ft. |
| Challenge | 5 (XP 1,800; PB +3) |
| Skills | Athletics +7, Perception +5, Stealth +5 |
| Senses | Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 15 |
| Languages | Common, Draconic |
| Resistances | Damage type chosen for the Draconic Origin trait below |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +4 |
| Dex | 14 | +2 | +5 |
| Con | 16 | +3 | +3 |
| Int | 10 | +0 | +0 |
| Wis | 15 | +2 | +5 |
| Cha | 14 | +2 | +2 |


## Source text

<!-- SRD page 297 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Dragon, Neutral\
> AC 18  Initiative +5 (15)\
> HP 105 (14d8 + 42)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 19 +4 +4 Dex 14 +2 +5 Con 16 +3 +3\
> Int 10 +0 +0 Wis 15 +2 +5 Cha 14 +2 +2\
> Skills Athletics +7, Perception +5, Stealth +5\
> Resistances Damage type chosen for the Draconic Origin trait below\
> Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 15\
> Languages Common, Draconic\
> CR 5 (XP 1,800; PB +3)


### Traits

Draconic Origin. The half-dragon is related to a type of dragon associated with one of the following damage types (GM’s choice): Acid, Cold, Fire, Lightning, or Poison. This choice affects other aspects of the stat block.

### Actions

Multiattack. The half-dragon makes two Claw attacks.

Claw. Melee Attack Roll: +7, reach 10 ft. Hit: 6 (1d4 + 4) Slashing damage plus 7 (2d6) damage of the type chosen for the Draconic Origin trait.

Dragon’s Breath (Recharge 5–6). Dexterity Saving Throw: DC 14, each creature in a 30-foot Cone. Failure: 28 (8d6) damage of the type chosen for the Draconic Origin trait. Success: Half damage.

### Bonus Actions

Leap. The half-dragon jumps up to 30 feet by spending 10 feet of movement.

Original layout: [[../../srd-5.2.1/pages/297|page 297]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/half_dragon ` |
| ` implementation ` | ` dnd2024:creature_half_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` athletics ` | ` 7 ` |
| ` perception ` | ` 5 ` |
| ` stealth ` | ` 5 ` |

#### Damage resistances

<!-- splat:rows path=splat_rule/parameters/damage_resistances -->

| Row | parameter |
| --- | --- |
| ` 0 ` | ` draconic_origin ` |

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
| ` row_1 ` | ` Blindsight ` | ` 10 ` |
| ` row_2 ` | ` Darkvision ` | ` 60 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common", "draconic"] ` |

#### Features

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 7
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 1
        sides: 4
      add: 4
    average: 6
  - type:
      parameter: draconic_origin
    amount:
      dice:
        count: 2
        sides: 6
      add: 0
    average: 7
```

##### Leap

<!-- splat:values path=splat_rule/parameters/features/leap -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` movement.jump ` |
| ` maximum_ft ` | ` 30 ` |
| ` movement_cost_ft ` | ` 10 ` |

##### Draconic origin

<!-- splat:values path=splat_rule/parameters/features/draconic_origin -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` choice.bind ` |
| ` parameter ` | ` draconic_origin ` |
| ` chooser ` | ` gm ` |
| ` choices ` | ` ["acid", "cold", "fire", "lightning", "poison"] ` |
| ` affects ` | ` ["damage_resistance", "claw_damage_type", "dragons_breath_damage_type"] ` |

##### Dragons breath

<!-- splat:yaml path=splat_rule/parameters/features/dragons_breath -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 14
targets:
  kind: creature
  count: all
  area:
    shape: cone
    origin: actor
    size_ft: 30
failure:
- op: damage
  components:
  - type:
      parameter: draconic_origin
    amount:
      dice:
        count: 8
        sides: 6
      add: 0
    average: 28
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

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: claw
```

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |
