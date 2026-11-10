---
splat_schema: 2
splat_id: 953cf25b-fffe-5ccd-a448-d996e6ebb148
splat_kind: creature
splat_key: dnd2024:creatures/hell_hound
splat_name: Hell Hound
splat_origin: dnd2024:reference/creature/hell_hound
splat_sources:
- SRD 5.2.1, pp. 297, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 165, Habitat
---

# Hell Hound


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Fiend |
| Alignment | Lawful Evil |
| Armor Class | 15 |
| Hit Points | 58 (9d8 + 18) |
| Initiative | +1 (11) |
| Speed | 50 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Skills | Perception +5 |
| Senses | Darkvision 60 ft.; Passive Perception 15 |
| Languages | Understands Infernal but can’t speak |
| Immunities | Fire |
| Habitat | Mountain, Underdark, Planar (Lower Planes) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 12 | +1 | +1 |
| Con | 14 | +2 | +2 |
| Int | 6 | -2 | -2 |
| Wis | 13 | +1 | +1 |
| Cha | 6 | -2 | -2 |


## Source text

<!-- SRD page 297 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Fiend, Lawful Evil\
> AC 15 Initiative +1 (11)\
> HP 58 (9d8 + 18)\
> Speed 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 12 +1 +1 Con 14 +2 +2\
> Int 6 −2 −2 Wis 13 +1 +1 Cha 6 −2 −2\
> Skills Perception +5\
> Immunities Fire\
> Senses Darkvision 60 ft.; Passive Perception 15\
> Languages Understands Infernal but can’t speak\
> CR 3 (XP 700; PB +2)


### Traits

Pack Tactics. The hound has Advantage on an attack roll against a creature if at least one of the hound’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

### Actions

Multiattack. The hound makes two Bite attacks.

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Piercing damage plus 3 (1d6) Fire damage.

Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 12, each creature in a 15-foot Cone. Failure: 17 (5d6) Fire damage. Success: Half damage.

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
| ` mechanic ` | ` dnd2024:creatures/hell_hound ` |
| ` implementation ` | ` dnd2024:creature_hell_hound ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |
| ` passive_perception ` | ` 15 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` false ` |
| ` known ` | ` ["infernal"] ` |

#### Features

##### Pack tactics

<!-- splat:yaml path=splat_rule/parameters/features/pack_tactics -->

```yaml
economy: passive
op: roll.advantage
rolls:
- attack
target_kind: creature
when:
  ally_within_target_ft: 5
  ally_excluded_conditions:
  - incapacitated
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 5
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
        count: 1
        sides: 8
      add: 3
    average: 7
  - type: fire
    amount:
      dice:
        count: 1
        sides: 6
      add: 0
    average: 3
```

##### Fire breath

<!-- splat:yaml path=splat_rule/parameters/features/fire_breath -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 12
targets:
  kind: creature
  count: all
  area:
    shape: cone
    origin: actor
    size_ft: 15
failure:
- op: damage
  components:
  - type: fire
    amount:
      dice:
        count: 5
        sides: 6
      add: 0
    average: 17
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
