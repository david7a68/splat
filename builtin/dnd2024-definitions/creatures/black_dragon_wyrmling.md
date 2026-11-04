---
splat_schema: 2
splat_id: d84fe7b2-40c2-553a-b531-dacc33a2079d
splat_kind: creature
splat_key: dnd2024:creatures/black_dragon_wyrmling
splat_name: Black Dragon Wyrmling
splat_origin: dnd2024:reference/creature/black_dragon_wyrmling
splat_sources:
- SRD 5.2.1, pp. 263, 264, https://www.dndbeyond.com/srd
---

# Black Dragon Wyrmling


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Dragon (Chromatic) |
| Alignment | Chaotic Evil |
| Armor Class | 17 |
| Hit Points | 33 (6d8 + 6) |
| Initiative | +4 (14) |
| Speed | 30 ft., Fly 60 ft., Swim 30 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Perception +4, Stealth +4 |
| Senses | Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 |
| Languages | Draconic |
| Immunities | Acid |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +2 |
| Dex | 14 | +2 | +4 |
| Con | 13 | +1 | +1 |
| Int | 10 | +0 | +0 |
| Wis | 11 | +0 | +2 |
| Cha | 13 | +1 | +1 |


## Source text

<!-- SRD page 263 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Dragon (Chromatic), Chaotic Evil\
> AC 17 Initiative +4 (14)\
> HP 33 (6d8 + 6)\
> Speed 30 ft., Fly 60 ft., Swim 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 15 +2 +2 Dex 14 +2 +4 Con 13 +1 +1\
> Int 10 +0 +0 Wis 11 +0 +2 Cha 13 +1 +1\
> Skills Perception +4, Stealth +4\
> Immunities Acid\
> Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14\
> Languages Draconic\
> CR 2 (XP 450; PB +2)


<!-- SRD page 264 -->

### Traits

Amphibious. The dragon can breathe air and water.

### Actions

Multiattack. The dragon makes two Rend attacks.

Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Slashing damage plus 2 (1d4) Acid damage.

Acid Breath (Recharge 5–6). Dexterity Saving Throw: DC 11, each creature in a 15-foot-long, 5-foot-wide Line. Failure: 22 (5d8) Acid damage. Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/263|page 263]], [[../../srd-5.2.1/pages/264|page 264]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/black_dragon_wyrmling ` |
| ` implementation ` | ` dnd2024:creature_black_dragon_wyrmling ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["acid"] ` |
| ` passive_perception ` | ` 14 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |
| ` stealth ` | ` 4 ` |

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
| ` row_1 ` | ` Blindsight ` | ` 10 ` |
| ` row_2 ` | ` Darkvision ` | ` 60 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["draconic"] ` |

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` breathing.allow ` |
| ` media ` | ` ["air", "water"] ` |

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 4
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
        sides: 6
      add: 2
    average: 5
  - type: acid
    amount:
      dice:
        count: 1
        sides: 4
      add: 0
    average: 2
```

##### Acid breath

<!-- splat:yaml path=splat_rule/parameters/features/acid_breath -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 11
targets:
  kind: creature
  count: all
  area:
    shape: line
    origin: actor
    size_ft: 15
    width_ft: 5
failure:
- op: damage
  components:
  - type: acid
    amount:
      dice:
        count: 5
        sides: 8
      add: 0
    average: 22
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
  feature: rend
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
>   value: 60
> swim_speed:
>   unit: ft
>   value: 30
> ```

