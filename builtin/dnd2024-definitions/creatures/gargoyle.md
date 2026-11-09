---
splat_schema: 2
splat_id: 9607ffda-788a-5ade-8b84-7afa39d5dfa5
splat_kind: creature
splat_key: dnd2024:creatures/gargoyle
splat_name: Gargoyle
splat_origin: dnd2024:reference/creature/gargoyle
splat_sources:
- SRD 5.2.1, pp. 286, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 128, Habitat
---

# Gargoyle


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Elemental |
| Alignment | Chaotic Evil |
| Armor Class | 15 |
| Hit Points | 67 (9d8 + 27) |
| Initiative | +2 (12) |
| Speed | 30 ft., Fly 60 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Stealth +4 |
| Senses | Darkvision 60 ft.; Passive Perception 10 |
| Languages | Primordial (Terran) |
| Immunities | Poison; Exhaustion, Petrified, Poisoned |
| Habitat | Underdark, Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +2 |
| Dex | 11 | +0 | +0 |
| Con | 16 | +3 | +3 |
| Int | 6 | -2 | -2 |
| Wis | 11 | +0 | +0 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 286 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Elemental, Chaotic Evil\
> AC 15 Initiative +2 (12)\
> HP 67 (9d8 + 27)\
> Speed 30 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 15 +2 +2 Dex 11 +0 +0 Con 16 +3 +3\
> Int 6 −2 −2 Wis 11 +0 +0 Cha 7 −2 −2\
> Skills Stealth +4\
> Immunities Poison; Exhaustion, Petrified, Poisoned\
> Senses Darkvision 60 ft.; Passive Perception 10\
> Languages Primordial (Terran)\
> CR 2 (XP 450; PB +2)


### Traits

Flyby. The gargoyle doesn’t provoke an Opportunity Attack when it flies out of an enemy’s reach.

### Actions

Multiattack. The gargoyle makes two Claw attacks.

Claw. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/286|page 286]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/gargoyle ` |
| ` implementation ` | ` dnd2024:creature_gargoyle ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "petrified", "poisoned"] ` |
| ` passive_perception ` | ` 10 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 4 ` |

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
| ` primordial_dialects ` | ` ["terran"] ` |

#### Features

##### Flyby

<!-- splat:values path=splat_rule/parameters/features/flyby -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` opportunity_attack.suppress ` |
| ` trigger ` | ` actor.leaves_enemy_reach ` |
| ` movement_modes ` | ` ["fly"] ` |

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

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
        count: 2
        sides: 4
      add: 2
    average: 7
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 60
> ```

