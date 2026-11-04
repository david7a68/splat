---
splat_schema: 2
splat_id: cf88db2d-0998-5e7a-b360-41fc7b993a4e
splat_kind: creature
splat_key: dnd2024:creatures/zombie
splat_name: Zombie
splat_origin: dnd2024:reference/creature/zombie
splat_sources:
- SRD 5.2.1, pp. 343, 344, https://www.dndbeyond.com/srd
---

# Zombie


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Undead |
| Alignment | Neutral Evil |
| Armor Class | 8 |
| Hit Points | 15 (2d8 + 6) |
| Initiative | -2 (8) |
| Speed | 20 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 8 |
| Languages | Understands Common plus one other language but can’t speak |
| Immunities | Poison; Exhaustion, Poisoned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 13 | +1 | +1 |
| Dex | 6 | -2 | -2 |
| Con | 16 | +3 | +3 |
| Int | 3 | -4 | -4 |
| Wis | 6 | -2 | +0 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 343 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Undead, Neutral Evil\
> AC 8   Initiative −2 (8)\
> HP 15 (2d8 + 6)\
> Speed 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 13 +1 +1 Dex 6 −2 −2 Con 16 +3 +3\
> Int 3 −4 −4 Wis 6 −2 +0 Cha 5 −3 −3\
> Immunities Poison; Exhaustion, Poisoned\
> Senses Darkvision 60 ft.; Passive Perception 8\
> Languages Understands Common plus one other language but can’t speak\
> CR 1/4 (XP 50; PB +2)


### Traits

Undead Fortitude. If damage reduces the zombie to 0 Hit Points, it makes a Constitution saving throw (DC 5 plus the damage taken) unless the damage is Radiant or from a Critical Hit. On a successful save, the zombie drops to 1 Hit Point instead.

<!-- SRD page 344 -->

### Actions

Slam. Melee Attack Roll: +3, reach 5 ft. Hit: 5 (1d8 + 1) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/343|page 343]], [[../../srd-5.2.1/pages/344|page 344]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/zombie ` |
| ` implementation ` | ` dnd2024:creature_zombie ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "poisoned"] ` |
| ` passive_perception ` | ` 8 ` |

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
| ` known ` | ` ["common"] ` |
| ` additional_choice_count ` | ` 1 ` |

#### Features

##### Undead fortitude

<!-- splat:yaml path=splat_rule/parameters/features/undead_fortitude -->

```yaml
economy: passive
trigger: damage.reduces_actor_to_zero_hp
exclude:
  damage_type: radiant
  critical_hit: true
save:
  ability: constitution
  dc:
    add:
    - 5
    - event_field: damage_taken
  success:
    hp_instead: 1
```

##### Slam

<!-- splat:yaml path=splat_rule/parameters/features/slam -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 3
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 1
        sides: 8
      add: 1
    average: 5
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
