---
splat_schema: 2
splat_id: d989aa4d-b3f7-5d11-88ff-6def51fa42e1
splat_kind: creature
splat_key: dnd2024:creatures/steam_mephit
splat_name: Steam Mephit
splat_origin: dnd2024:reference/creature/steam_mephit
splat_sources:
- SRD 5.2.1, pp. 308, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 208, Habitat
---

# Steam Mephit


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Elemental |
| Alignment | Neutral Evil |
| Armor Class | 10 |
| Hit Points | 17 (5d6) |
| Initiative | +0 (10) |
| Speed | 30 ft., Fly 30 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Skills | Stealth +2 |
| Senses | Darkvision 60 ft.; Passive Perception 10 |
| Languages | Primordial (Aquan, Ignan) |
| Immunities | Fire, Poison; Exhaustion, Poisoned |
| Habitat | Planar (Elemental Planes) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 5 | -3 | -3 |
| Dex | 11 | +0 | +0 |
| Con | 10 | +0 | +0 |
| Int | 11 | +0 | +0 |
| Wis | 10 | +0 | +0 |
| Cha | 12 | +1 | +1 |


## Source text

<!-- SRD page 308 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Elemental, Neutral Evil\
> AC 10 Initiative +0 (10)\
> HP 17 (5d6)\
> Speed 30 ft., Fly 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 5 −3 −3 Dex 11 +0 +0 Con 10 +0 +0\
> Int 11 +0 +0 Wis 10 +0 +0 Cha 12 +1 +1\
> Skills Stealth +2\
> Immunities Fire, Poison; Exhaustion, Poisoned\
> Senses Darkvision 60 ft.; Passive Perception 10\
> Languages Primordial (Aquan, Ignan)\
> CR 1/4 (XP 50; PB +2)


### Traits

Blurred Form. Attack rolls against the mephit are made with Disadvantage unless the mephit has the Incapacitated condition.

Death Burst. The mephit explodes when it dies. Dexterity Saving Throw: DC 10, each creature in a 5-foot Emanation originating from the mephit. Failure: 5 (2d4) Fire damage. Success: Half damage.

### Actions

Claw. Melee Attack Roll: +2, reach 5 ft. Hit: 2 (1d4) Slashing damage plus 2 (1d4) Fire damage.

Steam Breath (Recharge 6). Constitution Saving Throw: DC 10, each creature in a 15-foot Cone. Failure: 5 (2d4) Fire damage, and the target’s Speed decreases by 10 feet until the end of the mephit’s next turn. Success: Half damage only. Failure or Success: Being underwater doesn’t grant Resistance to this Fire damage.

Original layout: [[../../srd-5.2.1/pages/308|page 308]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/steam_mephit ` |
| ` implementation ` | ` dnd2024:creature_steam_mephit ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire", "poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "poisoned"] ` |
| ` passive_perception ` | ` 10 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 2 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["primordial"] ` |
| ` primordial_dialects ` | ` ["aquan", "ignan"] ` |

#### Features

##### Blurred form

<!-- splat:yaml path=splat_rule/parameters/features/blurred_form -->

```yaml
economy: passive
op: roll.disadvantage
rolls:
- attack_against_actor
unless:
  actor_condition: incapacitated
```

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 2
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 1
        sides: 4
      add: 0
    average: 2
  - type: fire
    amount:
      dice:
        count: 1
        sides: 4
      add: 0
    average: 2
```

##### Death burst

<!-- splat:yaml path=splat_rule/parameters/features/death_burst -->

```yaml
economy: passive
op: saving_throw
ability: dexterity
dc: 10
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    origin: actor
    size_ft: 5
failure:
- op: damage
  components:
  - type: fire
    amount:
      dice:
        count: 2
        sides: 4
      add: 0
    average: 5
success:
- op: damage
  from: failure_damage
  multiplier: 0.5
  round: down
trigger: actor.died
```

##### Steam breath

<!-- splat:yaml path=splat_rule/parameters/features/steam_breath -->

```yaml
economy: action
op: saving_throw
ability: constitution
dc: 10
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
        count: 2
        sides: 4
      add: 0
    average: 5
- op: speed.add
  amount_ft: -10
  duration:
    until: actor.next_turn_end
success:
- op: damage
  from: failure_damage
  multiplier: 0.5
  round: down
ignore_resistance_sources:
- underwater
recharge:
  die_sides: 6
  success_min: 6
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
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
> fly_speed:
>   unit: ft
>   value: 30
> ```

