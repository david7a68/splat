---
splat_schema: 2
splat_id: a252daaf-d3b9-5f4a-97fc-dcd183b40d3f
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_bats
splat_name: Swarm of Bats
splat_origin: dnd2024:reference/creature/swarm_of_bats
splat_sources:
- SRD 5.2.1, pp. 361, https://www.dndbeyond.com/srd
---

# Swarm of Bats


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large Swarm of Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 11 (2d10) |
| Initiative | +2 (12) |
| Speed | 5 ft., Fly 30 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Senses | Blindsight 60 ft.; Passive Perception 11 |
| Languages | None |
| Resistances | Bludgeoning, Piercing, Slashing |
| Immunities | Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 5 | -3 | -3 |
| Dex | 15 | +2 | +2 |
| Con | 10 | +0 | +0 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 4 | -3 | -3 |


## Source text

<!-- SRD page 361 -->

Large Swarm of Tiny Beasts, Unaligned\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 12 Initiative +2 (12)\
> HP 11 (2d10)\
> Speed 5 ft., Fly 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str  5 −3 −3 Dex 15 +2 +2 Con 10 +0 +0\
> Int 2 −4 −4 Wis 12 +1 +1 Cha  4 −3 −3\
> Resistances Bludgeoning, Piercing, Slashing\
> Immunities Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned\
> Senses Blindsight 60 ft.; Passive Perception 11\
> Languages None\
> CR 1/4 (XP 50; PB +2)


### Traits

Swarm. The swarm can occupy another creature’s space and vice versa, and the swarm can move through any opening large enough for a Tiny bat. The swarm can’t regain Hit Points or gain Temporary Hit Points.

### Actions

Bites. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (2d4) Piercing damage, or 2 (1d4) Piercing damage if the swarm is Bloodied.

Original layout: [[../../srd-5.2.1/pages/361|page 361]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/swarm_of_bats ` |
| ` implementation ` | ` dnd2024:creature_swarm_of_bats ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["bludgeoning", "piercing", "slashing"] ` |
| ` condition_immunities ` | ` ["charmed", "frightened", "grappled", "paralyzed", "petrified", "prone", "restrained", "stunned"] ` |
| ` passive_perception ` | ` 11 ` |

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 60 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Swarm

<!-- splat:yaml path=splat_rule/parameters/features/swarm -->

```yaml
economy: passive
op: swarm.form
shared_spaces_bidirectional: true
minimum_opening:
  creature_size: tiny
  creature_shape: bat
regain_hp: false
gain_temporary_hp: false
```

##### Bites

<!-- splat:yaml path=splat_rule/parameters/features/bites -->

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
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 4
      add: 0
    average: 5
  replacement_when:
    if:
      actor_bloodied: true
    components:
    - type: piercing
      amount:
        dice:
          count: 1
          sides: 4
        add: 0
      average: 2
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

