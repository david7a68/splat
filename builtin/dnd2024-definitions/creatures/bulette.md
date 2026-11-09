---
splat_schema: 2
splat_id: 463e7b55-64bc-5b8a-80ee-af2711e4d4b0
splat_kind: creature
splat_key: dnd2024:creatures/bulette
splat_name: Bulette
splat_origin: dnd2024:reference/creature/bulette
splat_sources:
- SRD 5.2.1, pp. 272, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 63, Habitat
---

# Bulette


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 17 |
| Hit Points | 94 (9d10 + 45) |
| Initiative | +0 (10) |
| Speed | 40 ft., Burrow 40 ft. |
| Challenge | 5 (XP 1,800; PB +3) |
| Skills | Perception +6 |
| Senses | Darkvision 60 ft., Tremorsense 120 ft.; Passive Perception 16 |
| Languages | None |
| Habitat | Grassland, Hill, Mountain |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +4 |
| Dex | 11 | +0 | +0 |
| Con | 21 | +5 | +5 |
| Int | 2 | -4 | -4 |
| Wis | 10 | +0 | +0 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 272 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Monstrosity, Unaligned\
> AC 17 Initiative +0 (10)\
> HP 94 (9d10 + 45)\
> Speed 40 ft., Burrow 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 19 +4 +4 Dex 11 +0 +0 Con 21 +5 +5\
> Int 2 −4 −4 Wis 10 +0 +0 Cha 5 −3 −3\
> Skills Perception +6\
> Senses Darkvision 60 ft., Tremorsense 120 ft.; Passive Perception 16\
> Languages None\
> CR 5 (XP 1,800; PB +3)


### Actions

Multiattack. The bulette makes two Bite attacks.

Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 17 (2d12 + 4) Piercing damage.

Deadly Leap. The bulette spends 5 feet of movement to jump to a space within 15 feet that contains one or more Large or smaller creatures. Dexterity Saving Throw: DC 15, each creature in the bulette’s destination space. Failure: 19 (3d12) Bludgeoning damage, and the target has the Prone condition. Success: Half damage, and the target is pushed 5 feet straight away from the bulette.

### Bonus Actions

Leap. The bulette jumps up to 30 feet by spending 10 feet of movement.

Original layout: [[../../srd-5.2.1/pages/272|page 272]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/bulette ` |
| ` implementation ` | ` dnd2024:creature_bulette ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 16 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 6 ` |

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
| ` row_1 ` | ` Darkvision ` | ` 60 ` |
| ` row_2 ` | ` Tremorsense ` | ` 120 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 7
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 12
      add: 4
    average: 17
```

##### Leap

<!-- splat:values path=splat_rule/parameters/features/leap -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` movement.jump ` |
| ` maximum_ft ` | ` 30 ` |
| ` movement_cost_ft ` | ` 10 ` |

##### Deadly leap

<!-- splat:yaml path=splat_rule/parameters/features/deadly_leap -->

```yaml
economy: action
op: sequence
steps:
- op: movement.jump
  movement_cost_ft: 5
  maximum_ft: 15
  destination_contains:
    minimum_creatures: 1
    maximum_size: large
- op: saving_throw
  ability: dexterity
  dc: 15
  targets:
    kind: creature
    count: all
    location: actor_destination_space
  failure:
  - op: damage
    components:
    - type: bludgeoning
      amount:
        dice:
          count: 3
          sides: 12
        add: 0
      average: 19
  - op: condition.apply
    condition: prone
  success:
  - op: damage
    from: failure_damage
    multiplier: 0.5
    round: down
  - op: push
    distance_ft: 5
    direction: straight_away_from_actor
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: bite
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> burrow_speed:
>   unit: ft
>   value: 40
> ```

